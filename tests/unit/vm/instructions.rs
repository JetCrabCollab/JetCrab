use jetcrab::vm::instructions::{Instruction, Opcode};

#[test]
fn test_instruction_creation() {
    let instruction = Instruction::new(Opcode::Push, vec![42]);
    
    assert_eq!(instruction.opcode, Opcode::Push);
    assert_eq!(instruction.operands, vec![42]);
}

#[test]
fn test_instruction_clone() {
    let original = Instruction::new(Opcode::Add, vec![1, 2]);
    let cloned = original.clone();
    
    assert_eq!(original.opcode, cloned.opcode);
    assert_eq!(original.operands, cloned.operands);
}

#[test]
fn test_instruction_debug() {
    let instruction = Instruction::new(Opcode::Call, vec![5]);
    let debug_str = format!("{:?}", instruction);
    
    assert!(debug_str.contains("Call"));
    assert!(debug_str.contains("5"));
}

#[test]
fn test_opcode_variants() {
    let push = Opcode::Push;
    let pop = Opcode::Pop;
    let add = Opcode::Add;
    let sub = Opcode::Sub;
    let mul = Opcode::Mul;
    let div = Opcode::Div;
    
    assert!(matches!(push, Opcode::Push));
    assert!(matches!(pop, Opcode::Pop));
    assert!(matches!(add, Opcode::Add));
    assert!(matches!(sub, Opcode::Sub));
    assert!(matches!(mul, Opcode::Mul));
    assert!(matches!(div, Opcode::Div));
}

#[test]
fn test_instruction_with_operands() {
    let instruction = Instruction::new(Opcode::Load, vec![10, 20, 30]);
    
    assert_eq!(instruction.operands.len(), 3);
    assert_eq!(instruction.operands[0], 10);
    assert_eq!(instruction.operands[1], 20);
    assert_eq!(instruction.operands[2], 30);
}
