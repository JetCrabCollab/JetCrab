//! # Module Loader
//!
//! ES6 module loading and bundling for JetCrab Runtime.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::debug;

/// Module loader for JetCrab Runtime
pub struct ModuleLoader {
    modules: HashMap<String, ModuleInfo>,
    base_path: PathBuf,
}

/// Information about a loaded module
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub path: PathBuf,
    pub content: String,
    pub dependencies: Vec<String>,
    pub exports: HashMap<String, String>,
}

impl ModuleLoader {
    /// Create a new module loader
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            base_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Load a module from file path
    pub async fn load_module(
        &mut self,
        path: &Path,
    ) -> Result<ModuleInfo, Box<dyn std::error::Error>> {
        debug!("Loading module: {:?}", path);

        let module_key = path.to_string_lossy().to_string();
        if let Some(module) = self.modules.get(&module_key) {
            return Ok(module.clone());
        }

        let content = fs::read_to_string(path).await?;

        let dependencies = self.parse_dependencies(&content);
        let exports = self.parse_exports(&content);

        let module_info = ModuleInfo {
            path: path.to_path_buf(),
            content,
            dependencies,
            exports,
        };

        self.modules.insert(module_key, module_info.clone());

        debug!("Module loaded successfully: {:?}", path);
        Ok(module_info)
    }

    /// Resolve module path
    pub fn resolve_module(
        &self,
        from: &Path,
        specifier: &str,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        debug!("Resolving module: {} from {:?}", specifier, from);

        if specifier.starts_with("./") || specifier.starts_with("../") {
            let from_dir = from.parent().unwrap_or(Path::new("."));
            Ok(from_dir.join(specifier))
        } else if specifier.starts_with('/') {
            Ok(PathBuf::from(specifier))
        } else {
            Ok(self.base_path.join("node_modules").join(specifier))
        }
    }

    pub async fn bundle(&mut self, entry: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut modules: Vec<(PathBuf, String)> = Vec::new();
        let mut visited = std::collections::HashSet::new();
        self.collect_modules(entry, &mut modules, &mut visited)?;
        Ok(Self::generate_bundle(entry, &modules))
    }

    pub fn bundle_sync(&mut self, entry: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut modules: Vec<(PathBuf, String)> = Vec::new();
        let mut visited = std::collections::HashSet::new();
        self.collect_modules(entry, &mut modules, &mut visited)?;
        Ok(Self::generate_bundle(entry, &modules))
    }

    fn generate_bundle(entry: &Path, modules: &[(PathBuf, String)]) -> String {
        let mut out = String::from("(function(){const __m={};");
        out.push_str("function __run(p,c,dir){const m={exports:{}};__m[p]=m;");
        out.push_str("function req(id){let r=(dir+id.replace('./','')).replace(/\\/?$/,'');if(!r.endsWith('.js'))r+='.js';if(__m[r])return __m[r].exports;throw new Error('Module not found: '+id);}");
        out.push_str("(function(module,exports,require){const __dirname=dir;const __filename=p;eval(c);})(m,m.exports,req);return m.exports;}");
        for (path, content) in modules {
            let p = path.to_string_lossy().replace('\\', "/");
            let dir = path.parent().map(|x| x.to_string_lossy().replace('\\', "/") + "/").unwrap_or_default();
            let escaped: String = content.replace('\\', "\\\\").replace('`', "\\`");
            out.push_str(&format!("__run(\"{}\",`{}`,\"{}\");", p, escaped, dir));
        }
        out.push_str("})();");
        out
    }

    fn collect_modules(
        &mut self,
        path: &Path,
        modules: &mut Vec<(PathBuf, String)>,
        visited: &mut std::collections::HashSet<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        if visited.contains(&path) {
            return Ok(());
        }
        visited.insert(path.clone());
        let content = std::fs::read_to_string(&path)?;
        let deps = self.parse_dependencies(&content);
        for spec in &deps {
            let dep_path = Self::resolve_path(&path, spec);
            if dep_path.exists() {
                self.collect_modules(&dep_path, modules, visited)?;
            }
        }
        modules.push((path, content));
        Ok(())
    }

    /// Get module information
    pub fn get_module(&self, path: &Path) -> Option<&ModuleInfo> {
        let module_key = path.to_string_lossy().to_string();
        self.modules.get(&module_key)
    }

    /// List all loaded modules
    pub fn list_modules(&self) -> Vec<&ModuleInfo> {
        self.modules.values().collect()
    }

    /// Parse dependencies from module content
    fn parse_dependencies(&self, content: &str) -> Vec<String> {
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with("import ") {
                if let Some(specifier) = self.extract_import_specifier(line) {
                    dependencies.push(specifier);
                }
            } else if line.starts_with("require(") {
                if let Some(specifier) = self.extract_require_specifier(line) {
                    dependencies.push(specifier);
                }
            }
        }

        dependencies
    }

    /// Parse exports from module content
    fn parse_exports(&self, content: &str) -> HashMap<String, String> {
        let mut exports = HashMap::new();

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with("export ") {
                if let Some((name, value)) = self.extract_export(line) {
                    exports.insert(name, value);
                }
            }
        }

        exports
    }

    /// Extract import specifier from import statement
    fn extract_import_specifier(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find('"') {
            if let Some(end) = line[start + 1..].find('"') {
                return Some(line[start + 1..start + 1 + end].to_string());
            }
        }
        None
    }

    /// Extract require specifier from require statement
    fn extract_require_specifier(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find('"') {
            if let Some(end) = line[start + 1..].find('"') {
                return Some(line[start + 1..start + 1 + end].to_string());
            }
        }
        None
    }

    /// Extract export name and value from export statement
    fn extract_export(&self, line: &str) -> Option<(String, String)> {
        if line.contains("export const") {
            if let Some(name_start) = line.find("const ") {
                let name_part = &line[name_start + 6..];
                if let Some(name_end) = name_part.find(' ') {
                    let name = name_part[..name_end].to_string();
                    return Some((name, "const".to_string()));
                }
            }
        }
        None
    }

    fn resolve_path(from: &Path, specifier: &str) -> PathBuf {
        let from_dir = from.parent().unwrap_or(Path::new("."));
        let mut p = from_dir.join(specifier);
        if p.extension().is_none() {
            p.set_extension("js");
        }
        p
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loader_creation() {
        let loader = ModuleLoader::new();
        assert!(loader.modules.is_empty());
    }
}
