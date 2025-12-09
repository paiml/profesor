//! # Profesor Lab Runner
//!
//! WASM-safe code execution sandbox and test runner for labs.
//!
//! Provides sandboxed execution with timeouts and memory limits,
//! following the Jidoka principle of stopping on errors.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

mod feedback;
mod runner;
mod sandbox;

pub use feedback::{
    DifferenceType, ErrorCategory, ErrorExplanation, FeedbackGenerator, OutputComparison,
};
pub use runner::{TestResult, TestResults, TestRunner};
pub use sandbox::{ExecutionResult, Sandbox, SandboxConfig};
