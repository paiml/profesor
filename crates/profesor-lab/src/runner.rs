//! Lab test runner.
//!
//! Runs test suites for lab submissions and provides detailed feedback.

use alloc::string::String;
use alloc::vec::Vec;
use profesor_core::{Lab, TestCase};
use serde::{Deserialize, Serialize};

use crate::sandbox::{ExecutionResult, Sandbox};

/// Test runner for lab submissions.
#[derive(Debug, Clone)]
pub struct TestRunner {
    sandbox: Sandbox,
}

impl TestRunner {
    /// Create a new test runner with default sandbox.
    #[must_use]
    pub fn new() -> Self {
        Self {
            sandbox: Sandbox::new(),
        }
    }

    /// Create a test runner with a custom sandbox.
    #[must_use]
    pub fn with_sandbox(sandbox: Sandbox) -> Self {
        Self { sandbox }
    }

    /// Run all tests for a lab submission.
    #[must_use]
    pub fn run_tests(&self, code: &str, lab: &Lab) -> TestResults {
        let mut results = Vec::new();

        for test in &lab.test_suite.tests {
            let result = self.run_single_test(code, lab, test);
            results.push(result);
        }

        let passed_count = results.iter().filter(|r| r.passed).count();
        let all_passed = passed_count == results.len();

        TestResults {
            results,
            all_passed,
            passed_count,
            total_count: lab.test_suite.test_count(),
        }
    }

    /// Run a single test case.
    fn run_single_test(&self, code: &str, lab: &Lab, test: &TestCase) -> TestResult {
        let exec_result = self.sandbox.execute(code, lab.language, &test.input);

        match exec_result {
            ExecutionResult::Success {
                output,
                duration_ms,
            } => {
                let passed = output.trim() == test.expected_output.trim();
                TestResult {
                    name: test.name.clone(),
                    passed,
                    expected: test.expected_output.clone(),
                    actual: output,
                    duration_ms: Some(duration_ms),
                    error: None,
                }
            }
            ExecutionResult::RuntimeError { error, line } => TestResult {
                name: test.name.clone(),
                passed: false,
                expected: test.expected_output.clone(),
                actual: String::new(),
                duration_ms: None,
                error: Some(alloc::format!(
                    "Runtime error{}: {}",
                    line.map_or(String::new(), |l| alloc::format!(" at line {}", l)),
                    error
                )),
            },
            ExecutionResult::Timeout { partial_output } => TestResult {
                name: test.name.clone(),
                passed: false,
                expected: test.expected_output.clone(),
                actual: partial_output,
                duration_ms: None,
                error: Some("Execution timed out".into()),
            },
            ExecutionResult::MemoryExceeded { used_bytes } => TestResult {
                name: test.name.clone(),
                passed: false,
                expected: test.expected_output.clone(),
                actual: String::new(),
                duration_ms: None,
                error: Some(alloc::format!(
                    "Memory limit exceeded: {} bytes used",
                    used_bytes
                )),
            },
            ExecutionResult::Error { message } => TestResult {
                name: test.name.clone(),
                passed: false,
                expected: test.expected_output.clone(),
                actual: String::new(),
                duration_ms: None,
                error: Some(message),
            },
        }
    }

    /// Calculate a score based on test results.
    #[must_use]
    pub fn calculate_score(&self, results: &TestResults, max_points: u32) -> u32 {
        if results.total_count == 0 {
            return 0;
        }

        let ratio = results.passed_count as f32 / results.total_count as f32;
        (ratio * max_points as f32) as u32
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of running a single test.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestResult {
    /// Name of the test
    pub name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Expected output
    pub expected: String,
    /// Actual output
    pub actual: String,
    /// Execution duration in milliseconds
    pub duration_ms: Option<u32>,
    /// Error message if failed
    pub error: Option<String>,
}

impl TestResult {
    /// Check if the test failed.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        !self.passed
    }

    /// Get a summary of the result.
    #[must_use]
    pub fn summary(&self) -> String {
        if self.passed {
            alloc::format!("✓ {}", self.name)
        } else if let Some(ref err) = self.error {
            alloc::format!("✗ {}: {}", self.name, err)
        } else {
            alloc::format!(
                "✗ {}: expected '{}', got '{}'",
                self.name,
                self.expected,
                self.actual
            )
        }
    }
}

/// Results of running all tests for a lab.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestResults {
    /// Individual test results
    pub results: Vec<TestResult>,
    /// Whether all tests passed
    pub all_passed: bool,
    /// Number of tests that passed
    pub passed_count: usize,
    /// Total number of tests
    pub total_count: usize,
}

impl TestResults {
    /// Get the pass rate as a percentage (0.0 - 1.0).
    #[must_use]
    pub fn pass_rate(&self) -> f32 {
        if self.total_count == 0 {
            return 0.0;
        }
        self.passed_count as f32 / self.total_count as f32
    }

    /// Get all failed tests.
    #[must_use]
    pub fn failed_tests(&self) -> Vec<&TestResult> {
        self.results.iter().filter(|r| !r.passed).collect()
    }

