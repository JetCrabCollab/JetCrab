# Source Maps (Planned)

Source map support for JetCrab is planned for better debugging experience.

## Status

- **Planned**: Stack traces will map to original source
- **Planned**: Debugger breakpoint mapping
- **Implementation**: Requires Chitin guest (QuickJS WASM) to emit and consume source maps

## Roadmap

1. Parse `//# sourceMappingURL=` in bundled output
2. Map error locations (line/column) to original files
3. Integrate with Chitin error reporting
