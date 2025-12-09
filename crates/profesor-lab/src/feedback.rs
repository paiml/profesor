//! Error explanations and feedback generation.
//!
//! Implements the Jidoka principle: when execution stops, provide actionable
//! feedback to help the learner understand what went wrong and how to fix it.

use alloc::string::String;
use alloc::vec::Vec;
use profesor_core::Language;

/// Feedback generator for lab execution results.
pub struct FeedbackGenerator;

impl FeedbackGenerator {
    /// Generate helpful feedback for a runtime error.
    #[must_use]
    pub fn explain_error(error: &str, language: Language) -> ErrorExplanation {
        match language {
            Language::Rust => Self::explain_rust_error(error),
            Language::Python => Self::explain_python_error(error),
            _ => ErrorExplanation::generic(error),
        }
    }

    /// Explain a Rust compilation/runtime error.
    fn explain_rust_error(error: &str) -> ErrorExplanation {
        let error_lower = error.to_lowercase();

        if error_lower.contains("cannot borrow") {
            ErrorExplanation {
                category: ErrorCategory::BorrowChecker,
                summary: "Borrow checker error".into(),
                explanation: "Rust's borrow checker prevents data races by ensuring references follow ownership rules.".into(),
                suggestion: "Consider using .clone() to create an owned copy, or restructure your code to avoid overlapping borrows.".into(),
                related_concepts: alloc::vec!["Ownership".into(), "Borrowing".into(), "Lifetimes".into()],
            }
        } else if error_lower.contains("type mismatch")
            || error_lower.contains("expected") && error_lower.contains("found")
        {
            ErrorExplanation {
                category: ErrorCategory::TypeMismatch,
                summary: "Type mismatch error".into(),
                explanation: "The types don't match what the function or operation expects.".into(),
                suggestion: "Check the function signature and ensure you're passing the correct types. You may need type conversion.".into(),
                related_concepts: alloc::vec!["Type System".into(), "Type Inference".into()],
            }
        } else if error_lower.contains("cannot find") || error_lower.contains("not found") {
            ErrorExplanation {
                category: ErrorCategory::NotFound,
                summary: "Item not found".into(),
                explanation:
                    "The compiler cannot find the variable, function, or type you're referencing."
                        .into(),
                suggestion:
                    "Check for typos in the name. Ensure the item is in scope or properly imported."
                        .into(),
                related_concepts: alloc::vec![
                    "Scope".into(),
                    "Modules".into(),
                    "use statements".into()
                ],
            }
        } else if error_lower.contains("overflow") {
            ErrorExplanation {
                category: ErrorCategory::RuntimeError,
                summary: "Arithmetic overflow".into(),
                explanation: "The calculation resulted in a value too large or too small for the data type.".into(),
                suggestion: "Consider using checked arithmetic methods like checked_add() or a larger integer type.".into(),
                related_concepts: alloc::vec!["Integer Types".into(), "Overflow".into()],
            }
        } else if error_lower.contains("index out of bounds") {
            ErrorExplanation {
                category: ErrorCategory::RuntimeError,
                summary: "Index out of bounds".into(),
                explanation: "You tried to access an element at an index that doesn't exist in the collection.".into(),
                suggestion: "Check the length of the collection before accessing. Consider using .get() which returns Option.".into(),
                related_concepts: alloc::vec!["Arrays".into(), "Vectors".into(), "Option".into()],
            }
        } else if error_lower.contains("unwrap")
            || error_lower.contains("called `option::unwrap()` on a `none`")
        {
            ErrorExplanation {
                category: ErrorCategory::RuntimeError,
                summary: "Unwrap on None/Err".into(),
                explanation: "Called unwrap() on a None or Err value, which causes a panic.".into(),
                suggestion: "Use pattern matching, if let, or ? operator instead of unwrap() for proper error handling.".into(),
                related_concepts: alloc::vec!["Option".into(), "Result".into(), "Error Handling".into()],
            }
        } else {
            ErrorExplanation::generic(error)
        }
    }

