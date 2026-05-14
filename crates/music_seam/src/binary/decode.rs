use super::*;

/// Decodes a sectioned `.seam` byte stream into a validated SEAM artifact.
///
/// # Errors
///
/// Returns [`AssemblyError`] if the header, sections, payload lengths, opcodes, or references are
/// invalid.
pub fn decode_binary(bytes: &[u8]) -> AssemblyResult<Artifact> {
    let mut cursor = Cursor::new(bytes);
    let magic = cursor.read_exact(4)?;
    if magic != SEAM_MAGIC {
        return Err(AssemblyError::InvalidBinaryHeader);
    }
    let major = cursor.read_u16()?;
    let minor = cursor.read_u16()?;
    if major != BINARY_MAJOR_VERSION || minor != BINARY_MINOR_VERSION {
        let version = (u32::from(major) << 16) | u32::from(minor);
        return Err(AssemblyError::UnsupportedBinaryVersion(version));
    }

    let mut artifact = Artifact::new();
    decode_strings(&mut cursor, &mut artifact)?;
    decode_types(&mut cursor, &mut artifact)?;
    decode_constants(&mut cursor, &mut artifact)?;
    decode_globals(&mut cursor, &mut artifact)?;
    decode_procedures(&mut cursor, &mut artifact)?;
    decode_shapes(&mut cursor, &mut artifact)?;
    decode_foreigns(&mut cursor, &mut artifact)?;
    decode_exports(&mut cursor, &mut artifact)?;
    decode_data(&mut cursor, &mut artifact)?;
    while !cursor.is_eof() {
        let next = cursor
            .peek_u8()
            .ok_or(AssemblyError::BinaryPayloadTruncated)?;
        if next == section_tag_byte(SectionTag::StackEffects) {
            decode_stack_effects(&mut cursor, &mut artifact)?;
        } else if next == section_tag_byte(SectionTag::RootMaps) {
            decode_root_maps(&mut cursor, &mut artifact)?;
        } else if next == section_tag_byte(SectionTag::BlockSignatures) {
            decode_block_signatures(&mut cursor, &mut artifact)?;
        } else if next == section_tag_byte(SectionTag::Closures) {
            decode_closures(&mut cursor, &mut artifact)?;
        } else if next == section_tag_byte(SectionTag::Meta) {
            decode_meta(&mut cursor, &mut artifact)?;
        } else if next == section_tag_byte(SectionTag::Manifest) {
            decode_manifest(&mut cursor, &mut artifact)?;
        } else if next == section_tag_byte(SectionTag::Imports) {
            decode_imports(&mut cursor, &mut artifact)?;
        } else {
            return Err(AssemblyError::text_parse_source("unknown trailing section"));
        }
    }
    artifact.validate()?;
    Ok(artifact)
}

/// Validates a `.seam` binary blob by decoding and checking the resulting artifact.
///
/// # Errors
///
/// Returns [`AssemblyError`] if decoding or artifact validation fails.
pub fn validate_binary(bytes: &[u8]) -> AssemblyResult {
    let _ = decode_binary(bytes)?;
    Ok(())
}

fn decode_strings(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Strings)?;
    for _ in 0..cursor.read_u32()? {
        let bytes = cursor.read_bytes()?;
        let text = String::from_utf8(bytes).map_err(AssemblyError::text_parse_source)?;
        let _ = artifact.push_string_record(&text);
    }
    Ok(())
}

fn decode_types(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Types)?;
    for _ in 0..cursor.read_u32()? {
        let name = cursor.read_idx()?;
        let term = cursor.read_idx()?;
        let _ = artifact.types.alloc(TypeDescriptor::new(name, term));
    }
    Ok(())
}

