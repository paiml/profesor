# Profesor: WASM-Native Learning Platform

**Version**: 0.1.0
**Status**: Specification
**Layer**: 7 (Presentation)
**Target**: `wasm32-unknown-unknown` (Pure WASM, Zero JavaScript)

## CRITICAL: Pure WASM Policy (STOP THE LINE)

### ABSOLUTE PROHIBITION: NO JAVASCRIPT

This project enforces **ZERO JavaScript**. All functionality must be implemented in pure Rust compiled to WASM.

**FORBIDDEN** (will cause immediate rejection):
- ❌ Any `.js` or `.ts` files in the codebase
- ❌ `npm`, `yarn`, `pnpm`, or any Node.js tooling
- ❌ JavaScript glue code or bindings
- ❌ wasm-bindgen JavaScript output (use raw WASM only)
- ❌ Any npm package dependencies
- ❌ Webpack, Vite, Rollup, or any JS bundlers

**REQUIRED**:
- ✅ Pure Rust → `wasm32-unknown-unknown` compilation
- ✅ Raw WASM module instantiation via browser WebAssembly API
- ✅ All interop via WASM imports/exports (no JS bridge)
- ✅ `ruchy serve` for local development (see wos patterns)

### Component Source Priority (MANDATORY)

Before adding ANY external dependency, you MUST check existing PAIML stack components:

1. **FIRST**: Check `../batuta/src/` for reusable components:
   - `wasm.rs` - WASM utilities and types
   - `types.rs` - Core type definitions
   - `config.rs` - Configuration patterns
   - `tui/` - Terminal UI components (adapt for web)
   - `content/` - Content management patterns

2. **SECOND**: Check `../wos/` for quality patterns:
   - Extreme TDD methodology
   - Property-based testing (proptest 10K inputs)
   - Mutation testing (90%+ kill rate)
   - E2E testing patterns (Playwright)
   - PMAT quality gates integration

3. **THIRD**: Check other PAIML stack crates (`trueno`, `aprender`, `presentar`)

4. **LAST RESORT**: External crates (must be WASM-compatible, no JS)

**Enforcement**: PR reviews must include "Component Source Check" section documenting which existing components were evaluated before any new code was written.

## 1. Executive Summary

Profesor is an open-source, WASM-native learning management system built entirely on the PAIML Sovereign AI Stack. It provides Coursera-like functionality (courses, quizzes, labs, simulations) compiled to WebAssembly with zero JavaScript dependencies.

### 1.1 Design Philosophy

> "The best way to learn is by doing." — Richard Feynman

Profesor embodies **Genchi Genbutsu** (現地現物) — go and see for yourself. Rather than passive video consumption, learners interact with live simulations, execute real code, and receive immediate feedback.

### 1.2 Toyota Way Alignment

> **[Toyota Way Annotation 1] Respect for People**: While technical principles are listed below, the overarching pillar of "Respect for People" must guide the UX. System latency, confusing error messages, or accessibility barriers are a form of disrespect to the learner's time and effort.

| Principle | Application |
|-----------|-------------|
| **Jidoka** | Auto-grading with immediate feedback stops incorrect learning |
| **Poka-Yoke** | Structured quiz formats prevent submission errors |
| **Genchi Genbutsu** | Live code execution, not screenshots |
| **Heijunka** | Adaptive pacing balances cognitive load |
| **Kaizen** | Spaced repetition improves retention continuously |
| **Mieruka** | Progress dashboards for visual management |

## 2. Architecture

### 2.1 Pure WASM Stack (Zero JavaScript)

> **[Toyota Way Annotation 2] Jidoka in Architecture**: The architecture must support "stopping the line." If the `TRUENO` compute layer detects a numerical anomaly (e.g., NaN in a grade calculation), it should halt immediately and signal `PRESENTAR` to display a user-friendly error, preventing the propagation of corrupt state.

```
┌─────────────────────────────────────────────────────────────────┐
│                    PROFESOR WASM BUNDLE                         │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   Courses   │  │   Quizzes   │  │    Labs     │             │
│  │   Module    │  │   Engine    │  │   Runner    │             │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │
│         │                │                │                     │
│         └────────────────┼────────────────┘                     │
│                          │                                      │
│  ┌───────────────────────▼───────────────────────┐             │
│  │              PRESENTAR (UI Layer)              │             │
│  │  Widgets · Layout · Events · YAML Config       │             │
│  └───────────────────────┬───────────────────────┘             │
│                          │                                      │
│  ┌───────────────────────▼───────────────────────┐             │
│  │              APRENDER (ML Layer)               │             │
│  │  Adaptive Difficulty · Analytics · Clustering  │             │
│  └───────────────────────┬───────────────────────┘             │
│                          │                                      │
│  ┌───────────────────────▼───────────────────────┐             │
│  │              TRUENO (Compute Layer)            │             │
│  │  SIMD Math · Scoring · State Management        │             │
│  └───────────────────────────────────────────────┘             │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │  WebAssembly    │
                    │  Runtime (V8)   │
                    └─────────────────┘
```

