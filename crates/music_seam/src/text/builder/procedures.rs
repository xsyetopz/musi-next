use super::symbols::{ensure_label, must_get, parse_quoted, parse_symbol, tokenize};
use super::*;

impl TextBuilder {
    pub(crate) fn parse_procedure(&mut self, header: &str, lines: &[&str]) -> AssemblyResult {
        let parts = tokenize(header)?;
        if parts.len() < 4 {
            return Err(text_expected_form(
                ".procedure $Name [params <count>] locals <count> [result [$Type ...]] entry <label> body <id> callconv <kind> visibility <kind> [export] [hot] [cold]",
            ));
        }
        let name = parse_symbol(&parts[1])?;
        let mut params = 0_u16;
        let mut param_tys = Vec::new();
        let mut local_tys = Vec::new();
        let mut idx = 2;
        if parts.get(idx).map(String::as_str) == Some("params") {
            params = must_get(parts.get(idx + 1), "procedure params")?
                .parse()
                .map_err(|_| text_invalid_operand("params count", parts[3].as_str()))?;
            idx += 2;
        }
        if parts.get(idx).map(String::as_str) == Some("param_types") {
            idx += 1;
            (param_tys, idx) = self.parse_procedure_type_list(
                &parts,
                idx,
                "procedure parameter types",
                "procedure parameter type",
            )?;
        }
        if parts.get(idx).map(String::as_str) != Some("locals") {
            return Err(text_expected_form(
                ".procedure $Name [params <count>] [param_types [$Type ...]] locals <count> [local_types [$Type ...]] [result [$Type ...]] entry <label> body <id> callconv <kind> visibility <kind> [export] [hot] [cold]",
            ));
        }
        let locals = must_get(parts.get(idx + 1), "locals count")?
            .parse()
            .map_err(|_| text_invalid_operand("locals count", parts[idx + 1].as_str()))?;
        idx += 2;
        if parts.get(idx).map(String::as_str) == Some("local_types") {
            idx += 1;
            (local_tys, idx) = self.parse_procedure_type_list(
                &parts,
                idx,
                "procedure local types",
                "procedure local type",
            )?;
        }
        let mut result_tys = Vec::new();
        if parts.get(idx).map(String::as_str) == Some("result") {
            idx += 1;
            (result_tys, idx) = self.parse_procedure_type_list(
                &parts,
                idx,
                "procedure result types",
                "procedure result type",
            )?;
        }
        let mut entry_label = 0_u16;
        if parts.get(idx).map(String::as_str) == Some("entry") {
            entry_label = must_get(parts.get(idx + 1), "procedure entry label")?
                .parse()
                .map_err(|_| {
                    text_invalid_operand("procedure entry label", parts[idx + 1].as_str())
                })?;
            idx += 2;
        }
        let mut bytecode_body = 0_u32;
        if parts.get(idx).map(String::as_str) == Some("body") {
            bytecode_body = must_get(parts.get(idx + 1), "procedure bytecode body")?
                .parse()
                .map_err(|_| {
                    text_invalid_operand("procedure bytecode body", parts[idx + 1].as_str())
                })?;
            idx += 2;
        }
        let mut block_signature_table = None;
        if parts.get(idx).map(String::as_str) == Some("block_table") {
            let raw = must_get(parts.get(idx + 1), "procedure block table")?
                .parse()
                .map_err(|_| {
                    text_invalid_operand("procedure block table", parts[idx + 1].as_str())
                })?;
            block_signature_table = Some(Idx::from_raw(raw));
            idx += 2;
        }
        let mut root_map_table = None;
        if parts.get(idx).map(String::as_str) == Some("root_map") {
            let raw = must_get(parts.get(idx + 1), "procedure root map")?
                .parse()
                .map_err(|_| text_invalid_operand("procedure root map", parts[idx + 1].as_str()))?;
            root_map_table = Some(Idx::from_raw(raw));
            idx += 2;
        }
        let mut domain_requirements = Vec::new();
        if parts.get(idx).map(String::as_str) == Some("domains") {
            idx += 1;
            let Some(first_token) = parts.get(idx).map(String::as_str) else {
                return Err(text_missing_operand("procedure domains"));
            };
            if first_token != "[" {
                return Err(text_expected_form("domains [\"domain\" ...]"));
            }
            idx += 1;
            loop {
                let Some(token) = parts.get(idx) else {
                    return Err(text_missing_operand("procedure domains"));
                };
                if token == "]" {
                    idx += 1;
                    break;
                }
                domain_requirements.push(self.intern_string(&parse_quoted(token)?));
                idx += 1;
            }
            if domain_requirements.is_empty() {
                return Err(text_missing_operand("procedure domain"));
            }
        }
        let mut calling_convention = ProcedureCallingConvention::Managed;
        if parts.get(idx).map(String::as_str) == Some("callconv") {
            let token = must_get(parts.get(idx + 1), "procedure calling convention")?;
            calling_convention = ProcedureCallingConvention::from_str(token)
                .ok_or_else(|| text_invalid_operand("procedure calling convention", token))?;
            idx += 2;
        }
        let mut visibility = ProcedureVisibility::Private;
        if parts.get(idx).map(String::as_str) == Some("visibility") {
            let token = must_get(parts.get(idx + 1), "procedure visibility")?;
            visibility = ProcedureVisibility::from_str(token)
                .ok_or_else(|| text_invalid_operand("procedure visibility", token))?;
            idx += 2;
        }
        let mut export = false;
        let mut hot = false;
        let mut cold = false;
        for token in parts.iter().skip(idx) {
            match token.as_str() {
                "export" => export = true,
                "hot" => hot = true,
                "cold" => cold = true,
                _ => return Err(text_invalid_operand("procedure flag", token.as_str())),
            }
        }

        let mut labels = Vec::<StringId>::new();
        let mut label_ids = HashMap::<String, u16>::new();
        let mut code = Vec::<CodeEntry>::new();
        for raw_line in lines {
            if raw_line.is_empty() {
                continue;
            }
            if let Some(label_name) = raw_line.strip_suffix(':') {
                let label_id = ensure_label(
                    &mut self.artifact,
                    &mut labels,
                    &mut label_ids,
                    String::from(label_name),
                )?;
                code.push(CodeEntry::Label(Label { id: label_id }));
                continue;
            }
            let entry = self.parse_instruction(raw_line, &mut labels, &mut label_ids)?;
            code.push(CodeEntry::Instruction(entry));
        }

        let procedure = ProcedureDescriptor::new(
            self.intern_string(&name),
            params,
            locals,
            code.into_boxed_slice(),
        )
        .with_param_tys(param_tys.into_boxed_slice())
        .with_local_tys(local_tys.into_boxed_slice())
        .with_result_tys(result_tys.into_boxed_slice())
        .with_entry_label(entry_label)
        .with_bytecode_body(bytecode_body)
        .with_domain_requirements(domain_requirements.into_boxed_slice())
        .with_calling_convention(calling_convention)
        .with_visibility(visibility)
        .with_export(export)
        .with_hot(hot)
        .with_cold(cold)
        .with_labels(labels.into_boxed_slice());
        let mut procedure = procedure;
        if let Some(block_signature_table) = block_signature_table {
            procedure = procedure.with_block_signature_table(block_signature_table);
        }
        if let Some(root_map_table) = root_map_table {
            procedure = procedure.with_root_map_table(root_map_table);
        }
        let id = self.ensure_procedure_symbol(&name);
        *self.artifact.procedures.get_mut(id) = procedure;
        Ok(())
    }