fn decode_constants(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Constants)?;
    for _ in 0..cursor.read_u32()? {
        let name = cursor.read_idx()?;
        let kind = cursor.read_u8()?;
        let constant_value = match kind {
            0 => ConstantValue::Int(cursor.read_i64()?),
            1 => ConstantValue::Float(f64::from_bits(cursor.read_u64()?)),
            2 => ConstantValue::Bool(cursor.read_u8()? != 0),
            3 => ConstantValue::String(cursor.read_idx()?),
            4 => ConstantValue::Syntax {
                shape: match cursor.read_u8()? {
                    0 => SyntaxShape::Expr,
                    1 => SyntaxShape::Module,
                    _ => {
                        return Err(AssemblyError::text_parse_source("unknown syntax shape"));
                    }
                },
                text: cursor.read_idx()?,
            },
            _ => {
                return Err(AssemblyError::text_parse_source("unknown constant kind"));
            }
        };
        let _ = artifact
            .constants
            .alloc(ConstantDescriptor::new(name, constant_value));
    }
    Ok(())
}

fn decode_globals(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Globals)?;
    for _ in 0..cursor.read_u32()? {
        let name = cursor.read_idx()?;
        let export = cursor.read_u8()? != 0;
        let initializer = if cursor.read_u8()? == 0 {
            None
        } else {
            Some(cursor.read_idx()?)
        };
        let mut descriptor = GlobalDescriptor::new(name).with_export(export);
        if let Some(initializer) = initializer {
            descriptor = descriptor.with_initializer(initializer);
        }
        let _ = artifact.globals.alloc(descriptor);
    }
    Ok(())
}

fn decode_procedures(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Procedures)?;
    for _ in 0..cursor.read_u32()? {
        let name = cursor.read_idx()?;
        let params = cursor.read_u16()?;
        let locals = cursor.read_u16()?;
        let param_ty_count = usize::from(cursor.read_u16()?);
        let mut param_tys = Vec::with_capacity(param_ty_count);
        for _ in 0..param_ty_count {
            param_tys.push(cursor.read_idx()?);
        }
        let local_ty_count = usize::from(cursor.read_u16()?);
        let mut local_tys = Vec::with_capacity(local_ty_count);
        for _ in 0..local_ty_count {
            local_tys.push(cursor.read_idx()?);
        }
        let result_ty_count = usize::from(cursor.read_u16()?);
        let mut result_tys = Vec::with_capacity(result_ty_count);
        for _ in 0..result_ty_count {
            result_tys.push(cursor.read_idx()?);
        }
        let entry_label = cursor.read_u16()?;
        let bytecode_body = cursor.read_u32()?;
        let block_signature_table = match cursor.read_u8()? {
            0 => None,
            1 => Some(Idx::from_raw(cursor.read_u32()?)),
            _ => {
                return Err(AssemblyError::text_parse_source(
                    "invalid procedure block table marker",
                ));
            }
        };
        let root_map_table = match cursor.read_u8()? {
            0 => None,
            1 => Some(Idx::from_raw(cursor.read_u32()?)),
            _ => {
                return Err(AssemblyError::text_parse_source(
                    "invalid procedure root map marker",
                ));
            }
        };
        let domain_count = usize::from(cursor.read_u16()?);
        let mut domain_requirements = Vec::with_capacity(domain_count);
        for _ in 0..domain_count {
            domain_requirements.push(cursor.read_idx()?);
        }
        let Some(calling_convention) = ProcedureCallingConvention::from_wire(cursor.read_u8()?)
        else {
            return Err(AssemblyError::text_parse_source(
                "unknown procedure calling convention",
            ));
        };
        let Some(visibility) = ProcedureVisibility::from_wire(cursor.read_u8()?) else {
            return Err(AssemblyError::text_parse_source(
                "unknown procedure visibility",
            ));
        };
        let export = cursor.read_u8()? != 0;
        let hot = cursor.read_u8()? != 0;
        let cold = cursor.read_u8()? != 0;
        let label_count = usize::from(cursor.read_u16()?);
        let mut labels = Vec::with_capacity(label_count);
        for _ in 0..label_count {
            labels.push(cursor.read_idx()?);
        }
        let code_count = read_len(cursor, "code entry count")?;
        let mut code = Vec::with_capacity(code_count);
        for _ in 0..code_count {
            let kind = cursor.read_u8()?;
            let entry = match kind {
                0 => CodeEntry::Label(Label {
                    id: cursor.read_u16()?,
                }),
                1 => {
                    let opcode_code = decode_opcode(cursor)?;
                    let Some(opcode) = Opcode::from_wire_code(opcode_code) else {
                        return Err(AssemblyError::UnknownOpcode(opcode_code));
                    };
                    let operand = decode_operand(cursor)?;
                    CodeEntry::Instruction(Instruction::new(opcode, operand))
                }
                _ => {
                    return Err(AssemblyError::text_parse_source("unknown code entry kind"));
                }
            };
            code.push(entry);
        }
        let mut descriptor =
            ProcedureDescriptor::new(name, params, locals, code.into_boxed_slice())
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
        if let Some(block_signature_table) = block_signature_table {
            descriptor = descriptor.with_block_signature_table(block_signature_table);
        }
        if let Some(root_map_table) = root_map_table {
            descriptor = descriptor.with_root_map_table(root_map_table);
        }
        let _ = artifact.procedures.alloc(descriptor);
    }
    Ok(())
}

