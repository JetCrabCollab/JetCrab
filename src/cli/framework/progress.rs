use crate::cli::framework::{CliContext, CliError, CliResult};
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ProgressBar {
    total: u64,
    current: Arc<Mutex<u64>>,
    start_time: Instant,
    width: usize,
    show_percentage: bool,
    show_eta: bool,
    show_speed: bool,
    prefix: String,
    suffix: String,
}

impl ProgressBar {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            current: Arc::new(Mutex::new(0)),
            start_time: Instant::now(),
            width: 50,
            show_percentage: true,
            show_eta: true,
            show_speed: true,
            prefix: String::new(),
            suffix: String::new(),
        }
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn with_suffix(mut self, suffix: String) -> Self {
        self.suffix = suffix;
        self
    }

    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    pub fn show_eta(mut self, show: bool) -> Self {
        self.show_eta = show;
        self
    }

    pub fn show_speed(mut self, show: bool) -> Self {
        self.show_speed = show;
        self
    }

    pub fn inc(&self, delta: u64) {
        if let Ok(mut current) = self.current.lock() {
            *current = (*current + delta).min(self.total);
        }
    }

    pub fn set(&self, current: u64) {
        if let Ok(mut cur) = self.current.lock() {
            *cur = current.min(self.total);
        }
    }

    pub fn finish(&self) {
        self.set(self.total);
        self.display();
        println!();
    }

    pub fn display(&self) {
        let current = self.current.lock().unwrap();
        let percentage = if self.total > 0 {
            (*current as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        let filled = (percentage / 100.0 * self.width as f64) as usize;
        let bar = "█".repeat(filled) + &"░".repeat(self.width - filled);

        let mut output = format!("\r{} [{}] ", self.prefix, bar);

        if self.show_percentage {
            output.push_str(&format!("{:.1}% ", percentage));
        }

        if self.show_speed && self.start_time.elapsed().as_secs() > 0 {
            let speed = *current as f64 / self.start_time.elapsed().as_secs_f64();
            output.push_str(&format!("{:.1}/s ", speed));
        }

        if self.show_eta && *current > 0 && *current < self.total {
            let elapsed = self.start_time.elapsed();
            let eta = Duration::from_secs_f64(
                (elapsed.as_secs_f64() / *current as f64) * (self.total - *current) as f64,
            );
            output.push_str(&format!("ETA: {:?} ", eta));
        }

        output.push_str(&self.suffix);

        print!("{}", output);
        io::stdout().flush().unwrap();
    }
}

pub struct Spinner {
    frames: Vec<&'static str>,
    current_frame: usize,
    message: String,
    running: Arc<Mutex<bool>>,
}

impl Spinner {
    pub fn new(message: String) -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current_frame: 0,
            message,
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn with_frames(mut self, frames: Vec<&'static str>) -> Self {
        self.frames = frames;
        self
    }

    pub fn start(&self) {
        *self.running.lock().unwrap() = true;
        let running = Arc::clone(&self.running);
        let frames = self.frames.clone();
        let message = self.message.clone();

        thread::spawn(move || {
            let mut current_frame = 0;
            while *running.lock().unwrap() {
                print!("\r{} {}", frames[current_frame], message);
                io::stdout().flush().unwrap();
                current_frame = (current_frame + 1) % frames.len();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
        print!("\r{}", " ".repeat(self.message.len() + 3));
        print!("\r");
        io::stdout().flush().unwrap();
    }

    pub fn update_message(&mut self, message: String) {
        self.message = message;
    }
}

pub struct MultiProgress {
    bars: Vec<ProgressBar>,
    current_bar: usize,
}

impl MultiProgress {
    pub fn new() -> Self {
        Self {
            bars: Vec::new(),
            current_bar: 0,
        }
    }

    pub fn add_bar(&mut self, bar: ProgressBar) -> usize {
        let index = self.bars.len();
        self.bars.push(bar);
        index
    }

    pub fn get_bar(&self, index: usize) -> Option<&ProgressBar> {
        self.bars.get(index)
    }

    pub fn get_bar_mut(&mut self, index: usize) -> Option<&mut ProgressBar> {
        self.bars.get_mut(index)
    }

    pub fn display_all(&self) {
        for (i, bar) in self.bars.iter().enumerate() {
            if i > 0 {
                println!();
            }
            bar.display();
        }
    }
}

pub struct ProgressTracker {
    total_operations: u64,
    completed_operations: u64,
    failed_operations: u64,
    start_time: Instant,
    current_operation: String,
}

impl ProgressTracker {
    pub fn new(total_operations: u64) -> Self {
        Self {
            total_operations,
            completed_operations: 0,
            failed_operations: 0,
            start_time: Instant::now(),
            current_operation: String::new(),
        }
    }

    pub fn start_operation(&mut self, operation: String) {
        self.current_operation = operation;
    }

    pub fn complete_operation(&mut self) {
        self.completed_operations += 1;
        self.current_operation.clear();
    }

    pub fn fail_operation(&mut self) {
        self.failed_operations += 1;
        self.current_operation.clear();
    }

    pub fn get_status(&self) -> String {
        let total = self.completed_operations + self.failed_operations;
        let percentage = if self.total_operations > 0 {
            (total as f64 / self.total_operations as f64) * 100.0
        } else {
            0.0
        };

        let elapsed = self.start_time.elapsed();
        let rate = if elapsed.as_secs() > 0 {
            total as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        format!(
            "Progress: {}/{} ({:.1}%) | Completed: {} | Failed: {} | Rate: {:.1}/s | Elapsed: {:?}",
            total,
            self.total_operations,
            percentage,
            self.completed_operations,
            self.failed_operations,
            rate,
            elapsed
        )
    }

    pub fn is_complete(&self) -> bool {
        (self.completed_operations + self.failed_operations) >= self.total_operations
    }
}

pub fn show_progress_indicator<F, T>(
    message: &str,
    operation: F,
) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnOnce() -> Result<T, Box<dyn std::error::Error>>,
{
    let spinner = Spinner::new(message.to_string());
    spinner.start();

    let result = operation();

    spinner.stop();

    match &result {
        Ok(_) => println!("✅ {}", message),
        Err(_) => println!("❌ {}", message),
    }

    result
}

pub fn show_progress_bar<F, T>(
    total: u64,
    message: &str,
    operation: F,
) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnOnce(&ProgressBar) -> Result<T, Box<dyn std::error::Error>>,
{
    let progress_bar = ProgressBar::new(total)
        .with_prefix(message.to_string())
        .show_percentage(true)
        .show_eta(true)
        .show_speed(true);

    let result = operation(&progress_bar);

    progress_bar.finish();

    match &result {
        Ok(_) => println!("✅ {} completed", message),
        Err(_) => println!("❌ {} failed", message),
    }

    result
}

#[derive(Debug, Clone)]
pub struct AdvancedProgressBar {
    total: u64,
    current: Arc<Mutex<u64>>,
    start_time: Instant,
    width: usize,
    show_percentage: bool,
    show_eta: bool,
    show_speed: bool,
    show_elapsed: bool,
    show_remaining: bool,
    prefix: String,
    suffix: String,
    style: ProgressStyle,
    color: bool,
    template: String,
    rate_limit: Duration,
    last_update: Arc<Mutex<Instant>>,
}

#[derive(Debug, Clone)]
pub enum ProgressStyle {
    Bar,
    Spinner,
    Percentage,
    Minimal,
    Custom(String),
}

impl AdvancedProgressBar {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            current: Arc::new(Mutex::new(0)),
            start_time: Instant::now(),
            width: 50,
            show_percentage: true,
            show_eta: true,
            show_speed: true,
            show_elapsed: true,
            show_remaining: true,
            prefix: String::new(),
            suffix: String::new(),
            style: ProgressStyle::Bar,
            color: true,
            template: String::new(),
            rate_limit: Duration::from_millis(100),
            last_update: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn with_suffix(mut self, suffix: String) -> Self {
        self.suffix = suffix;
        self
    }

    pub fn with_style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    pub fn with_template(mut self, template: String) -> Self {
        self.template = template;
        self
    }

    pub fn with_rate_limit(mut self, rate_limit: Duration) -> Self {
        self.rate_limit = rate_limit;
        self
    }

    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    pub fn show_eta(mut self, show: bool) -> Self {
        self.show_eta = show;
        self
    }

    pub fn show_speed(mut self, show: bool) -> Self {
        self.show_speed = show;
        self
    }

    pub fn show_elapsed(mut self, show: bool) -> Self {
        self.show_elapsed = show;
        self
    }

    pub fn show_remaining(mut self, show: bool) -> Self {
        self.show_remaining = show;
        self
    }

    pub fn inc(&self, delta: u64) {
        if let Ok(mut current) = self.current.lock() {
            *current = (*current + delta).min(self.total);
        }
        self.update_if_needed();
    }

    pub fn set(&self, current: u64) {
        if let Ok(mut cur) = self.current.lock() {
            *cur = current.min(self.total);
        }
        self.update_if_needed();
    }

    pub fn finish(&self) {
        self.set(self.total);
        self.display();
        println!();
    }

    fn update_if_needed(&self) {
        let now = Instant::now();
        if let Ok(mut last_update) = self.last_update.lock() {
            if now.duration_since(*last_update) >= self.rate_limit {
                self.display();
                *last_update = now;
            }
        }
    }

    pub fn display(&self) {
        let current = self.current.lock().unwrap();
        let elapsed = self.start_time.elapsed();

        let output = match &self.style {
            ProgressStyle::Bar => self.format_bar(*current, elapsed),
            ProgressStyle::Spinner => self.format_spinner(*current, elapsed),
            ProgressStyle::Percentage => self.format_percentage(*current, elapsed),
            ProgressStyle::Minimal => self.format_minimal(*current, elapsed),
            ProgressStyle::Custom(template) => self.format_custom(*current, elapsed, template),
        };

        print!("\r{}", output);
        io::stdout().flush().unwrap();
    }

    fn format_bar(&self, current: u64, elapsed: Duration) -> String {
        let percentage = if self.total > 0 {
            (current as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        let filled = (percentage / 100.0 * self.width as f64) as usize;
        let bar = "█".repeat(filled) + &"░".repeat(self.width - filled);

        let mut output = format!("\r{} [{}] ", self.prefix, bar);

        if self.show_percentage {
            output.push_str(&format!("{:.1}% ", percentage));
        }

        if self.show_speed && elapsed.as_secs() > 0 {
            let speed = current as f64 / elapsed.as_secs_f64();
            output.push_str(&format!("{:.1}/s ", speed));
        }

        if self.show_eta && current > 0 && current < self.total {
            let eta = Duration::from_secs_f64(
                (elapsed.as_secs_f64() / current as f64) * (self.total - current) as f64,
            );
            output.push_str(&format!("ETA: {:?} ", eta));
        }

        if self.show_elapsed {
            output.push_str(&format!("Elapsed: {:?} ", elapsed));
        }

        if self.show_remaining && current < self.total {
            let remaining = self.total - current;
            output.push_str(&format!("Remaining: {} ", remaining));
        }

        output.push_str(&self.suffix);
        output
    }

    fn format_spinner(&self, current: u64, elapsed: Duration) -> String {
        let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame_index = (elapsed.as_millis() / 100) as usize % frames.len();
        let spinner = frames[frame_index];

        let mut output = format!("\r{} {} ", self.prefix, spinner);

        if self.show_percentage && self.total > 0 {
            let percentage = (current as f64 / self.total as f64) * 100.0;
            output.push_str(&format!("{:.1}% ", percentage));
        }

        if self.show_speed && elapsed.as_secs() > 0 {
            let speed = current as f64 / elapsed.as_secs_f64();
            output.push_str(&format!("{:.1}/s ", speed));
        }

        output.push_str(&self.suffix);
        output
    }

    fn format_percentage(&self, current: u64, elapsed: Duration) -> String {
        let percentage = if self.total > 0 {
            (current as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        let mut output = format!("\r{} {:.1}% ", self.prefix, percentage);

        if self.show_speed && elapsed.as_secs() > 0 {
            let speed = current as f64 / elapsed.as_secs_f64();
            output.push_str(&format!("({:.1}/s) ", speed));
        }

        if self.show_elapsed {
            output.push_str(&format!("{:?} ", elapsed));
        }

        output.push_str(&self.suffix);
        output
    }

    fn format_minimal(&self, current: u64, elapsed: Duration) -> String {
        let mut output = format!("\r{} {}/{} ", self.prefix, current, self.total);

        if self.show_speed && elapsed.as_secs() > 0 {
            let speed = current as f64 / elapsed.as_secs_f64();
            output.push_str(&format!("({:.1}/s) ", speed));
        }

        output.push_str(&self.suffix);
        output
    }

    fn format_custom(&self, current: u64, elapsed: Duration, template: &str) -> String {
        let percentage = if self.total > 0 {
            (current as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        let speed = if elapsed.as_secs() > 0 {
            current as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        let eta = if current > 0 && current < self.total {
            Duration::from_secs_f64(
                (elapsed.as_secs_f64() / current as f64) * (self.total - current) as f64,
            )
        } else {
            Duration::ZERO
        };

        let remaining = if current < self.total {
            self.total - current
        } else {
            0
        };

        template
            .replace("{prefix}", &self.prefix)
            .replace("{suffix}", &self.suffix)
            .replace("{current}", &current.to_string())
            .replace("{total}", &self.total.to_string())
            .replace("{percentage}", &format!("{:.1}", percentage))
            .replace("{speed}", &format!("{:.1}", speed))
            .replace("{eta}", &format!("{:?}", eta))
            .replace("{elapsed}", &format!("{:?}", elapsed))
            .replace("{remaining}", &remaining.to_string())
    }
}

pub struct ProgressManager {
    bars: Arc<Mutex<HashMap<String, Arc<AdvancedProgressBar>>>>,
    context: Option<CliContext>,
    auto_cleanup: bool,
    max_bars: usize,
}

impl ProgressManager {
    pub fn new() -> Self {
        Self {
            bars: Arc::new(Mutex::new(HashMap::new())),
            context: None,
            auto_cleanup: true,
            max_bars: 10,
        }
    }

    pub fn with_context(mut self, context: CliContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_auto_cleanup(mut self, enabled: bool) -> Self {
        self.auto_cleanup = enabled;
        self
    }

    pub fn with_max_bars(mut self, max_bars: usize) -> Self {
        self.max_bars = max_bars;
        self
    }

    pub fn create_bar(&self, id: String, total: u64) -> CliResult<Arc<AdvancedProgressBar>> {
        let mut bars = self.bars.lock().unwrap();

        if bars.len() >= self.max_bars {
            return Err(CliError::ValidationError {
                field: "max_bars".to_string(),
                reason: format!(
                    "Maximum number of progress bars ({}) exceeded",
                    self.max_bars
                ),
            });
        }

        let bar = Arc::new(AdvancedProgressBar::new(total));
        bars.insert(id.clone(), bar.clone());

        Ok(bar)
    }

    pub fn get_bar(&self, id: &str) -> Option<Arc<AdvancedProgressBar>> {
        let bars = self.bars.lock().unwrap();
        bars.get(id).cloned()
    }

    pub fn remove_bar(&self, id: &str) -> CliResult<()> {
        let mut bars = self.bars.lock().unwrap();

        if bars.remove(id).is_some() {
            Ok(())
        } else {
            Err(CliError::ValidationError {
                field: "id".to_string(),
                reason: format!("Progress bar '{}' not found", id),
            })
        }
    }

    pub fn list_bars(&self) -> Vec<String> {
        let bars = self.bars.lock().unwrap();
        bars.keys().cloned().collect()
    }

    pub fn clear_all(&self) {
        let mut bars = self.bars.lock().unwrap();
        bars.clear();
    }

    pub fn cleanup_finished(&self) {
        let mut bars = self.bars.lock().unwrap();
        bars.retain(|_, bar| {
            let current = bar.current.lock().unwrap();
            *current < bar.total
        });
    }
}

impl Default for ProgressManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ProgressContext {
    manager: Arc<ProgressManager>,
    current_bar: Option<String>,
}

impl ProgressContext {
    pub fn new(manager: Arc<ProgressManager>) -> Self {
        Self {
            manager,
            current_bar: None,
        }
    }

    pub fn with_bar(mut self, bar_id: String) -> Self {
        self.current_bar = Some(bar_id);
        self
    }

    pub fn create_and_set_bar(
        &self,
        id: String,
        total: u64,
    ) -> CliResult<Arc<AdvancedProgressBar>> {
        let bar = self.manager.create_bar(id.clone(), total)?;
        Ok(bar)
    }

    pub fn get_current_bar(&self) -> Option<Arc<AdvancedProgressBar>> {
        if let Some(bar_id) = &self.current_bar {
            self.manager.get_bar(bar_id)
        } else {
            None
        }
    }

    pub fn set_current_bar(&mut self, bar_id: String) {
        self.current_bar = Some(bar_id);
    }

    pub fn clear_current_bar(&mut self) {
        self.current_bar = None;
    }
}

pub fn create_progress_manager() -> Arc<ProgressManager> {
    Arc::new(ProgressManager::new())
}

pub fn create_progress_context(manager: Arc<ProgressManager>) -> ProgressContext {
    ProgressContext::new(manager)
}
