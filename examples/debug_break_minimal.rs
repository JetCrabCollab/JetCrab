use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Break Minimal ===");
    
    let mut engine = Engine::new();
    
    println!("\n1. Testing minimal break:");
    
    println!("\n   Step 1: Test break in simple while loop");
    let result1 = engine.evaluate("let i = 0; while (i < 3) { if (i > 1) break; i++; } i");
    println!("   Result: {:?}", result1);
}