fn decode_shapes(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Shapes)?;
    for _ in 0..cursor.read_u32()? {
        let name = cursor.read_idx()?;
        let payload_ty = if cursor.read_u8()? != 0 {
            Some(cursor.read_idx()?)
        } else {
            None
        };
        let witness = if cursor.read_u8()? != 0 {
            Some(cursor.read_idx()?)
        } else {
            None
        };
        let dispatch_table = if cursor.read_u8()? != 0 {
            Some(cursor.read_idx()?)
        } else {
            None
        };
        let layout_identity = if cursor.read_u8()? != 0 {
            Some(cursor.read_idx()?)
        } else {
            None
        };
        let mut descriptor = ShapeDescriptor::new(name).with_root_visible(cursor.read_u8()? != 0);
        if let Some(payload_ty) = payload_ty {
            descriptor = descriptor.with_payload_ty(payload_ty);
        }
        if let Some(witness) = witness {
            descriptor = descriptor.with_witness(witness);
        }
        if let Some(dispatch_table) = dispatch_table {
            descriptor = descriptor.with_dispatch_table(dispatch_table);
        }
        if let Some(layout_identity) = layout_identity {
            descriptor = descriptor.with_layout_identity(layout_identity);
        }
        let _ = artifact.shapes.alloc(descriptor);
    }
    Ok(())
}