### 2.2 PAIML Stack Dependencies (Exhaustive)

| Crate | Version | Purpose | WASM Support |
|-------|---------|---------|--------------|
| `trueno` | 0.8.x | SIMD scoring, state vectors | Full |
| `aprender` | 0.14.x | Adaptive learning, clustering | Full (no rayon) |
| `presentar` | 0.1.x | UI widgets, layout, events | Full (primary target) |
| `presentar-core` | 0.1.x | Core types, traits | Full |
| `presentar-widgets` | 0.1.x | Quiz UI components | Full |
| `presentar-yaml` | 0.1.x | Course manifest parsing | Full |
| `alimentar` | 0.x.x | Data serialization | Full |

**Explicitly Excluded from WASM Bundle** (not WASM-compatible as runtime dependencies):
- `batuta` (CLI, filesystem) - **BUT**: Use as component source (copy/adapt code patterns)
- `repartir` (distributed compute)
- `entrenar` (GPU training)
- `certeza` (native testing)

> **Component Reuse Policy**: While `batuta` cannot be a Cargo dependency for WASM targets, its source code in `../batuta/src/` contains valuable patterns (types, config, WASM utilities) that MUST be evaluated and adapted before writing new code.

### 2.3 Crate Structure

```
profesor/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── profesor-core/      # Core types, traits
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── course.rs   # Course, Module, Lesson
│   │   │   ├── quiz.rs     # Question, Answer, Score
│   │   │   ├── lab.rs      # Lab, TestCase, Execution
│   │   │   └── progress.rs # Progress, Analytics
│   │   └── Cargo.toml
│   │
│   ├── profesor-quiz/      # Quiz engine
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── engine.rs   # Quiz state machine
│   │   │   ├── grader.rs   # Auto-grading logic
│   │   │   ├── adaptive.rs # ML-powered difficulty
│   │   │   └── types.rs    # Question types
│   │   └── Cargo.toml
│   │
│   ├── profesor-lab/       # Lab runner
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── sandbox.rs  # WASM sandbox execution
│   │   │   ├── runner.rs   # Test runner
│   │   │   └── feedback.rs # Error explanations
│   │   └── Cargo.toml
│   │
│   ├── profesor-sim/       # Simulations
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── state.rs    # State machine
│   │   │   ├── physics.rs  # trueno-powered physics
│   │   │   └── render.rs   # Visualization
│   │   └── Cargo.toml
│   │
│   ├── profesor-ui/        # Presentar widgets
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── quiz_widget.rs
│   │   │   ├── lab_widget.rs
│   │   │   ├── progress_widget.rs
│   │   │   └── course_nav.rs
│   │   └── Cargo.toml
│   │
│   └── profesor/           # Main entry point
│       ├── src/
│       │   ├── lib.rs      # WASM entry
│       │   └── app.rs      # Application state
│       └── Cargo.toml
│
├── courses/                # Course content (YAML)
│   └── example-course/
│       ├── course.yaml
│       ├── module-01/
│       │   ├── lesson-01.yaml
│       │   ├── quiz-01.yaml
│       │   └── lab-01.yaml
│       └── assets/
│
└── docs/
    └── profesor-spec.md    # This file
```

## 3. Core Types

### 3.1 Course Structure

```rust
// profesor-core/src/course.rs

/// A complete course (e.g., "Rust Fundamentals")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: CourseId,
    pub title: String,
    pub description: String,
    pub level: CourseLevel,
    pub modules: Vec<Module>,
    pub prerequisites: Vec<CourseId>,
    pub estimated_hours: u32,
}

/// Course difficulty level
#[derive(Debug, Clone, Copy)]
pub enum CourseLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// A module within a course (e.g., "Week 1: Ownership")
///
/// > **[Toyota Way Annotation 3] Heijunka (Leveling)**: Modules must be scoped to ensure a level workflow. 
/// > Avoid "lumpy" content where one module is 1 hour and the next is 10 hours. 
/// > Standardizing module size (e.g., ~2-4 hours) allows learners to establish a consistent "takt time" for their study.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: ModuleId,
    pub title: String,
    pub lessons: Vec<Lesson>,
    pub quiz: Option<Quiz>,
    pub lab: Option<Lab>,
    pub unlock_criteria: UnlockCriteria,
}

/// A single lesson (e.g., "1.1 What is Ownership?")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: LessonId,
    pub title: String,
    pub content: LessonContent,
    pub duration_minutes: u32,
}

/// Lesson content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LessonContent {
    /// Markdown text content
    Text(String),
    /// Interactive code example
    InteractiveCode { code: String, language: Language },
    /// Embedded simulation
    Simulation { sim_id: SimulationId },
    /// Video reference (external URL)
    Video { url: String, duration_seconds: u32 },
}
```

### 3.2 Quiz Types

