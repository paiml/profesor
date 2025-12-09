//! # Profesor
//!
//! WASM-native learning management system built on the PAIML Sovereign AI Stack.
//!
//! Provides Coursera-like functionality (courses, quizzes, labs, simulations)
//! compiled to WebAssembly with zero JavaScript dependencies.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                  PROFESOR WASM BUNDLE                    │
//! │                                                          │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │
//! │  │   Courses   │  │   Quizzes   │  │    Labs     │      │
//! │  │   Module    │  │   Engine    │  │   Runner    │      │
//! │  └─────────────┘  └─────────────┘  └─────────────┘      │
//! │                                                          │
//! │                    PROFESOR-CORE                         │
//! │              Types · Traits · Serialization              │
//! └─────────────────────────────────────────────────────────┘
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

mod wasm;

// Re-export WASM FFI functions
pub use wasm::*;

// Re-export all public types from sub-crates
pub use profesor_core::{
    Answer, Blank, Course, CourseId, CourseLevel, CourseProgress, CourseStatus, Difficulty,
    Feedback, Hint, Lab, LabCompletion, LabId, LabStep, Language, LearnerProgress, Lesson,
    LessonContent, LessonId, Module, ModuleId, Question, QuestionId, Quiz, QuizAttempt, QuizId,
    Score, SimulationId, StarterFile, StepValidation, TestCase, TestSuite, UnlockCriteria,
};

pub use profesor_lab::{
    ExecutionResult, Sandbox, SandboxConfig, TestResult, TestResults, TestRunner,
};

pub use profesor_quiz::{Grader, QuizEngine, QuizState};

pub use profesor_sim::{
    Action, PhysicsWorld, RigidBody, SimState, Simulation, Transition, Trigger, Vec2,
};

mod app;

pub use app::{App, AppEvent, AppState};

/// Library version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check if running in WASM environment.
#[must_use]
pub const fn is_wasm() -> bool {
    cfg!(target_arch = "wasm32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_core_types_accessible() {
        let course = Course::new("test", "Test Course");
        assert_eq!(course.id.as_str(), "test");
    }

    #[test]
    fn test_quiz_engine_accessible() {
        let quiz = Quiz::new("quiz", "Test Quiz");
        let engine = QuizEngine::new(quiz);
        assert!(matches!(engine.state(), QuizState::NotStarted));
    }

    #[test]
    fn test_lab_runner_accessible() {
        let _runner = TestRunner::new();
        // TestRunner is accessible
    }

    #[test]
    fn test_physics_accessible() {
        let world = PhysicsWorld::new();
        assert_eq!(world.body_count(), 0);
    }
}