    /// Explain a Python error.
    fn explain_python_error(error: &str) -> ErrorExplanation {
        let error_lower = error.to_lowercase();

        if error_lower.contains("indentationerror") {
            ErrorExplanation {
                category: ErrorCategory::SyntaxError,
                summary: "Indentation error".into(),
                explanation: "Python uses indentation to define code blocks. Your indentation is inconsistent.".into(),
                suggestion: "Use consistent indentation (4 spaces recommended). Don't mix tabs and spaces.".into(),
                related_concepts: alloc::vec!["Code Blocks".into(), "Syntax".into()],
            }
        } else if error_lower.contains("nameerror") {
            ErrorExplanation {
                category: ErrorCategory::NotFound,
                summary: "Name not defined".into(),
                explanation: "You're using a variable or function that hasn't been defined yet."
                    .into(),
                suggestion: "Check for typos. Make sure the variable is defined before you use it."
                    .into(),
                related_concepts: alloc::vec!["Variables".into(), "Scope".into()],
            }
        } else if error_lower.contains("typeerror") {
            ErrorExplanation {
                category: ErrorCategory::TypeMismatch,
                summary: "Type error".into(),
                explanation: "An operation was applied to an object of inappropriate type.".into(),
                suggestion:
                    "Check the types of your variables. You may need to convert between types."
                        .into(),
                related_concepts: alloc::vec!["Types".into(), "Type Conversion".into()],
            }
        } else if error_lower.contains("indexerror") {
            ErrorExplanation {
                category: ErrorCategory::RuntimeError,
                summary: "Index out of range".into(),
                explanation: "You tried to access a list index that doesn't exist.".into(),
                suggestion: "Check len() before accessing. Remember Python uses 0-based indexing."
                    .into(),
                related_concepts: alloc::vec!["Lists".into(), "Indexing".into()],
            }
        } else if error_lower.contains("keyerror") {
            ErrorExplanation {
                category: ErrorCategory::RuntimeError,
                summary: "Key not found".into(),
                explanation: "The dictionary key you're looking for doesn't exist.".into(),
                suggestion: "Use .get() method which returns None for missing keys, or check with 'in' operator first.".into(),
                related_concepts: alloc::vec!["Dictionaries".into(), "Keys".into()],
            }
        } else if error_lower.contains("zerodivisionerror") {
            ErrorExplanation {
                category: ErrorCategory::RuntimeError,
                summary: "Division by zero".into(),
                explanation: "You attempted to divide a number by zero.".into(),
                suggestion: "Add a check before division to ensure the divisor is not zero.".into(),
                related_concepts: alloc::vec!["Arithmetic".into(), "Error Handling".into()],
            }
        } else {
            ErrorExplanation::generic(error)
        }
    }

    /// Generate feedback comparing expected vs actual output.
    #[must_use]
    pub fn compare_outputs(expected: &str, actual: &str) -> OutputComparison {
        let expected_trimmed = expected.trim();
        let actual_trimmed = actual.trim();

        if expected_trimmed == actual_trimmed {
            return OutputComparison {
                matches: true,
                difference: DifferenceType::None,
                hint: None,
            };
        }

        // Check for whitespace differences
        if expected_trimmed.replace(' ', "") == actual_trimmed.replace(' ', "") {
            return OutputComparison {
                matches: false,
                difference: DifferenceType::Whitespace,
                hint: Some(
                    "Check your spacing - the content is correct but whitespace differs.".into(),
                ),
            };
        }

        // Check for case differences
        if expected_trimmed.to_lowercase() == actual_trimmed.to_lowercase() {
            return OutputComparison {
                matches: false,
                difference: DifferenceType::Case,
                hint: Some("Check capitalization - the content matches but case differs.".into()),
            };
        }

        // Check for newline differences
        let expected_lines: Vec<_> = expected_trimmed.lines().collect();
        let actual_lines: Vec<_> = actual_trimmed.lines().collect();

        if expected_lines.len() != actual_lines.len() {
            return OutputComparison {
                matches: false,
                difference: DifferenceType::LineCount,
                hint: Some(alloc::format!(
                    "Expected {} lines but got {}.",
                    expected_lines.len(),
                    actual_lines.len()
                )),
            };
        }

        // Find first differing line
        for (i, (exp, act)) in expected_lines.iter().zip(actual_lines.iter()).enumerate() {
            if exp != act {
                return OutputComparison {
                    matches: false,
                    difference: DifferenceType::Content,
                    hint: Some(alloc::format!(
                        "Line {} differs: expected '{}', got '{}'",
                        i + 1,
                        exp,
                        act
                    )),
                };
            }
        }

        OutputComparison {
            matches: false,
            difference: DifferenceType::Content,
            hint: None,
        }
    }
}