```rust
// profesor-core/src/quiz.rs

/// A quiz with multiple questions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: QuizId,
    pub title: String,
    pub questions: Vec<Question>,
    pub time_limit: Option<Duration>,
    pub passing_score: f32,  // 0.0 - 1.0
    pub shuffle: bool,
    pub max_attempts: Option<u32>,
}

/// Question types supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Question {
    /// Multiple choice (single answer)
    MultipleChoice {
        id: QuestionId,
        prompt: String,
        options: Vec<String>,
        correct: usize,
        explanation: String,
        points: u32,
    },

    /// Multiple select (multiple answers)
    MultipleSelect {
        id: QuestionId,
        prompt: String,
        options: Vec<String>,
        correct: Vec<usize>,
        explanation: String,
        points: u32,
    },

    /// Code completion (fill in the blank)
    CodeCompletion {
        id: QuestionId,
        prompt: String,
        code_template: String,
        blanks: Vec<Blank>,
        test_cases: Vec<TestCase>,
        points: u32,
    },

    /// Drag and drop ordering
    Ordering {
        id: QuestionId,
        prompt: String,
        items: Vec<String>,
        correct_order: Vec<usize>,
        explanation: String,
        points: u32,
    },

    /// Matching pairs
    Matching {
        id: QuestionId,
        prompt: String,
        left: Vec<String>,
        right: Vec<String>,
        correct_pairs: Vec<(usize, usize)>,
        points: u32,
    },

    /// Free-form code (graded by test cases)
    FreeformCode {
        id: QuestionId,
        prompt: String,
        language: Language,
        starter_code: String,
        test_cases: Vec<TestCase>,
        hidden_test_cases: Vec<TestCase>,
        points: u32,
    },
}

/// A blank in code completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blank {
    pub id: String,
    pub acceptable_answers: Vec<String>,
    pub hint: Option<String>,
}

/// Test case for code validation
///
/// > **[Toyota Way Annotation 4] Poka-Yoke (Mistake Proofing)**: Test cases should not only verify specific outputs but also assert against common mistakes.
/// > For example, if a user is likely to confuse 0-indexing with 1-indexing, a test case should specifically catch that error and provide a helpful hint, preventing the defect from moving downstream (mental model formation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub input: String,
    pub expected_output: String,
    pub timeout_ms: u32,
}
```

### 3.3 Lab Types

```rust
// profesor-core/src/lab.rs

/// A hands-on coding lab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lab {
    pub id: LabId,
    pub title: String,
    pub description: String,
    pub language: Language,
    pub difficulty: Difficulty,
    pub estimated_minutes: u32,
    pub instructions: Vec<LabStep>,
    pub starter_files: Vec<StarterFile>,
    pub test_suite: TestSuite,
    pub hints: Vec<Hint>,
}

/// A step in the lab instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabStep {
    pub number: u32,
    pub title: String,
    pub description: String,
    pub validation: Option<StepValidation>,
}

/// Validation for a lab step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepValidation {
    /// Check that a function exists
    FunctionExists { name: String },
    /// Check that tests pass
    TestsPass { test_names: Vec<String> },
    /// Check output matches
    OutputMatches { expected: String },
}

/// Languages supported for labs
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    SQL,
    Markdown,
}

/// Starter file for a lab
///
/// > **[Toyota Way Annotation 5] Genchi Genbutsu (Go and See)**: The starter file must represent a working "known good" state or a clean slate that compiles/runs immediately.
/// > We must not provide broken boilerplate. The learner should be able to "go and see" the initial state working before they begin modification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterFile {
    pub path: String,
    pub content: String,
    pub readonly: bool,
}
```

### 3.4 Progress Tracking

```rust
// profesor-core/src/progress.rs
use aprender::clustering::KMeans;

/// Learner progress across all courses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnerProgress {
    pub learner_id: LearnerId,
    pub courses: HashMap<CourseId, CourseProgress>,
    pub total_xp: u64,
    pub streak_days: u32,
    pub last_activity: Timestamp,
}

/// Progress within a single course
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseProgress {
    pub course_id: CourseId,
    pub status: CourseStatus,
    pub modules_completed: Vec<ModuleId>,
    pub current_module: Option<ModuleId>,
    pub quiz_scores: HashMap<QuizId, QuizAttempt>,
    pub lab_completions: HashMap<LabId, LabCompletion>,
    pub started_at: Timestamp,
    pub completed_at: Option<Timestamp>,
}

/// Adaptive difficulty using aprender
pub struct AdaptiveDifficulty {
    /// K-means clustering of learner performance
    clusterer: KMeans<f32>,
    /// Performance feature vector
    features: Vec<f32>,
}

impl AdaptiveDifficulty {
    /// Recommend next question difficulty based on history
    pub fn recommend(&self, history: &[QuizAttempt]) -> Difficulty {
        // Extract features: accuracy, speed, streak
        let features = self.extract_features(history);

        // Cluster to find learner's current level
        let cluster = self.clusterer.predict(&features);

        // Map cluster to difficulty
        match cluster {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            2 => Difficulty::Hard,
            _ => Difficulty::Medium,
        }
    }

    fn extract_features(&self, history: &[QuizAttempt]) -> Vec<f32> {
        // Use trueno for SIMD-accelerated feature extraction
        use trueno::stats::{mean, std_dev};

        let scores: Vec<f32> = history.iter().map(|a| a.score).collect();
        let times: Vec<f32> = history.iter().map(|a| a.duration_secs as f32).collect();

        vec![
            mean(&scores),
            std_dev(&scores),
            mean(&times),
            history.len() as f32,
        ]
    }
}
```

