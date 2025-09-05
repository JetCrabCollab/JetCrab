//! # Native FS Module Implementation
//!
//! Real file system operations in Rust using std::fs and tokio::fs.

use boa_engine::{js_string, Context, JsObject, JsResult, JsString, JsValue, NativeFunction};
use std::fs;
use std::path::Path;
use tracing::{error, info};

/// Native FS Module implementation
pub struct NativeFsModule;

impl NativeFsModule {
    pub fn new() -> Self {
        Self
    }

    /// Register the native FS module
    pub fn register(&self, context: &mut Context) -> JsResult<()> {
        info!("📁 Registering Native FS Module...");

        let fs_object = JsObject::default();

        let read_file_sync = NativeFunction::from_fn_ptr(Self::read_file_sync);
        fs_object.set::<JsString, JsValue>(
            js_string!("readFileSync"),
            JsValue::from(read_file_sync.to_js_function(context.realm())),
            false,
            context,
        )?;

        let write_file_sync = NativeFunction::from_fn_ptr(Self::write_file_sync);
        fs_object.set::<JsString, JsValue>(
            js_string!("writeFileSync"),
            JsValue::from(write_file_sync.to_js_function(context.realm())),
            false,
            context,
        )?;

        let exists_sync = NativeFunction::from_fn_ptr(Self::exists_sync);
        fs_object.set::<JsString, JsValue>(
            js_string!("existsSync"),
            JsValue::from(exists_sync.to_js_function(context.realm())),
            false,
            context,
        )?;

        let stat_sync = NativeFunction::from_fn_ptr(Self::stat_sync);
        fs_object.set::<JsString, JsValue>(
            js_string!("statSync"),
            JsValue::from(stat_sync.to_js_function(context.realm())),
            false,
            context,
        )?;

        let mkdir_sync = NativeFunction::from_fn_ptr(Self::mkdir_sync);
        fs_object.set::<JsString, JsValue>(
            js_string!("mkdirSync"),
            JsValue::from(mkdir_sync.to_js_function(context.realm())),
            false,
            context,
        )?;

        let rmdir_sync = NativeFunction::from_fn_ptr(Self::rmdir_sync);
        fs_object.set::<JsString, JsValue>(
            js_string!("rmdirSync"),
            JsValue::from(rmdir_sync.to_js_function(context.realm())),
            false,
            context,
        )?;

        let unlink_sync = NativeFunction::from_fn_ptr(Self::unlink_sync);
        fs_object.set::<JsString, JsValue>(
            js_string!("unlinkSync"),
            JsValue::from(unlink_sync.to_js_function(context.realm())),
            false,
            context,
        )?;

        let readdir_sync = NativeFunction::from_fn_ptr(Self::readdir_sync);
        fs_object.set::<JsString, JsValue>(
            js_string!("readdirSync"),
            JsValue::from(readdir_sync.to_js_function(context.realm())),
            false,
            context,
        )?;

        let constants = JsObject::default();
        constants.set(js_string!("F_OK"), 0, false, context)?;
        constants.set(js_string!("R_OK"), 4, false, context)?;
        constants.set(js_string!("W_OK"), 2, false, context)?;
        constants.set(js_string!("X_OK"), 1, false, context)?;
        fs_object.set(js_string!("constants"), constants, false, context)?;

        context
            .global_object()
            .set(js_string!("fs"), fs_object, false, context)?;

        info!("✅ Native FS Module registered successfully");
        Ok(())
    }

    /// Native implementation of fs.readFileSync
    fn read_file_sync(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(boa_engine::JsError::from(
                boa_engine::JsNativeError::typ().with_message("File path is required"),
            ));
        }

        let path = args[0].to_string(context)?;
        let path_str = path.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;
        info!("📖 Reading file: {}", path_str);

