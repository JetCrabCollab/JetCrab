use clap::Args;
use clap::Parser;
use std::path::PathBuf;

// Default help template
const HELP_TEMPLATE: &str = "\
{before-help}{usage-heading}
    {usage}

{all-args}{after-help}";

const AFTER_HELP: &str = "\
Environment variables:
   CHITIN_DEBUG                   ','-separated list of core modules that should
                                  print debug information
   CHITIN_DISABLE_COLORS          set to 1 to disable colors in the REPL
   CHITIN_EXTRA_CA_CERTS          path to additional CA certificates file. Only read
                                  once during process startup.
   CHITIN_NO_WARNINGS             set to 1 to silence process warnings
   CHITIN_PATH                    ';'-separated list of directories prefixed to the
                                  module search path
   CHITIN_PENDING_DEPRECATION     set to 1 to emit pending deprecation warnings
   CHITIN_PRESERVE_SYMLINKS       set to 1 to preserve symbolic links when resolving
                                  and caching modules
   CHITIN_REDIRECT_WARNINGS       write warnings to path instead of stderr
   CHITIN_REPL_HISTORY            path to the persistent REPL history file
   CHITIN_TLS_REJECT_UNAUTHORIZED set to 0 to disable TLS certificate validation
   CHITIN_COVERAGE                directory to output coverage JSON to
   FORCE_COLOR                    when set to 'true', 1, 2, 3, or an empty string
                                  causes NO_COLOR and CHITIN_DISABLE_COLORS to be
                                  ignored.
   NO_COLOR                       Alias for CHITIN_DISABLE_COLORS
";

/// JetCrab - A modern JavaScript runtime in Rust
#[derive(Parser, Debug)]
#[command(
    name = "jetcrab",
    about = "A modern JavaScript runtime in Rust",
    version = "0.4.0",
    help_template = HELP_TEMPLATE,
    after_help = AFTER_HELP,
    disable_help_flag = true,
    disable_version_flag = true,
)]
pub struct Cli {
    #[command(flatten)]
    pub main: MainOptions,

    #[command(flatten)]
    pub permissions: PermissionOptions,

    #[command(flatten)]
    pub diagnostics: DiagnosticOptions,

    #[command(flatten)]
    pub experimental: ExperimentalOptions,

    #[command(flatten)]
    pub test: TestOptions,

    #[command(flatten)]
    pub network: NetworkOptions,

    #[command(flatten)]
    pub debug: DebugOptions,
    
    #[command(flatten)]
    pub report: ReportOptions,
    
    // Positional Arguments
    /// Script to execute
    #[arg(index = 1)]
    pub script: Option<String>,

    /// Arguments for the script
    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    pub script_args: Vec<String>,
}

#[derive(Args, Debug)]
pub struct PermissionOptions {
    /// allow use of addons when any permissions are set
    #[arg(long)]
    pub allow_addons: bool,

    /// allow use of child process when any permissions are set
    #[arg(long)]
    pub allow_child_process: bool,
    
    /// allow permissions to read the filesystem
    #[arg(long)]
    pub allow_fs_read: Option<String>,

    /// allow permissions to write in the filesystem
    #[arg(long)]
    pub allow_fs_write: Option<String>,

    /// allow use of inspector when any permissions are set
    #[arg(long)]
    pub allow_inspector: bool,

    /// allow use of network when any permissions are set
    #[arg(long)]
    pub allow_net: bool,

    /// allow wasi when any permissions are set
    #[arg(long)]
    pub allow_wasi: bool,

    /// allow worker threads when any permissions are set
    #[arg(long)]
    pub allow_worker: bool,

    /// enable the permission system
    #[arg(long)]
    pub permission: bool,
}

#[derive(Args, Debug)]
pub struct DiagnosticOptions {
    /// Start the V8 CPU profiler on start up
    #[arg(long)]
    pub cpu_prof: bool,

    /// Directory for V8 CPU profiles
    #[arg(long)]
    pub cpu_prof_dir: Option<PathBuf>,