## 4. Quiz Engine

### 4.1 State Machine

```rust
// profesor-quiz/src/engine.rs

/// Quiz state machine (Jidoka - stop on error)
#[derive(Debug, Clone)]
pub enum QuizState {
    /// Not started
    NotStarted,
    /// In progress
    InProgress {
        current_question: usize,
        answers: Vec<Option<Answer>>,
        start_time: Timestamp,
    },
    /// Reviewing answers (immediate feedback)
    ///
    /// > **[Toyota Way Annotation 6] Kaizen (Continuous Improvement)**: The reviewing state is the most critical for Kaizen.
    /// > It should not just display "Wrong", but provide a specific "Improvement Action" (e.g., "Review section 3.2").
    /// > Feedback loops must be tight and actionable to drive continuous improvement in understanding.
    Reviewing {
        answers: Vec<Answer>,
        feedback: Vec<Feedback>,
    },
    /// Completed
    Completed {
        score: Score,
        duration: Duration,
        attempt_number: u32,
    },
}

/// Quiz engine
pub struct QuizEngine {
    quiz: Quiz,
    state: QuizState,
    adaptive: AdaptiveDifficulty,
}

impl QuizEngine {
    /// Start a new quiz attempt
    pub fn start(&mut self) -> Result<&Question, QuizError> {
        self.state = QuizState::InProgress {
            current_question: 0,
            answers: vec![None; self.quiz.questions.len()],
            start_time: Timestamp::now(),
        };
        self.current_question()
    }

    /// Submit answer for current question (Jidoka)
    pub fn submit_answer(&mut self, answer: Answer) -> Result<Feedback, QuizError> {
        // Immediate feedback (Jidoka principle)
        let feedback = self.grade_answer(&answer);

        // Store answer
        if let QuizState::InProgress { answers, current_question, .. } = &mut self.state {
            answers[*current_question] = Some(answer);
        }

        Ok(feedback)
    }

    /// Grade a single answer
    fn grade_answer(&self, answer: &Answer) -> Feedback {
        let question = self.current_question().unwrap();
        let (correct, explanation) = match (question, answer) {
            (Question::MultipleChoice { correct, explanation, .. }, Answer::Choice(idx)) => {
                (*correct == *idx, explanation.clone())
            }
            (Question::FreeformCode { test_cases, .. }, Answer::Code(code)) => {
                // Run in WASM sandbox
                let results = self.run_tests(code, test_cases);
                let passed = results.iter().all(|r| r.passed);
                (passed, self.format_test_results(&results))
            }
            _ => (false, "Invalid answer type".to_string()),
        };

        Feedback { correct, explanation, points_earned: if correct { question.points() } else { 0 } }
    }
}
```

### 4.2 Grading

```rust
// profesor-quiz/src/grader.rs
use trueno::Vec;

/// Auto-grader for quizzes
pub struct Grader;

impl Grader {
    /// Grade a complete quiz attempt
    pub fn grade(quiz: &Quiz, answers: &[Answer]) -> Score {
        let mut points_earned = 0u32;
        let mut points_possible = 0u32;
        let mut correct_count = 0usize;

        for (question, answer) in quiz.questions.iter().zip(answers.iter()) {
            points_possible += question.points();

            if Self::is_correct(question, answer) {
                points_earned += question.points();
                correct_count += 1;
            }
        }

        Score {
            points_earned,
            points_possible,
            percentage: points_earned as f32 / points_possible as f32,
            correct_count,
            total_questions: quiz.questions.len(),
            passed: (points_earned as f32 / points_possible as f32) >= quiz.passing_score,
        }
    }

    /// Check if answer is correct
    fn is_correct(question: &Question, answer: &Answer) -> bool {
        match (question, answer) {
            (Question::MultipleChoice { correct, .. }, Answer::Choice(idx)) => {
                *correct == *idx
            }
            (Question::MultipleSelect { correct, .. }, Answer::MultiChoice(indices)) => {
                let mut sorted_correct = correct.clone();
                let mut sorted_answer = indices.clone();
                sorted_correct.sort();
                sorted_answer.sort();
                sorted_correct == sorted_answer
            }
            (Question::Ordering { correct_order, .. }, Answer::Order(order)) => {
                correct_order == order
            }
            (Question::Matching { correct_pairs, .. }, Answer::Pairs(pairs)) => {
                let mut sorted_correct: Vec<_> = correct_pairs.iter().copied().collect();
                let mut sorted_answer: Vec<_> = pairs.iter().copied().collect();
                sorted_correct.sort();
                sorted_answer.sort();
                sorted_correct == sorted_answer
            }
            _ => false,
        }
    }
}
```