fn decode_foreigns(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Foreigns)?;
    for _ in 0..cursor.read_u32()? {
        let name = cursor.read_idx()?;
        let param_len = usize::from(cursor.read_u16()?);
        let mut param_tys = Vec::with_capacity(param_len);
        for _ in 0..param_len {
            param_tys.push(cursor.read_idx()?);
        }
        let result_ty = cursor.read_idx()?;
        let abi = cursor.read_idx()?;
        let symbol = cursor.read_idx()?;
        let link = match cursor.read_u8()? {
            0 => None,
            1 => Some(cursor.read_idx()?),
            _ => {
                return Err(AssemblyError::text_parse_source(
                    "invalid foreign link marker",
                ));
            }
        };
        let domain = match cursor.read_u8()? {
            0 => None,
            1 => Some(cursor.read_idx()?),
            _ => {
                return Err(AssemblyError::text_parse_source(
                    "invalid foreign domain marker",
                ));
            }
        };
        let pinned_len = usize::from(cursor.read_u16()?);
        let mut pinned_params = Vec::with_capacity(pinned_len);
        for _ in 0..pinned_len {
            pinned_params.push(cursor.read_u16()?);
        }
        let nullable_len = usize::from(cursor.read_u16()?);
        let mut nullable_params = Vec::with_capacity(nullable_len);
        for _ in 0..nullable_len {
            nullable_params.push(cursor.read_u16()?);
        }
        let nullable_result = cursor.read_u8()? != 0;
        let lifetime = match cursor.read_u8()? {
            0 => None,
            1 => Some(cursor.read_idx()?),
            _ => {
                return Err(AssemblyError::text_parse_source(
                    "invalid foreign lifetime marker",
                ));
            }
        };
        let mut descriptor =
            ForeignDescriptor::new(name, param_tys.into_boxed_slice(), result_ty, abi, symbol)
                .with_pinned_params(pinned_params.into_boxed_slice())
                .with_nullable_params(nullable_params.into_boxed_slice())
                .with_nullable_result(nullable_result)
                .with_export(cursor.read_u8()? != 0)
                .with_hot(cursor.read_u8()? != 0)
                .with_cold(cursor.read_u8()? != 0);
        if let Some(link) = link {
            descriptor = descriptor.with_link(link);
        }
        if let Some(domain) = domain {
            descriptor = descriptor.with_domain(domain);
        }
        if let Some(lifetime) = lifetime {
            descriptor = descriptor.with_lifetime(lifetime);
        }
        let _ = artifact.foreigns.alloc(descriptor);
    }
    Ok(())
}

fn decode_exports(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Exports)?;
    let count = cursor.read_u32()?;
    for _ in 0..count {
        let name = Idx::from_raw(cursor.read_u32()?);
        let kind = cursor.read_u8()?;
        let target_raw = cursor.read_u32()?;
        let target = match kind {
            0 => ExportTarget::Procedure(Idx::from_raw(target_raw)),
            1 => ExportTarget::Global(Idx::from_raw(target_raw)),
            2 => ExportTarget::Foreign(Idx::from_raw(target_raw)),
            3 => ExportTarget::Type(Idx::from_raw(target_raw)),
            4 => ExportTarget::Shape(Idx::from_raw(target_raw)),
            _ => return Err(AssemblyError::InvalidBinaryHeader),
        };
        let opaque = cursor.read_u8()? != 0;
        let _ = artifact
            .exports
            .alloc(ExportDescriptor::new(name, opaque, target));
    }
    Ok(())
}