    /// Sampling interval in microseconds for CPU profiler
    #[arg(long, default_value = "1000")]
    pub cpu_prof_interval: u64,

    /// File name for CPU profile
    #[arg(long)]
    pub cpu_prof_name: Option<String>,

    /// Start the V8 heap profiler on start up
    #[arg(long)]
    pub heap_prof: bool,

    /// Directory for V8 heap profiles
    #[arg(long)]
    pub heap_prof_dir: Option<PathBuf>,

    /// Sampling interval in bytes for heap profiler
    #[arg(long, default_value = "524288")]
    pub heap_prof_interval: u64,

    /// File name for heap profile
    #[arg(long)]
    pub heap_prof_name: Option<String>,

    /// set dir for all output files
    #[arg(long)]
    pub diagnostic_dir: Option<PathBuf>,

    /// Generate V8 profiler output
    #[arg(long)]
    pub prof: bool,
    
    /// process V8 profiler output generated using --prof
    #[arg(long)]
    pub prof_process: bool,
    
    /// help system profilers to translate JavaScript interpreted frames
    #[arg(long)]
    pub interpreted_frames_native_stack: bool,

    /// track heap object allocations for heap snapshots
    #[arg(long)]
    pub track_heap_objects: bool,
}

#[derive(Args, Debug)]
pub struct ExperimentalOptions {
    /// experimental import support for addons
    #[arg(long)]
    pub experimental_addon_modules: bool,

    /// experimental EventSource API
    #[arg(long)]
    pub experimental_eventsource: bool,

    /// experimental ES Module import.meta.resolve() parentURL support
    #[arg(long)]
    pub experimental_import_meta_resolve: bool,

    /// use the specified module as a custom loader
    #[arg(long, alias = "loader")]
    pub experimental_loader: Option<String>,

    /// experimental network inspection support
    #[arg(long)]
    pub experimental_network_inspection: bool,

    /// experimental QUIC support
    #[arg(long)]
    pub experimental_quic: bool,

    /// enable code coverage in the test runner
    #[arg(long)]
    pub experimental_test_coverage: bool,

    /// enable module mocking in the test runner
    #[arg(long)]
    pub experimental_test_module_mocks: bool,

    /// experimental ES Module support in vm module
    #[arg(long)]
    pub experimental_vm_modules: bool,

    /// experimental Web Storage API
    #[arg(long, alias = "no-webstorage")]
    pub experimental_webstorage: bool,
    
    /// experimental frozen intrinsics support
    #[arg(long)]
    pub frozen_intrinsics: bool,

    /// experimental node:sqlite module
    #[arg(long, alias = "no-experimental-sqlite")]
    pub experimental_sqlite: bool,

    /// Experimental type-stripping for TypeScript files.
    #[arg(long, alias = "no-experimental-strip-types")]
    pub experimental_strip_types: bool,

    /// experimental WebSocket API
    #[arg(long, alias = "no-experimental-websocket")]
    pub experimental_websocket: bool,
}

#[derive(Args, Debug)]
pub struct TestOptions {
    /// launch test runner on startup
    #[arg(long)]
    pub test: bool,

    /// specify test runner concurrency
    #[arg(long)]
    pub test_concurrency: Option<usize>,

    /// the branch coverage minimum threshold
    #[arg(long)]
    pub test_coverage_branches: Option<u32>,

    /// exclude files from coverage report
    #[arg(long)]
    pub test_coverage_exclude: Option<String>,

    /// the function coverage minimum threshold
    #[arg(long)]
    pub test_coverage_functions: Option<u32>,

    /// include files in coverage report
    #[arg(long)]
    pub test_coverage_include: Option<String>,

    /// the line coverage minimum threshold
    #[arg(long)]
    pub test_coverage_lines: Option<u32>,

    /// force test runner to exit upon completion
    #[arg(long)]
    pub test_force_exit: bool,

    /// specifies the path to the global setup file
    #[arg(long)]
    pub test_global_setup: Option<String>,

