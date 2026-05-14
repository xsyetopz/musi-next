use crate::artifact::StringId;
use crate::artifact::TypeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectHeaderDescriptor {
    pub layout_ty: Option<TypeId>,
    pub mark_bits: u8,
    pub generation_bits: u8,
    pub pinned: bool,
    pub remembered: bool,
    pub large: bool,
    pub weak_capable: bool,
    pub forwarding: bool,
    pub size_field: bool,
}

impl ObjectHeaderDescriptor {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            layout_ty: None,
            mark_bits: 0,
            generation_bits: 0,
            pinned: false,
            remembered: false,
            large: false,
            weak_capable: false,
            forwarding: false,
            size_field: false,
        }
    }

    #[must_use]
    pub const fn with_layout_ty(mut self, layout_ty: TypeId) -> Self {
        self.layout_ty = Some(layout_ty);
        self
    }

    #[must_use]
    pub const fn with_mark_bits(mut self, mark_bits: u8) -> Self {
        self.mark_bits = mark_bits;
        self
    }

    #[must_use]
    pub const fn with_generation_bits(mut self, generation_bits: u8) -> Self {
        self.generation_bits = generation_bits;
        self
    }

    #[must_use]
    pub const fn with_pinned(mut self, pinned: bool) -> Self {
        self.pinned = pinned;
        self
    }

    #[must_use]
    pub const fn with_remembered(mut self, remembered: bool) -> Self {
        self.remembered = remembered;
        self
    }

    #[must_use]
    pub const fn with_large(mut self, large: bool) -> Self {
        self.large = large;
        self
    }

    #[must_use]
    pub const fn with_weak_capable(mut self, weak_capable: bool) -> Self {
        self.weak_capable = weak_capable;
        self
    }

    #[must_use]
    pub const fn with_forwarding(mut self, forwarding: bool) -> Self {
        self.forwarding = forwarding;
        self
    }

    #[must_use]
    pub const fn with_size_field(mut self, size_field: bool) -> Self {
        self.size_field = size_field;
        self
    }
}

impl Default for ObjectHeaderDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFieldDescriptor {
    pub name: Option<StringId>,
    pub ty: TypeId,
    pub logical_index: u32,
    pub offset: Option<u32>,
    pub storage: Option<StringId>,
    pub mutable: bool,
    pub gc_pointer: bool,
    pub public: bool,
    pub hidden: bool,
}

impl DataFieldDescriptor {
    #[must_use]
    pub const fn new(ty: TypeId, logical_index: u32) -> Self {
        Self {
            name: None,
            ty,
            logical_index,
            offset: None,
            storage: None,
            mutable: false,
            gc_pointer: false,
            public: false,
            hidden: false,
        }
    }

    #[must_use]
    pub const fn with_name(mut self, name: StringId) -> Self {
        self.name = Some(name);
        self
    }

    #[must_use]
    pub const fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[must_use]
    pub const fn with_storage(mut self, storage: StringId) -> Self {
        self.storage = Some(storage);
        self
    }

    #[must_use]
    pub const fn with_mutable(mut self, mutable: bool) -> Self {
        self.mutable = mutable;
        self
    }

    #[must_use]
    pub const fn with_gc_pointer(mut self, gc_pointer: bool) -> Self {
        self.gc_pointer = gc_pointer;
        self
    }

    #[must_use]
    pub const fn with_public(mut self, public: bool) -> Self {
        self.public = public;
        self
    }

    #[must_use]
    pub const fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataVariantDescriptor {
    pub name: StringId,
    pub tag: i64,
    pub field_tys: Box<[TypeId]>,
    pub layout_fields: Box<[DataFieldDescriptor]>,
    pub public: bool,
    pub hidden: bool,
}

impl DataVariantDescriptor {
    #[must_use]
    pub fn new(name: StringId, tag: i64, field_tys: Box<[TypeId]>) -> Self {
        Self {
            name,
            tag,
            field_tys,
            layout_fields: Box::new([]),
            public: false,
            hidden: false,
        }
    }

    #[must_use]
    pub fn with_layout_fields(mut self, layout_fields: Box<[DataFieldDescriptor]>) -> Self {
        self.layout_fields = layout_fields;
        self
    }

    #[must_use]
    pub const fn with_public(mut self, public: bool) -> Self {
        self.public = public;
        self
    }

    #[must_use]
    pub const fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataDescriptor {
    pub name: StringId,
    pub variant_count: u32,
    pub field_count: u32,
    pub variants: Box<[DataVariantDescriptor]>,
    pub repr_kind: Option<StringId>,
    pub layout_align: Option<u32>,
    pub layout_pack: Option<u32>,
    pub frozen: bool,
    pub object_header: Option<ObjectHeaderDescriptor>,
}

impl DataDescriptor {
    /// # Panics
    ///
    /// Panics if the variant count or total field count does not fit in `u32`.
    #[must_use]
    pub fn new(name: StringId, variants: Box<[DataVariantDescriptor]>) -> Self {
        let variant_count =
            u32::try_from(variants.len()).expect("data variant count should fit in u32");
        let field_count = variants
            .iter()
            .map(|variant| variant.field_tys.len())
            .sum::<usize>();
        let field_count = u32::try_from(field_count).expect("data field count should fit in u32");
        Self {
            name,
            variant_count,
            field_count,
            variants,
            repr_kind: None,
            layout_align: None,
            layout_pack: None,
            frozen: false,
            object_header: None,
        }
    }

    #[must_use]
    pub const fn with_repr_kind(mut self, repr_kind: StringId) -> Self {
        self.repr_kind = Some(repr_kind);
        self
    }

    #[must_use]
    pub const fn with_layout_align(mut self, layout_align: u32) -> Self {
        self.layout_align = Some(layout_align);
        self
    }

    #[must_use]
    pub const fn with_layout_pack(mut self, layout_pack: u32) -> Self {
        self.layout_pack = Some(layout_pack);
        self
    }

    #[must_use]
    pub const fn with_frozen(mut self, frozen: bool) -> Self {
        self.frozen = frozen;
        self
    }

    #[must_use]
    pub const fn with_object_header(mut self, object_header: ObjectHeaderDescriptor) -> Self {
        self.object_header = Some(object_header);
        self
    }
}
