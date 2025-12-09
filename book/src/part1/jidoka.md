# Jidoka: Immediate Feedback

Jidoka (autonomation with a human touch) is the principle of building quality into the process by stopping immediately when a problem is detected.

## The Manufacturing Origin

In Toyota's factories, any worker can pull the "andon cord" to stop the production line when they see a defect. This might seem wasteful, but it:

1. Prevents defective products from continuing down the line
2. Forces immediate investigation of the root cause
3. Results in higher quality long-term

## Jidoka in Learning

In Profesor, the equivalent is **immediate feedback on every answer**:

```rust
// When a learner submits an answer, feedback is immediate
let feedback = engine.submit_answer(Answer::Choice(1))?;

// The feedback includes:
// - Whether the answer was correct
// - Points earned
// - Detailed explanation
println!("Correct: {}", feedback.correct);
println!("Points: {}", feedback.points_earned);
println!("Why: {}", feedback.explanation);
```

## Why Immediate Feedback Matters

Research shows that delayed feedback is significantly less effective than immediate feedback:

| Feedback Timing | Retention Rate | Error Correction |
|-----------------|----------------|------------------|
| Immediate | 90% | Immediate |
| End of quiz | 60% | Often missed |
| Next day | 30% | Rarely corrected |

By providing feedback immediately after each answer, Profesor:

1. **Prevents wrong mental models** from solidifying
2. **Enables real-time learning** as the quiz progresses
3. **Reduces test anxiety** by removing surprises
4. **Increases engagement** through continuous interaction

## Implementation

The `Feedback` struct carries all information needed for jidoka:

```rust
pub struct Feedback {
    /// Whether the answer was correct
    pub correct: bool,
    /// Points earned for this answer
    pub points_earned: u32,
    /// Explanation of the correct answer
    pub explanation: String,
}
```

The quiz engine generates feedback immediately upon answer submission:

```rust
impl QuizEngine {
    pub fn submit_answer(&mut self, answer: Answer) -> Result<Feedback, QuizError> {
        // Grade immediately
        let feedback = self.grade_current_question(&answer);

        // Update state
        self.record_answer(answer, feedback.clone());

        // Return feedback to learner
        Ok(feedback)
    }
}
```

## Error Explanations

For lab exercises, the `FeedbackGenerator` provides detailed error explanations:

```rust
let explanation = FeedbackGenerator::explain_error(
    "cannot borrow `x` as mutable because it is also borrowed as immutable",
    Language::Rust,
);

// Returns:
// - Category: BorrowChecker
// - Summary: "Borrow checker error"
// - Explanation: "Rust's borrow checker prevents data races..."
// - Suggestion: "Consider using .clone() or restructure code..."
// - Related concepts: ["Ownership", "Borrowing", "Lifetimes"]
```

## The Stop-and-Fix Philosophy

Just as Toyota workers stop the line to fix problems, Profesor encourages learners to:

1. **Stop** when they get an answer wrong
2. **Read** the explanation carefully
3. **Understand** why the correct answer is correct
4. **Correct** their mental model before continuing

This is more valuable than rushing through a quiz and seeing results at the end.

## Next: Poka-Yoke

[Poka-Yoke: Error Prevention](./poka-yoke.md) - How we design questions and exercises to prevent mistakes in the first place.
