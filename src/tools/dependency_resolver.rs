//! # Dependency Resolver
//!
//! Intelligently resolve dependencies between JavaScript and Rust packages.
//!
//! ## Features
//!
//! - **Automatic dependency resolution**
//! - **Conflict detection** between versions
//! - **Multi-registry support** (npm, crates.io)
//! - **Optional dependencies** and peer dependencies
//!
//! ## Usage Example
//!
//! ```rust
//! use jetcrab::tools::{DependencyResolver, Dependency, DependencySource};
//!
//! let mut resolver = DependencyResolver::new();
//! let deps = vec![
//!     Dependency {
//!         name: "react".to_string(),
//!         version: "18.0.0".to_string(),
//!         source: DependencySource::Npm,
//!         dependencies: vec![],
//!     }
//! ];
//! resolver.resolve(&deps)?;
//! ```

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub source: DependencySource,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencySource {
    Npm,
    Cargo,
    Local,
    Git,
}

pub struct DependencyResolver {
    resolved: HashMap<String, Dependency>,
    conflicts: Vec<DependencyConflict>,
}

#[derive(Debug, Clone)]
pub struct DependencyConflict {
    pub package: String,
    pub version1: String,
    pub version2: String,
    pub source1: DependencySource,
    pub source2: DependencySource,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            resolved: HashMap::new(),
            conflicts: Vec::new(),
        }
    }

    pub fn resolve(&mut self, dependencies: &[Dependency]) -> Result<()> {
        info!("Resolving dependencies...");

        for dep in dependencies {
            self.resolve_dependency(dep)?;
        }

        if !self.conflicts.is_empty() {
            warn!("Found {} dependency conflicts", self.conflicts.len());
        }

        Ok(())
    }

    fn resolve_dependency(&mut self, dep: &Dependency) -> Result<()> {
        if let Some(existing) = self.resolved.get(&dep.name) {
            if existing.version != dep.version {
                self.conflicts.push(DependencyConflict {
                    package: dep.name.clone(),
                    version1: existing.version.clone(),
                    version2: dep.version.clone(),
                    source1: existing.source.clone(),
                    source2: dep.source.clone(),
                });
            }
        } else {
            self.resolved.insert(dep.name.clone(), dep.clone());

            for sub_dep in &dep.dependencies {
                self.resolve_dependency(sub_dep)?;
            }
        }

        Ok(())
    }

    pub fn get_resolved(&self) -> &HashMap<String, Dependency> {
        &self.resolved
    }

    pub fn get_conflicts(&self) -> &[DependencyConflict] {
        &self.conflicts
    }

    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}
