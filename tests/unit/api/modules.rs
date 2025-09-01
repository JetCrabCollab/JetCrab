use jetcrab::api::modules::{ModuleInfo, FileSystemModuleProvider, ModuleRegistry};
use jetcrab::vm::value::Value;
use std::path::PathBuf;

#[test]
fn test_module_info() {
    let mut module = ModuleInfo::new("test".to_string(), "console.log('hello')".to_string());
    module.add_export("default".to_string(), Value::String("test".to_string()));

    assert_eq!(module.exports.len(), 1);
    assert!(module.get_export("default").is_some());
}

#[test]
fn test_file_system_provider() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/tmp"));
    let module = ModuleInfo::new("test".to_string(), "export default 'hello'".to_string());
    provider.add_module(module);

    let resolution = provider.resolve_module("test", None).unwrap();
    assert_eq!(resolution.module_id, "test");
}

#[test]
fn test_module_registry() {
    let mut registry = ModuleRegistry::new();
    let provider = Box::new(FileSystemModuleProvider::new(PathBuf::from("/tmp")));

    registry.register_provider("file".to_string(), provider);
    assert!(registry.get_provider("file").is_some());
}
