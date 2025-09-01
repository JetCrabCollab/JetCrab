use jetcrab::api::*;
use jetcrab::vm::value::Value;
use std::path::PathBuf;

#[test]
fn test_engine_new() {
    let engine = Engine::new();
    assert!(engine.is_ok());
}

#[test]
fn test_engine_evaluate_basic() {
    let mut engine = Engine::new().unwrap();
    let result = engine.evaluate("42");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(42.0));
}

#[test]
fn test_engine_evaluate_arithmetic() {
    let mut engine = Engine::new().unwrap();
    let result = engine.evaluate("2 + 3 * 4");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(14.0));
}

#[test]
fn test_engine_evaluate_string() {
    let mut engine = Engine::new().unwrap();
    let result = engine.evaluate("\"Hello, World!\"");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::String("Hello, World!".to_string()));
}

#[test]
fn test_engine_evaluate_boolean() {
    let mut engine = Engine::new().unwrap();
    let result = engine.evaluate("true");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Boolean(true));
}

#[test]
fn test_engine_evaluate_object() {
    let mut engine = Engine::new().unwrap();
    let result = engine.evaluate("{name: 'Alice', age: 25}");
    assert!(result.is_ok());
}

#[test]
fn test_engine_evaluate_array() {
    let mut engine = Engine::new().unwrap();
    let result = engine.evaluate("[1, 2, 3]");
    assert!(result.is_ok());
}

#[test]
fn test_engine_evaluate_function() {
    let mut engine = Engine::new().unwrap();
    let code = "function add(a, b) { return a + b; } add(5, 3)";
    let result = engine.evaluate(code);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(8.0));
}

#[test]
fn test_engine_evaluate_template_literal() {
    let mut engine = Engine::new().unwrap();
    let code = "let name = 'World'; `Hello ${name}!`";
    let result = engine.evaluate(code);
    assert!(result.is_ok());
}

#[test]
fn test_engine_evaluate_console_log() {
    let mut engine = Engine::new().unwrap();
    let result = engine.evaluate("console.log('test')");
    assert!(result.is_ok());
}

#[test]
fn test_engine_evaluate_json_stringify() {
    let mut engine = Engine::new().unwrap();
    let result = engine.evaluate("JSON.stringify({name: 'test'})");
    assert!(result.is_ok());
}

#[test]
fn test_engine_evaluate_math_sqrt() {
    let mut engine = Engine::new().unwrap();
    let result = engine.evaluate("Math.sqrt(16)");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Number(4.0));
}

#[test]
fn test_config_new() {
    let config = Config::new();
    assert_eq!(config.max_memory, 1024 * 1024 * 1024);
    assert_eq!(config.gc_threshold, 0.8);
}

#[test]
fn test_config_builder() {
    let config = Config::builder()
        .max_memory(2048 * 1024 * 1024)
        .gc_threshold(0.9)
        .build();
    assert_eq!(config.max_memory, 2048 * 1024 * 1024);
    assert_eq!(config.gc_threshold, 0.9);
}

#[test]
fn test_api_error_display() {
    let error = ApiError::ParseError {
        message: "Test error".to_string(),
        position: Some(10),
    };
    let display = format!("{}", error);
    assert!(display.contains("Test error"));
}

#[test]
fn test_api_error_from_parse_error() {
    let parse_error = jetcrab::parser::error::ParseError::UnexpectedToken {
        expected: "number".to_string(),
        found: "string".to_string(),
        position: 5,
    };
    let api_error: ApiError = parse_error.into();
    assert!(matches!(api_error, ApiError::ParseError { .. }));
}

#[test]
fn test_debugger_new() {
    let debugger = Debugger::new();
    assert!(!debugger.is_enabled());
}

#[test]
fn test_debugger_enable_disable() {
    let mut debugger = Debugger::new();
    debugger.enable();
    assert!(debugger.is_enabled());
    debugger.disable();
    assert!(!debugger.is_enabled());
}

#[test]
fn test_debugger_add_breakpoint() {
    let mut debugger = Debugger::new();
    debugger.add_breakpoint(10);
    assert!(debugger.has_breakpoint(10));
}

#[test]
fn test_debugger_remove_breakpoint() {
    let mut debugger = Debugger::new();
    debugger.add_breakpoint(10);
    debugger.remove_breakpoint(10);
    assert!(!debugger.has_breakpoint(10));
}

#[test]
fn test_profiler_new() {
    let profiler = Profiler::new();
    assert_eq!(profiler.get_function_timings().len(), 0);
}

#[test]
fn test_profiler_start_stop() {
    let mut profiler = Profiler::new();
    profiler.start_profiling();
    let report = profiler.stop_profiling();
    assert!(report.generate_report().contains("Profiling Report"));
}

#[test]
fn test_profiler_record_function() {
    let mut profiler = Profiler::new();
    profiler.record_function_call("test_function", std::time::Duration::from_millis(100));
    let timings = profiler.get_function_timings();
    assert!(timings.contains_key("test_function"));
}

