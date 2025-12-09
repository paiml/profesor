//! Strongly-typed identifiers for all entities.
//!
//! Each ID type is a newtype wrapper around a string, providing type safety
//! and preventing accidental mixing of different ID types.

use alloc::string::String;
use core::fmt;
use serde::{Deserialize, Serialize};

/// Macro to generate ID types with common implementations
macro_rules! define_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            /// Create a new ID from a string
            #[must_use]
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            /// Get the inner string value
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self::new(s)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self::new(s)
            }
        }
    };
}

define_id!(CourseId, "Unique identifier for a course");
define_id!(ModuleId, "Unique identifier for a module within a course");
define_id!(LessonId, "Unique identifier for a lesson within a module");
define_id!(QuizId, "Unique identifier for a quiz");
define_id!(QuestionId, "Unique identifier for a question within a quiz");
define_id!(LabId, "Unique identifier for a lab");
define_id!(SimulationId, "Unique identifier for a simulation");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_id_creation() {
        let id = CourseId::new("rust-fundamentals");
        assert_eq!(id.as_str(), "rust-fundamentals");
    }

    #[test]
    fn test_id_from_str() {
        let id: CourseId = "test-course".into();
        assert_eq!(id.as_str(), "test-course");
    }

    #[test]
    fn test_id_equality() {
        let id1 = QuizId::new("quiz-01");
        let id2 = QuizId::new("quiz-01");
        let id3 = QuizId::new("quiz-02");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_id_display() {
        let id = ModuleId::new("mod-ownership");
        assert_eq!(format!("{}", id), "mod-ownership");
    }

    #[test]
    fn test_id_clone() {
        let id1 = LessonId::new("lesson-01");
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_course_id_roundtrip(s in "[a-z][a-z0-9-]{0,50}") {
            let id = CourseId::new(&s);
            prop_assert_eq!(id.as_str(), s.as_str());
        }

        #[test]
        fn test_quiz_id_equality_reflexive(s in "[a-z][a-z0-9-]{0,50}") {
            let id = QuizId::new(&s);
            prop_assert_eq!(&id, &id);
        }

        #[test]
        fn test_module_id_clone_equality(s in "[a-z][a-z0-9-]{0,50}") {
            let id1 = ModuleId::new(&s);
            let id2 = id1.clone();
            prop_assert_eq!(id1, id2);
        }
    }
}
