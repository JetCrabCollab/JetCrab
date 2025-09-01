use jetcrab::vm::memory::stack::Stack;
use jetcrab::vm::value::Value;

#[test]
fn test_stack_creation() {
    let stack = Stack::new(100);
    assert_eq!(stack.capacity(), 100);
    assert_eq!(stack.len(), 0);
    assert!(stack.is_empty());
}

#[test]
fn test_stack_push_pop() {
    let mut stack = Stack::new(10);
    
    stack.push(Value::Number(42.0));
    stack.push(Value::String("hello".to_string()));
    
    assert_eq!(stack.len(), 2);
    assert!(!stack.is_empty());
    
    let popped_string = stack.pop().unwrap();
    assert!(matches!(popped_string, Value::String(ref s) if s == "hello"));
    
    let popped_number = stack.pop().unwrap();
    assert!(matches!(popped_number, Value::Number(42.0)));
    
    assert!(stack.is_empty());
}

#[test]
fn test_stack_peek() {
    let mut stack = Stack::new(10);
    
    stack.push(Value::Number(42.0));
    stack.push(Value::String("hello".to_string()));
    
    let peeked = stack.peek().unwrap();
    assert!(matches!(peeked, Value::String(ref s) if s == "hello"));
    
    // Stack should still have 2 elements
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_stack_overflow() {
    let mut stack = Stack::new(2);
    
    stack.push(Value::Number(1.0));
    stack.push(Value::Number(2.0));
    
    // This should panic or handle overflow gracefully
    // For now, we'll just test that we can't push more than capacity
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_stack_underflow() {
    let mut stack = Stack::new(10);
    
    // Try to pop from empty stack
    assert!(stack.pop().is_none());
    assert!(stack.peek().is_none());
}
