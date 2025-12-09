//! Lab types and structures.
//!
//! Defines hands-on coding labs with test suites and validation.

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::ids::LabId;
use crate::quiz::TestCase;

/// A hands-on coding lab.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lab {
    /// Unique identifier
    pub id: LabId,
    /// Human-readable title
    pub title: String,
    /// Lab description
    pub description: String,
    /// Programming language
    pub language: Language,
    /// Difficulty level
    pub difficulty: Difficulty,
    /// Estimated time in minutes
    pub estimated_minutes: u32,
    /// Step-by-step instructions
    pub instructions: Vec<LabStep>,
    /// Starter files provided to the learner
    pub starter_files: Vec<StarterFile>,
    /// Test suite for validation
    pub test_suite: TestSuite,
    /// Hints available to the learner
    pub hints: Vec<Hint>,
}

impl Lab {
    /// Create a new lab.
    #[must_use]
    pub fn new(id: impl Into<LabId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: String::new(),
            language: Language::Rust,
            difficulty: Difficulty::Beginner,
            estimated_minutes: 30,
            instructions: Vec::new(),
            starter_files: Vec::new(),
            test_suite: TestSuite::default(),
            hints: Vec::new(),
        }
    }

    /// Set the description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the language.
    #[must_use]
    pub fn with_language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    /// Set the difficulty.
    #[must_use]
    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }

    /// Set the estimated time.
    #[must_use]
    pub fn with_estimated_minutes(mut self, minutes: u32) -> Self {
        self.estimated_minutes = minutes;
        self
    }

    /// Add an instruction step.
    #[must_use]
    pub fn with_step(mut self, step: LabStep) -> Self {
        self.instructions.push(step);
        self
    }

    /// Add a starter file.
    #[must_use]
    pub fn with_starter_file(mut self, file: StarterFile) -> Self {
        self.starter_files.push(file);
        self
    }

    /// Set the test suite.
    #[must_use]
    pub fn with_test_suite(mut self, suite: TestSuite) -> Self {
        self.test_suite = suite;
        self
    }

    /// Add a hint.
    #[must_use]
    pub fn with_hint(mut self, hint: Hint) -> Self {
        self.hints.push(hint);
        self
    }

    /// Get the number of steps.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.instructions.len()
    }
}

/// Programming languages supported for labs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Language {
    /// Rust programming language
    #[default]
    Rust,
    /// Python programming language
    Python,
    /// JavaScript (executed via WASM interpreter)
    JavaScript,
    /// TypeScript (transpiled to JavaScript)
    TypeScript,
    /// SQL queries
    Sql,
    /// Markdown (for documentation exercises)
    Markdown,
}

impl Language {
    /// Get the file extension for this language.
    #[must_use]
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::Python => "py",
            Self::JavaScript => "js",
            Self::TypeScript => "ts",
            Self::Sql => "sql",
            Self::Markdown => "md",
        }
    }

    /// Get the display name.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Python => "Python",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Sql => "SQL",
            Self::Markdown => "Markdown",
        }
    }
}

/// Lab difficulty level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Difficulty {
    /// Easy exercises for beginners
    #[default]
    Beginner,
    /// Moderate difficulty
    Intermediate,
    /// Challenging exercises
    Advanced,
    /// Expert-level challenges
    Expert,
}

impl Difficulty {
    /// Get a human-readable label.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Advanced => "Advanced",
            Self::Expert => "Expert",
        }
    }
}

/// A step in the lab instructions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LabStep {
    /// Step number (1-indexed)
    pub number: u32,
    /// Step title
    pub title: String,
    /// Step description (markdown)
    pub description: String,
    /// Optional validation for this step
    pub validation: Option<StepValidation>,
}

impl LabStep {
    /// Create a new lab step.
    #[must_use]
    pub fn new(number: u32, title: impl Into<String>) -> Self {
        Self {
            number,
            title: title.into(),
            description: String::new(),
            validation: None,
        }
    }

    /// Set the description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the validation.
    #[must_use]
    pub fn with_validation(mut self, validation: StepValidation) -> Self {
        self.validation = Some(validation);
        self
    }
}

/// Validation for a lab step.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepValidation {
    /// Check that a function exists
    FunctionExists {
        /// Name of the function to check
        name: String,
    },
    /// Check that specific tests pass
    TestsPass {
        /// Names of tests that must pass
        test_names: Vec<String>,
    },
    /// Check that output matches expected
    OutputMatches {
        /// Expected output
        expected: String,
    },
}

