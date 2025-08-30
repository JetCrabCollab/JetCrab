use jetcrab::api::FileSystemModuleProvider;
use jetcrab::{
    CallbackRegistry, EngineConfig, EventManager, Inspector, ModuleInfo, ModuleRegistry,
    ModuleSystem, OptimizationLevel, SecurityLevel,
};
use serde_json::json;
use std::path::PathBuf;
use std::time::Duration;

fn main() {
    println!("=== JetCrab Advanced API Demo ===\n");

    // 1. Configuration and Customization
    println!("1. Engine Configuration:");
    let config = EngineConfig::new()
        .with_optimization(OptimizationLevel::Aggressive)
        .with_security_level(SecurityLevel::Strict)
        .with_module_system(ModuleSystem::ES6)
        .with_debugging(true)
        .with_profiling(true)
        .with_timeout(Duration::from_secs(60))
        .with_execution_limits(5000, 10_000_000);

    match config.validate() {
        Ok(()) => println!("  ✅ Configuration validated successfully"),
        Err(e) => println!("  ❌ Configuration error: {}", e),
    }

    println!("  Production ready: {}", config.is_production_ready());
    println!("  Optimization level: {:?}", config.optimization_level);
    println!("  Security level: {:?}", config.security_level);
    println!();

    // 2. Debugging and Profiling
    println!("2. Debugging and Profiling:");
    let mut inspector = Inspector::new();

    // Start inspection
    inspector.start_inspection();

    // Add breakpoints
    {
        let debugger = inspector.get_debugger();
        let breakpoint = jetcrab::Breakpoint::new("main".to_string(), 10, 5);
        debugger.add_breakpoint(breakpoint);
        println!("  ✅ Breakpoint added");
    }

    // Start profiling
    {
        let profiler = inspector.get_profiler();
        profiler.start_profiling();
        println!("  ✅ Profiling started");
    }

    println!("  ✅ Debugging and profiling started");
    println!();

    // 3. Event System
    println!("3. Event System:");
    let mut event_manager = EventManager::new();

    // Add event listeners
    event_manager.get_emitter().on("code_executed", |event| {
        println!("    📡 Event: {} - {}", event.event_type, event.data);
    });

    event_manager.get_emitter().on("error_occurred", |event| {
        println!("    🚨 Error: {}", event.data);
    });

    // Emit events
    event_manager.get_emitter().emit(
        "code_executed",
        json!({
            "source": "main.js",
            "line": 42,
            "result": "success"
        }),
    );

    event_manager.get_emitter().emit(
        "error_occurred",
        json!({
            "type": "syntax_error",
            "message": "Unexpected token",
            "line": 15
        }),
    );
    println!();

    // 4. Callback Registry
    println!("4. Callback Registry:");
    let mut callback_registry = CallbackRegistry::new();

    // Register callbacks
    callback_registry.register("validate_input", "Validates user input", |data| {
        let input = data.as_str().unwrap_or("");
        if input.len() > 100 {
            Ok(json!({ "valid": false, "error": "Input too long" }))
        } else {
            Ok(json!({ "valid": true, "length": input.len() }))
        }
    });

    callback_registry.register("format_output", "Formats output data", |data| {
        Ok(json!({ "formatted": format!("Processed: {}", data) }))
    });

    // Use callbacks
    let validation_result = callback_registry
        .call("validate_input", json!("Hello World"))
        .unwrap();
    let formatting_result = callback_registry
        .call("format_output", json!("test data"))
        .unwrap();

    println!("  ✅ Validation result: {}", validation_result);
    println!("  ✅ Formatting result: {}", formatting_result);
    println!(
        "  Registered callbacks: {}",
        callback_registry.list_callbacks().len()
    );
    println!();

    // 5. Module System
    println!("5. Module System:");
    let mut module_registry = ModuleRegistry::new();

    // Create a file system provider
    let mut fs_provider = FileSystemModuleProvider::new(PathBuf::from("./modules"));

    // Add some modules
    let math_module = ModuleInfo::new(
        "math".to_string(),
        r#"
        export function add(a, b) { return a + b; }
        export function multiply(a, b) { return a * b; }
        export const PI = 3.14159;
    "#
        .to_string(),
    );

    let utils_module = ModuleInfo::new(
        "utils".to_string(),
        r#"
        export function formatDate(date) { return date.toISOString(); }
        export function generateId() { return Math.random().toString(36).substr(2, 9); }
    "#
        .to_string(),
    );

    fs_provider.add_module(math_module);
    fs_provider.add_module(utils_module);

    // Register the provider
    module_registry.register_provider("file".to_string(), Box::new(fs_provider));

    println!("  ✅ Module system initialized");
    println!(
        "  Providers registered: {}",
        module_registry.get_provider("file").is_some() as usize
    );
    println!();

    // 6. Event Chains
    println!("6. Event Chains:");
    let events = vec![
        "validation_started".to_string(),
        "validation_completed".to_string(),
        "processing_started".to_string(),
        "processing_completed".to_string(),
    ];

    // Add listeners for chain events first
    event_manager.get_emitter().on("validation_started", |_| {
        println!("    🔄 Validation started");
    });

    event_manager.get_emitter().on("validation_completed", |_| {
        println!("    ✅ Validation completed");
    });

    event_manager.get_emitter().on("processing_started", |_| {
        println!("    ⚙️  Processing started");
    });

    event_manager.get_emitter().on("processing_completed", |_| {
        println!("    🎉 Processing completed");
    });

    // Now create and execute the chain
    let mut chain = event_manager.create_event_chain(events);

    // Execute the chain
    println!("  Executing event chain:");
    while !chain.is_complete() {
        chain.trigger_next(json!({ "step": "data" })).unwrap();
    }
    println!();

    // 7. Stop Inspection and Generate Report
    println!("7. Inspection Report:");
    let report = inspector.stop_inspection();
    println!("  {}", report);
    println!();

    // 8. Production Configuration
    println!("8. Production Configuration:");
    let production_config = EngineConfig::new()
        .with_security_level(SecurityLevel::Strict)
        .with_debugging(false)
        .with_profiling(false)
        .with_optimization(OptimizationLevel::Aggressive);

    println!(
        "  Production ready: {}",
        production_config.is_production_ready()
    );
    println!("  Security level: {:?}", production_config.security_level);
    println!(
        "  Debugging enabled: {}",
        production_config.enable_debugging
    );
    println!();

    println!("=== Advanced API Demo Complete ===");
    println!("This demonstrates the complete JetCrab API including:");
    println!("✅ Configuration and customization");
    println!("✅ Debugging and profiling");
    println!("✅ Event system and callbacks");
    println!("✅ Module system");
    println!("✅ Event chains");
    println!("✅ Production-ready configurations");
}
