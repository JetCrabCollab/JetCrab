# Node.js API Compatibility

JetCrab aims for compatibility with common Node.js APIs to ease migration.

## Implemented

| API | Status | Notes |
|-----|--------|-------|
| console | Done | log, error, warn, info |
| process | Done | version, argv, cwd, env |
| fetch | Done | HTTP client |
| fs (sync) | Done | readFileSync, writeFileSync, existsSync, statSync, mkdirSync, readdirSync, unlinkSync, rmdirSync |
| path | Done | join, resolve, dirname, basename |
| Buffer | Done | Basic Uint8Array extensions |

## In Progress

| API | Status |
|-----|--------|
| require() | Bundler supports CommonJS |
| import/export | Bundler concatenates in dependency order |

## Planned

| API | Notes |
|-----|-------|
| http/https (server) | Planned |
| child_process | Planned |
| worker_threads | Planned |
| events.EventEmitter | Planned |
| stream | Planned |
| crypto | Partial |
