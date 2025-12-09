//! # Profesor Quiz Engine
//!
//! Quiz state machine and grading logic for the Profesor LMS.
//!
//! Provides automatic grading with immediate feedback (Jidoka principle)
//! and adaptive difficulty based on learner performance.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

mod engine;
mod grader;

pub use engine::{QuizEngine, QuizState};
pub use grader::Grader;
