# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-09

### Added

- **profesor-core**: Core types and structures
  - Course, Module, Lesson hierarchy
  - Quiz, Question, Answer types
  - Lab, LabStep, TestCase structures
  - Progress tracking types
  - Strongly-typed IDs (CourseId, QuizId, etc.)

- **profesor-quiz**: Quiz engine with state machine
  - QuizEngine with NotStarted → InProgress → Completed states
  - Immediate feedback on every answer (Jidoka principle)
  - Grader for automatic scoring
  - Support for MultipleChoice and MultipleSelect questions

- **profesor-lab**: Lab execution environment
  - TestRunner for executing test suites
  - Sandbox configuration for isolated execution
  - FeedbackGenerator with language-specific error explanations
  - Output comparison with whitespace/case tolerance

- **profesor-sim**: Physics and state machine simulations
  - 2D PhysicsWorld with gravity and bounds
  - RigidBody with mass, restitution, and collisions
  - Vec2 math operations
  - State machine for interactive simulations
  - RenderConfig for visualization

- **profesor**: WASM facade crate
  - 21 FFI functions for browser integration
  - Memory management (alloc_bytes, free_bytes)
  - Quiz engine exports
  - Physics engine exports
  - App state management

- **Documentation**
  - Comprehensive README with examples
  - mdbook-based documentation
  - 3 runnable examples (quiz_demo, physics_demo, course_demo)

- **Testing**
  - 156 unit tests
  - Property-based tests with proptest
  - no_std compatibility tests

### Technical Details

- Pure WASM compilation (wasm32-unknown-unknown)
- Zero JavaScript dependencies
- 48KB optimized WASM bundle
- no_std compatible with feature flags
- Serde serialization for all types

[Unreleased]: https://github.com/paiml/profesor/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/paiml/profesor/releases/tag/v0.1.0
