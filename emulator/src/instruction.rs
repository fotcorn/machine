use crate::instruction_set::InstructionSize;
use crate::instruction_set::InstructionType;
use crate::instruction_set::Register;

pub enum InstructionParam {
    Immediate(i64),
    Register(Register),
}

pub struct Instruction {
    pub instruction: InstructionType,
    pub size: InstructionSize,
    pub params: Vec<InstructionParam>,
}