fn decode_data(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Data)?;
    let count = cursor.read_u32()?;
    for _ in 0..count {
        let name = Idx::from_raw(cursor.read_u32()?);
        let variant_count = cursor.read_u32()?;
        let field_count = cursor.read_u32()?;
        let variant_len = cursor.read_u32()?;
        let mut variants = Vec::with_capacity(usize::try_from(variant_len).unwrap_or(usize::MAX));
        for _ in 0..variant_len {
            let variant_name = Idx::from_raw(cursor.read_u32()?);
            let tag = cursor.read_i64()?;
            let field_len = cursor.read_u32()?;
            let mut field_tys =
                Vec::with_capacity(usize::try_from(field_len).unwrap_or(usize::MAX));
            for _ in 0..field_len {
                field_tys.push(Idx::from_raw(cursor.read_u32()?));
            }
            let layout_field_len = cursor.read_u32()?;
            let mut layout_fields =
                Vec::with_capacity(usize::try_from(layout_field_len).unwrap_or(usize::MAX));
            for _ in 0..layout_field_len {
                let name = if cursor.read_u8()? != 0 {
                    Some(cursor.read_idx()?)
                } else {
                    None
                };
                let ty = cursor.read_idx()?;
                let logical_index = cursor.read_u32()?;
                let offset = if cursor.read_u8()? != 0 {
                    Some(cursor.read_u32()?)
                } else {
                    None
                };
                let storage = if cursor.read_u8()? != 0 {
                    Some(cursor.read_idx()?)
                } else {
                    None
                };
                let mut field = DataFieldDescriptor::new(ty, logical_index);
                if let Some(name) = name {
                    field = field.with_name(name);
                }
                if let Some(offset) = offset {
                    field = field.with_offset(offset);
                }
                if let Some(storage) = storage {
                    field = field.with_storage(storage);
                }
                field = field
                    .with_mutable(cursor.read_u8()? != 0)
                    .with_gc_pointer(cursor.read_u8()? != 0)
                    .with_public(cursor.read_u8()? != 0)
                    .with_hidden(cursor.read_u8()? != 0);
                layout_fields.push(field);
            }
            variants.push(
                DataVariantDescriptor::new(variant_name, tag, field_tys.into_boxed_slice())
                    .with_layout_fields(layout_fields.into_boxed_slice())
                    .with_public(cursor.read_u8()? != 0)
                    .with_hidden(cursor.read_u8()? != 0),
            );
        }
        let repr_kind = if cursor.read_u8()? != 0 {
            Some(Idx::from_raw(cursor.read_u32()?))
        } else {
            None
        };
        let layout_align = if cursor.read_u8()? != 0 {
            Some(cursor.read_u32()?)
        } else {
            None
        };
        let layout_pack = if cursor.read_u8()? != 0 {
            Some(cursor.read_u32()?)
        } else {
            None
        };
        let frozen = cursor.read_u8()? != 0;
        let mut descriptor = DataDescriptor::new(name, variants.into_boxed_slice());
        debug_assert_eq!(descriptor.variant_count, variant_count);
        debug_assert_eq!(descriptor.field_count, field_count);
        if let Some(repr_kind) = repr_kind {
            descriptor = descriptor.with_repr_kind(repr_kind);
        }
        if let Some(layout_align) = layout_align {
            descriptor = descriptor.with_layout_align(layout_align);
        }
        if let Some(layout_pack) = layout_pack {
            descriptor = descriptor.with_layout_pack(layout_pack);
        }
        descriptor = descriptor.with_frozen(frozen);
        if cursor.read_u8()? != 0 {
            let layout_ty = if cursor.read_u8()? != 0 {
                Some(cursor.read_idx()?)
            } else {
                None
            };
            let mut header = ObjectHeaderDescriptor::new()
                .with_mark_bits(cursor.read_u8()?)
                .with_generation_bits(cursor.read_u8()?)
                .with_pinned(cursor.read_u8()? != 0)
                .with_remembered(cursor.read_u8()? != 0)
                .with_large(cursor.read_u8()? != 0)
                .with_weak_capable(cursor.read_u8()? != 0)
                .with_forwarding(cursor.read_u8()? != 0)
                .with_size_field(cursor.read_u8()? != 0);
            if let Some(layout_ty) = layout_ty {
                header = header.with_layout_ty(layout_ty);
            }
            descriptor = descriptor.with_object_header(header);
        }
        let _ = artifact.data.alloc(descriptor);
    }
    Ok(())
}

fn decode_meta(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Meta)?;
    for _ in 0..cursor.read_u32()? {
        let target = cursor.read_idx()?;
        let key = cursor.read_idx()?;
        let value_len = usize::from(cursor.read_u16()?);
        let mut values = Vec::with_capacity(value_len);
        for _ in 0..value_len {
            values.push(cursor.read_idx()?);
        }
        let _ = artifact
            .meta
            .alloc(MetaDescriptor::new(target, key, values.into_boxed_slice()));
    }
    Ok(())
}

fn decode_manifest(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Manifest)?;
    for _ in 0..cursor.read_u32()? {
        let package = cursor.read_idx()?;
        let version = cursor.read_idx()?;
        let profile = cursor.read_idx()?;
        let mut descriptor = ManifestDescriptor::new(package, version, profile);
        match cursor.read_u8()? {
            0 => {}
            1 => descriptor = descriptor.with_entry(cursor.read_idx()?),
            _ => {
                return Err(AssemblyError::text_parse_source(
                    "invalid manifest entry marker",
                ));
            }
        }
        let _ = artifact.manifest.alloc(descriptor);
    }
    Ok(())
}