/// Detailed error explanation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorExplanation {
    /// Category of error
    pub category: ErrorCategory,
    /// Short summary
    pub summary: String,
    /// Detailed explanation
    pub explanation: String,
    /// Actionable suggestion
    pub suggestion: String,
    /// Related concepts to review
    pub related_concepts: Vec<String>,
}

impl ErrorExplanation {
    /// Create a generic explanation for unknown errors.
    fn generic(error: &str) -> Self {
        Self {
            category: ErrorCategory::Unknown,
            summary: "Error occurred".into(),
            explanation: error.into(),
            suggestion: "Review your code and check for common mistakes.".into(),
            related_concepts: Vec::new(),
        }
    }
}

/// Categories of errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Syntax error (parsing failed)
    SyntaxError,
    /// Type mismatch
    TypeMismatch,
    /// Variable/function not found
    NotFound,
    /// Borrow checker error (Rust specific)
    BorrowChecker,
    /// Runtime error
    RuntimeError,
    /// Timeout
    Timeout,
    /// Memory limit exceeded
    MemoryExceeded,
    /// Unknown error
    Unknown,
}

/// Result of comparing expected vs actual output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputComparison {
    /// Whether outputs match
    pub matches: bool,
    /// Type of difference
    pub difference: DifferenceType,
    /// Helpful hint
    pub hint: Option<String>,
}