/// A starter file provided to the learner.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StarterFile {
    /// File path relative to project root
    pub path: String,
    /// Initial content
    pub content: String,
    /// Whether the file is read-only
    pub readonly: bool,
}

impl StarterFile {
    /// Create a new editable starter file.
    #[must_use]
    pub fn new(path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
            readonly: false,
        }
    }

    /// Create a read-only starter file.
    #[must_use]
    pub fn readonly(path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
            readonly: true,
        }
    }
}

/// Test suite for a lab.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TestSuite {
    /// Test cases
    pub tests: Vec<TestCase>,
}

impl TestSuite {
    /// Create a new empty test suite.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a test case.
    #[must_use]
    pub fn with_test(mut self, test: TestCase) -> Self {
        self.tests.push(test);
        self
    }

    /// Get the number of tests.
    #[must_use]
    pub fn test_count(&self) -> usize {
        self.tests.len()
    }
}

/// A hint for a lab.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hint {
    /// Which step this hint applies to
    pub step: u32,
    /// The hint text
    pub text: String,
}

impl Hint {
    /// Create a new hint.
    #[must_use]
    pub fn new(step: u32, text: impl Into<String>) -> Self {
        Self {
            step,
            text: text.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lab_creation() {
        let lab = Lab::new("lab-01", "First Lab")
            .with_description("Learn the basics")
            .with_language(Language::Rust)
            .with_difficulty(Difficulty::Beginner)
            .with_estimated_minutes(45);

        assert_eq!(lab.id.as_str(), "lab-01");
        assert_eq!(lab.title, "First Lab");
        assert_eq!(lab.language, Language::Rust);
        assert_eq!(lab.difficulty, Difficulty::Beginner);
        assert_eq!(lab.estimated_minutes, 45);
    }

    #[test]
    fn test_lab_steps() {
        let lab = Lab::new("test", "Test")
            .with_step(LabStep::new(1, "Step 1"))
            .with_step(LabStep::new(2, "Step 2"));

        assert_eq!(lab.step_count(), 2);
    }

    #[test]
    fn test_language_extension() {
        assert_eq!(Language::Rust.extension(), "rs");
        assert_eq!(Language::Python.extension(), "py");
        assert_eq!(Language::JavaScript.extension(), "js");
    }

    #[test]
    fn test_language_name() {
        assert_eq!(Language::Rust.name(), "Rust");
        assert_eq!(Language::TypeScript.name(), "TypeScript");
    }

    #[test]
    fn test_starter_file() {
        let file = StarterFile::new("src/lib.rs", "// starter code");
        assert!(!file.readonly);

        let readonly = StarterFile::readonly("Cargo.toml", "[package]");
        assert!(readonly.readonly);
    }

    #[test]
    fn test_step_validation() {
        let step = LabStep::new(1, "Create function")
            .with_validation(StepValidation::FunctionExists { name: "add".into() });

        assert!(step.validation.is_some());
    }

    #[test]
    fn test_test_suite() {
        let suite = TestSuite::new()
            .with_test(TestCase::new("test1"))
            .with_test(TestCase::new("test2"));

        assert_eq!(suite.test_count(), 2);
    }

    #[test]
    fn test_hint() {
        let hint = Hint::new(1, "Use the + operator");
        assert_eq!(hint.step, 1);
        assert_eq!(hint.text, "Use the + operator");
    }

    #[test]
    fn test_difficulty_label() {
        assert_eq!(Difficulty::Beginner.label(), "Beginner");
        assert_eq!(Difficulty::Expert.label(), "Expert");
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_lab_step_count_matches(num_steps in 0usize..20) {
            let mut lab = Lab::new("test", "Test");
            for i in 0..num_steps {
                lab = lab.with_step(LabStep::new(i as u32 + 1, format!("Step {}", i + 1)));
            }
            prop_assert_eq!(lab.step_count(), num_steps);
        }

        #[test]
        fn test_test_suite_count_matches(num_tests in 0usize..20) {
            let mut suite = TestSuite::new();
            for i in 0..num_tests {
                suite = suite.with_test(TestCase::new(format!("test{}", i)));
            }
            prop_assert_eq!(suite.test_count(), num_tests);
        }
    }
}
