use jetcrab::vm::registers::Registers;
use jetcrab::vm::value::Value;

#[test]
fn test_registers_creation() {
    let registers = Registers::new(16);
    
    assert_eq!(registers.capacity(), 16);
    assert_eq!(registers.len(), 0);
}

#[test]
fn test_registers_set_get() {
    let mut registers = Registers::new(8);
    
    registers.set(0, Value::Number(42.0));
    registers.set(1, Value::String("hello".to_string()));
    
    let value0 = registers.get(0).unwrap();
    let value1 = registers.get(1).unwrap();
    
    assert!(matches!(value0, Value::Number(42.0)));
    assert!(matches!(value1, Value::String(ref s) if s == "hello"));
}

#[test]
fn test_registers_out_of_bounds() {
    let mut registers = Registers::new(4);
    
    // Try to set value beyond capacity
    registers.set(5, Value::Number(42.0));
    
    // Try to get value beyond capacity
    assert!(registers.get(5).is_none());
}

#[test]
fn test_registers_clear() {
    let mut registers = Registers::new(4);
    
    registers.set(0, Value::Number(42.0));
    registers.set(1, Value::String("hello".to_string()));
    
    assert_eq!(registers.len(), 2);
    
    registers.clear();
    assert_eq!(registers.len(), 0);
}

#[test]
fn test_registers_is_empty() {
    let mut registers = Registers::new(4);
    
    assert!(registers.is_empty());
    
    registers.set(0, Value::Number(42.0));
    assert!(!registers.is_empty());
    
    registers.clear();
    assert!(registers.is_empty());
}

#[test]
fn test_registers_capacity() {
    let registers = Registers::new(32);
    assert_eq!(registers.capacity(), 32);
    
    let registers = Registers::new(64);
    assert_eq!(registers.capacity(), 64);
}
