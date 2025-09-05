# JetCrab CLI Reference

This document provides a comprehensive reference for the JetCrab command-line interface.

## Overview

JetCrab provides two main CLI tools:
- **`jetcrab`**: Main JavaScript runtime and execution tool
- **`claw`**: Package manager for JavaScript and Rust packages

## jetcrab Command

The main JetCrab command for running JavaScript code.

### Usage
```bash
jetcrab [OPTIONS] <COMMAND>
```

### Global Options
- `-h, --help`: Show help information
- `-V, --version`: Show version information
- `-v, --verbose`: Enable verbose output
- `--log-level <LEVEL>`: Set logging level (error, warn, info, debug, trace)

### Commands

#### `run <FILE>`
Run a JavaScript file.

```bash
jetcrab run script.js
jetcrab run --args "arg1" "arg2" script.js
```

**Options:**
- `--args <ARGS>...`: Command line arguments to pass to the script
- `--timeout <SECONDS>`: Set execution timeout
- `--max-memory <MB>`: Set maximum memory usage

**Examples:**
```bash
# Run a simple script
jetcrab run hello.js

# Run with arguments
jetcrab run --args "world" "universe" greet.js

# Run with timeout
jetcrab run --timeout 30 long-running.js
```

#### `eval <CODE>`
Evaluate JavaScript code directly.

```bash
jetcrab eval "console.log('Hello, World!')"
jetcrab eval "2 + 2"
```

**Options:**
- `--timeout <SECONDS>`: Set execution timeout
- `--max-memory <MB>`: Set maximum memory usage

**Examples:**
```bash
# Simple expression
jetcrab eval "42 + 8"

# Complex code
jetcrab eval "const fs = require('fs'); console.log(fs.readFileSync('package.json', 'utf8'))"
```

#### `repl`
Start an interactive REPL (Read-Eval-Print Loop).

```bash
jetcrab repl
```

**Options:**
- `--history <FILE>`: Specify history file location
- `--no-history`: Disable command history
- `--prompt <TEXT>`: Customize the prompt

**Examples:**
```bash
# Start REPL
jetcrab repl

# Start REPL with custom history
jetcrab repl --history ~/.jetcrab_history

# Start REPL with custom prompt
jetcrab repl --prompt "jetcrab> "
```

#### `dev`
Start development mode with file watching.

```bash
jetcrab dev [FILE]
```

**Options:**
- `--watch`: Watch for file changes
- `--port <PORT>`: Set development server port
- `--host <HOST>`: Set development server host
- `--open`: Open browser automatically

**Examples:**
```bash
# Start dev server
jetcrab dev

# Start dev server with file watching
jetcrab dev --watch

# Start dev server on specific port
jetcrab dev --port 3000
```

#### `build`
Build the project for production.

```bash
jetcrab build
```

**Options:**
- `--target <TARGET>`: Build for specific target
- `--release`: Build in release mode
- `--optimize`: Enable optimizations

**Examples:**
```bash
# Build project
jetcrab build

# Build for release
jetcrab build --release

# Build for specific target
jetcrab build --target x86_64-unknown-linux-gnu
```

## claw Command

The Claw package manager for managing JavaScript and Rust dependencies.

### Usage
```bash
claw [OPTIONS] <COMMAND>
```

### Global Options
- `-h, --help`: Show help information
- `-V, --version`: Show version information
- `-v, --verbose`: Enable verbose output
- `--config <FILE>`: Specify configuration file

### Commands

#### `init [NAME]`
Initialize a new JetCrab project.

```bash
claw init
claw init my-project
```

**Options:**
- `--name <NAME>`: Project name
- `--version <VERSION>`: Initial version
- `--description <DESC>`: Project description
- `--author <AUTHOR>`: Project author

**Examples:**
```bash
# Initialize in current directory
claw init

# Initialize with specific name
claw init my-awesome-project

# Initialize with metadata
claw init --name "My Project" --version "1.0.0" --author "John Doe"
```

#### `install [PACKAGES]...`
Install JavaScript or Rust packages.

```bash
claw install package-name
claw install --rust crate-name
claw install --dev dev-package
```

**Options:**
- `--rust`: Install Rust crate
- `--dev`: Install as development dependency
- `--optional`: Install as optional dependency
- `--save-exact`: Save exact version

**Examples:**
```bash
# Install JavaScript package
claw install lodash

# Install Rust crate
claw install --rust serde

# Install development dependency
claw install --dev jest

# Install multiple packages
claw install express lodash --rust serde tokio
```

#### `uninstall [PACKAGES]...`
Uninstall packages.

```bash
claw uninstall package-name
claw uninstall --rust crate-name
```

**Options:**
- `--rust`: Uninstall Rust crate
- `--save`: Update package.json/claw.json

**Examples:**
```bash
# Uninstall JavaScript package
claw uninstall lodash

# Uninstall Rust crate
claw uninstall --rust serde
```

#### `list`
List installed packages.

```bash
claw list
```

**Options:**
- `--rust`: List only Rust crates
- `--js`: List only JavaScript packages
- `--dev`: List only development dependencies

**Examples:**
```bash
# List all packages
claw list

# List only Rust crates
claw list --rust

# List only JavaScript packages
claw list --js
```

