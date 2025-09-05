use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{self, format::Writer, time::FormatTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Compact,
    Full,
    Json,
    Pretty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(with = "level_serde")]
    pub level: Level,
    pub format: LogFormat,
    pub output: LogOutput,
    pub include_timestamp: bool,
    pub include_target: bool,
    pub include_file: bool,
    pub include_line: bool,
    pub color: bool,
    pub structured: bool,
    pub context: HashMap<String, String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            format: LogFormat::Compact,
            output: LogOutput::Stderr,
            include_timestamp: true,
            include_target: false,
            include_file: false,
            include_line: false,
            color: true,
            structured: false,
            context: HashMap::new(),
        }
    }
}

pub struct CliLogger {
    config: LoggingConfig,
}

impl CliLogger {
    pub fn new(config: LoggingConfig) -> Self {
        Self { config }
    }

    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filter = tracing_subscriber::filter::LevelFilter::from_level(self.config.level);

        let layer = match self.config.format {
            LogFormat::Compact => {
                let layer = fmt::layer()
                    .compact()
                    .with_timer(CustomTimer)
                    .with_target(self.config.include_target)
                    .with_file(self.config.include_file)
                    .with_line_number(self.config.include_line)
                    .with_ansi(self.config.color);

                match &self.config.output {
                    LogOutput::Stdout => layer.boxed(),
                    LogOutput::Stderr => layer.with_writer(std::io::stderr).boxed(),
                    LogOutput::File(path) => {
                        let file = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)?;
                        layer.with_writer(file).boxed()
                    }
                    LogOutput::Both => layer.with_writer(std::io::stderr).boxed(),
                }
            }
            LogFormat::Full => {
                let layer = fmt::layer()
                    .with_timer(CustomTimer)
                    .with_target(self.config.include_target)
                    .with_file(self.config.include_file)
                    .with_line_number(self.config.include_line)
                    .with_ansi(self.config.color);

                match &self.config.output {
                    LogOutput::Stdout => layer.boxed(),
                    LogOutput::Stderr => layer.with_writer(std::io::stderr).boxed(),
                    LogOutput::File(path) => {
                        let file = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)?;
                        layer.with_writer(file).boxed()
                    }
                    LogOutput::Both => layer.with_writer(std::io::stderr).boxed(),
                }
            }
            LogFormat::Json => {
                let layer = fmt::layer().with_timer(CustomTimer).with_ansi(false);

                match &self.config.output {
                    LogOutput::Stdout => layer.boxed(),
                    LogOutput::Stderr => layer.with_writer(std::io::stderr).boxed(),
                    LogOutput::File(path) => {
                        let file = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)?;
                        layer.with_writer(file).boxed()
                    }
                    LogOutput::Both => layer.with_writer(std::io::stderr).boxed(),
                }
            }
            LogFormat::Pretty => {
                let layer = fmt::layer()
                    .pretty()
                    .with_timer(CustomTimer)
                    .with_target(self.config.include_target)
                    .with_file(self.config.include_file)
                    .with_line_number(self.config.include_line)
                    .with_ansi(self.config.color);

                match &self.config.output {
                    LogOutput::Stdout => layer.boxed(),
                    LogOutput::Stderr => layer.with_writer(std::io::stderr).boxed(),
                    LogOutput::File(path) => {
                        let file = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)?;
                        layer.with_writer(file).boxed()
                    }
                    LogOutput::Both => layer.with_writer(std::io::stderr).boxed(),
                }
            }
        };

        tracing_subscriber::registry()
            .with(filter)
            .with(layer)
            .try_init()
            .map_err(|e| format!("Failed to initialize logging: {}", e))?;

        Ok(())
    }

    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.config.context.insert(key, value);
        self
    }

    pub fn with_level(mut self, level: Level) -> Self {
        self.config.level = level;
        self
    }

    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.config.format = format;
        self
    }

    pub fn with_output(mut self, output: LogOutput) -> Self {
        self.config.output = output;
        self
    }
}

struct CustomTimer;

impl FormatTime for CustomTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Utc::now().format("%H:%M:%S%.3f"))
    }
}

pub fn init_logging(verbose: bool, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    let level = if debug {
        Level::DEBUG
    } else if verbose {
        Level::INFO
    } else {
        Level::WARN
    };

    let config = LoggingConfig {
        level,
        format: LogFormat::Compact,
        output: LogOutput::Stderr,
        include_timestamp: true,
        include_target: false,
        include_file: false,
        include_line: false,
        color: true,
        structured: false,
        context: HashMap::new(),
    };

    let logger = CliLogger::new(config);
    match logger.init() {
        Ok(()) => Ok(()),
        Err(e) => {
            if e.to_string().contains("already been set") {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

pub fn init_structured_logging(
    verbose: bool,
    debug: bool,
    log_file: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let level = if debug {
        Level::DEBUG
    } else if verbose {
        Level::INFO
    } else {
        Level::WARN
    };

    let output = if let Some(file) = log_file {
        LogOutput::File(file)
    } else {
        LogOutput::Stderr
    };

    let config = LoggingConfig {
        level,
        format: LogFormat::Json,
        output,
        include_timestamp: true,
        include_target: true,
        include_file: true,
        include_line: true,
        color: false,
        structured: true,
        context: HashMap::new(),
    };

    let logger = CliLogger::new(config);
    logger.init()
}

mod level_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use tracing::Level;

    pub fn serialize<S>(level: &Level, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        level.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Level, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "error" => Ok(Level::ERROR),
            "warn" => Ok(Level::WARN),
            "info" => Ok(Level::INFO),
            "debug" => Ok(Level::DEBUG),
            "trace" => Ok(Level::TRACE),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid log level: {}",
                s
            ))),
        }
    }
}