        match fs::read_to_string(&path_str) {
            Ok(content) => {
                info!("✅ File read successfully: {} bytes", content.len());
                Ok(js_string!(content).into())
            }
            Err(e) => {
                error!("❌ Failed to read file {}: {}", path_str, e);
                Err(boa_engine::JsError::from(
                    boa_engine::JsNativeError::typ()
                        .with_message(format!("Failed to read file: {}", e)),
                ))
            }
        }
    }

    /// Native implementation of fs.writeFileSync
    fn write_file_sync(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        if args.len() < 2 {
            return Err(boa_engine::JsError::from(
                boa_engine::JsNativeError::typ().with_message("File path and data are required"),
            ));
        }

        let path = args[0].to_string(context)?;
        let data = args[1].to_string(context)?;
        let path_str = path.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;
        let data_str = data.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;

        info!("📝 Writing file: {} ({} bytes)", path_str, data_str.len());

        match fs::write(&path_str, data_str) {
            Ok(_) => {
                info!("✅ File written successfully: {}", path_str);
                Ok(JsValue::undefined())
            }
            Err(e) => {
                error!("❌ Failed to write file {}: {}", path_str, e);
                Err(boa_engine::JsError::from(
                    boa_engine::JsNativeError::typ()
                        .with_message(format!("Failed to write file: {}", e)),
                ))
            }
        }
    }

    /// Native implementation of fs.existsSync
    fn exists_sync(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(boa_engine::JsError::from(
                boa_engine::JsNativeError::typ().with_message("File path is required"),
            ));
        }

        let path = args[0].to_string(context)?;
        let path_str = path.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;
        let exists = Path::new(&path_str).exists();

        info!("🔍 Checking if file exists: {} -> {}", path_str, exists);
        Ok(JsValue::from(exists))
    }

    /// Native implementation of fs.statSync
    fn stat_sync(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(boa_engine::JsError::from(
                boa_engine::JsNativeError::typ().with_message("File path is required"),
            ));
        }

        let path = args[0].to_string(context)?;
        let path_str = path.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;

        match fs::metadata(&path_str) {
            Ok(metadata) => {
                let stat_object = JsObject::default();
                let is_file = metadata.is_file();
                let is_dir = metadata.is_dir();

                stat_object.set(js_string!("isFile"), is_file, false, context)?;

                stat_object.set(js_string!("isDirectory"), is_dir, false, context)?;

                stat_object.set(js_string!("size"), metadata.len(), false, context)?;

                if let Ok(mtime) = metadata.modified() {
                    if let Ok(duration) = mtime.duration_since(std::time::UNIX_EPOCH) {
                        stat_object.set(
                            js_string!("mtime"),
                            JsValue::from(duration.as_secs() * 1000),
                            false,
                            context,
                        )?;
                    }
                }

                info!(
                    "📊 File stats: {} (size: {} bytes)",
                    path_str,
                    metadata.len()
                );
                Ok(stat_object.into())
            }
            Err(e) => {
                error!("❌ Failed to get file stats {}: {}", path_str, e);
                Err(boa_engine::JsError::from(
                    boa_engine::JsNativeError::typ()
                        .with_message(format!("Failed to get file stats: {}", e)),
                ))
            }
        }
    }

    /// Native implementation of fs.mkdirSync
    fn mkdir_sync(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(boa_engine::JsError::from(
                boa_engine::JsNativeError::typ().with_message("Directory path is required"),
            ));
        }

        let path = args[0].to_string(context)?;
        let path_str = path.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;

        info!("📁 Creating directory: {}", path_str);

        match fs::create_dir_all(&path_str) {
            Ok(_) => {
                info!("✅ Directory created successfully: {}", path_str);
                Ok(JsValue::undefined())
            }
            Err(e) => {
                error!("❌ Failed to create directory {}: {}", path_str, e);
                Err(boa_engine::JsError::from(
                    boa_engine::JsNativeError::typ()
                        .with_message(format!("Failed to create directory: {}", e)),
                ))
            }
        }
    }

    /// Native implementation of fs.rmdirSync
    fn rmdir_sync(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(boa_engine::JsError::from(
                boa_engine::JsNativeError::typ().with_message("Directory path is required"),
            ));
        }

        let path = args[0].to_string(context)?;
        let path_str = path.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;

        info!("🗑️ Removing directory: {}", path_str);

        match fs::remove_dir(&path_str) {
            Ok(_) => {
                info!("✅ Directory removed successfully: {}", path_str);
                Ok(JsValue::undefined())
            }
            Err(e) => {
                error!("❌ Failed to remove directory {}: {}", path_str, e);
                Err(boa_engine::JsError::from(
                    boa_engine::JsNativeError::typ()
                        .with_message(format!("Failed to remove directory: {}", e)),
                ))
            }
        }
    }

    /// Native implementation of fs.unlinkSync
    fn unlink_sync(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(boa_engine::JsError::from(
                boa_engine::JsNativeError::typ().with_message("File path is required"),
            ));
        }

        let path = args[0].to_string(context)?;
        let path_str = path.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;

        info!("🗑️ Removing file: {}", path_str);

        match fs::remove_file(&path_str) {
            Ok(_) => {
                info!("✅ File removed successfully: {}", path_str);
                Ok(JsValue::undefined())
            }
            Err(e) => {
                error!("❌ Failed to remove file {}: {}", path_str, e);
                Err(boa_engine::JsError::from(
                    boa_engine::JsNativeError::typ()
                        .with_message(format!("Failed to remove file: {}", e)),
                ))
            }
        }
    }

    /// Native implementation of fs.readdirSync
    fn readdir_sync(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(boa_engine::JsError::from(
                boa_engine::JsNativeError::typ().with_message("Directory path is required"),
            ));
        }

        let path = args[0].to_string(context)?;
        let path_str = path.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;

        info!("📂 Reading directory: {}", path_str);

        match fs::read_dir(&path_str) {
            Ok(entries) => {
                let mut files = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(file_name) = entry.file_name().to_str() {
                            files.push(js_string!(file_name).into());
                        }
                    }
                }

                info!("✅ Directory read successfully: {} files", files.len());
                Ok(JsValue::from(
                    boa_engine::object::builtins::JsArray::from_iter(files, context),
                ))
            }
            Err(e) => {
                error!("❌ Failed to read directory {}: {}", path_str, e);
                Err(boa_engine::JsError::from(
                    boa_engine::JsNativeError::typ()
                        .with_message(format!("Failed to read directory: {}", e)),
                ))
            }
        }
    }
}

impl Default for NativeFsModule {
    fn default() -> Self {
        Self::new()
    }
}
