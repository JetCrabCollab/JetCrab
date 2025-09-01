use crate::vm::instructions::Instruction;

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
}

impl Bytecode {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Bytecode { instructions }
    }
}
