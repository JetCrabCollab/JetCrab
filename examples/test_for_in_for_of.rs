use jetcrab::api::Engine;

fn main() {
    println!("=== Test For...In and For...Of Loops ===");
    
    let mut engine = Engine::new();
    
    println!("\n1. Testing for...in loop (object properties):");
    
    println!("\n   Step 1: Test for...in with object");
    let result1 = engine.evaluate("let obj = {a: 1, b: 2, c: 3}; let keys = []; for (let key in obj) { keys.push(key); } keys.join(',')");
    println!("   Result: {:?}", result1);
    
    println!("\n2. Testing for...of loop (array values):");
    
    println!("\n   Step 2: Test for...of with array");
    let result2 = engine.evaluate("let arr = [10, 20, 30]; let sum = 0; for (let value of arr) { sum += value; } sum");
    println!("   Result: {:?}", result2);
    
    println!("\n3. Testing for...of with string:");
    
    println!("\n   Step 3: Test for...of with string");
    let result3 = engine.evaluate("let str = 'hello'; let chars = []; for (let char of str) { chars.push(char); } chars.join('')");
    println!("   Result: {:?}", result3);
    
    println!("\n4. Testing nested for...in and for...of:");
    
    println!("\n   Step 4: Test nested loops");
    let result4 = engine.evaluate("let matrix = {row1: [1, 2], row2: [3, 4]}; let total = 0; for (let rowKey in matrix) { for (let value of matrix[rowKey]) { total += value; } } total");
    println!("   Result: {:?}", result4);
    
    println!("\n=== Test Complete ===");
}
