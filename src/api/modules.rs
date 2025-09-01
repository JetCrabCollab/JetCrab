use crate::api::error::ApiError;
use crate::vm::value::Value;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub id: String,
    pub path: Option<PathBuf>,
    pub source: String,
    pub exports: HashMap<String, Value>,
    pub dependencies: Vec<String>,
    pub is_loaded: bool,
}

impl ModuleInfo {
    pub fn new(id: String, source: String) -> Self {
        Self {
            id,
            path: None,
            source,
            exports: HashMap::new(),
            dependencies: Vec::new(),
            is_loaded: false,
        }
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    pub fn add_export(&mut self, name: String, value: Value) {
        self.exports.insert(name, value);
    }

    pub fn get_export(&self, name: &str) -> Option<&Value> {
        self.exports.get(name)
    }

    pub fn add_dependency(&mut self, module_id: String) {
        if !self.dependencies.contains(&module_id) {
            self.dependencies.push(module_id);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleResolution {
    pub module_id: String,
    pub absolute_path: Option<PathBuf>,
    pub is_external: bool,
}

pub trait ModuleProvider: Send + Sync {
    fn resolve_module(
        &self,
        specifier: &str,
        from: Option<&str>,
    ) -> Result<ModuleResolution, ApiError>;
    fn load_module(&self, resolution: &ModuleResolution) -> Result<String, ApiError>;
    fn get_module_info(&self, module_id: &str) -> Option<&ModuleInfo>;
}

pub struct FileSystemModuleProvider {
    #[allow(dead_code)]
    base_path: PathBuf,
    modules: HashMap<String, ModuleInfo>,
}

impl FileSystemModuleProvider {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            modules: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module: ModuleInfo) {
        self.modules.insert(module.id.clone(), module);
    }
}

impl ModuleProvider for FileSystemModuleProvider {
    fn resolve_module(
        &self,
        specifier: &str,
        from: Option<&str>,
    ) -> Result<ModuleResolution, ApiError> {
        // Simple resolution logic - can be enhanced
        let module_id = if specifier.starts_with('.') || specifier.starts_with('/') {
            // Relative path
            if let Some(from_path) = from {
                format!("{from_path}/{specifier}")
            } else {
                specifier.to_string()
            }
        } else {
            // Absolute module name
            specifier.to_string()
        };

        Ok(ModuleResolution {
            module_id,
            absolute_path: None,
            is_external: false,
        })
    }

    fn load_module(&self, resolution: &ModuleResolution) -> Result<String, ApiError> {
        if let Some(module) = self.modules.get(&resolution.module_id) {
            Ok(module.source.clone())
        } else {
            Err(ApiError::ResourceError {
                resource: resolution.module_id.clone(),
                message: "Module not found".to_string(),
                position: None,
            })
        }
    }

    fn get_module_info(&self, module_id: &str) -> Option<&ModuleInfo> {
        self.modules.get(module_id)
    }
}

pub struct ModuleLoader {
    provider: Box<dyn ModuleProvider>,
    loaded_modules: HashMap<String, ModuleInfo>,
    module_cache: HashMap<String, Value>,
}

impl ModuleLoader {
    pub fn new(provider: Box<dyn ModuleProvider>) -> Self {
        Self {
            provider,
            loaded_modules: HashMap::new(),
            module_cache: HashMap::new(),
        }
    }

    pub fn load_module(&mut self, specifier: &str, from: Option<&str>) -> Result<Value, ApiError> {
        let resolution = self.provider.resolve_module(specifier, from)?;

        if let Some(cached) = self.module_cache.get(&resolution.module_id) {
            return Ok(cached.clone());
        }

        let source = self.provider.load_module(&resolution)?;
        let module_info = ModuleInfo::new(resolution.module_id.clone(), source);

        // Parse and execute the module
        let module_value = self.execute_module(&module_info)?;

        // Cache the result
        let module_id = resolution.module_id.clone();
        self.module_cache.insert(module_id, module_value.clone());
        self.loaded_modules
            .insert(resolution.module_id, module_info);

        Ok(module_value)
    }

    fn execute_module(&self, _module_info: &ModuleInfo) -> Result<Value, ApiError> {
        // This would integrate with the engine to execute the module
        // For now, return a placeholder
        Ok(Value::Object(crate::vm::handle::create_object_handle(0)))
    }

    pub fn get_loaded_modules(&self) -> &HashMap<String, ModuleInfo> {
        &self.loaded_modules
    }

    pub fn clear_cache(&mut self) {
        self.module_cache.clear();
    }
}

pub struct ModuleRegistry {
    loaders: HashMap<String, Box<dyn ModuleProvider>>,
    default_loader: Option<Box<dyn ModuleProvider>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            loaders: HashMap::new(),
            default_loader: None,
        }
    }

    pub fn register_provider(&mut self, scheme: String, provider: Box<dyn ModuleProvider>) {
        self.loaders.insert(scheme, provider);
    }

    pub fn set_default_provider(&mut self, provider: Box<dyn ModuleProvider>) {
        self.default_loader = Some(provider);
    }

    pub fn get_provider(&self, scheme: &str) -> Option<&dyn ModuleProvider> {
        self.loaders.get(scheme).map(|p| p.as_ref())
    }

    pub fn get_default_provider(&self) -> Option<&dyn ModuleProvider> {
        self.default_loader.as_ref().map(|p| p.as_ref())
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
