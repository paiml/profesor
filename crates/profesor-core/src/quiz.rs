//! Quiz types and structures.
//!
//! Defines quizzes, questions, and scoring for assessments.

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::ids::{QuestionId, QuizId};
use crate::lab::Language;

/// A quiz with multiple questions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quiz {
    /// Unique identifier
    pub id: QuizId,
    /// Human-readable title
    pub title: String,
    /// List of questions
    pub questions: Vec<Question>,
    /// Optional time limit in seconds
    pub time_limit_secs: Option<u32>,
    /// Minimum score to pass (0.0 - 1.0)
    pub passing_score: f32,
    /// Whether to shuffle question order
    pub shuffle: bool,
    /// Maximum number of attempts allowed
    pub max_attempts: Option<u32>,
}

impl Quiz {
    /// Create a new quiz.
    #[must_use]
    pub fn new(id: impl Into<QuizId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            questions: Vec::new(),
            time_limit_secs: None,
            passing_score: 0.7,
            shuffle: false,
            max_attempts: None,
        }
    }

    /// Add a question to the quiz.
    #[must_use]
    pub fn with_question(mut self, question: Question) -> Self {
        self.questions.push(question);
        self
    }

    /// Set the time limit in seconds.
    #[must_use]
    pub fn with_time_limit(mut self, seconds: u32) -> Self {
        self.time_limit_secs = Some(seconds);
        self
    }

    /// Set the passing score threshold.
    #[must_use]
    pub fn with_passing_score(mut self, score: f32) -> Self {
        self.passing_score = score.clamp(0.0, 1.0);
        self
    }

    /// Enable question shuffling.
    #[must_use]
    pub fn with_shuffle(mut self, shuffle: bool) -> Self {
        self.shuffle = shuffle;
        self
    }

    /// Set the maximum attempts.
    #[must_use]
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = Some(attempts);
        self
    }

    /// Get the total possible points for the quiz.
    #[must_use]
    pub fn total_points(&self) -> u32 {
        self.questions.iter().map(|q| q.points()).sum()
    }

    /// Get the number of questions.
    #[must_use]
    pub fn question_count(&self) -> usize {
        self.questions.len()
    }
}

/// Question types supported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Question {
    /// Multiple choice (single answer)
    MultipleChoice {
        /// Unique ID
        id: QuestionId,
        /// The question prompt
        prompt: String,
        /// Available options
        options: Vec<String>,
        /// Index of the correct option (0-based)
        correct: usize,
        /// Explanation shown after answering
        explanation: String,
        /// Points for correct answer
        points: u32,
    },
    /// Multiple select (multiple correct answers)
    MultipleSelect {
        /// Unique ID
        id: QuestionId,
        /// The question prompt
        prompt: String,
        /// Available options
        options: Vec<String>,
        /// Indices of correct options
        correct: Vec<usize>,
        /// Explanation shown after answering
        explanation: String,
        /// Points for correct answer
        points: u32,
    },
    /// Code completion (fill in the blank)
    CodeCompletion {
        /// Unique ID
        id: QuestionId,
        /// The question prompt
        prompt: String,
        /// Code template with blanks
        code_template: String,
        /// Blank definitions
        blanks: Vec<Blank>,
        /// Test cases to validate
        test_cases: Vec<TestCase>,
        /// Points for correct answer
        points: u32,
    },
    /// Drag and drop ordering
    Ordering {
        /// Unique ID
        id: QuestionId,
        /// The question prompt
        prompt: String,
        /// Items to order
        items: Vec<String>,
        /// Correct order (indices)
        correct_order: Vec<usize>,
        /// Explanation shown after answering
        explanation: String,
        /// Points for correct answer
        points: u32,
    },
    /// Matching pairs
    Matching {
        /// Unique ID
        id: QuestionId,
        /// The question prompt
        prompt: String,
        /// Left side items
        left: Vec<String>,
        /// Right side items
        right: Vec<String>,
        /// Correct pairs as (left_idx, right_idx)
        correct_pairs: Vec<(usize, usize)>,
        /// Points for correct answer
        points: u32,
    },
    /// Free-form code (graded by test cases)
    FreeformCode {
        /// Unique ID
        id: QuestionId,
        /// The question prompt
        prompt: String,
        /// Programming language
        language: Language,
        /// Starter code template
        starter_code: String,
        /// Visible test cases
        test_cases: Vec<TestCase>,
        /// Hidden test cases (for grading)
        hidden_test_cases: Vec<TestCase>,
        /// Points for correct answer
        points: u32,
    },
}