    /// Get a summary of results.
    #[must_use]
    pub fn summary(&self) -> String {
        alloc::format!(
            "{}/{} tests passed ({}%)",
            self.passed_count,
            self.total_count,
            (self.pass_rate() * 100.0) as u32
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use profesor_core::{Lab, Language, TestSuite};

    fn create_test_lab() -> Lab {
        Lab::new("test-lab", "Test Lab")
            .with_language(Language::Rust)
            .with_test_suite(
                TestSuite::new()
                    .with_test(TestCase::new("test1").with_input("").with_expected("hello"))
                    .with_test(
                        TestCase::new("test2")
                            .with_input("world")
                            .with_expected("hello world"),
                    ),
            )
    }

    #[test]
    fn test_runner_creation() {
        let runner = TestRunner::new();
        assert!(runner.sandbox.config().timeout_ms > 0);
    }

    #[test]
    fn test_runner_default() {
        let runner = TestRunner::default();
        assert!(runner.sandbox.config().timeout_ms > 0);
    }

    #[test]
    fn test_runner_with_sandbox() {
        let sandbox = Sandbox::new().with_timeout_ms(5000);
        let runner = TestRunner::with_sandbox(sandbox);
        assert_eq!(runner.sandbox.config().timeout_ms, 5000);
    }

    #[test]
    fn test_run_tests_empty_lab() {
        let runner = TestRunner::new();
        let lab = Lab::new("empty", "Empty").with_language(Language::Rust);
        let results = runner.run_tests("fn main() {}", &lab);
        assert!(results.all_passed);
        assert_eq!(results.total_count, 0);
    }

    #[test]
    fn test_run_tests_with_tests() {
        let runner = TestRunner::new();
        let lab = create_test_lab();
        let results = runner.run_tests("fn main() {}", &lab);
        // In sandbox mode, this will return errors since we can't actually execute
        assert_eq!(results.total_count, 2);
    }

    #[test]
    fn test_result_is_failed() {
        let passed = TestResult {
            name: "test_pass".into(),
            passed: true,
            expected: "5".into(),
            actual: "5".into(),
            duration_ms: Some(10),
            error: None,
        };
        assert!(!passed.is_failed());

        let failed = TestResult {
            name: "test_fail".into(),
            passed: false,
            expected: "3".into(),
            actual: "4".into(),
            duration_ms: None,
            error: None,
        };
        assert!(failed.is_failed());
    }

    #[test]
    fn test_result_summary() {
        let passed = TestResult {
            name: "test_add".into(),
            passed: true,
            expected: "5".into(),
            actual: "5".into(),
            duration_ms: Some(10),
            error: None,
        };
        assert!(passed.summary().contains("✓"));
        assert!(passed.summary().contains("test_add"));

        let failed_no_error = TestResult {
            name: "test_sub".into(),
            passed: false,
            expected: "3".into(),
            actual: "4".into(),
            duration_ms: None,
            error: None,
        };
        assert!(failed_no_error.summary().contains("✗"));
        assert!(failed_no_error.summary().contains("expected '3'"));
        assert!(failed_no_error.summary().contains("got '4'"));

        let failed_with_error = TestResult {
            name: "test_error".into(),
            passed: false,
            expected: "".into(),
            actual: "".into(),
            duration_ms: None,
            error: Some("Runtime error".into()),
        };
        assert!(failed_with_error.summary().contains("✗"));
        assert!(failed_with_error.summary().contains("Runtime error"));
    }

    #[test]
    fn test_results_pass_rate() {
        let results = TestResults {
            results: alloc::vec![
                TestResult {
                    name: "t1".into(),
                    passed: true,
                    expected: "".into(),
                    actual: "".into(),
                    duration_ms: None,
                    error: None,
                },
                TestResult {
                    name: "t2".into(),
                    passed: false,
                    expected: "".into(),
                    actual: "".into(),
                    duration_ms: None,
                    error: Some("failed".into()),
                },
            ],
            all_passed: false,
            passed_count: 1,
            total_count: 2,
        };

        assert!((results.pass_rate() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_results_pass_rate_empty() {
        let results = TestResults {
            results: alloc::vec![],
            all_passed: true,
            passed_count: 0,
            total_count: 0,
        };
        assert!((results.pass_rate()).abs() < f32::EPSILON);
    }

    #[test]
    fn test_results_failed_tests() {
        let results = TestResults {
            results: alloc::vec![
                TestResult {
                    name: "pass".into(),
                    passed: true,
                    expected: "".into(),
                    actual: "".into(),
                    duration_ms: None,
                    error: None,
                },
                TestResult {
                    name: "fail".into(),
                    passed: false,
                    expected: "".into(),
                    actual: "".into(),
                    duration_ms: None,
                    error: None,
                },
            ],
            all_passed: false,
            passed_count: 1,
            total_count: 2,
        };

        let failed = results.failed_tests();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].name, "fail");
    }

    #[test]
    fn test_results_summary() {
        let results = TestResults {
            results: alloc::vec![],
            all_passed: false,
            passed_count: 8,
            total_count: 10,
        };

        let summary = results.summary();
        assert!(summary.contains("8/10"));
        assert!(summary.contains("80%"));
    }

    #[test]
    fn test_calculate_score() {
        let runner = TestRunner::new();

        let results = TestResults {
            results: alloc::vec![],
            all_passed: true,
            passed_count: 8,
            total_count: 10,
        };

        let score = runner.calculate_score(&results, 100);
        assert_eq!(score, 80);
    }

    #[test]
    fn test_calculate_score_full() {
        let runner = TestRunner::new();

        let results = TestResults {
            results: alloc::vec![],
            all_passed: true,
            passed_count: 10,
            total_count: 10,
        };

        let score = runner.calculate_score(&results, 100);
        assert_eq!(score, 100);
    }

    #[test]
    fn test_empty_results_score() {
        let runner = TestRunner::new();

        let results = TestResults {
            results: alloc::vec![],
            all_passed: true,
            passed_count: 0,
            total_count: 0,
        };

        let score = runner.calculate_score(&results, 100);
        assert_eq!(score, 0);
    }
}
