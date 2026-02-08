## Emoji Browser

<a href="https://www.edger.dev/emoji/browser" target="_blank">Open web app in new tab</a>

Search emoji by shortcode, built as demo project showcasing dioxus-class with TailwindCSS and DaisyUI.

## Development

### Prerequisites
- Rust toolchain
- Node.js and npm (for TailwindCSS)
- [just](https://github.com/casey/just) command runner
- [dioxus-cli](https://dioxuslabs.com/) (`cargo install dioxus-cli`)

### Setup
```bash
# Install npm dependencies for TailwindCSS
just npm-install
```

### Building
```bash
# Build CSS classes and generate tailwind.css
just build

# Or use the build-classes alias
just build-classes
```

The build process:
1. Extracts all CSS classes from Rust code using the `build-classes` feature
2. Generates `css/classes.rs` with all used classes
3. Creates `css/classes.html` for TailwindCSS to scan
4. Runs TailwindCSS to generate `assets/css/tailwind.css`

### Running
```bash
# Build and start development server
just serve

# Or manually after building
dx serve --addr 0.0.0.0
```

### Release Build
```bash
# Build optimized release version
just release
```

**Note**: You may see a `wasm-opt failed with status code signal: 6 (SIGABRT)` error during release builds. This is a known issue with wasm-opt and DWARF debug symbols, but the build will complete successfully. The error can be safely ignored as long as you see "Client build completed successfully!" at the end.

If you want to avoid the error completely, you can manually optimize the WASM file after building without wasm-opt (though this is usually not necessary).

### Development Workflow
```bash
# Watch for changes and auto-rebuild CSS (requires cargo-watch)
just watch-classes

# Clean build artifacts
just clean
```

### Available Commands
Run `just --list` to see all available commands.
