# Crate Structure

Profesor is organized into five crates with clear responsibilities.

## profesor-core

Core types shared across all crates:
- IDs: CourseId, QuizId, LabId, etc.
- Course structure: Course, Module, Lesson
- Quiz types: Quiz, Question, Answer
- Lab types: Lab, LabStep, TestCase

## profesor-quiz

Quiz engine with state machine:
- QuizEngine: manages quiz state
- Grader: grades answers
- QuizState: NotStarted → InProgress → Completed

## profesor-lab

Lab execution environment:
- TestRunner: executes test suites
- Sandbox: isolated execution
- FeedbackGenerator: explains errors

## profesor-sim

Physics and state machine simulations:
- PhysicsWorld: 2D physics
- Vec2, RigidBody: physics primitives
- Simulation: state machine wrapper

## profesor (facade)

Main entry point that re-exports all public types and provides WASM FFI.