#### `update [PACKAGES]...`
Update packages to latest versions.

```bash
claw update
claw update package-name
```

**Options:**
- `--rust`: Update Rust crates
- `--js`: Update JavaScript packages
- `--major`: Update to latest major version

**Examples:**
```bash
# Update all packages
claw update

# Update specific package
claw update lodash

# Update Rust crates
claw update --rust
```

#### `build`
Build the project.

```bash
claw build
```

**Options:**
- `--release`: Build in release mode
- `--target <TARGET>`: Build for specific target
- `--clean`: Clean before building

**Examples:**
```bash
# Build project
claw build

# Build for release
claw build --release

# Clean and build
claw build --clean
```

#### `run [SCRIPT]`
Run a script or the main entry point.

```bash
claw run
claw run start
claw run build
```

**Options:**
- `--args <ARGS>...`: Arguments to pass to script
- `--timeout <SECONDS>`: Set execution timeout

**Examples:**
```bash
# Run main script
claw run

# Run specific script
claw run start

# Run with arguments
claw run --args "arg1" "arg2"
```

#### `dev`
Start development mode.

```bash
claw dev
```

**Options:**
- `--watch`: Watch for file changes
- `--port <PORT>`: Set development server port
- `--host <HOST>`: Set development server host

**Examples:**
```bash
# Start dev mode
claw dev

# Start dev mode with watching
claw dev --watch

# Start dev mode on specific port
claw dev --port 3000
```

#### `test`
Run tests.

```bash
claw test
```

**Options:**
- `--watch`: Watch for file changes
- `--coverage`: Generate coverage report
- `--verbose`: Verbose output

**Examples:**
```bash
# Run tests
claw test

# Run tests with coverage
claw test --coverage

# Run tests in watch mode
claw test --watch
```

#### `lint`
Lint the code.

```bash
claw lint
```

**Options:**
- `--fix`: Fix issues automatically
- `--format`: Check formatting
- `--strict`: Use strict linting rules

**Examples:**
```bash
# Lint code
claw lint

# Lint and fix
claw lint --fix

# Lint with strict rules
claw lint --strict
```

#### `format`
Format the code.

```bash
claw format
```

**Options:**
- `--check`: Check formatting without changing files
- `--write`: Write formatted code to files

**Examples:**
```bash
# Format code
claw format

# Check formatting
claw format --check

# Format and write
claw format --write
```

## Configuration

### jetcrab Configuration
JetCrab can be configured using environment variables or a configuration file.

**Environment Variables:**
- `JETCRAB_LOG_LEVEL`: Set logging level
- `JETCRAB_MAX_MEMORY`: Set maximum memory usage
- `JETCRAB_TIMEOUT`: Set default timeout

**Configuration File:**
Create a `jetcrab.toml` file in your project root:

```toml
[api]
enabled_apis = ["console", "process", "fs", "http"]
disabled_apis = []
experimental_apis = ["worker_threads"]

[performance]
enable_lazy_loading = true
api_timeout_ms = 5000

[logging]
level = "info"
format = "json"

[development]
enable_source_maps = true
enable_hot_reload = true
```

### claw Configuration
Claw uses `claw.json` for project configuration:

```json
{
  "name": "my-project",
  "version": "1.0.0",
  "description": "My JetCrab project",
  "main": "src/index.js",
  "scripts": {
    "start": "jetcrab run src/index.js",
    "build": "jetcrab build",
    "test": "jetcrab test",
    "dev": "jetcrab dev"
  },
  "dependencies": {
    "lodash": "^4.17.21"
  },
  "rust_dependencies": {
    "serde": "1.0"
  },
  "devDependencies": {
    "jest": "^29.0.0"
  }
}
```

## Examples

### Basic Usage
```bash
# Run a JavaScript file
jetcrab run app.js

# Start interactive REPL
jetcrab repl

# Evaluate code directly
jetcrab eval "console.log('Hello, World!')"
```

### Package Management
```bash
# Initialize new project
claw init my-project
cd my-project

# Install packages
claw install express lodash
claw install --rust serde tokio

# Run the project
claw run
```

### Development Workflow
```bash
# Start development server
claw dev --watch

# Run tests
claw test

# Lint code
claw lint

# Format code
claw format
```

## Troubleshooting

### Common Issues

#### Command Not Found
```bash
# Check if JetCrab is installed
which jetcrab
which claw

# Check installation
cargo install --list | grep jetcrab
```

#### Permission Denied
```bash
# Check file permissions
ls -la script.js

# Make file executable
chmod +x script.js
```

#### Memory Issues
```bash
# Run with memory limit
jetcrab run --max-memory 512 script.js

# Check system memory
free -h
```

#### Timeout Issues
```bash
# Run with longer timeout
jetcrab run --timeout 60 long-script.js

# Check script for infinite loops
```

## Resources

- **[JetCrab Guide](jetcrab-guide.md)** - Comprehensive usage guide
- **[API Reference](api-reference.md)** - API documentation
- **[Examples](examples.md)** - Code examples
- **[Contributing Guide](../contributing.md)** - How to contribute

---

**JetCrab CLI Reference** - Command-line interface documentation 🦀
