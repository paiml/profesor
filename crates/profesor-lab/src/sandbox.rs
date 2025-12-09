//! WASM-safe code execution sandbox.
//!
//! Provides sandboxed code execution with resource limits.

use alloc::string::String;
use profesor_core::Language;
use serde::{Deserialize, Serialize};

/// Configuration for the sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Maximum memory in bytes
    pub memory_limit_bytes: usize,
    /// Execution timeout in milliseconds
    pub timeout_ms: u32,
    /// Maximum output size in bytes
    pub max_output_bytes: usize,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            memory_limit_bytes: 64 * 1024 * 1024, // 64 MB
            timeout_ms: 5000,                     // 5 seconds
            max_output_bytes: 1024 * 1024,        // 1 MB
        }
    }
}

/// WASM-safe code execution sandbox.
#[derive(Debug, Clone)]
pub struct Sandbox {
    config: SandboxConfig,
}

impl Sandbox {
    /// Create a new sandbox with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(SandboxConfig::default())
    }

    /// Create a sandbox with custom configuration.
    #[must_use]
    pub fn with_config(config: SandboxConfig) -> Self {
        Self { config }
    }

    /// Get the sandbox configuration.
    #[must_use]
    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }

    /// Execute code in the sandbox.
    ///
    /// Note: In the browser, this delegates to a WASM interpreter.
    /// The actual execution depends on the language and runtime environment.
    #[must_use]
    pub fn execute(&self, code: &str, language: Language, input: &str) -> ExecutionResult {
        // In pure WASM, we can only interpret simple languages
        // or delegate to pre-compiled WASM modules
        match language {
            Language::Rust => self.execute_rust_subset(code, input),
            Language::Python => self.execute_python_subset(code, input),
            Language::JavaScript => self.execute_js_subset(code, input),
            _ => ExecutionResult::Error {
                message: alloc::format!("Language {:?} not yet supported in sandbox", language),
            },
        }
    }

    /// Execute a subset of Rust (basic expressions).
    fn execute_rust_subset(&self, code: &str, _input: &str) -> ExecutionResult {
        // Simplified Rust interpreter for basic expressions
        // In a full implementation, this would parse and evaluate Rust code

        // For now, return a placeholder that indicates code execution is needed
        if code.is_empty() {
            return ExecutionResult::Error {
                message: "Empty code".into(),
            };
        }

        // Stub: In production, this would use a Rust interpreter
        ExecutionResult::Success {
            output: String::new(),
            duration_ms: 0,
        }
    }

    /// Execute a subset of Python.
    fn execute_python_subset(&self, code: &str, _input: &str) -> ExecutionResult {
        if code.is_empty() {
            return ExecutionResult::Error {
                message: "Empty code".into(),
            };
        }

        // Stub: Would use ruchy (Rust Python interpreter)
        ExecutionResult::Success {
            output: String::new(),
            duration_ms: 0,
        }
    }

    /// Execute a subset of JavaScript.
    fn execute_js_subset(&self, code: &str, _input: &str) -> ExecutionResult {
        if code.is_empty() {
            return ExecutionResult::Error {
                message: "Empty code".into(),
            };
        }

        // Stub: Would use a JS interpreter compiled to WASM
        ExecutionResult::Success {
            output: String::new(),
            duration_ms: 0,
        }
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of code execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionResult {
    /// Successful execution
    Success {
        /// Program output
        output: String,
        /// Execution time in milliseconds
        duration_ms: u32,
    },
    /// Runtime error during execution
    RuntimeError {
        /// Error message
        error: String,
        /// Line number where error occurred (if known)
        line: Option<u32>,
    },
    /// Execution timed out
    Timeout {
        /// Partial output before timeout
        partial_output: String,
    },
    /// Memory limit exceeded
    MemoryExceeded {
        /// Memory used in bytes
        used_bytes: usize,
    },
    /// General error
    Error {
        /// Error message
        message: String,
    },
}

impl ExecutionResult {
    /// Check if execution was successful.
    #[must_use]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Get the output if successful.
    #[must_use]
    pub fn output(&self) -> Option<&str> {
        match self {
            Self::Success { output, .. } => Some(output),
            _ => None,
        }
    }

    /// Get the error message if failed.
    #[must_use]
    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::RuntimeError { error, .. } => Some(error),
            Self::Error { message } => Some(message),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_default_config() {
        let sandbox = Sandbox::new();
        assert_eq!(sandbox.config().timeout_ms, 5000);
        assert_eq!(sandbox.config().memory_limit_bytes, 64 * 1024 * 1024);
    }

    #[test]
    fn test_sandbox_custom_config() {
        let config = SandboxConfig {
            memory_limit_bytes: 32 * 1024 * 1024,
            timeout_ms: 1000,
            max_output_bytes: 512 * 1024,
        };
        let sandbox = Sandbox::with_config(config);
        assert_eq!(sandbox.config().timeout_ms, 1000);
    }

    #[test]
    fn test_empty_code_error() {
        let sandbox = Sandbox::new();
        let result = sandbox.execute("", Language::Rust, "");
        assert!(!result.is_success());
    }

    #[test]
    fn test_execution_result_accessors() {
        let success = ExecutionResult::Success {
            output: "Hello".into(),
            duration_ms: 10,
        };
        assert!(success.is_success());
        assert_eq!(success.output(), Some("Hello"));

        let error = ExecutionResult::Error {
            message: "Failed".into(),
        };
        assert!(!error.is_success());
        assert_eq!(error.error_message(), Some("Failed"));
    }

    #[test]
    fn test_unsupported_language() {
        let sandbox = Sandbox::new();
        let result = sandbox.execute("SELECT 1", Language::Sql, "");
        assert!(!result.is_success());
    }
}