impl Question {
    /// Get the points value for this question.
    #[must_use]
    pub fn points(&self) -> u32 {
        match self {
            Self::MultipleChoice { points, .. }
            | Self::MultipleSelect { points, .. }
            | Self::CodeCompletion { points, .. }
            | Self::Ordering { points, .. }
            | Self::Matching { points, .. }
            | Self::FreeformCode { points, .. } => *points,
        }
    }

    /// Get the question ID.
    #[must_use]
    pub fn id(&self) -> &QuestionId {
        match self {
            Self::MultipleChoice { id, .. }
            | Self::MultipleSelect { id, .. }
            | Self::CodeCompletion { id, .. }
            | Self::Ordering { id, .. }
            | Self::Matching { id, .. }
            | Self::FreeformCode { id, .. } => id,
        }
    }

    /// Get the question prompt.
    #[must_use]
    pub fn prompt(&self) -> &str {
        match self {
            Self::MultipleChoice { prompt, .. }
            | Self::MultipleSelect { prompt, .. }
            | Self::CodeCompletion { prompt, .. }
            | Self::Ordering { prompt, .. }
            | Self::Matching { prompt, .. }
            | Self::FreeformCode { prompt, .. } => prompt,
        }
    }
}

/// A blank in code completion questions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Blank {
    /// Identifier for this blank (e.g., "blank1")
    pub id: String,
    /// List of acceptable answers
    pub acceptable_answers: Vec<String>,
    /// Optional hint
    pub hint: Option<String>,
}

impl Blank {
    /// Create a new blank.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            acceptable_answers: Vec::new(),
            hint: None,
        }
    }

    /// Add an acceptable answer.
    #[must_use]
    pub fn with_answer(mut self, answer: impl Into<String>) -> Self {
        self.acceptable_answers.push(answer.into());
        self
    }

    /// Set the hint.
    #[must_use]
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Check if an answer is acceptable.
    #[must_use]
    pub fn is_acceptable(&self, answer: &str) -> bool {
        self.acceptable_answers.iter().any(|a| a == answer)
    }
}

/// Test case for code validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestCase {
    /// Name of the test case
    pub name: String,
    /// Input to the code
    pub input: String,
    /// Expected output
    pub expected_output: String,
    /// Timeout in milliseconds
    pub timeout_ms: u32,
}

impl TestCase {
    /// Create a new test case.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input: String::new(),
            expected_output: String::new(),
            timeout_ms: 5000,
        }
    }

    /// Set the input.
    #[must_use]
    pub fn with_input(mut self, input: impl Into<String>) -> Self {
        self.input = input.into();
        self
    }

    /// Set the expected output.
    #[must_use]
    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected_output = expected.into();
        self
    }

    /// Set the timeout.
    #[must_use]
    pub fn with_timeout_ms(mut self, timeout: u32) -> Self {
        self.timeout_ms = timeout;
        self
    }
}

/// An answer submitted by a learner.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Answer {
    /// Single choice selection
    Choice(usize),
    /// Multiple choices selected
    MultiChoice(Vec<usize>),
    /// Ordering of items
    Order(Vec<usize>),
    /// Matching pairs
    Pairs(Vec<(usize, usize)>),
    /// Code answer
    Code(String),
    /// Filled blanks (blank_id -> answer)
    Blanks(Vec<(String, String)>),
}

/// Feedback for an answered question.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Feedback {
    /// Whether the answer was correct
    pub correct: bool,
    /// Explanation or feedback text
    pub explanation: String,
    /// Points earned
    pub points_earned: u32,
}

impl Feedback {
    /// Create feedback for a correct answer.
    #[must_use]
    pub fn correct(explanation: impl Into<String>, points: u32) -> Self {
        Self {
            correct: true,
            explanation: explanation.into(),
            points_earned: points,
        }
    }

    /// Create feedback for an incorrect answer.
    #[must_use]
    pub fn incorrect(explanation: impl Into<String>) -> Self {
        Self {
            correct: false,
            explanation: explanation.into(),
            points_earned: 0,
        }
    }
}

/// Score for a completed quiz.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Score {
    /// Points earned
    pub points_earned: u32,
    /// Total possible points
    pub points_possible: u32,
    /// Percentage score (0.0 - 1.0)
    pub percentage: f32,
    /// Number of correct answers
    pub correct_count: usize,
    /// Total number of questions
    pub total_questions: usize,
    /// Whether the quiz was passed
    pub passed: bool,
}