## 5. Lab Runner

### 5.1 WASM Sandbox Execution

```rust
// profesor-lab/src/sandbox.rs

/// WASM-safe code execution sandbox
pub struct Sandbox {
    /// Memory limit in bytes
    memory_limit: usize,
    /// Execution timeout
    timeout: Duration,
}

impl Sandbox {
    /// Execute code in sandbox (WASM-safe)
    pub fn execute(&self, code: &str, language: Language, input: &str) -> ExecutionResult {
        match language {
            Language::Rust => self.execute_rust(code, input),
            Language::Python => self.execute_python(code, input),
            Language::JavaScript => self.execute_javascript(code, input),
            _ => ExecutionResult::Error("Unsupported language".into()),
        }
    }

    /// Execute Rust code (compile to WASM, run in nested sandbox)
    fn execute_rust(&self, code: &str, input: &str) -> ExecutionResult {
        // Use embedded Rust-to-WASM compiler (rustc subset)
        // This is a simplified interpreter for basic Rust
        let ast = parse_rust_subset(code)?;
        let result = interpret_ast(&ast, input, self.timeout)?;
        ExecutionResult::Success { output: result, duration: elapsed }
    }

    /// Execute Python via embedded interpreter
    fn execute_python(&self, code: &str, input: &str) -> ExecutionResult {
        // Use ruchy (Rust Python interpreter) compiled to WASM
        // ruchy is part of PAIML stack
        ruchy::execute(code, input, self.timeout)
    }
}

/// Execution result
#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Success {
        output: String,
        duration: Duration,
    },
    RuntimeError {
        error: String,
        line: Option<u32>,
    },
    /// > **[Toyota Way Annotation 7] Jidoka (Automation with Human Touch)**:
    /// > A timeout or memory error is a system halt (Andon cord pull). 
    /// > We must distinguish these from logical errors. The UI should explain *why* it stopped (e.g., "Infinite loop detected")
    /// > rather than just saying "Failed", empowering the learner to fix the process.
    Timeout,
    MemoryExceeded,
    Error(String),
}
```

### 5.2 Test Runner

```rust
// profesor-lab/src/runner.rs

/// Lab test runner
pub struct TestRunner {
    sandbox: Sandbox,
}

impl TestRunner {
    /// Run all tests for a lab submission
    pub fn run_tests(&self, submission: &Submission, lab: &Lab) -> TestResults {
        let mut results = Vec::new();

        for test in &lab.test_suite.tests {
            let result = self.run_single_test(submission, test);
            results.push(result);
        }

        TestResults {
            tests: results,
            all_passed: results.iter().all(|r| r.passed),
            score: self.calculate_score(&results, lab),
        }
    }

    /// Run a single test case
    fn run_single_test(&self, submission: &Submission, test: &TestCase) -> TestResult {
        let exec_result = self.sandbox.execute(
            &submission.code,
            submission.language,
            &test.input,
        );

        match exec_result {
            ExecutionResult::Success { output, duration } => {
                let passed = output.trim() == test.expected_output.trim();
                TestResult {
                    name: test.name.clone(),
                    passed,
                    expected: test.expected_output.clone(),
                    actual: output,
                    duration: Some(duration),
                    error: None,
                }
            }
            ExecutionResult::RuntimeError { error, line } => {
                TestResult {
                    name: test.name.clone(),
                    passed: false,
                    expected: test.expected_output.clone(),
                    actual: String::new(),
                    duration: None,
                    error: Some(format!("Runtime error at line {:?}: {}", line, error)),
                }
            }
            ExecutionResult::Timeout => {
                TestResult {
                    name: test.name.clone(),
                    passed: false,
                    expected: test.expected_output.clone(),
                    actual: String::new(),
                    duration: None,
                    error: Some("Execution timed out".into()),
                }
            }
            _ => TestResult {
                name: test.name.clone(),
                passed: false,
                expected: test.expected_output.clone(),
                actual: String::new(),
                duration: None,
                error: Some("Execution failed".into()),
            },
        }
    }
}
```

## 6. Simulations

### 6.1 State Machine Simulations

```rust
// profesor-sim/src/state.rs

/// Interactive simulation state machine
pub struct Simulation {
    pub id: SimulationId,
    pub title: String,
    pub initial_state: SimState,
    pub transitions: Vec<Transition>,
    pub render_config: RenderConfig,
}

/// Simulation state
#[derive(Debug, Clone)]
pub struct SimState {
    pub variables: HashMap<String, Value>,
    pub entities: Vec<Entity>,
    pub time: f64,
}

/// State transition
#[derive(Debug, Clone)]
pub struct Transition {
    pub trigger: Trigger,
    pub condition: Option<Condition>,
    pub actions: Vec<Action>,
}

/// Trigger types
///
/// > **[Toyota Way Annotation 8] Genchi Genbutsu**: Triggers should map to physical actions where possible.
/// > Instead of abstract "clicks", use "Drag" or "StateChange" to simulate physically manipulating the machinery or variables,
/// > bringing the learner closer to the "real thing" (even in a virtual context).
#[derive(Debug, Clone)]
pub enum Trigger {
    UserClick { target: String },
    UserDrag { target: String },
    Timer { interval_ms: u32 },
    StateChange { variable: String },
}

/// Actions to perform
#[derive(Debug, Clone)]
pub enum Action {
    SetVariable { name: String, value: Value },
    MoveEntity { id: String, to: Position },
    PlayAnimation { id: String, animation: String },
    ShowMessage { text: String },
    AdvanceStep,
}
```