#[test]
fn test_inspector_new() {
    let inspector = Inspector::new();
    assert!(!inspector.get_debugger().is_enabled());
}

#[test]
fn test_inspector_start_stop() {
    let mut inspector = Inspector::new();
    inspector.start_inspection();
    assert!(inspector.get_debugger().is_enabled());
    let report = inspector.stop_inspection();
    assert!(report.contains("Profiling Report"));
}

#[test]
fn test_inspector_add_event_listener() {
    let mut inspector = Inspector::new();
    inspector.add_event_listener(
        "test_event".to_string(),
        Box::new(|_data| println!("Event received")),
    );
}

#[test]
fn test_event_emitter_new() {
    let emitter = EventEmitter::new();
    assert_eq!(emitter.get_listeners("test_event").len(), 0);
}

#[test]
fn test_event_emitter_add_listener() {
    let mut emitter = EventEmitter::new();
    emitter.add_listener("test_event", |data| println!("{}", data));
    assert_eq!(emitter.get_listeners("test_event").len(), 1);
}

#[test]
fn test_event_emitter_emit() {
    let mut emitter = EventEmitter::new();
    let mut received_data = String::new();
    emitter.add_listener("test_event", |data| received_data = data);
    emitter.emit("test_event", "test_data".to_string());
    assert_eq!(received_data, "test_data");
}

#[test]
fn test_callback_registry_new() {
    let registry = CallbackRegistry::new();
    assert_eq!(registry.callbacks.len(), 0);
}

#[test]
fn test_callback_registry_register() {
    let mut registry = CallbackRegistry::new();
    registry.register("test_callback", "Test callback", |data| {
        Ok(serde_json::json!({"result": data}))
    });
    assert!(registry.callbacks.contains_key("test_callback"));
}

#[test]
fn test_callback_registry_call() {
    let mut registry = CallbackRegistry::new();
    registry.register("test_callback", "Test callback", |data| {
        Ok(serde_json::json!({"result": data}))
    });
    let result = registry.call("test_callback", serde_json::json!("test_data"));
    assert!(result.is_ok());
}

#[test]
fn test_event_manager_new() {
    let manager = EventManager::new();
    assert_eq!(manager.get_emitter().get_listeners("test_event").len(), 0);
}

#[test]
fn test_event_manager_add_filter() {
    let mut manager = EventManager::new();
    manager.add_event_filter("test_event", |_data| true);
    manager.emit_filtered("test_event", serde_json::json!("test_data"));
}

#[test]
fn test_file_system_module_provider_new() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/test"));
    assert_eq!(provider.modules.len(), 0);
}

#[test]
fn test_file_system_module_provider_add_module() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/test"));
    let module = ModuleInfo::new("test_module".to_string(), "console.log('test');".to_string());
    provider.add_module(module);
    assert_eq!(provider.modules.len(), 1);
}

#[test]
fn test_file_system_module_provider_resolve_module() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/test"));
    let resolution = provider.resolve_module("test_module", None);
    assert!(resolution.is_ok());
    let resolution = resolution.unwrap();
    assert_eq!(resolution.module_id, "test_module");
    assert!(!resolution.is_external);
}

#[test]
fn test_file_system_module_provider_load_module() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/test"));
    let module = ModuleInfo::new("test_module".to_string(), "console.log('test');".to_string());
    provider.add_module(module);
    
    let resolution = ModuleResolution {
        module_id: "test_module".to_string(),
        absolute_path: None,
        is_external: false,
    };
    let source = provider.load_module(&resolution);
    assert!(source.is_ok());
    assert_eq!(source.unwrap(), "console.log('test');");
}

#[test]
fn test_file_system_module_provider_get_module_info() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/test"));
    let module = ModuleInfo::new("test_module".to_string(), "console.log('test');".to_string());
    provider.add_module(module);
    
    let info = provider.get_module_info("test_module");
    assert!(info.is_some());
    assert_eq!(info.unwrap().id, "test_module");
}

#[test]
fn test_module_loader_new() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/test"));
    let loader = ModuleLoader::new(Box::new(provider));
    assert_eq!(loader.loaded_modules.len(), 0);
    assert_eq!(loader.module_cache.len(), 0);
}

#[test]
fn test_module_loader_load_module() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/test"));
    let module = ModuleInfo::new("test_module".to_string(), "42".to_string());
    provider.add_module(module);
    
    let mut loader = ModuleLoader::new(Box::new(provider));
    let result = loader.load_module("test_module", None);
    assert!(result.is_ok());
}

#[test]
fn test_compiler_new() {
    let compiler = Compiler::new();
    assert!(compiler.is_ok());
}

#[test]
fn test_compiler_compile() {
    let mut compiler = Compiler::new().unwrap();
    let result = compiler.compile("42");
    assert!(result.is_ok());
}

#[test]
fn test_interpreter_new() {
    let interpreter = Interpreter::new();
    assert!(interpreter.is_ok());
}

#[test]
fn test_interpreter_interpret() {
    let mut interpreter = Interpreter::new().unwrap();
    let result = interpreter.interpret("42");
    assert!(result.is_ok());
}
