use crate::cli::framework::error::{CliError, CliResult};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct InputValidator {
    allowed_extensions: HashSet<String>,
    blocked_extensions: HashSet<String>,
    max_file_size: u64,
    max_path_length: usize,
}

impl Default for InputValidator {
    fn default() -> Self {
        Self {
            allowed_extensions: HashSet::from([
                "js".to_string(),
                "ts".to_string(),
                "json".to_string(),
                "toml".to_string(),
                "yaml".to_string(),
                "yml".to_string(),
                "rs".to_string(),
                "md".to_string(),
                "txt".to_string(),
            ]),
            blocked_extensions: HashSet::from([
                "exe".to_string(),
                "bat".to_string(),
                "cmd".to_string(),
                "sh".to_string(),
                "ps1".to_string(),
                "scr".to_string(),
                "com".to_string(),
            ]),
            max_file_size: 100 * 1024 * 1024, // 100MB
            max_path_length: 260,
        }
    }
}

impl InputValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_allowed_extensions(mut self, extensions: Vec<String>) -> Self {
        self.allowed_extensions = extensions.into_iter().collect();
        self
    }

    pub fn with_blocked_extensions(mut self, extensions: Vec<String>) -> Self {
        self.blocked_extensions = extensions.into_iter().collect();
        self
    }

    pub fn with_max_file_size(mut self, size: u64) -> Self {
        self.max_file_size = size;
        self
    }

    pub fn with_max_path_length(mut self, length: usize) -> Self {
        self.max_path_length = length;
        self
    }

    pub fn validate_file_path(&self, path: &Path) -> CliResult<()> {
        if !path.exists() {
            return Err(CliError::FileNotFound {
                path: path.to_string_lossy().to_string(),
            });
        }

        if !path.is_file() {
            return Err(CliError::ValidationError {
                field: "path".to_string(),
                reason: "Path must be a file".to_string(),
            });
        }

        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();

            if self.blocked_extensions.contains(&ext) {
                return Err(CliError::ValidationError {
                    field: "extension".to_string(),
                    reason: format!("File extension '{}' is not allowed", ext),
                });
            }

            if !self.allowed_extensions.contains(&ext) {
                return Err(CliError::ValidationError {
                    field: "extension".to_string(),
                    reason: format!("File extension '{}' is not supported", ext),
                });
            }
        }

        if path.to_string_lossy().len() > self.max_path_length {
            return Err(CliError::ValidationError {
                field: "path".to_string(),
                reason: format!("Path too long (max {} characters)", self.max_path_length),
            });
        }

        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.len() > self.max_file_size {
                return Err(CliError::ValidationError {
                    field: "file_size".to_string(),
                    reason: format!("File too large (max {} bytes)", self.max_file_size),
                });
            }
        }

        Ok(())
    }

    pub fn validate_directory_path(&self, path: &Path) -> CliResult<()> {
        if !path.exists() {
            return Err(CliError::FileNotFound {
                path: path.to_string_lossy().to_string(),
            });
        }

        if !path.is_dir() {
            return Err(CliError::ValidationError {
                field: "path".to_string(),
                reason: "Path must be a directory".to_string(),
            });
        }

        if path.to_string_lossy().len() > self.max_path_length {
            return Err(CliError::ValidationError {
                field: "path".to_string(),
                reason: format!("Path too long (max {} characters)", self.max_path_length),
            });
        }

        Ok(())
    }

    pub fn validate_package_name(&self, name: &str) -> CliResult<()> {
        if name.is_empty() {
            return Err(CliError::ValidationError {
                field: "package_name".to_string(),
                reason: "Package name cannot be empty".to_string(),
            });
        }

        if name.len() > 214 {
            return Err(CliError::ValidationError {
                field: "package_name".to_string(),
                reason: "Package name too long (max 214 characters)".to_string(),
            });
        }

        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
        {
            return Err(CliError::ValidationError {
                field: "package_name".to_string(),
                reason: "Package name contains invalid characters".to_string(),
            });
        }

        if name.starts_with('.') || name.starts_with('_') {
            return Err(CliError::ValidationError {
                field: "package_name".to_string(),
                reason: "Package name cannot start with '.' or '_'".to_string(),
            });
        }

        if name.contains("..") {
            return Err(CliError::ValidationError {
                field: "package_name".to_string(),
                reason: "Package name cannot contain '..'".to_string(),
            });
        }

        Ok(())
    }

    pub fn validate_url(&self, url: &str) -> CliResult<()> {
        if url.is_empty() {
            return Err(CliError::ValidationError {
                field: "url".to_string(),
                reason: "URL cannot be empty".to_string(),
            });
        }

        if let Err(_) = url::Url::parse(url) {
            return Err(CliError::ValidationError {
                field: "url".to_string(),
                reason: "Invalid URL format".to_string(),
            });
        }

        Ok(())
    }

    pub fn validate_port(&self, port: u16) -> CliResult<()> {
        if port == 0 {
            return Err(CliError::ValidationError {
                field: "port".to_string(),
                reason: "Port cannot be 0".to_string(),
            });
        }

        if port < 1024 && port != 80 && port != 443 {
            return Err(CliError::ValidationError {
                field: "port".to_string(),
                reason: "Ports below 1024 require root privileges".to_string(),
            });
        }

        Ok(())
    }

    pub fn validate_timeout(&self, timeout_ms: u64) -> CliResult<()> {
        if timeout_ms == 0 {
            return Err(CliError::ValidationError {
                field: "timeout".to_string(),
                reason: "Timeout cannot be 0".to_string(),
            });
        }

        if timeout_ms > 300000 {
            return Err(CliError::ValidationError {
                field: "timeout".to_string(),
                reason: "Timeout too large (max 5 minutes)".to_string(),
            });
        }

        Ok(())
    }

    pub fn validate_retry_count(&self, retries: u32) -> CliResult<()> {
        if retries > 10 {
            return Err(CliError::ValidationError {
                field: "retries".to_string(),
                reason: "Too many retries (max 10)".to_string(),
            });
        }

        Ok(())
    }

    pub fn sanitize_path(&self, path: &str) -> CliResult<PathBuf> {
        let path = path.trim();

        if path.is_empty() {
            return Err(CliError::ValidationError {
                field: "path".to_string(),
                reason: "Path cannot be empty".to_string(),
            });
        }

        let path_buf = PathBuf::from(path);

        if path_buf.to_string_lossy().contains("..") {
            return Err(CliError::ValidationError {
                field: "path".to_string(),
                reason: "Path cannot contain '..'".to_string(),
            });
        }

        Ok(path_buf)
    }

    pub fn sanitize_string(&self, input: &str, max_length: usize) -> CliResult<String> {
        let sanitized = input.trim().to_string();

        if sanitized.is_empty() {
            return Err(CliError::ValidationError {
                field: "input".to_string(),
                reason: "Input cannot be empty".to_string(),
            });
        }

        if sanitized.len() > max_length {
            return Err(CliError::ValidationError {
                field: "input".to_string(),
                reason: format!("Input too long (max {} characters)", max_length),
            });
        }

        if sanitized.contains('\0') {
            return Err(CliError::ValidationError {
                field: "input".to_string(),
                reason: "Input contains null characters".to_string(),
            });
        }

        Ok(sanitized)
    }
}

pub fn validate_required_arg<T>(value: Option<T>, arg_name: &str) -> CliResult<T> {
    value.ok_or_else(|| CliError::MissingArgument {
        argument: arg_name.to_string(),
    })
}

pub fn validate_positive_number(value: i64, field_name: &str) -> CliResult<()> {
    if value <= 0 {
        return Err(CliError::ValidationError {
            field: field_name.to_string(),
            reason: "Value must be positive".to_string(),
        });
    }
    Ok(())
}

pub fn validate_range<T>(value: T, min: T, max: T, field_name: &str) -> CliResult<()>
where
    T: PartialOrd + std::fmt::Display,
{
    if value < min || value > max {
        return Err(CliError::ValidationError {
            field: field_name.to_string(),
            reason: format!("Value must be between {} and {}", min, max),
        });
    }
    Ok(())
}