### 6.2 Physics Simulations (trueno-powered)

```rust
// profesor-sim/src/physics.rs
use trueno::{Vec2, Vec3, Mat4};

/// Physics simulation using trueno SIMD
pub struct PhysicsWorld {
    /// Entities with physics bodies
    bodies: Vec<RigidBody>,
    /// Gravity vector
    gravity: Vec2,
    /// Time step
    dt: f32,
}

/// Rigid body for physics simulation
#[derive(Debug, Clone)]
pub struct RigidBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub restitution: f32,  // Bounciness
}

impl PhysicsWorld {
    /// Step the simulation forward (SIMD-accelerated)
    pub fn step(&mut self) {
        // Use trueno SIMD for batch position updates
        for body in &mut self.bodies {
            // Apply gravity
            body.acceleration = body.acceleration + self.gravity;

            // Integrate velocity
            body.velocity = body.velocity + body.acceleration * self.dt;

            // Integrate position
            body.position = body.position + body.velocity * self.dt;

            // Reset acceleration
            body.acceleration = Vec2::ZERO;
        }

        // Collision detection and response
        self.resolve_collisions();
    }

    /// Batch collision detection using trueno
    fn resolve_collisions(&mut self) {
        // Simplified AABB collision for demonstration
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                if self.check_collision(i, j) {
                    self.resolve_collision(i, j);
                }
            }
        }
    }
}
```

## 7. UI Components (Presentar)

### 7.1 Quiz Widget

```rust
// profesor-ui/src/quiz_widget.rs
use presentar::prelude::*;

/// Quiz question widget
pub struct QuizWidget {
    quiz_engine: QuizEngine,
    current_view: QuizView,
}

impl Widget for QuizWidget {
    fn render(&self, ctx: &mut RenderContext) -> Element {
        match &self.current_view {
            QuizView::Question(q) => self.render_question(ctx, q),
            QuizView::Feedback(f) => self.render_feedback(ctx, f),
            QuizView::Results(r) => self.render_results(ctx, r),
        }
    }

    fn handle_event(&mut self, event: Event) -> Option<Message> {
        match event {
            Event::Click { target } if target == "submit" => {
                let answer = self.collect_answer();
                let feedback = self.quiz_engine.submit_answer(answer).ok()?;
                self.current_view = QuizView::Feedback(feedback);
                Some(Message::AnswerSubmitted)
            }
            Event::Click { target } if target == "next" => {
                self.quiz_engine.next_question().ok()?;
                self.current_view = QuizView::Question(self.quiz_engine.current_question()?.clone());
                Some(Message::NextQuestion)
            }
            _ => None,
        }
    }
}

impl QuizWidget {
    fn render_question(&self, ctx: &mut RenderContext, question: &Question) -> Element {
        Column::new()
            .child(Text::new(&question.prompt()).size(18))
            .child(self.render_answer_input(question))
            .child(
                Button::new("Submit")
                    .id("submit")
                    .style(ButtonStyle::Primary)
            )
            .spacing(16)
            .into()
    }

    fn render_answer_input(&self, question: &Question) -> Element {
        match question {
            Question::MultipleChoice { options, .. } => {
                RadioGroup::new(options.iter().map(|o| o.as_str()).collect())
                    .into()
            }
            Question::MultipleSelect { options, .. } => {
                CheckboxGroup::new(options.iter().map(|o| o.as_str()).collect())
                    .into()
            }
            Question::FreeformCode { starter_code, language, .. } => {
                CodeEditor::new()
                    .language(*language)
                    .initial_code(starter_code)
                    .into()
            }
            _ => Text::new("Unsupported question type").into(),
        }
    }
}
```

### 7.2 Lab Widget