    fn parse_procedure_type_list(
        &mut self,
        parts: &[String],
        mut idx: usize,
        missing_list: &'static str,
        missing_item: &'static str,
    ) -> AssemblyResult<(Vec<TypeId>, usize)> {
        let Some(first_token) = parts.get(idx).map(String::as_str) else {
            return Err(text_missing_operand(missing_list));
        };
        let mut out = Vec::new();
        let mut opened_inline = false;
        if first_token == "[" {
            idx += 1;
        } else if let Some(inline) = first_token.strip_prefix('[') {
            opened_inline = true;
            if let Some(single) = inline.strip_suffix(']') {
                if single.is_empty() {
                    return Err(text_missing_operand(missing_item));
                }
                let type_name = parse_symbol(single)?;
                out.push(self.ensure_type_symbol(&type_name, &type_name));
                idx += 1;
            } else if inline.is_empty() {
                idx += 1;
            } else {
                let type_name = parse_symbol(inline)?;
                out.push(self.ensure_type_symbol(&type_name, &type_name));
                idx += 1;
            }
        } else {
            return Err(text_expected_form(
                ".procedure $Name [params <count>] [param_types [$Type ...]] locals <count> [result [$Type ...]] [export] [hot] [cold]",
            ));
        }
        if !opened_inline && parts.get(idx).map(String::as_str) == Some("]") {
            return Err(text_missing_operand(missing_item));
        }
        if !(opened_inline
            && parts
                .get(idx.saturating_sub(1))
                .is_some_and(|token| token.ends_with(']')))
        {
            loop {
                let Some(token) = parts.get(idx) else {
                    return Err(text_missing_operand(missing_list));
                };
                if token == "]" {
                    idx += 1;
                    break;
                }
                if let Some(last_type) = token.strip_suffix(']') {
                    if last_type.is_empty() {
                        return Err(text_missing_operand(missing_item));
                    }
                    let type_name = parse_symbol(last_type)?;
                    out.push(self.ensure_type_symbol(&type_name, &type_name));
                    idx += 1;
                    break;
                }
                let type_name = parse_symbol(token)?;
                out.push(self.ensure_type_symbol(&type_name, &type_name));
                idx += 1;
            }
        }
        if out.is_empty() {
            return Err(text_missing_operand(missing_item));
        }
        Ok((out, idx))
    }

    fn parse_instruction(
        &mut self,
        line: &str,
        labels: &mut Vec<StringId>,
        label_ids: &mut LabelIdMap,
    ) -> AssemblyResult<Instruction> {
        let parts = tokenize(line)?;
        let Some(opcode_text) = parts.first() else {
            return Err(text_missing_operand("opcode"));
        };
        let Some(opcode) = Opcode::from_mnemonic(opcode_text) else {
            return Err(text_unknown_opcode(opcode_text));
        };
        let operand = self.parse_operand(opcode.operand_shape(), &parts, labels, label_ids)?;
        Ok(Instruction::new(opcode, operand))
    }
}