    /// configures the type of test isolation used in the test runner
    #[arg(long, alias = "test-isolation")]
    pub experimental_test_isolation: Option<String>,

    /// run tests whose name matches this regular expression
    #[arg(long)]
    pub test_name_pattern: Option<String>,

    /// run tests with 'only' option set
    #[arg(long)]
    pub test_only: bool,

    /// report test output using the given reporter
    #[arg(long)]
    pub test_reporter: Option<String>,

    /// report given reporter to the given destination
    #[arg(long)]
    pub test_reporter_destination: Option<String>,

    /// specifies the path to the rerun state file
    #[arg(long)]
    pub test_rerun_failures: Option<String>,

    /// run test at specific shard
    #[arg(long)]
    pub test_shard: Option<String>,
    
    /// run tests whose name do not match this regular expression
    #[arg(long)]
    pub test_skip_pattern: Option<String>,

    /// specify test runner timeout
    #[arg(long)]
    pub test_timeout: Option<u64>,
    
    /// regenerate test snapshots
    #[arg(long)]
    pub test_update_snapshots: bool,
}

#[derive(Args, Debug)]
pub struct NetworkOptions {
    /// use an insecure HTTP parser that accepts invalid HTTP headers
    #[arg(long)]
    pub insecure_http_parser: bool,
    
    /// set default value of verbatim in dns.lookup
    #[arg(long)]
    pub dns_result_order: Option<String>,

    /// use an alternative default TLS cipher list
    #[arg(long)]
    pub tls_cipher_list: Option<String>,

    /// log TLS decryption keys to named file
    #[arg(long)]
    pub tls_keylog: Option<String>,

    /// set default TLS maximum to TLSv1.2 (default: TLSv1.3)
    #[arg(long)]
    pub tls_max_v1_2: bool,

    /// set default TLS maximum to TLSv1.3 (default: TLSv1.3)
    #[arg(long)]
    pub tls_max_v1_3: bool,

    /// set default TLS minimum to TLSv1.0 (default: TLSv1.2)
    #[arg(long)]
    pub tls_min_v1_0: bool,

    /// set default TLS minimum to TLSv1.1 (default: TLSv1.2)
    #[arg(long)]
    pub tls_min_v1_1: bool,

    /// set default TLS minimum to TLSv1.2 (default: TLSv1.2)
    #[arg(long)]
    pub tls_min_v1_2: bool,

    /// set default TLS minimum to TLSv1.3 (default: TLSv1.2)
    #[arg(long)]
    pub tls_min_v1_3: bool,
    
    /// use bundled CA store (default)
    #[arg(long)]
    pub use_bundled_ca: bool,

    /// use OpenSSL's default CA store
    #[arg(long)]
    pub use_openssl_ca: bool,
    
    /// use system's CA store
    #[arg(long)]
    pub use_system_ca: bool,
}

#[derive(Args, Debug)]
pub struct DebugOptions {
    /// activate inspector on host:port (default: 127.0.0.1:9229)
    #[arg(long)]
    pub inspect: Option<String>,

    /// activate inspector on host:port and break at start of user script
    #[arg(long)]
    pub inspect_brk: Option<String>,

    /// set host:port for inspector
    #[arg(long, alias = "debug-port")]
    pub inspect_port: Option<String>,
    
    /// comma separated list of destinations for inspector uid
    #[arg(long)]
    pub inspect_publish_uid: Option<String>,

    /// activate inspector on host:port and wait for debugger to be attached
    #[arg(long)]
    pub inspect_wait: Option<String>,
    
    /// show stack traces on deprecations
    #[arg(long)]
    pub trace_deprecation: bool,
    
    /// Print accesses to the environment variables
    #[arg(long)]
    pub trace_env: bool,
    
    /// show stack trace when an environment exits
    #[arg(long)]
    pub trace_exit: bool,
    
    /// show stack traces on promise initialization and resolution
    #[arg(long)]
    pub trace_promises: bool,
    