impl Score {
    /// Calculate a score from points.
    #[must_use]
    pub fn calculate(
        points_earned: u32,
        points_possible: u32,
        passing_threshold: f32,
        correct_count: usize,
        total_questions: usize,
    ) -> Self {
        let percentage = if points_possible > 0 {
            points_earned as f32 / points_possible as f32
        } else {
            0.0
        };

        Self {
            points_earned,
            points_possible,
            percentage,
            correct_count,
            total_questions,
            passed: percentage >= passing_threshold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz_creation() {
        let quiz = Quiz::new("quiz-01", "Module 1 Quiz")
            .with_passing_score(0.8)
            .with_time_limit(1800)
            .with_max_attempts(3);

        assert_eq!(quiz.id.as_str(), "quiz-01");
        assert_eq!(quiz.passing_score, 0.8);
        assert_eq!(quiz.time_limit_secs, Some(1800));
        assert_eq!(quiz.max_attempts, Some(3));
    }

    #[test]
    fn test_quiz_total_points() {
        let quiz = Quiz::new("test", "Test")
            .with_question(Question::MultipleChoice {
                id: "q1".into(),
                prompt: "Q1".into(),
                options: vec!["A".into(), "B".into()],
                correct: 0,
                explanation: "".into(),
                points: 10,
            })
            .with_question(Question::MultipleChoice {
                id: "q2".into(),
                prompt: "Q2".into(),
                options: vec!["A".into(), "B".into()],
                correct: 1,
                explanation: "".into(),
                points: 20,
            });

        assert_eq!(quiz.total_points(), 30);
        assert_eq!(quiz.question_count(), 2);
    }

    #[test]
    fn test_question_points_accessor() {
        let q = Question::MultipleChoice {
            id: "q1".into(),
            prompt: "Test".into(),
            options: vec![],
            correct: 0,
            explanation: "".into(),
            points: 15,
        };
        assert_eq!(q.points(), 15);
    }

    #[test]
    fn test_blank_acceptable_answers() {
        let blank = Blank::new("blank1")
            .with_answer("a + b")
            .with_answer("return a + b;");

        assert!(blank.is_acceptable("a + b"));
        assert!(blank.is_acceptable("return a + b;"));
        assert!(!blank.is_acceptable("a - b"));
    }

    #[test]
    fn test_score_calculation() {
        let score = Score::calculate(80, 100, 0.7, 8, 10);

        assert_eq!(score.points_earned, 80);
        assert_eq!(score.points_possible, 100);
        assert!((score.percentage - 0.8).abs() < f32::EPSILON);
        assert!(score.passed);
    }

    #[test]
    fn test_score_failing() {
        let score = Score::calculate(50, 100, 0.7, 5, 10);
        assert!(!score.passed);
    }

    #[test]
    fn test_feedback_correct() {
        let fb = Feedback::correct("Great job!", 10);
        assert!(fb.correct);
        assert_eq!(fb.points_earned, 10);
    }

    #[test]
    fn test_feedback_incorrect() {
        let fb = Feedback::incorrect("Try again");
        assert!(!fb.correct);
        assert_eq!(fb.points_earned, 0);
    }

    #[test]
    fn test_passing_score_clamped() {
        let quiz = Quiz::new("test", "Test").with_passing_score(1.5);
        assert_eq!(quiz.passing_score, 1.0);

        let quiz2 = Quiz::new("test", "Test").with_passing_score(-0.5);
        assert_eq!(quiz2.passing_score, 0.0);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_quiz_total_points_matches_sum(
            points in prop::collection::vec(1u32..100, 1..20)
        ) {
            let mut quiz = Quiz::new("test", "Test");
            let expected: u32 = points.iter().sum();

            for (i, &p) in points.iter().enumerate() {
                quiz = quiz.with_question(Question::MultipleChoice {
                    id: format!("q{}", i).into(),
                    prompt: "Q".into(),
                    options: vec!["A".into()],
                    correct: 0,
                    explanation: "".into(),
                    points: p,
                });
            }

            prop_assert_eq!(quiz.total_points(), expected);
        }

        #[test]
        fn test_score_percentage_in_range(
            earned in 0u32..1000,
            possible in 1u32..1000
        ) {
            let score = Score::calculate(
                earned.min(possible),
                possible,
                0.5,
                0,
                0
            );
            prop_assert!(score.percentage >= 0.0);
            prop_assert!(score.percentage <= 1.0);
        }

        #[test]
        fn test_passing_score_always_clamped(score in -10.0f32..10.0) {
            let quiz = Quiz::new("test", "Test").with_passing_score(score);
            prop_assert!(quiz.passing_score >= 0.0);
            prop_assert!(quiz.passing_score <= 1.0);
        }
    }
}
