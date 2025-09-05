//! # Native Process Module Implementation
//!
//! Real process operations in Rust using std::process and std::env.

use boa_engine::{js_string, Context, JsObject, JsResult, JsString, JsValue, NativeFunction};
use std::env;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

/// Native Process Module implementation
pub struct NativeProcessModule {
    start_time: SystemTime,
}

impl NativeProcessModule {
    pub fn new() -> Self {
        Self {
            start_time: SystemTime::now(),
        }
    }

    /// Register the native Process module
    pub fn register(&self, context: &mut Context) -> JsResult<()> {
        info!("⚙️ Registering Native Process Module...");

        let process_object = JsObject::default();

        process_object.set(js_string!("version"), js_string!("0.4.0"), false, context)?;
        process_object.set(
            js_string!("platform"),
            js_string!("jetcrab"),
            false,
            context,
        )?;
        process_object.set(js_string!("arch"), js_string!("x64"), false, context)?;
        process_object.set(js_string!("pid"), process::id(), false, context)?;
        process_object.set(js_string!("ppid"), std::process::id(), false, context)?;

        let uptime = NativeFunction::from_fn_ptr(Self::uptime);
        process_object.set::<JsString, JsValue>(
            js_string!("uptime"),
            JsValue::from(uptime.to_js_function(context.realm())),
            false,
            context,
        )?;

        let memory_usage = NativeFunction::from_fn_ptr(Self::memory_usage);
        process_object.set::<JsString, JsValue>(
            js_string!("memoryUsage"),
            JsValue::from(memory_usage.to_js_function(context.realm())),
            false,
            context,
        )?;

        let cwd = NativeFunction::from_fn_ptr(Self::cwd);
        process_object.set::<JsString, JsValue>(
            js_string!("cwd"),
            JsValue::from(cwd.to_js_function(context.realm())),
            false,
            context,
        )?;

        let chdir = NativeFunction::from_fn_ptr(Self::chdir);
        process_object.set::<JsString, JsValue>(
            js_string!("chdir"),
            JsValue::from(chdir.to_js_function(context.realm())),
            false,
            context,
        )?;

        let exit = NativeFunction::from_fn_ptr(Self::exit);
        process_object.set::<JsString, JsValue>(
            js_string!("exit"),
            JsValue::from(exit.to_js_function(context.realm())),
            false,
            context,
        )?;

        let argv = self.get_argv(context)?;
        process_object.set::<JsString, JsValue>(js_string!("argv"), argv, false, context)?;

        let env_obj = self.get_env(context)?;
        process_object.set::<JsString, JsValue>(js_string!("env"), env_obj, false, context)?;

        context
            .global_object()
            .set(js_string!("process"), process_object, false, context)?;

        info!("✅ Native Process Module registered successfully");
        Ok(())
    }

    /// Native implementation of process.uptime
    fn uptime(_this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let start_time = SystemTime::now();
        let uptime = start_time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as f64;

        info!("⏱️ Process uptime: {} seconds", uptime);
        Ok(JsValue::from(uptime))
    }

    /// Native implementation of process.memoryUsage
    fn memory_usage(
        _this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let memory_obj = JsObject::default();

        let rss = 1024 * 1024; // 1MB simulated
        let heap_total = 512 * 1024; // 512KB simulated
        let heap_used = 256 * 1024; // 256KB simulated
        let external = 64 * 1024; // 64KB simulated

        memory_obj.set(js_string!("rss"), rss, false, context)?;
        memory_obj.set(js_string!("heapTotal"), heap_total, false, context)?;
        memory_obj.set(js_string!("heapUsed"), heap_used, false, context)?;
        memory_obj.set(js_string!("external"), external, false, context)?;

        info!(
            "💾 Memory usage: rss={}, heapTotal={}, heapUsed={}, external={}",
            rss, heap_total, heap_used, external
        );

        Ok(memory_obj.into())
    }

    /// Native implementation of process.cwd
    fn cwd(_this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        match env::current_dir() {
            Ok(path) => {
                let cwd = path.to_string_lossy().to_string();
                info!("📁 Current working directory: {}", cwd);
                Ok(js_string!(cwd).into())
            }
            Err(e) => {
                info!("❌ Failed to get current directory: {}", e);
                Ok(js_string!(".").into())
            }
        }
    }

    /// Native implementation of process.chdir
    fn chdir(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(boa_engine::JsError::from(
                boa_engine::JsNativeError::typ().with_message("Directory path is required"),
            ));
        }

        let path = args[0].to_string(context)?;
        let path_str = path.to_std_string().map_err(|_e| {
            boa_engine::JsNativeError::typ().with_message("String conversion error")
        })?;

        match env::set_current_dir(&path_str) {
            Ok(_) => {
                info!("📁 Changed directory to: {}", path_str);
                Ok(JsValue::undefined())
            }
            Err(e) => {
                info!("❌ Failed to change directory to {}: {}", path_str, e);
                Err(boa_engine::JsError::from(
                    boa_engine::JsNativeError::typ()
                        .with_message(format!("Failed to change directory: {}", e)),
                ))
            }
        }
    }

    /// Native implementation of process.exit
    fn exit(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let code = if !args.is_empty() {
            args[0].to_number(context)? as i32
        } else {
            0
        };

        info!("🚪 Process exiting with code: {}", code);
        process::exit(code);
    }

    /// Get process arguments
    fn get_argv(&self, context: &mut Context) -> JsResult<JsValue> {
        let args: Vec<String> = env::args().collect();
        let mut js_args = Vec::new();

        for arg in args {
            js_args.push(js_string!(arg).into());
        }

        info!("📝 Process arguments: {:?}", js_args);
        Ok(JsValue::from(
            boa_engine::object::builtins::JsArray::from_iter(js_args, context),
        ))
    }

    /// Get environment variables
    fn get_env(&self, context: &mut Context) -> JsResult<JsValue> {
        let env_obj = JsObject::default();

        for (key, value) in env::vars() {
            env_obj.set(js_string!(key), js_string!(value), false, context)?;
        }

        info!(
            "🌍 Environment variables loaded: {} vars",
            env::vars().count()
        );
        Ok(env_obj.into())
    }
}

impl Default for NativeProcessModule {
    fn default() -> Self {
        Self::new()
    }
}
