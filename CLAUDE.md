# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Bevy game engine workspace project for creating small games in Rust. The project uses a monorepo structure with shared common libraries and individual game projects.

## Build and Development Commands

```bash
# Build entire workspace
cargo build

# Run a specific game
cargo run -p hello-bevy
cargo run -p <game-name>

# Build for release (optimized)
cargo build --release

# Run tests for entire workspace
cargo test

# Run tests for a specific package
cargo test -p bevy-games-common
cargo test -p <game-name>

# Format code
cargo fmt

# Check code with clippy
cargo clippy -- -D warnings

# Watch and rebuild on changes (requires cargo-watch)
cargo watch -x "run -p hello-bevy"
```

## Architecture

### Workspace Structure
- **Cargo workspace** with shared dependencies defined at workspace level
- **common/** - Shared library (`bevy-games-common`) containing reusable components
- **games/** - Individual game projects, each as a separate Cargo package

### Common Library Modules (`bevy-games-common`)
The common library provides shared functionality through these modules:
- `components` - Shared ECS components (Health, Velocity, Player, etc.)
- `systems` - Reusable systems (movement, collision, health management)
- `resources` - Game resources (GameSettings, Score, GameState)
- `utils` - Utility functions (math, random generation, camera setup)

All modules are re-exported through `bevy_games_common::prelude::*` for convenient importing.

### Game Development Pattern
Each game follows this structure:
1. Imports `bevy::prelude::*` and `bevy_games_common::prelude::*`
2. Initializes Bevy App with DefaultPlugins
3. Sets up game state, resources, and systems
4. Implements game-specific logic while leveraging common components

### Adding New Games
To create a new game in the workspace:
1. Create directory: `games/<game-name>/`
2. Add Cargo.toml with workspace inheritance:
   ```toml
   [package]
   name = "<game-name>"
   version = "0.1.0"
   edition.workspace = true

   [dependencies]
   bevy = { workspace = true }
   bevy-games-common = { path = "../../common" }
   ```
3. Implement game in `src/main.rs`

## Key Dependencies
- **Bevy 0.14** - Game engine framework
- **rand 0.8** - Random number generation
- **serde/serde_json** - Serialization support
- Rust stable toolchain with rustfmt and clippy

## Performance Settings
- Development builds use optimized dependencies (opt-level 3) while keeping main code debuggable
- Release builds enable LTO and single codegen unit for maximum performance