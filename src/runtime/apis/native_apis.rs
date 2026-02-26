//! # Native APIs
//!
//! Real implementation of Node.js native modules in Rust using NativeFunction.

use chitin::boa_engine::{js_string, Context, JsObject, JsResult, JsString, JsValue, NativeFunction};
use tracing::info;

/// Native APIs implementation
pub struct NativeAPIs;

impl NativeAPIs {
    pub fn new() -> Self {
        Self
    }

    /// Register all native APIs
    pub fn register(&self, context: &mut Context) -> JsResult<()> {
        info!("🦀 Registering Native APIs...");

        self.register_console(context)?;

        let http_module = crate::runtime::apis::networking::HttpAPI::new();
        http_module.register(context)?;

        let fs_module = crate::runtime::apis::filesystem::NativeFsModule::new();
        fs_module.register(context)?;

        let process_module = crate::runtime::apis::system::NativeProcessModule::new();
        process_module.register(context)?;

        let require_api = crate::runtime::apis::core::RequireAPI::new();
        require_api.register(context)?;

        let timers_module = crate::runtime::apis::utility::TimersAPI::new();
        timers_module.register(context).map_err(|_e| {
            chitin::boa_engine::JsNativeError::typ().with_message("Failed to register timers")
        })?;

        info!("✅ All Native APIs registered successfully");
        Ok(())
    }

    /// Register console object with native implementation
    fn register_console(&self, context: &mut Context) -> JsResult<()> {
        info!("📝 Registering Native Console Module...");

        let console_object = JsObject::default();

        let log_fn = NativeFunction::from_fn_ptr(Self::console_log);
        console_object.set::<JsString, JsValue>(
            js_string!("log"),
            JsValue::from(log_fn.to_js_function(context.realm())),
            false,
            context,
        )?;

        let error_fn = NativeFunction::from_fn_ptr(Self::console_error);
        console_object.set::<JsString, JsValue>(
            js_string!("error"),
            JsValue::from(error_fn.to_js_function(context.realm())),
            false,
            context,
        )?;

        let warn_fn = NativeFunction::from_fn_ptr(Self::console_warn);
        console_object.set::<JsString, JsValue>(
            js_string!("warn"),
            JsValue::from(warn_fn.to_js_function(context.realm())),
            false,
            context,
        )?;

        let info_fn = NativeFunction::from_fn_ptr(Self::console_info);
        console_object.set::<JsString, JsValue>(
            js_string!("info"),
            JsValue::from(info_fn.to_js_function(context.realm())),
            false,
            context,
        )?;

        context
            .global_object()
            .set(js_string!("console"), console_object, false, context)?;

        info!("✅ Native Console Module registered successfully");
        Ok(())
    }

    /// Native implementation of console.log
    fn console_log(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let message = Self::format_args(args, context)?;
        println!("{}", message);
        Ok(JsValue::undefined())
    }

    /// Native implementation of console.error
    fn console_error(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let message = Self::format_args(args, context)?;
        eprintln!("ERROR: {}", message);
        Ok(JsValue::undefined())
    }

    /// Native implementation of console.warn
    fn console_warn(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let message = Self::format_args(args, context)?;
        eprintln!("WARN: {}", message);
        Ok(JsValue::undefined())
    }

    /// Native implementation of console.info
    fn console_info(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let message = Self::format_args(args, context)?;
        println!("INFO: {}", message);
        Ok(JsValue::undefined())
    }

    /// Format arguments for console output
    fn format_args(args: &[JsValue], context: &mut Context) -> JsResult<String> {
        let mut parts = Vec::new();
        for arg in args {
            let str_value = arg.to_string(context)?;
            parts.push(str_value.to_std_string().map_err(|e| {
                chitin::boa_engine::JsNativeError::typ().with_message("String conversion error")
            })?);
        }
        Ok(parts.join(" "))
    }
}

impl Default for NativeAPIs {
    fn default() -> Self {
        Self::new()
    }
}
