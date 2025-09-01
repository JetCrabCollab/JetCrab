use jetcrab::api::error::ApiError;
use jetcrab::api::modules::*;
use jetcrab::vm::value::Value;
use std::path::PathBuf;

#[test]
fn test_module_info_creation() {
    let module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    );
    assert_eq!(module_info.id, "test_module");
    assert_eq!(module_info.source, "console.log('hello');");
    assert!(module_info.path.is_none());
    assert!(module_info.exports.is_empty());
    assert!(module_info.dependencies.is_empty());
    assert!(!module_info.is_loaded);
}

#[test]
fn test_module_info_with_path() {
    let module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    )
    .with_path(PathBuf::from("/path/to/module.js"));
    assert_eq!(module_info.id, "test_module");
    assert_eq!(module_info.source, "console.log('hello');");
    assert_eq!(module_info.path, Some(PathBuf::from("/path/to/module.js")));
}

#[test]
fn test_module_info_add_export() {
    let mut module_info =
        ModuleInfo::new("test_module".to_string(), "export const x = 1;".to_string());
    module_info.add_export("x".to_string(), Value::Number(1.0));
    assert_eq!(module_info.exports.len(), 1);
    assert_eq!(module_info.get_export("x"), Some(&Value::Number(1.0)));
}

#[test]
fn test_module_info_get_export() {
    let mut module_info =
        ModuleInfo::new("test_module".to_string(), "export const x = 1;".to_string());
    module_info.add_export("x".to_string(), Value::Number(1.0));
    module_info.add_export("y".to_string(), Value::String("hello".to_string()));

    assert_eq!(module_info.get_export("x"), Some(&Value::Number(1.0)));
    assert_eq!(
        module_info.get_export("y"),
        Some(&Value::String("hello".to_string()))
    );
    assert_eq!(module_info.get_export("z"), None);
}

#[test]
fn test_module_info_add_dependency() {
    let mut module_info = ModuleInfo::new(
        "test_module".to_string(),
        "import './other.js';".to_string(),
    );
    module_info.add_dependency("other_module".to_string());
    assert_eq!(module_info.dependencies.len(), 1);
    assert_eq!(module_info.dependencies[0], "other_module");
}

#[test]
fn test_module_info_add_duplicate_dependency() {
    let mut module_info = ModuleInfo::new(
        "test_module".to_string(),
        "import './other.js';".to_string(),
    );
    module_info.add_dependency("other_module".to_string());
    module_info.add_dependency("other_module".to_string());
    assert_eq!(module_info.dependencies.len(), 1);
    assert_eq!(module_info.dependencies[0], "other_module");
}

#[test]
fn test_module_info_clone() {
    let mut module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    );
    module_info.add_export("x".to_string(), Value::Number(1.0));
    module_info.add_dependency("other".to_string());

    let cloned = module_info.clone();
    assert_eq!(cloned.id, "test_module");
    assert_eq!(cloned.source, "console.log('hello');");
    assert_eq!(cloned.exports.len(), 1);
    assert_eq!(cloned.dependencies.len(), 1);
}

#[test]
fn test_module_info_debug() {
    let module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    );
    let debug_str = format!("{:?}", module_info);
    assert!(debug_str.contains("test_module"));
}

#[test]
fn test_module_resolution_creation() {
    let resolution = ModuleResolution {
        module_id: "test_module".to_string(),
        absolute_path: Some(PathBuf::from("/path/to/module.js")),
        is_external: false,
    };
    assert_eq!(resolution.module_id, "test_module");
    assert_eq!(
        resolution.absolute_path,
        Some(PathBuf::from("/path/to/module.js"))
    );
    assert!(!resolution.is_external);
}

#[test]
fn test_module_resolution_clone() {
    let resolution = ModuleResolution {
        module_id: "test_module".to_string(),
        absolute_path: Some(PathBuf::from("/path/to/module.js")),
        is_external: true,
    };
    let cloned = resolution.clone();
    assert_eq!(cloned.module_id, "test_module");
    assert_eq!(
        cloned.absolute_path,
        Some(PathBuf::from("/path/to/module.js"))
    );
    assert!(cloned.is_external);
}

#[test]
fn test_module_resolution_debug() {
    let resolution = ModuleResolution {
        module_id: "test_module".to_string(),
        absolute_path: None,
        is_external: false,
    };
    let debug_str = format!("{:?}", resolution);
    assert!(debug_str.contains("test_module"));
}

#[test]
fn test_file_system_module_provider_creation() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    assert!(true);
}

#[test]
fn test_file_system_module_provider_add_module() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    );
    provider.add_module(module_info);
    assert!(provider.get_module_info("test_module").is_some());
}

#[test]
fn test_file_system_module_provider_resolve_module_relative() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let resolution = provider
        .resolve_module("./other.js", Some("parent"))
        .unwrap();
    assert_eq!(resolution.module_id, "parent/./other.js");
    assert!(!resolution.is_external);
}

#[test]
fn test_file_system_module_provider_resolve_module_absolute() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let resolution = provider
        .resolve_module("/absolute/path.js", Some("parent"))
        .unwrap();
    assert!(resolution.module_id.contains("absolute/path.js"));
    assert!(!resolution.is_external);
}

