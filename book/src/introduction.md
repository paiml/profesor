# The Profesor Book

## WASM-Native Learning Management System

Profesor is a learning management system compiled entirely to WebAssembly, designed to run directly in the browser with zero JavaScript dependencies. This book documents the system's design philosophy, architecture, and implementation details.

## Why Profesor?

Traditional LMS platforms are built on server-heavy architectures that require constant network connectivity. Profesor takes a different approach:

- **Pure WASM**: The entire engine runs in the browser
- **No JavaScript**: Zero JS dependencies means smaller bundles and better security
- **Offline-First**: Works without network connectivity
- **Instant Feedback**: Sub-millisecond response times for quiz grading

## Toyota Way Principles

Profesor is designed around principles from the Toyota Production System:

### Jidoka (Built-in Quality)
When a learner makes an error, provide immediate, actionable feedback. Don't let mistakes compound.

### Poka-Yoke (Error Prevention)
Structure exercises to prevent common mistakes. Guided lab steps and structured question formats reduce confusion.

### Kaizen (Continuous Improvement)
Small, incremental learning steps build mastery over time. Each module builds on previous knowledge.

### Mieruka (Visual Management)
Visual simulations and progress dashboards make abstract concepts concrete and track learning progress.

## Test-Driven Development

Every feature in Profesor is developed test-first:

1. **Write the test** - Define expected behavior
2. **Watch it fail** - Ensure the test is actually testing something
3. **Implement** - Write minimal code to pass
4. **Refactor** - Improve design while keeping tests green

This approach ensures:
- 100% feature coverage
- Confidence in refactoring
- Living documentation through tests
- Regression prevention

## Quick Start

```rust
use profesor::{Quiz, QuizEngine, Question, QuestionId, Answer};

// Create a quiz
let quiz = Quiz::new("demo", "Demo Quiz")
    .with_question(Question::MultipleChoice {
        id: QuestionId::new("q1"),
        prompt: "What is 2 + 2?".into(),
        options: vec!["3".into(), "4".into(), "5".into()],
        correct: 1,
        explanation: "2 + 2 = 4".into(),
        points: 10,
    });

// Run the quiz engine
let mut engine = QuizEngine::new(quiz);
engine.start().expect("Failed to start");
let feedback = engine.submit_answer(Answer::Choice(1)).expect("Failed to submit");
println!("Correct: {}", feedback.correct);
```

## Examples

Run the interactive examples:

```bash
# Quiz engine demo
cargo run --example quiz_demo -p profesor

# Physics simulation demo
cargo run --example physics_demo -p profesor

# Course structure demo
cargo run --example course_demo -p profesor
```

## Next Steps

- [Part I: Toyota Way in Learning](./part1/core-principles.md) - Design philosophy
- [Part II: Architecture](./part2/system-overview.md) - System design
- [Part III: Quiz Engine](./part3/quiz-overview.md) - Core quiz functionality
