use crate::vm::instructions::Instruction;

pub struct BytecodeOptimizer;

impl BytecodeOptimizer {
    pub fn optimize(instructions: Vec<Instruction>) -> Vec<Instruction> {
        let mut optimized = instructions;

        optimized = Self::remove_dead_code(optimized);
        optimized = Self::constant_folding(optimized);
        optimized = Self::peephole_optimization(optimized);

        optimized
    }

    fn remove_dead_code(instructions: Vec<Instruction>) -> Vec<Instruction> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < instructions.len() {
            let instruction = &instructions[i];

            match instruction {
                Instruction::Pop => {
                    if let Some(Instruction::PushConst(_)) = result.last() {
                        result.pop();
                    } else {
                        result.push(instruction.clone());
                    }
                }
                _ => {
                    result.push(instruction.clone());
                }
            }

            i += 1;
        }

        result
    }

    fn constant_folding(instructions: Vec<Instruction>) -> Vec<Instruction> {
        instructions
    }

    fn peephole_optimization(instructions: Vec<Instruction>) -> Vec<Instruction> {
        instructions
    }
}
