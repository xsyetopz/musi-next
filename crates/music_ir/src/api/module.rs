use super::{
    IrCallable, IrDataDef, IrForeignDef, IrGlobal, IrMetaRecord, IrModuleInitPart, IrShapeDef,
};
use music_module::ModuleKey;
use music_sema::{ExportedValue, SurfaceTy};

#[derive(Debug, Clone)]
pub struct IrModule {
    module_key: ModuleKey,
    static_imports: Box<[ModuleKey]>,
    types: Box<[SurfaceTy]>,
    exports: Box<[ExportedValue]>,
    callables: Box<[IrCallable]>,
    globals: Box<[IrGlobal]>,
    init_parts: Box<[IrModuleInitPart]>,
    data_defs: Box<[IrDataDef]>,
    foreigns: Box<[IrForeignDef]>,
    shapes: Box<[IrShapeDef]>,
    meta: Box<[IrMetaRecord]>,
}

#[derive(Debug, Clone)]
pub struct IrModuleParts {
    pub exports: Box<[ExportedValue]>,
    pub callables: Box<[IrCallable]>,
    pub globals: Box<[IrGlobal]>,
    pub init_parts: Box<[IrModuleInitPart]>,
    pub data_defs: Box<[IrDataDef]>,
    pub foreigns: Box<[IrForeignDef]>,
    pub shapes: Box<[IrShapeDef]>,
    pub meta: Box<[IrMetaRecord]>,
}

impl IrModule {
    #[must_use]
    pub fn new(
        module_key: ModuleKey,
        static_imports: Box<[ModuleKey]>,
        types: Box<[SurfaceTy]>,
        parts: IrModuleParts,
    ) -> Self {
        Self {
            module_key,
            static_imports,
            types,
            exports: parts.exports,
            callables: parts.callables,
            globals: parts.globals,
            init_parts: parts.init_parts,
            data_defs: parts.data_defs,
            foreigns: parts.foreigns,
            shapes: parts.shapes,
            meta: parts.meta,
        }
    }

    #[must_use]
    pub const fn module_key(&self) -> &ModuleKey {
        &self.module_key
    }

    #[must_use]
    pub fn static_imports(&self) -> &[ModuleKey] {
        &self.static_imports
    }

    #[must_use]
    pub fn types(&self) -> &[SurfaceTy] {
        &self.types
    }

    #[must_use]
    pub fn exports(&self) -> &[ExportedValue] {
        &self.exports
    }

    #[must_use]
    pub fn callables(&self) -> &[IrCallable] {
        &self.callables
    }

    #[must_use]
    pub fn globals(&self) -> &[IrGlobal] {
        &self.globals
    }

    #[must_use]
    pub fn init_parts(&self) -> &[IrModuleInitPart] {
        &self.init_parts
    }

    #[must_use]
    pub fn data_defs(&self) -> &[IrDataDef] {
        &self.data_defs
    }

    #[must_use]
    pub fn foreigns(&self) -> &[IrForeignDef] {
        &self.foreigns
    }

    #[must_use]
    pub fn shapes(&self) -> &[IrShapeDef] {
        &self.shapes
    }

    #[must_use]
    pub fn meta(&self) -> &[IrMetaRecord] {
        &self.meta
    }

    #[must_use]
    pub fn exported_value(&self, name: &str) -> Option<&ExportedValue> {
        self.exports
            .iter()
            .find(|value| value.name.as_ref() == name)
    }
}