```rust
// profesor-ui/src/lab_widget.rs
use presentar::prelude::*;

/// Interactive lab widget
pub struct LabWidget {
    lab: Lab,
    current_step: usize,
    editor_content: String,
    test_results: Option<TestResults>,
    runner: TestRunner,
}

impl Widget for LabWidget {
    fn render(&self, ctx: &mut RenderContext) -> Element {
        Row::new()
            .child(self.render_instructions())
            .child(self.render_editor())
            .child(self.render_output())
            .spacing(8)
            .into()
    }
}

impl LabWidget {
    fn render_instructions(&self) -> Element {
        Column::new()
            .child(Text::new(&self.lab.title).size(20).bold())
            .child(Text::new(&self.lab.description))
            .child(self.render_steps())
            .width(Length::FillPortion(1))
            .into()
    }

    fn render_editor(&self) -> Element {
        Column::new()
            .child(
                CodeEditor::new()
                    .language(self.lab.language)
                    .initial_code(&self.editor_content)
                    .on_change(Message::CodeChanged)
            )
            .child(
                Row::new()
                    .child(Button::new("Run Tests").on_click(Message::RunTests))
                    .child(Button::new("Submit").on_click(Message::Submit))
            )
            .width(Length::FillPortion(2))
            .into()
    }

    fn render_output(&self) -> Element {
        match &self.test_results {
            Some(results) => self.render_test_results(results),
            /// > **[Toyota Way Annotation 9] Mieruka (Visual Control)**:
            /// > When tests run, the output must be immediately visible and color-coded.
            /// > "Green" for pass, "Red" for fail. The status of the system must be obvious at a glance.
            /// > Hidden states or log-only errors violate Mieruka.
            None => Text::new("Run tests to see results").into(),
        }
    }
}
```

## 8. Course Manifest (YAML)

### 8.1 Course Definition

```yaml
# courses/rust-fundamentals/course.yaml
id: rust-fundamentals
title: "Rust Fundamentals"
description: "Learn Rust from scratch with hands-on exercises"
level: beginner
estimated_hours: 40
prerequisites: []

modules:
  - id: mod-01-basics
    title: "Module 1: Rust Basics"
    unlock_criteria:
      type: none  # First module is always unlocked
    lessons:
      - file: module-01/lesson-01.yaml
      - file: module-01/lesson-02.yaml
    quiz: module-01/quiz-01.yaml
    lab: module-01/lab-01.yaml

  - id: mod-02-ownership
    title: "Module 2: Ownership & Borrowing"
    unlock_criteria:
      type: module_completed
      module: mod-01-basics
    lessons:
      - file: module-02/lesson-01.yaml
      - file: module-02/lesson-02.yaml
    quiz: module-02/quiz-01.yaml
    lab: module-02/lab-01.yaml
```

### 8.2 Quiz Definition

```yaml
# courses/rust-fundamentals/module-01/quiz-01.yaml
id: quiz-mod01
title: "Module 1 Quiz: Rust Basics"
time_limit: 1800  # 30 minutes in seconds
passing_score: 0.7
shuffle: true
max_attempts: 3

questions:
  - type: multiple_choice
    id: q1
    prompt: "What keyword is used to declare an immutable variable in Rust?"
    options:
      - "var"
      - "let"
      - "const"
      - "mut"
    correct: 1
    explanation: "In Rust, `let` declares a variable. By default, variables are immutable."
    points: 10

  - type: code_completion
    id: q2
    prompt: "Complete the function to return the sum of two integers:"
    code_template: |
      fn add(a: i32, b: i32) -> i32 {
          {{blank1}}
      }
    blanks:
      - id: blank1
        acceptable_answers:
          - "a + b"
          - "return a + b;"
          - "return a + b"
        hint: "Use the + operator"
    test_cases:
      - input: "add(2, 3)"
        expected: "5"
      - input: "add(-1, 1)"
        expected: "0"
    points: 20

  - type: freeform_code
    id: q3
    prompt: "Write a function `is_even` that returns true if a number is even."
    language: rust
    starter_code: |
      fn is_even(n: i32) -> bool {
          // Your code here
      }
    test_cases:
      - name: "even number"
        input: "is_even(4)"
        expected: "true"
      - name: "odd number"
        input: "is_even(7)"
        expected: "false"
    hidden_test_cases:
      - name: "zero"
        input: "is_even(0)"
        expected: "true"
      - name: "negative even"
        input: "is_even(-2)"
        expected: "true"
    points: 30
```

### 8.3 Lab Definition

```yaml
# courses/rust-fundamentals/module-01/lab-01.yaml
id: lab-mod01
title: "Lab: Your First Rust Program"
description: "Build a simple command-line calculator"
language: rust
difficulty: beginner
estimated_minutes: 45

instructions:
  - number: 1
    title: "Create the add function"
    description: |
      Implement a function that adds two numbers.

      ```rust
      fn add(a: f64, b: f64) -> f64
      ```
    validation:
      type: function_exists
      name: add

  - number: 2
    title: "Create the subtract function"
    description: "Implement subtraction similarly."
    validation:
      type: tests_pass
      test_names: ["test_subtract"]

  - number: 3
    title: "Create the main calculator"
    description: "Combine operations in a calculator function."
    validation:
      type: tests_pass
      test_names: ["test_calculator"]

starter_files:
  - path: "src/lib.rs"
    content: |
      // Implement your calculator functions here

      pub fn add(a: f64, b: f64) -> f64 {
          todo!()
      }

      pub fn subtract(a: f64, b: f64) -> f64 {
          todo!()
      }

      pub fn calculate(op: &str, a: f64, b: f64) -> Option<f64> {
          todo!()
      }
    readonly: false

test_suite:
  tests:
    - name: test_add
      code: |
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(add(-1.0, 1.0), 0.0);
    - name: test_subtract
      code: |
        assert_eq!(subtract(5.0, 3.0), 2.0);
    - name: test_calculator
      code: |
        assert_eq!(calculate("+", 2.0, 3.0), Some(5.0));
        assert_eq!(calculate("-", 5.0, 3.0), Some(2.0));
        assert_eq!(calculate("?", 1.0, 1.0), None);

hints:
  - step: 1
    text: "Remember that f64 supports the + operator"
  - step: 3
    text: "Use match to handle different operation strings"
```