    /// enable printing JavaScript stacktrace on SIGINT
    #[arg(long)]
    pub trace_sigint: bool,
    
    /// show stack trace when use of sync IO is detected after the first tick
    #[arg(long)]
    pub trace_sync_io: bool,
    
    /// prints TLS packet trace information to stderr
    #[arg(long)]
    pub trace_tls: bool,
    
    /// show stack traces for the `throw` behind uncaught exceptions
    #[arg(long)]
    pub trace_uncaught: bool,
    
    /// show stack traces on process warnings
    #[arg(long)]
    pub trace_warnings: bool,
}

#[derive(Args, Debug)]
pub struct ReportOptions {
    /// define custom report pathname
    #[arg(long, alias = "report-directory")]
    pub report_dir: Option<PathBuf>,

    /// define custom report file name
    #[arg(long)]
    pub report_filename: Option<String>,
    
    /// Exclude environment variables when generating report
    #[arg(long)]
    pub report_exclude_env: bool,
    
    /// exclude network interface diagnostics
    #[arg(long)]
    pub report_exclude_network: bool,

    /// generate diagnostic report on fatal (internal) errors
    #[arg(long)]
    pub report_on_fatalerror: bool,
    
    /// generate diagnostic report upon receiving signals
    #[arg(long)]
    pub report_on_signal: bool,
    
    /// causes diagnostic report to be produced on provided signal
    #[arg(long)]
    pub report_signal: Option<String>,

    /// generate diagnostic report on uncaught exceptions
    #[arg(long)]
    pub report_uncaught_exception: bool,
    
    /// output compact single-line JSON
    #[arg(long)]
    pub report_compact: bool,
}

#[derive(Args, Debug)]
pub struct MainOptions {
    /// script read from stdin
    #[arg(short = '-', long = "stdin")]
    pub stdin_arg: bool,

    /// indicate the end of node options
    #[arg(long = "")]
    pub end_of_options: bool,

    /// aborting instead of exiting causes a core file to be generated
    #[arg(long)]
    pub abort_on_uncaught_exception: bool,
    
    // Check
    /// syntax check script without executing
    #[arg(short = 'c', long)]
    pub check: bool,

    // Eval
    /// evaluate script
    #[arg(short = 'e', long)]
    pub eval: Option<String>,

    /// print source-able bash completion script
    #[arg(long)]
    pub completion_bash: bool,
    
    /// expose gc extension
    #[arg(long)]
    pub expose_gc: bool,
    
    // Help
    /// print node command line options
    #[arg(short = 'h', long)]
    pub help: bool,

    // Interactive
    /// always enter the REPL even if stdin does not appear to be a terminal
    #[arg(short = 'i', long)]
    pub interactive: bool,
    
    // No flags
    /// disable loading native addons
    #[arg(long)]
    pub no_addons: bool,

    /// silence deprecation warnings
    #[arg(long)]
    pub no_deprecation: bool,

    /// disable global module search paths
    #[arg(long)]
    pub no_global_search_paths: bool,

    /// silence all process warnings
    #[arg(long)]
    pub no_warnings: bool,

    // Print
    /// evaluate script and print result
    #[arg(short = 'p', long)]
    pub print: Option<String>,
    
    // Require
    /// CommonJS module to preload (option can be repeated)
    #[arg(short = 'r', long)]
    pub require: Vec<String>,

    /// Run a script specified in package.json
    #[arg(long)]
    pub run: Option<String>,
    
    // Version
    /// print Node.js version
    #[arg(short = 'v', long)]
    pub version: bool,
    
    // Watch
    /// run in watch mode
    #[arg(long)]
    pub watch: bool,

    /// path to watch
    #[arg(long)]
    pub watch_path: Option<PathBuf>,

    /// preserve outputs on watch mode restart
    #[arg(long)]
    pub watch_preserve_output: bool,
    
    // Unhandled Rejections
    /// define unhandled rejections behavior.
    #[arg(long, default_value = "throw")]
    pub unhandled_rejections: String,
}
