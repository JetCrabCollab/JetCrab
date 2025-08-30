use jetcrab::api::Engine;

fn main() {
    println!("=== Test Basic Module Support ===");
    
    let mut engine = Engine::new();
    
    println!("\n1. Testing import statement parsing:");
    
    println!("\n   Step 1: Test import statement");
    let result1 = engine.evaluate("import { name } from './module.js'");
    println!("   Result: {:?}", result1);
    
    println!("\n2. Testing export statement parsing:");
    
    println!("\n   Step 2: Test export statement");
    let result2 = engine.evaluate("export const value = 42");
    println!("   Result: {:?}", result2);
    
    println!("\n3. Testing default export:");
    
    println!("\n   Step 3: Test default export");
    let result3 = engine.evaluate("export default function() { return 'hello'; }");
    println!("   Result: {:?}", result3);
    
    println!("\n4. Testing namespace import:");
    
    println!("\n   Step 4: Test namespace import");
    let result4 = engine.evaluate("import * as utils from './utils.js'");
    println!("   Result: {:?}", result4);
    
    println!("\n=== Test Complete ===");
}