## 9. Academic References

1. **Adaptive Learning**: Corbett, A. T., & Anderson, J. R. (1994). Knowledge tracing: Modeling the acquisition of procedural knowledge. *User Modeling and User-Adapted Interaction*.

2. **Spaced Repetition**: Pimsleur, P. (1967). A memory schedule. *The Modern Language Journal*.

3. **Immediate Feedback**: Shute, V. J. (2008). Focus on formative feedback. *Review of Educational Research*.

4. **Active Learning**: Freeman, S., et al. (2014). Active learning increases student performance in science, engineering, and mathematics. *PNAS*.

5. **Mastery Learning**: Bloom, B. S. (1968). Learning for mastery. *Evaluation Comment*.

6. **Cognitive Load**: Sweller, J. (1988). Cognitive load during problem solving. *Cognitive Science*.

7. **Gamification**: Deterding, S., et al. (2011). From game design elements to gamefulness. *MindTrek*.

8. **Self-Determination**: Ryan, R. M., & Deci, E. L. (2000). Self-determination theory. *American Psychologist*.

9. **Zone of Proximal Development**: Vygotsky, L. S. (1978). Mind in society. *Harvard University Press*.

10. **Deliberate Practice**: Ericsson, K. A. (1993). The role of deliberate practice. *Psychological Review*.

## 10. Performance Requirements

| Metric | Target | Rationale |
|--------|--------|-----------|
| WASM Bundle Size | < 2 MB | Fast initial load |
| Quiz Render | < 16ms | 60fps interaction |
| Code Execution | < 5s | Acceptable wait time |
| Memory Usage | < 64 MB | Mobile-friendly |
| Time to Interactive | < 3s | User engagement |

> **[Toyota Way Annotation 10] Muda (Waste Elimination)**:
> "Waiting" is one of the 7 wastes.
> Performance requirements are not just technical stats; they are a commitment to eliminating *Muda* for the user.
> Any delay > 3s breaks the "flow" and constitutes waste in the learning process.

## 11. Quality Methodology (Adapted from wos)

### 11.1 Extreme TDD (MANDATORY)

No production code without failing test first. Follow the wos pattern:

1. **RED**: Write failing test (unit + property + mutation)
2. **GREEN**: Minimum implementation to pass
3. **REFACTOR**: Optimize while maintaining all test passes
4. **VERIFY**: Run `make quality` + PMAT gates
5. **COMMIT**: Atomic commit with `[PROF-XXX]` ticket prefix

### 11.2 Testing Requirements (ALL Required)

| Test Type | Requirement | Rationale |
|-----------|-------------|-----------|
| Unit Tests | 95%+ coverage | Per global CLAUDE.md |
| Property Tests | 10K inputs per test (proptest) | Catch edge cases |
| Mutation Tests | 90%+ kill rate | Verify test effectiveness |
| Fuzz Tests | All parsers and input handlers | Security hardening |

### 11.3 PMAT Quality Gates (Block Commit on Failure)

```bash
# Zero tolerance gates (adapted from wos)
make pmat-satd        # Zero TODO/FIXME comments
make pmat-complexity  # Max cyclomatic complexity 15
make pmat-tdg         # Technical Debt Grade ≥ A-
make pmat-defects     # Zero .unwrap() in production code
```

### 11.4 Local Development Server Policy

**MANDATORY**: Use `ruchy serve` for ALL local development (adapted from wos).

**FORBIDDEN**:
- ❌ `python -m http.server`
- ❌ `npx http-server` / `npx serve`
- ❌ Any JavaScript-based dev server

**REQUIRED**:
```bash
# Install ruchy (PAIML stack HTTP server)
cargo install ruchy

# Development with hot reload
ruchy serve dist/ --port 8000 --watch --watch-wasm --verbose
```

**Why ruchy serve?**
- 12x faster than Python http.server
- WASM-optimized with COOP/COEP headers
- Hot reload for .wasm files
- Memory safe (Rust)
- Zero JavaScript dependency

## 12. Future Considerations

- **Peer Assessment**: Student-graded assignments
- **Certificates**: Verifiable completion credentials
- **Collaborative Labs**: Multi-user coding sessions
- **AI Tutoring**: LLM-powered hints (via WASM LLM)
- **Offline Mode**: Service worker + IndexedDB

---

**Document Version**: 1.0
**Last Updated**: 2024-12-08
**Authors**: PAIML Team