use crate::instruction::OperandShape;

use super::super::{Opcode, OpcodeFamily};
use super::info::{OpcodeInfo, opcode_info};

pub(super) const OPCODES: &[OpcodeInfo] = &[
    opcode_info(
        Opcode::MdlLoad,
        OpcodeFamily::Module,
        "mdl.load",
        OperandShape::None,
        0xB2,
    ),
    opcode_info(
        Opcode::MdlGet,
        OpcodeFamily::Module,
        "mdl.get",
        OperandShape::String,
        0xB3,
    ),
];
