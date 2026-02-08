# Agent Guidelines for dioxus-class

This document provides guidelines for AI coding agents working in the dioxus-class repository.

## Project Overview

dioxus-class is a Rust workspace that provides compile-time checked CSS class utilities for the Dioxus GUI framework. The project includes:
- `dioxus-class`: Core class wrapper and macros
- `dioxus-class-internal`: Internal class implementation
- `dioxus-class-macro`: Procedural macros for `class!` and `constant!`
- `dioxus-tailwindcss`: TailwindCSS constants and modifiers
- `dioxus-daisyui`: DaisyUI component constants
- `demo/emoji-browser`: Demo application showcasing the libraries

## Build Commands

### Standard Build
```bash
cargo build                    # Build all workspace members
cargo build --release          # Release build
cargo build -p dioxus-class    # Build specific package
```

### Test Commands
```bash
cargo test                           # Run all tests
cargo test -p dioxus-class          # Test specific package
cargo test test_name                # Run single test by name
cargo test --test integration_test  # Run specific integration test file
```

### Special Build Process for CSS Classes
The project uses a special build process to extract CSS classes for TailwindCSS:

```bash
# Generate classes.rs file (from demo/emoji-browser)
DIOXUS_CLASS_BUILD_PATH="$PWD/css/classes.rs" cargo test --features "web build-classes"

# Build with extracted classes
cargo build
cd css && cargo build
```

Use `just` for automated builds:
```bash
just build                    # Standard build
just build-doc                # Build documentation
just serve-doc                # Serve docs on port 8001
just install-dioxus-cli       # Install dioxus-cli via binstall
```

### Linting and Formatting
```bash
cargo fmt                     # Format code
cargo fmt --check            # Check formatting without changes
cargo clippy                 # Run clippy lints
cargo clippy --fix          # Auto-fix clippy warnings
cargo clippy --no-deps      # Lint only local code
```

## Code Style Guidelines

### General Conventions

#### Naming Conventions
- Use `snake_case` for:
  - Functions: `fn filter_emojis(input: &str) -> Vec<Props>`
  - Variables: `let current_dir = env::current_dir()?;`
  - Module names: `mod emoji_card;`
  - CSS class constants: `pub const text_center: &'static str = "text-center";`
  
- Use `PascalCase` for:
  - Types/Structs: `pub struct Class(pub Vec<String>)`
  - Enums: `pub enum Route { Home {} }`
  - Traits: `pub trait IntoClass`
  - Components: `pub fn App() -> Element`

- Use `SCREAMING_SNAKE_CASE` for:
  - Constants: `pub const NONE: Class = Class(vec![]);`
  - Static values: `pub static EMOJIS: GlobalSignal<Vec<Props>> = ...`

#### Allow Directives
Use these allow directives at the crate level:
```rust
#![allow(non_snake_case)]              // For Dioxus components
#![allow(non_upper_case_globals)]      // For CSS constants
```

### Imports Organization
```rust
// Standard library imports first
use std::fmt::Display;
use std::ops::Add;

// External crate imports
use dioxus::prelude::*;
use lazy_static::lazy_static;

// Internal crate imports
use crate::components::emoji_card;
use crate::pages::*;
```

### Type Annotations
- Always use explicit types for public APIs
- Use type inference for local variables when clear
- Prefer `&'static str` for CSS class constants
- Use `impl Trait` for complex return types when appropriate

### Error Handling
- Use `Result<T, Box<dyn std::error::Error>>` for build scripts
- Use `?` operator for error propagation
- Use `fehler` crate features where available (workspace dependency)
- For Option types, prefer `match` or `map` over `unwrap()`

### Macros and Proc Macros

#### constant! macro
Define CSS constants with automatic kebab-case conversion:
```rust
constant!(table column group);  // Creates table_column_group: "table-column-group"
constant!(text center);         // Creates text_center: "text-center"
```

#### class! macro
Use for compile-time checked CSS classes:
```rust
class: class!(card card_compact w_64 h_64 bg_base_300),
class: class!(text_center items_center hover(scale_105)),
```

### Component Guidelines

#### Component Structure
```rust
#[component]
pub fn ComponentName(prop1: Type1, prop2: Type2) -> Element {
    rsx! {
        div {
            class: class!(css_classes_here),
            "content"
        }
    }
}
```

#### Props Pattern
```rust
#[derive(Props, Clone, PartialEq)]
pub struct Props {
    pub field: Type,
}
```

### Documentation
- Include module-level docs from README.md: `#![doc = include_str!("../README.md")]`
- Document public functions and types
- Use `///` for doc comments
- Use `#[doc = "..."]` for generated docs in macros

### Build.rs Files
- Use `println!("cargo:rerun-if-changed=path/to/file")` to optimize rebuilds
- Return `Result<(), Box<dyn std::error::Error>>` from main()
- Use `env::current_dir()` and `Path` for file operations

## Special Features

### build-classes Feature
When enabled, the `class!` macro writes generated classes to a file specified by `DIOXUS_CLASS_BUILD_PATH`:
- Used to extract all CSS classes for TailwindCSS processing
- Enable with: `--features "build-classes"`
- Required for CSS framework integration

### Workspace Structure
- All crates share version 0.9.0
- Common dependencies defined in workspace `Cargo.toml`
- Demos use `publish = false` and `version = "0.0.0"`

## Testing Guidelines
- Disable doctests in demo/lib projects: `[lib] doctest = false`
- Use `cargo test --features "build-classes"` for class extraction tests
- Test files follow convention: `tests/*.rs` for integration tests

## Publishing
Use the justfile target for sequential publishing:
```bash
just publish-all  # Publishes in dependency order
```

## Important Notes
- CSS classes use underscores in Rust (`text_center`) but hyphens in CSS (`text-center`)
- The `Class` struct wraps `Vec<String>` and provides `Add<>` implementations for composition
- Modifiers like `hover()` are function-like syntax in the `class!` macro
- The build process involves: extract classes → validate format → generate HTML → run TailwindCSS