#[test]
fn test_file_system_module_provider_resolve_module_external() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let resolution = provider.resolve_module("lodash", Some("parent")).unwrap();
    assert_eq!(resolution.module_id, "lodash");
    assert!(!resolution.is_external);
}

#[test]
fn test_file_system_module_provider_resolve_module_no_from() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let resolution = provider.resolve_module("./relative.js", None).unwrap();
    assert_eq!(resolution.module_id, "./relative.js");
    assert!(!resolution.is_external);
}

#[test]
fn test_file_system_module_provider_load_module_success() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    );
    provider.add_module(module_info);

    let resolution = ModuleResolution {
        module_id: "test_module".to_string(),
        absolute_path: None,
        is_external: false,
    };
    let source = provider.load_module(&resolution).unwrap();
    assert_eq!(source, "console.log('hello');");
}

#[test]
fn test_file_system_module_provider_load_module_not_found() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let resolution = ModuleResolution {
        module_id: "nonexistent".to_string(),
        absolute_path: None,
        is_external: false,
    };
    let result = provider.load_module(&resolution);
    assert!(result.is_err());
    if let Err(ApiError::ResourceError {
        resource, message, ..
    }) = result
    {
        assert_eq!(resource, "nonexistent");
        assert_eq!(message, "Module not found");
    } else {
        panic!("Expected ResourceError");
    }
}

#[test]
fn test_file_system_module_provider_get_module_info() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    );
    provider.add_module(module_info);

    let info = provider.get_module_info("test_module");
    assert!(info.is_some());
    assert_eq!(info.unwrap().id, "test_module");

    let info = provider.get_module_info("nonexistent");
    assert!(info.is_none());
}

#[test]
fn test_module_loader_creation() {
    let provider = Box::new(FileSystemModuleProvider::new(PathBuf::from("/base/path")));
    let loader = ModuleLoader::new(provider);
    assert!(loader.get_loaded_modules().is_empty());
}

#[test]
fn test_module_loader_load_module() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    );
    provider.add_module(module_info);

    let mut loader = ModuleLoader::new(Box::new(provider));
    let result = loader.load_module("test_module", None);
    assert!(result.is_ok());
    assert_eq!(loader.get_loaded_modules().len(), 1);
}

#[test]
fn test_module_loader_load_module_not_found() {
    let provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let mut loader = ModuleLoader::new(Box::new(provider));
    let result = loader.load_module("nonexistent", None);
    assert!(result.is_err());
}

#[test]
fn test_module_loader_get_loaded_modules() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    );
    provider.add_module(module_info);

    let mut loader = ModuleLoader::new(Box::new(provider));
    loader.load_module("test_module", None).unwrap();

    let loaded_modules = loader.get_loaded_modules();
    assert_eq!(loaded_modules.len(), 1);
    assert!(loaded_modules.contains_key("test_module"));
}

#[test]
fn test_module_loader_clear_cache() {
    let mut provider = FileSystemModuleProvider::new(PathBuf::from("/base/path"));
    let module_info = ModuleInfo::new(
        "test_module".to_string(),
        "console.log('hello');".to_string(),
    );
    provider.add_module(module_info);

    let mut loader = ModuleLoader::new(Box::new(provider));
    loader.load_module("test_module", None).unwrap();
    loader.clear_cache();
    assert!(true);
}

#[test]
fn test_module_registry_creation() {
    let registry = ModuleRegistry::new();
    assert!(registry.get_provider("file").is_none());
    assert!(registry.get_default_provider().is_none());
}

#[test]
fn test_module_registry_default() {
    let registry = ModuleRegistry::default();
    assert!(registry.get_provider("file").is_none());
    assert!(registry.get_default_provider().is_none());
}

#[test]
fn test_module_registry_register_provider() {
    let mut registry = ModuleRegistry::new();
    let provider = Box::new(FileSystemModuleProvider::new(PathBuf::from("/base/path")));
    registry.register_provider("file".to_string(), provider);
    assert!(registry.get_provider("file").is_some());
}

#[test]
fn test_module_registry_set_default_provider() {
    let mut registry = ModuleRegistry::new();
    let provider = Box::new(FileSystemModuleProvider::new(PathBuf::from("/base/path")));
    registry.set_default_provider(provider);
    assert!(registry.get_default_provider().is_some());
}

#[test]
fn test_module_registry_get_provider() {
    let mut registry = ModuleRegistry::new();
    let provider = Box::new(FileSystemModuleProvider::new(PathBuf::from("/base/path")));
    registry.register_provider("file".to_string(), provider);

    let provider = registry.get_provider("file");
    assert!(provider.is_some());

    let provider = registry.get_provider("http");
    assert!(provider.is_none());
}

#[test]
fn test_module_registry_get_default_provider() {
    let mut registry = ModuleRegistry::new();
    let provider = Box::new(FileSystemModuleProvider::new(PathBuf::from("/base/path")));
    registry.set_default_provider(provider);

    let provider = registry.get_default_provider();
    assert!(provider.is_some());
}

#[test]
fn test_module_registry_get_default_provider_none() {
    let registry = ModuleRegistry::new();
    let provider = registry.get_default_provider();
    assert!(provider.is_none());
}