fn decode_imports(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Imports)?;
    for _ in 0..cursor.read_u32()? {
        let spec = cursor.read_idx()?;
        let resolved = cursor.read_idx()?;
        let _ = artifact
            .imports
            .alloc(ImportDescriptor::new(spec, resolved));
    }
    Ok(())
}

fn decode_stack_effects(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::StackEffects)?;
    for _ in 0..cursor.read_u32()? {
        let name = cursor.read_idx()?;
        let input_len = usize::from(cursor.read_u16()?);
        let mut input_tys = Vec::with_capacity(input_len);
        for _ in 0..input_len {
            input_tys.push(cursor.read_idx()?);
        }
        let output_len = usize::from(cursor.read_u16()?);
        let mut output_tys = Vec::with_capacity(output_len);
        for _ in 0..output_len {
            output_tys.push(cursor.read_idx()?);
        }
        let _ = artifact.stack_effects.alloc(StackEffectDescriptor::new(
            name,
            input_tys.into_boxed_slice(),
            output_tys.into_boxed_slice(),
        ));
    }
    Ok(())
}

fn decode_root_maps(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::RootMaps)?;
    for _ in 0..cursor.read_u32()? {
        let safe_point = cursor.read_idx()?;
        let Some(kind) = SafePointKind::from_wire(cursor.read_u8()?) else {
            return Err(AssemblyError::text_parse_source("unknown safe point kind"));
        };
        let procedure = match cursor.read_u8()? {
            0 => None,
            1 => Some(cursor.read_idx()?),
            _ => {
                return Err(AssemblyError::text_parse_source(
                    "invalid root-map procedure marker",
                ));
            }
        };
        let local_len = usize::from(cursor.read_u16()?);
        let mut local_slots = Vec::with_capacity(local_len);
        for _ in 0..local_len {
            local_slots.push(cursor.read_u16()?);
        }
        let stack_len = usize::from(cursor.read_u16()?);
        let mut stack_slots = Vec::with_capacity(stack_len);
        for _ in 0..stack_len {
            stack_slots.push(cursor.read_u16()?);
        }
        let capture_len = usize::from(cursor.read_u16()?);
        let mut capture_slots = Vec::with_capacity(capture_len);
        for _ in 0..capture_len {
            capture_slots.push(cursor.read_u16()?);
        }
        let defer_len = usize::from(cursor.read_u16()?);
        let mut defer_slots = Vec::with_capacity(defer_len);
        for _ in 0..defer_len {
            defer_slots.push(cursor.read_u16()?);
        }
        let pin_len = usize::from(cursor.read_u16()?);
        let mut pin_slots = Vec::with_capacity(pin_len);
        for _ in 0..pin_len {
            pin_slots.push(cursor.read_u16()?);
        }
        let mut descriptor = RootMapDescriptor::new(
            safe_point,
            local_slots.into_boxed_slice(),
            stack_slots.into_boxed_slice(),
        )
        .with_kind(kind)
        .with_capture_slots(capture_slots.into_boxed_slice())
        .with_defer_slots(defer_slots.into_boxed_slice())
        .with_pin_slots(pin_slots.into_boxed_slice());
        if let Some(procedure) = procedure {
            descriptor = descriptor.with_procedure(procedure);
        }
        let _ = artifact.root_maps.alloc(descriptor);
    }
    Ok(())
}

fn decode_block_signatures(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::BlockSignatures)?;
    for _ in 0..cursor.read_u32()? {
        let procedure = cursor.read_idx()?;
        let label = cursor.read_u16()?;
        let incoming_len = usize::from(cursor.read_u16()?);
        let mut incoming_tys = Vec::with_capacity(incoming_len);
        for _ in 0..incoming_len {
            incoming_tys.push(cursor.read_idx()?);
        }
        let _ = artifact
            .block_signatures
            .alloc(BlockSignatureDescriptor::new(
                procedure,
                label,
                incoming_tys.into_boxed_slice(),
            ));
    }
    Ok(())
}

