use std::collections::HashSet;

use music_arena::SliceRange;
use music_base::{Span, diag::DiagContext};
use music_hir::{HirArg, HirExprId, HirTyId, HirTyKind};
use music_names::{Ident, Symbol};

use crate::api::ExprFacts;

use super::super::state::DataDef;
use super::super::{CheckPass, DiagKind};
use super::{check_expr, peel_mut_ty};

type ExprIdList = Vec<HirExprId>;
type TyIdList = Vec<HirTyId>;
type VariantFieldNames = [Option<Box<str>>];

impl CheckPass<'_, '_, '_> {
    pub(super) fn check_variant_expr(&mut self, tag: Ident, args: SliceRange<HirArg>) -> ExprFacts {
        let builtins = self.builtins();
        if let Some(facts) = self.check_sum_constructor_variant(tag, args.clone()) {
            return facts;
        }

        let expected_ty = self
            .expected_ty()
            .and_then(|ty| self.variant_context_ty(ty));
        let expected_ty = expected_ty.or_else(|| self.infer_variant_context_ty(tag));
        let Some(expected_ty) = expected_ty else {
            self.check_variant_arg_effects(args);
            return ExprFacts::new(builtins.unknown);
        };

        let data_def = self.expected_data_def(expected_ty);
        let Some(data_def) = data_def else {
            self.check_variant_arg_effects(args);
            self.diag(tag.span, DiagKind::VariantMissingDataContext, "");
            return ExprFacts::new(builtins.unknown);
        };

        let tag_name = self.resolve_symbol(tag.name).to_owned();
        let Some(variant) = data_def.variant(&tag_name) else {
            self.check_variant_arg_effects(args);
            self.diag_with(
                tag.span,
                DiagKind::UnknownDataVariant,
                DiagContext::new().with("variant", tag_name),
            );
            return ExprFacts::new(expected_ty);
        };

        let expected_args = variant.field_tys().to_vec();
        let field_names = variant.field_names().to_vec();
        self.typecheck_variant_args(tag.span, &expected_args, &field_names, args);

        ExprFacts::new(expected_ty)
    }

    fn check_sum_constructor_variant(
        &mut self,
        tag: Ident,
        args: SliceRange<HirArg>,
    ) -> Option<ExprFacts> {
        let builtins = self.builtins();
        let expected_sum_ty = self.expected_ty().and_then(|ty| {
            let inner = peel_mut_ty(self, ty);
            matches!(self.ty(inner).kind, HirTyKind::Sum { .. }).then_some(inner)
        })?;
        let HirTyKind::Sum { left, right } = self.ty(expected_sum_ty).kind else {
            return Some(ExprFacts::new(builtins.unknown));
        };
        let tag_name = self.resolve_symbol(tag.name);
        let chosen = match tag_name {
            "Left" => Some(left),
            "Right" => Some(right),
            _ => None,
        }?;

        let _ = self.ensure_sum_data_def(left, right);
        let arg_exprs = self.args(args);
        if arg_exprs.iter().any(|arg| arg.name.is_some()) {
            self.diag(tag.span, DiagKind::VariantNamedFieldsUnexpected, "");
        }
        let expected_args: TyIdList = match &self.ty(chosen).kind {
            HirTyKind::Tuple { items } => self.ty_ids(*items),
            _ => vec![chosen],
        };
        self.typecheck_positional_args(
            tag.span,
            &expected_args,
            arg_exprs.into_iter().map(|arg| arg.expr).collect(),
            DiagKind::SumConstructorArityMismatch,
        );
        Some(ExprFacts::new(expected_sum_ty))
    }

    fn typecheck_variant_args(
        &mut self,
        diag_span: Span,
        expected_args: &[HirTyId],
        field_names: &VariantFieldNames,
        args: SliceRange<HirArg>,
    ) {
        let arg_nodes = self.args(args);
        let named_variant = field_names.iter().any(Option::is_some);
        let named_args = arg_nodes.iter().any(|arg| arg.name.is_some());
        if named_variant {
            self.typecheck_named_variant_args(
                diag_span,
                expected_args,
                field_names,
                arg_nodes,
                named_args,
            );
        } else {
            self.typecheck_ordinary_variant_args(diag_span, expected_args, arg_nodes, named_args);
        }
    }

    fn typecheck_named_variant_args(
        &mut self,
        diag_span: Span,
        expected_args: &[HirTyId],
        field_names: &VariantFieldNames,
        arg_nodes: Vec<HirArg>,
        named_args: bool,
    ) {
        if !named_args {
            self.diag(diag_span, DiagKind::VariantNamedFieldsRequired, "");
            self.typecheck_positional_variant_args(diag_span, expected_args, arg_nodes);
            return;
        }
        let mut seen = HashSet::<Symbol>::new();
        for arg in &arg_nodes {
            self.typecheck_named_variant_arg(diag_span, expected_args, field_names, arg, &mut seen);
        }
        self.report_missing_variant_fields(diag_span, field_names, &seen);
    }

    fn typecheck_named_variant_arg(
        &mut self,
        diag_span: Span,
        expected_args: &[HirTyId],
        field_names: &VariantFieldNames,
        arg: &HirArg,
        seen: &mut HashSet<Symbol>,
    ) {
        let Some(name) = arg.name else {
            self.diag(diag_span, DiagKind::VariantNamedFieldsRequired, "");
            return;
        };
        self.record_variant_field_name(name, seen);
        let expected = self.expected_variant_field_ty(name, field_names, expected_args);
        self.push_expected_ty(expected);
        let facts = check_expr(self, arg.expr);
        let _ = self.pop_expected_ty();
        let origin = self.expr(arg.expr).origin;
        self.type_mismatch(origin, expected, facts.ty);
    }

    fn record_variant_field_name(&mut self, name: Ident, seen: &mut HashSet<Symbol>) {
        if !seen.insert(name.name) {
            let field_name = self.resolve_symbol(name.name).to_owned();
            self.diag_with(
                name.span,
                DiagKind::DuplicateVariantField,
                DiagContext::new().with("field", field_name),
            );
        }
    }

    fn expected_variant_field_ty(
        &mut self,
        name: Ident,
        field_names: &VariantFieldNames,
        expected_args: &[HirTyId],
    ) -> HirTyId {
        let field_index = field_names
            .iter()
            .position(|field| field.as_deref() == Some(self.resolve_symbol(name.name)));
        field_index
            .and_then(|index| expected_args.get(index).copied())
            .unwrap_or_else(|| self.unknown_variant_field_ty(name))
    }

    fn unknown_variant_field_ty(&mut self, name: Ident) -> HirTyId {
        let field_name = self.resolve_symbol(name.name).to_owned();
        self.diag_with(
            name.span,
            DiagKind::UnknownVariantField,
            DiagContext::new().with("field", field_name),
        );
        self.builtins().unknown
    }

    fn report_missing_variant_fields(
        &mut self,
        diag_span: Span,
        field_names: &VariantFieldNames,
        seen: &HashSet<Symbol>,
    ) {
        for field_name in field_names.iter().flatten() {
            let expected_symbol = self.intern(field_name);
            if !seen.contains(&expected_symbol) {
                self.diag_with(
                    diag_span,
                    DiagKind::MissingVariantField,
                    DiagContext::new().with("field", field_name),
                );
            }
        }
    }

    fn typecheck_ordinary_variant_args(
        &mut self,
        diag_span: Span,
        expected_args: &[HirTyId],
        arg_nodes: Vec<HirArg>,
        named_args: bool,
    ) {
        if named_args {
            self.diag(diag_span, DiagKind::VariantNamedFieldsUnexpected, "");
        }
        self.typecheck_positional_variant_args(diag_span, expected_args, arg_nodes);
    }

    fn typecheck_positional_variant_args(
        &mut self,
        diag_span: Span,
        expected_args: &[HirTyId],
        arg_nodes: Vec<HirArg>,
    ) {
        self.typecheck_positional_args(
            diag_span,
            expected_args,
            arg_nodes.into_iter().map(|arg| arg.expr).collect(),
            DiagKind::VariantConstructorArityMismatch,
        );
    }

    fn check_variant_arg_effects(&mut self, args: SliceRange<HirArg>) {
        for arg in self.args(args) {
            let _ = check_expr(self, arg.expr);
        }
    }

    fn typecheck_positional_args(
        &mut self,
        diag_span: Span,
        expected_args: &[HirTyId],
        arg_exprs: ExprIdList,
        arity_diag: DiagKind,
    ) {
        let builtins = self.builtins();
        if expected_args.len() != arg_exprs.len() {
            self.diag(diag_span, arity_diag, "");
        }
        for (index, arg) in arg_exprs.into_iter().enumerate() {
            let expected = expected_args
                .get(index)
                .copied()
                .unwrap_or(builtins.unknown);
            self.push_expected_ty(expected);
            let facts = super::check_expr(self, arg);
            let _ = self.pop_expected_ty();
            let origin = self.expr(arg).origin;
            self.type_mismatch(origin, expected, facts.ty);
        }
    }

    fn variant_context_ty(&self, ty: HirTyId) -> Option<HirTyId> {
        self.expected_data_def(ty).map(|_| ty)
    }

    fn expected_data_def(&self, ty: HirTyId) -> Option<&DataDef> {
        match self.ty(ty).kind {
            HirTyKind::Bool => self.data_def("Bit"),
            HirTyKind::Named { name, .. } => self.data_def(self.resolve_symbol(name)),
            _ => None,
        }
    }

    fn infer_variant_context_ty(&mut self, tag: Ident) -> Option<HirTyId> {
        let tag_name = self.resolve_symbol(tag.name).to_owned();
        let mut matches = self
            .data_defs()
            .iter()
            .filter_map(|(name, data)| data.variant(&tag_name).is_some().then_some(name.clone()))
            .collect::<Vec<Box<str>>>();

        match matches.len() {
            0 => {
                self.diag_with(
                    tag.span,
                    DiagKind::UnknownDataVariant,
                    DiagContext::new().with("variant", tag_name),
                );
                None
            }
            1 => {
                let data_name = matches.pop()?;
                let name = self.intern(data_name.as_ref());
                let args = self.alloc_ty_list([]);
                Some(self.alloc_ty(HirTyKind::Named { name, args }))
            }
            _ => {
                self.diag(tag.span, DiagKind::AmbiguousVariantTag, "");
                None
            }
        }
    }
}
