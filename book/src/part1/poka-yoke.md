# Poka-Yoke: Error Prevention

Poka-Yoke (mistake-proofing) designs systems so that mistakes are impossible or immediately obvious.

## In Profesor

- Multiple choice questions have distinct, clear options
- Labs provide step-by-step guidance
- Starter code templates prevent syntax errors
- Validation checks catch common mistakes early

## Implementation

```rust
pub enum StepValidation {
    FunctionExists { name: String },
    TestsPass { test_names: Vec<String> },
    OutputMatches { expected: String },
}
```

Each lab step has optional validation to catch mistakes immediately.