fn decode_closures(cursor: &mut Cursor<'_>, artifact: &mut Artifact) -> AssemblyResult {
    require_section(cursor, SectionTag::Closures)?;
    for _ in 0..cursor.read_u32()? {
        let name = cursor.read_idx()?;
        let procedure = cursor.read_idx()?;
        let capture_count = cursor.read_u16()?;
        let capture_ty_len = usize::from(cursor.read_u16()?);
        let mut capture_tys = Vec::with_capacity(capture_ty_len);
        for _ in 0..capture_ty_len {
            capture_tys.push(cursor.read_idx()?);
        }
        let env_layout = if cursor.read_u8()? != 0 {
            Some(cursor.read_idx()?)
        } else {
            None
        };
        let param_ty_len = usize::from(cursor.read_u16()?);
        let mut param_tys = Vec::with_capacity(param_ty_len);
        for _ in 0..param_ty_len {
            param_tys.push(cursor.read_idx()?);
        }
        let result_ty_len = usize::from(cursor.read_u16()?);
        let mut result_tys = Vec::with_capacity(result_ty_len);
        for _ in 0..result_ty_len {
            result_tys.push(cursor.read_idx()?);
        }
        let domain = if cursor.read_u8()? != 0 {
            Some(cursor.read_idx()?)
        } else {
            None
        };
        let effect = if cursor.read_u8()? != 0 {
            Some(cursor.read_idx()?)
        } else {
            None
        };
        let mut descriptor = ClosureDescriptor::new(name, procedure, capture_count)
            .with_capture_tys(capture_tys.into_boxed_slice())
            .with_param_tys(param_tys.into_boxed_slice())
            .with_result_tys(result_tys.into_boxed_slice())
            .with_suspending(cursor.read_u8()? != 0);
        if let Some(env_layout) = env_layout {
            descriptor = descriptor.with_env_layout(env_layout);
        }
        if let Some(domain) = domain {
            descriptor = descriptor.with_domain(domain);
        }
        if let Some(effect) = effect {
            descriptor = descriptor.with_effect(effect);
        }
        let _ = artifact.closures.alloc(descriptor);
    }
    Ok(())
}

fn decode_operand(cursor: &mut Cursor<'_>) -> AssemblyResult<Operand> {
    Ok(match cursor.read_u8()? {
        0 => Operand::None,
        1 => Operand::I16(cursor.read_i16()?),
        2 => Operand::Local(cursor.read_u16()?),
        3 => Operand::String(cursor.read_idx()?),
        4 => Operand::Type(cursor.read_idx()?),
        5 => Operand::Constant(cursor.read_idx()?),
        6 => Operand::Global(cursor.read_idx()?),
        7 => Operand::Procedure(cursor.read_idx()?),
        8 => Operand::Foreign(cursor.read_idx()?),
        13 => Operand::WideProcedureCaptures {
            procedure: cursor.read_idx()?,
            captures: cursor.read_u8()?,
        },
        10 => Operand::Label(cursor.read_u16()?),
        11 => Operand::TypeLen {
            ty: cursor.read_idx()?,
            len: cursor.read_u16()?,
        },
        12 => {
            let count = usize::from(cursor.read_u16()?);
            let mut labels = Vec::with_capacity(count);
            for _ in 0..count {
                labels.push(cursor.read_u16()?);
            }
            Operand::BranchTable(labels.into_boxed_slice())
        }
        _ => return Err(AssemblyError::text_parse_source("unknown operand tag")),
    })
}

fn decode_opcode(cursor: &mut Cursor<'_>) -> AssemblyResult<u16> {
    let lead = cursor.read_u8()?;
    if lead != Opcode::extended_opcode_prefix() {
        return Ok(u16::from(lead));
    }
    cursor.read_u16()
}
