# Quiz Engine Overview

The quiz engine is a state machine that manages quiz attempts with immediate feedback.

## State Machine

```
NotStarted ──start()──> InProgress ──finish()──> Completed
                            │
                            └──submit_answer()──> (same state, feedback returned)
```

## Usage

```rust
let quiz = Quiz::new("demo", "Demo Quiz")
    .with_question(Question::MultipleChoice { ... });

let mut engine = QuizEngine::new(quiz);
let first_question = engine.start()?;

let feedback = engine.submit_answer(Answer::Choice(0))?;
println!("Correct: {}", feedback.correct);

let score = engine.finish()?;
println!("Final: {}/{}%", score.correct_count, score.percentage * 100.0);
```
