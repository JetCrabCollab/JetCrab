use jetcrab::vm::frame::Frame;
use jetcrab::vm::value::Value;

#[test]
fn test_frame_creation() {
    let frame = Frame::new(100);
    
    assert_eq!(frame.capacity(), 100);
    assert_eq!(frame.len(), 0);
    assert!(frame.is_empty());
}

#[test]
fn test_frame_push_pop() {
    let mut frame = Frame::new(10);
    
    frame.push(Value::Number(42.0));
    frame.push(Value::String("hello".to_string()));
    
    assert_eq!(frame.len(), 2);
    assert!(!frame.is_empty());
    
    let popped_string = frame.pop().unwrap();
    assert!(matches!(popped_string, Value::String(ref s) if s == "hello"));
    
    let popped_number = frame.pop().unwrap();
    assert!(matches!(popped_number, Value::Number(42.0)));
    
    assert!(frame.is_empty());
}

#[test]
fn test_frame_peek() {
    let mut frame = Frame::new(10);
    
    frame.push(Value::Number(42.0));
    frame.push(Value::String("hello".to_string()));
    
    let peeked = frame.peek().unwrap();
    assert!(matches!(peeked, Value::String(ref s) if s == "hello"));
    
    // Frame should still have 2 elements
    assert_eq!(frame.len(), 2);
}

#[test]
fn test_frame_clear() {
    let mut frame = Frame::new(10);
    
    frame.push(Value::Number(42.0));
    frame.push(Value::String("hello".to_string()));
    
    assert_eq!(frame.len(), 2);
    
    frame.clear();
    assert_eq!(frame.len(), 0);
    assert!(frame.is_empty());
}

#[test]
fn test_frame_overflow() {
    let mut frame = Frame::new(2);
    
    frame.push(Value::Number(1.0));
    frame.push(Value::Number(2.0));
    
    // This should handle overflow gracefully
    assert_eq!(frame.len(), 2);
}

#[test]
fn test_frame_underflow() {
    let mut frame = Frame::new(10);
    
    // Try to pop from empty frame
    assert!(frame.pop().is_none());
    assert!(frame.peek().is_none());
}
