use crate::bytecode::optimizer::BytecodeOptimizer;
use crate::bytecode::BytecodeGenerator;

use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::vm::instructions::Instruction;

pub struct Compiler {
    optimize: bool,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self { optimize: false }
    }

    pub fn with_optimization(mut self, optimize: bool) -> Self {
        self.optimize = optimize;
        self
    }

    pub fn compile(&mut self, source: &str) -> Result<Vec<Instruction>, String> {
        let mut parser = Parser::new(source);
        let ast = parser.parse().map_err(|e| format!("Parser error: {e}"))?;

        let mut analyzer = SemanticAnalyzer::new();
        analyzer
            .analyze(&ast)
            .map_err(|e| format!("Semantic error: {e}"))?;

        let mut generator = BytecodeGenerator::new();
        let mut instructions = generator.generate(&ast);

        if self.optimize {
            instructions = BytecodeOptimizer::optimize(instructions);
        }

        Ok(instructions)
    }

    pub fn compile_to_bytecode(
        &mut self,
        source: &str,
    ) -> Result<(Vec<Instruction>, Vec<String>), String> {
        let mut parser = Parser::new(source);
        let ast = parser.parse().map_err(|e| format!("Parser error: {e}"))?;

        let mut analyzer = SemanticAnalyzer::new();
        analyzer
            .analyze(&ast)
            .map_err(|e| format!("Semantic error: {e}"))?;

        let mut generator = BytecodeGenerator::new();
        let mut instructions = generator.generate(&ast);
        let constants = generator.get_constants().clone();

        if self.optimize {
            instructions = BytecodeOptimizer::optimize(instructions);
        }

        Ok((instructions, constants))
    }
}
