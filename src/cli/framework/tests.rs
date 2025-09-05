#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::framework::{
        load_config, validate_positive_number, validate_range, validate_required_arg, CliApp,
        CliCommand, CliConfig, CliContext, CliError, CliMetrics, CliResult, CommandTimer,
        CompletionGenerator, CompletionItem, CompletionType, ConfigManager, InputValidator,
        MetricsCollector, ProgressBar, ProgressTracker, Spinner,
    };
    use std::path::PathBuf;
    use std::time::Duration;

    struct MockCommand {
        name: String,
        should_fail: bool,
    }

    impl MockCommand {
        fn new(name: String, should_fail: bool) -> Self {
            Self { name, should_fail }
        }
    }

    impl CliCommand for MockCommand {
        fn name(&self) -> &'static str {
            "mock"
        }

        fn description(&self) -> &'static str {
            "Mock command for testing"
        }

        fn execute(&self, _context: &mut CliContext) -> CliResult<()> {
            if self.should_fail {
                Err(CliError::InternalError {
                    message: "Mock command failed".to_string(),
                })
            } else {
                Ok(())
            }
        }

        fn validate_args(&self, _args: &[String]) -> CliResult<()> {
            Ok(())
        }

        fn help(&self) -> String {
            "Mock command help".to_string()
        }
    }

    #[test]
    fn test_cli_app_creation() {
        let app = CliApp::new(
            "test".to_string(),
            "0.4.0".to_string(),
            "Test application".to_string(),
        );

        assert_eq!(app.name, "test");
        assert_eq!(app.version, "0.4.0");
        assert_eq!(app.description, "Test application");
    }

    #[test]
    fn test_cli_app_with_command() {
        let app = CliApp::new(
            "test".to_string(),
            "0.4.0".to_string(),
            "Test application".to_string(),
        );

        let command = MockCommand::new("test".to_string(), false);
        let app_with_command = app.add_command(Box::new(command));

        assert_eq!(app_with_command.commands.len(), 1);
    }

    #[test]
    fn test_cli_app_help() {
        let app = CliApp::new(
            "test".to_string(),
            "0.4.0".to_string(),
            "Test application".to_string(),
        );

        let command = MockCommand::new("test".to_string(), false);
        let app_with_command = app.add_command(Box::new(command));

        app_with_command.show_help();
        app_with_command.show_version();
    }

    #[test]
    fn test_cli_context_creation() {
        let config = CliConfig::default();
        let context = CliContext::new(config, false, false);

        assert!(!context.verbose);
        assert!(!context.debug);
        assert!(!context.session_id.is_empty());
    }

    #[test]
    fn test_cli_context_with_user_id() {
        let config = CliConfig::default();
        let context = CliContext::new(config, false, false).with_user_id("test_user".to_string());

        assert_eq!(context.user_id, Some("test_user".to_string()));
    }

    #[test]
    fn test_cli_error_exit_codes() {
        let invalid_arg = CliError::InvalidArgument {
            argument: "test".to_string(),
            reason: "test reason".to_string(),
        };
        assert_eq!(invalid_arg.exit_code(), 2);

        let missing_arg = CliError::MissingArgument {
            argument: "test".to_string(),
        };
        assert_eq!(missing_arg.exit_code(), 2);

        let permission_denied = CliError::PermissionDenied {
            operation: "read".to_string(),
            resource: "file".to_string(),
        };
        assert_eq!(permission_denied.exit_code(), 13);

        let internal_error = CliError::InternalError {
            message: "test".to_string(),
        };
        assert_eq!(internal_error.exit_code(), 70);
        assert!(internal_error.is_fatal());
    }

    #[test]
    fn test_input_validator_creation() {
        let validator = InputValidator::new();
        assert!(!validator.allowed_extensions.is_empty());
        assert!(!validator.blocked_extensions.is_empty());
    }

    #[test]
    fn test_input_validator_with_custom_extensions() {
        let validator = InputValidator::new()
            .with_allowed_extensions(vec!["rs".to_string(), "toml".to_string()])
            .with_blocked_extensions(vec!["exe".to_string()]);

        assert!(validator.allowed_extensions.contains(&"rs".to_string()));
        assert!(validator.allowed_extensions.contains(&"toml".to_string()));
        assert!(validator.blocked_extensions.contains(&"exe".to_string()));
    }

    #[test]
    fn test_validate_required_arg() {
        let value = Some("test".to_string());
        let result = validate_required_arg(value, "test_arg");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test".to_string());

        let none_value: Option<String> = None;
        let result = validate_required_arg(none_value, "test_arg");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_positive_number() {
        assert!(validate_positive_number(5, "test").is_ok());
        assert!(validate_positive_number(0, "test").is_err());
        assert!(validate_positive_number(-1, "test").is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, 1, 10, "test").is_ok());
        assert!(validate_range(0, 1, 10, "test").is_err());
        assert!(validate_range(11, 1, 10, "test").is_err());
    }

    #[test]
    fn test_config_manager_creation() {
        let manager = ConfigManager::new(None);
        assert!(manager
            .get_config_path()
            .to_string_lossy()
            .contains("jetcrab"));
    }

    #[test]
    fn test_config_manager_with_custom_path() {
        let custom_path = PathBuf::from("/tmp/test_config.toml");
        let manager = ConfigManager::new(Some(custom_path.clone()));
        assert_eq!(manager.get_config_path(), &custom_path);
    }

    #[test]
    fn test_config_validation() {
        let mut manager = ConfigManager::new(None);
        manager.get_mut().general.default_timeout_ms = 0;

        let result = manager.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new(true);
        let metrics = collector.get_metrics();
        assert!(!metrics.session_id.is_empty());
    }

    #[test]
    fn test_metrics_collector_command_timing() {
        let collector = MetricsCollector::new(true);

        collector.record_command_success("test_command".to_string(), Duration::from_millis(100));
        let metrics = collector.get_metrics();
        assert_eq!(metrics.commands_executed, 1);
        assert_eq!(metrics.commands_failed, 0);

        collector.record_command_failure(
            "test_command".to_string(),
            Duration::from_millis(50),
            "test error",
        );
        let metrics = collector.get_metrics();
        assert_eq!(metrics.commands_executed, 1);
        assert_eq!(metrics.commands_failed, 1);
    }

    #[test]
    fn test_command_timer() {
        let collector = MetricsCollector::new(true);
        let timer = collector
            .record_command_start("test_command".to_string())
            .unwrap();

        std::thread::sleep(Duration::from_millis(10));

        timer.finish_success();

        let metrics = collector.get_metrics();
        assert_eq!(metrics.commands_executed, 1);
    }

    #[test]
    fn test_progress_bar_creation() {
        let progress_bar = ProgressBar::new(100);
        assert_eq!(progress_bar.total, 100);
    }

    #[test]
    fn test_progress_bar_with_options() {
        let progress_bar = ProgressBar::new(100)
            .with_width(30)
            .with_prefix("Test: ".to_string())
            .with_suffix(" %".to_string())
            .show_percentage(true)
            .show_eta(true)
            .show_speed(true);

        assert_eq!(progress_bar.width, 30);
        assert_eq!(progress_bar.prefix, "Test: ");
        assert_eq!(progress_bar.suffix, " %");
        assert!(progress_bar.show_percentage);
        assert!(progress_bar.show_eta);
        assert!(progress_bar.show_speed);
    }

    #[test]
    fn test_progress_bar_operations() {
        let progress_bar = ProgressBar::new(100);

        progress_bar.inc(10);
        assert_eq!(*progress_bar.current.lock().unwrap(), 10);

        progress_bar.set(50);
        assert_eq!(*progress_bar.current.lock().unwrap(), 50);

        progress_bar.inc(60); // Should cap at 100
        assert_eq!(*progress_bar.current.lock().unwrap(), 100);
    }

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new("Testing...".to_string());
        assert_eq!(spinner.message, "Testing...");
        assert!(!spinner.frames.is_empty());
    }

    #[test]
    fn test_spinner_with_custom_frames() {
        let custom_frames = vec!["-", "\\", "|", "/"];
        let spinner = Spinner::new("Testing...".to_string()).with_frames(custom_frames.clone());

        assert_eq!(spinner.frames, custom_frames);
    }

    #[test]
    fn test_progress_tracker_creation() {
        let tracker = ProgressTracker::new(10);
        assert_eq!(tracker.total_operations, 10);
        assert_eq!(tracker.completed_operations, 0);
        assert_eq!(tracker.failed_operations, 0);
    }

    #[test]
    fn test_progress_tracker_operations() {
        let mut tracker = ProgressTracker::new(5);

        tracker.start_operation("test_op".to_string());
        assert_eq!(tracker.current_operation, "test_op");

        tracker.complete_operation();
        assert_eq!(tracker.completed_operations, 1);
        assert!(tracker.current_operation.is_empty());

        tracker.fail_operation();
        assert_eq!(tracker.failed_operations, 1);

        assert!(!tracker.is_complete());

        for _ in 0..4 {
            tracker.complete_operation();
        }

        assert!(tracker.is_complete());
    }

    #[test]
    fn test_completion_generator_creation() {
        let generator = CompletionGenerator::new();
        assert!(!generator.file_extensions.is_empty());
        assert!(!generator.directory_blacklist.is_empty());
    }

    #[test]
    fn test_completion_generator_with_custom_extensions() {
        let mut generator = CompletionGenerator::new();
        generator.add_command_completions(
            "test".to_string(),
            vec![
                CompletionItem::new("option1".to_string())
                    .with_description("First option".to_string()),
                CompletionItem::new("option2".to_string())
                    .with_description("Second option".to_string()),
            ],
        );

        let completions = generator.generate_completions("test", "opt");
        assert_eq!(completions.len(), 2);
    }

    #[test]
    fn test_completion_item_creation() {
        let item = CompletionItem::new("test".to_string())
            .with_description("Test description".to_string())
            .with_type(CompletionType::File);

        assert_eq!(item.value, "test");
        assert_eq!(item.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_error_context_creation() {
        let context = ErrorContext::new("test_command".to_string())
            .with_user_id("test_user".to_string())
            .with_session_id("test_session".to_string())
            .add_info("key1".to_string(), "value1".to_string())
            .add_info("key2".to_string(), "value2".to_string());

        assert_eq!(context.command, "test_command");
        assert_eq!(context.user_id, Some("test_user".to_string()));
        assert_eq!(context.session_id, Some("test_session".to_string()));
        assert_eq!(context.additional_info.len(), 2);
        assert_eq!(
            context.additional_info.get("key1"),
            Some(&"value1".to_string())
        );
        assert_eq!(
            context.additional_info.get("key2"),
            Some(&"value2".to_string())
        );
    }

    #[test]
    fn test_to_cli_error_trait() {
        let result: Result<String, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ));

        let cli_result = result.to_cli_error("test operation");
        assert!(cli_result.is_err());

        if let Err(CliError::InternalError { message }) = cli_result {
            assert!(message.contains("test operation"));
            assert!(message.contains("File not found"));
        } else {
            panic!("Expected InternalError");
        }
    }

    #[test]
    fn test_metrics_export() {
        let collector = MetricsCollector::new(true);
        collector.record_command_success("test".to_string(), Duration::from_millis(100));

        let json_export = collector.export_json();
        assert!(json_export.is_ok());
        let json_str = json_export.unwrap();
        assert!(json_str.contains("test"));

        let csv_export = collector.export_csv();
        assert!(csv_export.contains("test"));
        assert!(csv_export.contains("Session ID"));
    }
}
