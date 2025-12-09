//! # Profesor Core
//!
//! Core types and traits for the Profesor WASM-native learning platform.
//!
//! This crate provides the foundational types for courses, quizzes, labs,
//! and progress tracking. All types are designed for WASM compatibility
//! with zero JavaScript dependencies.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

pub mod course;
pub mod ids;
pub mod lab;
pub mod progress;
pub mod quiz;

pub use course::{Course, CourseLevel, Lesson, LessonContent, Module, UnlockCriteria};
pub use ids::{CourseId, LabId, LessonId, ModuleId, QuestionId, QuizId, SimulationId};
pub use lab::{Difficulty, Hint, Lab, LabStep, Language, StarterFile, StepValidation, TestSuite};
pub use progress::{CourseProgress, CourseStatus, LabCompletion, LearnerProgress, QuizAttempt};
pub use quiz::{Answer, Blank, Feedback, Question, Quiz, Score, TestCase};
