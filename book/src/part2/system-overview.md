# System Overview

Profesor is designed as a modular, no_std-compatible Rust library that compiles to pure WebAssembly.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                  PROFESOR WASM BUNDLE                    │
│                                                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │
│  │   Courses   │  │   Quizzes   │  │    Labs     │      │
│  │   Module    │  │   Engine    │  │   Runner    │      │
│  └─────────────┘  └─────────────┘  └─────────────┘      │
│                                                          │
│  ┌─────────────┐  ┌─────────────┐                       │
│  │   Physics   │  │    State    │                       │
│  │   Engine    │  │   Machines  │                       │
│  └─────────────┘  └─────────────┘                       │
│                                                          │
│                    PROFESOR-CORE                         │
│              Types · Traits · Serialization              │
└─────────────────────────────────────────────────────────┘
```

## Design Principles

### 1. Pure WASM, Zero JavaScript

The entire system compiles to `wasm32-unknown-unknown` with no JavaScript dependencies. This means:

- No JS runtime overhead
- Smaller bundle sizes
- Better security (no eval, no DOM access)
- Deterministic behavior

### 2. no_std Compatible

All crates support `#![no_std]` for maximum portability:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
```

### 3. Feature Flags

Each crate has `std` and `default` features:

```toml
[features]
default = ["std"]
std = ["profesor-core/std", "serde/std"]
```

### 4. Serde Serialization

All types are serializable for persistence and IPC:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: QuizId,
    pub title: String,
    pub questions: Vec<Question>,
    // ...
}
```

## Crate Dependency Graph

```
profesor (facade)
├── profesor-core (types & traits)
├── profesor-quiz (quiz engine)
│   └── profesor-core
├── profesor-lab (lab runner)
│   └── profesor-core
└── profesor-sim (simulations)
    └── profesor-core
```

## WASM FFI Layer

The `profesor` crate exposes C-compatible FFI functions for browser integration:

```rust
#[no_mangle]
pub extern "C" fn quiz_create_sample() -> QuizHandle {
    // Creates a sample quiz, returns opaque handle
}

#[no_mangle]
pub extern "C" fn quiz_start(handle: QuizHandle) -> i32 {
    // Starts the quiz, returns 0 on success
}

#[no_mangle]
pub extern "C" fn quiz_submit_choice(handle: QuizHandle, choice: u32) -> i32 {
    // Submits an answer, returns 1 if correct
}
```

## Memory Management

WASM memory is managed through explicit allocation:

```rust
#[no_mangle]
pub extern "C" fn alloc_bytes(len: usize) -> *mut u8 {
    let layout = Layout::from_size_align(len, 1).unwrap();
    unsafe { alloc::alloc::alloc(layout) }
}

#[no_mangle]
pub extern "C" fn free_bytes(ptr: *mut u8, len: usize) {
    let layout = Layout::from_size_align(len, 1).unwrap();
    unsafe { alloc::alloc::dealloc(ptr, layout) }
}
```

## Build Configuration

Optimized WASM builds use aggressive size optimization:

```toml
[profile.release]
opt-level = "z"      # Size optimization
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit
panic = "abort"      # No unwinding
strip = true         # Strip symbols
```

## Next: Crate Structure

[Crate Structure](./crate-structure.md) - Detailed look at each crate's responsibilities.