/// Types of output differences.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifferenceType {
    /// No difference
    None,
    /// Only whitespace differs
    Whitespace,
    /// Only case differs
    Case,
    /// Different number of lines
    LineCount,
    /// Content differs
    Content,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_borrow_error() {
        let explanation = FeedbackGenerator::explain_error(
            "cannot borrow `x` as mutable because it is also borrowed as immutable",
            Language::Rust,
        );

        assert_eq!(explanation.category, ErrorCategory::BorrowChecker);
        assert!(explanation.suggestion.contains("clone"));
    }

    #[test]
    fn test_rust_type_mismatch() {
        let explanation =
            FeedbackGenerator::explain_error("expected `i32`, found `&str`", Language::Rust);

        assert_eq!(explanation.category, ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_rust_not_found() {
        let explanation = FeedbackGenerator::explain_error(
            "cannot find value `foo` in this scope",
            Language::Rust,
        );

        assert_eq!(explanation.category, ErrorCategory::NotFound);
    }

    #[test]
    fn test_rust_overflow() {
        let explanation =
            FeedbackGenerator::explain_error("attempt to add with overflow", Language::Rust);

        assert_eq!(explanation.category, ErrorCategory::RuntimeError);
        assert!(explanation.suggestion.contains("checked"));
    }

    #[test]
    fn test_rust_index_out_of_bounds() {
        let explanation = FeedbackGenerator::explain_error(
            "index out of bounds: the len is 3 but the index is 5",
            Language::Rust,
        );

        assert_eq!(explanation.category, ErrorCategory::RuntimeError);
        assert!(explanation.suggestion.contains(".get()"));
    }

    #[test]
    fn test_rust_unwrap_panic() {
        let explanation = FeedbackGenerator::explain_error(
            "called `Option::unwrap()` on a `None` value",
            Language::Rust,
        );

        assert_eq!(explanation.category, ErrorCategory::RuntimeError);
    }

    #[test]
    fn test_python_indentation() {
        let explanation = FeedbackGenerator::explain_error(
            "IndentationError: unexpected indent",
            Language::Python,
        );

        assert_eq!(explanation.category, ErrorCategory::SyntaxError);
        assert!(explanation.suggestion.contains("4 spaces"));
    }

    #[test]
    fn test_python_name_error() {
        let explanation = FeedbackGenerator::explain_error(
            "NameError: name 'foo' is not defined",
            Language::Python,
        );

        assert_eq!(explanation.category, ErrorCategory::NotFound);
    }

    #[test]
    fn test_python_type_error() {
        let explanation = FeedbackGenerator::explain_error(
            "TypeError: unsupported operand type(s) for +: 'int' and 'str'",
            Language::Python,
        );

        assert_eq!(explanation.category, ErrorCategory::TypeMismatch);
    }

    #[test]
    fn test_python_index_error() {
        let explanation = FeedbackGenerator::explain_error(
            "IndexError: list index out of range",
            Language::Python,
        );

        assert_eq!(explanation.category, ErrorCategory::RuntimeError);
    }

    #[test]
    fn test_python_key_error() {
        let explanation =
            FeedbackGenerator::explain_error("KeyError: 'missing_key'", Language::Python);

        assert_eq!(explanation.category, ErrorCategory::RuntimeError);
        assert!(explanation.suggestion.contains(".get()"));
    }

    #[test]
    fn test_python_zero_division() {
        let explanation = FeedbackGenerator::explain_error(
            "ZeroDivisionError: division by zero",
            Language::Python,
        );

        assert_eq!(explanation.category, ErrorCategory::RuntimeError);
    }

    #[test]
    fn test_generic_error() {
        let explanation = FeedbackGenerator::explain_error("some unknown error", Language::Rust);

        assert_eq!(explanation.category, ErrorCategory::Unknown);
    }

    #[test]
    fn test_compare_outputs_match() {
        let result = FeedbackGenerator::compare_outputs("hello world", "hello world");

        assert!(result.matches);
        assert_eq!(result.difference, DifferenceType::None);
    }

    #[test]
    fn test_compare_outputs_whitespace() {
        let result = FeedbackGenerator::compare_outputs("hello world", "hello  world");

        assert!(!result.matches);
        assert_eq!(result.difference, DifferenceType::Whitespace);
        assert!(result.hint.unwrap().contains("spacing"));
    }

    #[test]
    fn test_compare_outputs_case() {
        let result = FeedbackGenerator::compare_outputs("Hello World", "hello world");

        assert!(!result.matches);
        assert_eq!(result.difference, DifferenceType::Case);
        assert!(result.hint.unwrap().contains("capitalization"));
    }

    #[test]
    fn test_compare_outputs_line_count() {
        let result = FeedbackGenerator::compare_outputs("line1\nline2", "line1");

        assert!(!result.matches);
        assert_eq!(result.difference, DifferenceType::LineCount);
        assert!(result.hint.unwrap().contains("2 lines"));
    }

    #[test]
    fn test_compare_outputs_content() {
        let result = FeedbackGenerator::compare_outputs("hello", "world");

        assert!(!result.matches);
        assert_eq!(result.difference, DifferenceType::Content);
    }

    #[test]
    fn test_compare_outputs_trim() {
        let result = FeedbackGenerator::compare_outputs("  hello  ", "hello");

        assert!(result.matches);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_compare_identical_outputs_always_match(s in "[a-zA-Z0-9 ]{1,100}") {
            let result = FeedbackGenerator::compare_outputs(&s, &s);
            prop_assert!(result.matches);
        }

        #[test]
        fn test_explain_error_never_panics(error in "[a-zA-Z0-9 :()]{0,200}") {
            // Should not panic for any input
            let _ = FeedbackGenerator::explain_error(&error, Language::Rust);
            let _ = FeedbackGenerator::explain_error(&error, Language::Python);
        }
    }
}
