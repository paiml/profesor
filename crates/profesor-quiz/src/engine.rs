//! Quiz state machine.
//!
//! Implements the quiz flow with immediate feedback (Jidoka principle).

use alloc::vec::Vec;
use profesor_core::{Answer, Feedback, Question, Quiz, Score};

use crate::grader::Grader;

/// Quiz state machine.
#[derive(Debug, Clone, PartialEq)]
pub enum QuizState {
    /// Quiz has not been started
    NotStarted,
    /// Quiz is in progress
    InProgress {
        /// Index of current question (0-based)
        current_question: usize,
        /// Answers submitted so far
        answers: Vec<Option<Answer>>,
        /// Feedback for each answered question
        feedback: Vec<Option<Feedback>>,
    },
    /// Quiz is being reviewed
    Reviewing {
        /// All answers
        answers: Vec<Answer>,
        /// Feedback for all questions
        feedback: Vec<Feedback>,
        /// Final score
        score: Score,
    },
    /// Quiz has been completed
    Completed {
        /// Final score
        score: Score,
        /// Attempt number
        attempt_number: u32,
    },
}

/// Quiz engine that manages quiz state and progression.
#[derive(Debug, Clone)]
pub struct QuizEngine {
    quiz: Quiz,
    state: QuizState,
    attempt_count: u32,
}

impl QuizEngine {
    /// Create a new quiz engine.
    #[must_use]
    pub fn new(quiz: Quiz) -> Self {
        Self {
            quiz,
            state: QuizState::NotStarted,
            attempt_count: 0,
        }
    }

    /// Get the current state.
    #[must_use]
    pub fn state(&self) -> &QuizState {
        &self.state
    }

    /// Get the quiz.
    #[must_use]
    pub fn quiz(&self) -> &Quiz {
        &self.quiz
    }

    /// Get the number of attempts made.
    #[must_use]
    pub fn attempt_count(&self) -> u32 {
        self.attempt_count
    }

    /// Check if another attempt is allowed.
    #[must_use]
    pub fn can_attempt(&self) -> bool {
        match self.quiz.max_attempts {
            Some(max) => self.attempt_count < max,
            None => true,
        }
    }

    /// Start a new quiz attempt.
    ///
    /// Returns an error if maximum attempts have been reached.
    pub fn start(&mut self) -> Result<&Question, QuizError> {
        if !self.can_attempt() {
            return Err(QuizError::MaxAttemptsReached);
        }

        if self.quiz.questions.is_empty() {
            return Err(QuizError::NoQuestions);
        }

        self.attempt_count += 1;
        let question_count = self.quiz.questions.len();

        self.state = QuizState::InProgress {
            current_question: 0,
            answers: alloc::vec![None; question_count],
            feedback: alloc::vec![None; question_count],
        };

        self.current_question()
    }

    /// Get the current question.
    pub fn current_question(&self) -> Result<&Question, QuizError> {
        match &self.state {
            QuizState::InProgress {
                current_question, ..
            } => self
                .quiz
                .questions
                .get(*current_question)
                .ok_or(QuizError::InvalidState),
            _ => Err(QuizError::InvalidState),
        }
    }

    /// Submit an answer for the current question.
    ///
    /// Returns immediate feedback (Jidoka principle).
    pub fn submit_answer(&mut self, answer: Answer) -> Result<Feedback, QuizError> {
        let (current_idx, question) = match &self.state {
            QuizState::InProgress {
                current_question, ..
            } => {
                let q = self
                    .quiz
                    .questions
                    .get(*current_question)
                    .ok_or(QuizError::InvalidState)?;
                (*current_question, q.clone())
            }
            _ => return Err(QuizError::InvalidState),
        };

        // Grade the answer immediately
        let feedback = Grader::grade_answer(&question, &answer);

        // Update state
        if let QuizState::InProgress {
            answers,
            feedback: fb,
            ..
        } = &mut self.state
        {
            answers[current_idx] = Some(answer);
            fb[current_idx] = Some(feedback.clone());
        }

        Ok(feedback)
    }

    /// Move to the next question.
    ///
    /// Returns the next question or an error if at the end.
    pub fn next_question(&mut self) -> Result<&Question, QuizError> {
        match &mut self.state {
            QuizState::InProgress {
                current_question,
                answers,
                ..
            } => {
                // Check if current question was answered
                if answers.get(*current_question).map_or(true, |a| a.is_none()) {
                    return Err(QuizError::QuestionNotAnswered);
                }

                let next_idx = *current_question + 1;
                if next_idx >= self.quiz.questions.len() {
                    return Err(QuizError::NoMoreQuestions);
                }

                *current_question = next_idx;
                self.quiz
                    .questions
                    .get(next_idx)
                    .ok_or(QuizError::InvalidState)
            }
            _ => Err(QuizError::InvalidState),
        }
    }

    /// Move to the previous question.
    pub fn previous_question(&mut self) -> Result<&Question, QuizError> {
        match &mut self.state {
            QuizState::InProgress {
                current_question, ..
            } => {
                if *current_question == 0 {
                    return Err(QuizError::NoPreviousQuestion);
                }

                *current_question -= 1;
                self.quiz
                    .questions
                    .get(*current_question)
                    .ok_or(QuizError::InvalidState)
            }
            _ => Err(QuizError::InvalidState),
        }
    }

    /// Finish the quiz and calculate the final score.
    pub fn finish(&mut self) -> Result<Score, QuizError> {
        let (_answers, feedback) = match &self.state {
            QuizState::InProgress {
                answers, feedback, ..
            } => {
                // Collect all answers, using empty for unanswered
                let collected_answers: Vec<Answer> = answers
                    .iter()
                    .map(|a| a.clone().unwrap_or(Answer::Choice(usize::MAX)))
                    .collect();

                let collected_feedback: Vec<Feedback> = feedback
                    .iter()
                    .map(|f| {
                        f.clone()
                            .unwrap_or_else(|| Feedback::incorrect("Not answered"))
                    })
                    .collect();

                (collected_answers, collected_feedback)
            }
            _ => return Err(QuizError::InvalidState),
        };

        let score = Grader::calculate_score(&self.quiz, &feedback);

        self.state = QuizState::Completed {
            score: score.clone(),
            attempt_number: self.attempt_count,
        };

        Ok(score)
    }

    /// Get the progress through the quiz (0.0 - 1.0).
    #[must_use]
    pub fn progress(&self) -> f32 {
        match &self.state {
            QuizState::NotStarted => 0.0,
            QuizState::InProgress { answers, .. } => {
                let answered = answers.iter().filter(|a| a.is_some()).count();
                if self.quiz.questions.is_empty() {
                    0.0
                } else {
                    answered as f32 / self.quiz.questions.len() as f32
                }
            }
            QuizState::Reviewing { .. } | QuizState::Completed { .. } => 1.0,
        }
    }
}

/// Errors that can occur during quiz operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuizError {
    /// Maximum number of attempts has been reached
    MaxAttemptsReached,
    /// No questions in the quiz
    NoQuestions,
    /// Operation not valid in current state
    InvalidState,
    /// Current question has not been answered
    QuestionNotAnswered,
    /// No more questions available
    NoMoreQuestions,
    /// No previous question (already at first)
    NoPreviousQuestion,
}

impl core::fmt::Display for QuizError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MaxAttemptsReached => write!(f, "Maximum attempts reached"),
            Self::NoQuestions => write!(f, "Quiz has no questions"),
            Self::InvalidState => write!(f, "Invalid state for this operation"),
            Self::QuestionNotAnswered => write!(f, "Current question not answered"),
            Self::NoMoreQuestions => write!(f, "No more questions"),
            Self::NoPreviousQuestion => write!(f, "No previous question"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use profesor_core::{Question, QuestionId, Quiz};

    fn create_test_quiz() -> Quiz {
        Quiz::new("test-quiz", "Test Quiz")
            .with_question(Question::MultipleChoice {
                id: QuestionId::new("q1"),
                prompt: "What is 2+2?".into(),
                options: alloc::vec!["3".into(), "4".into(), "5".into()],
                correct: 1,
                explanation: "2+2=4".into(),
                points: 10,
            })
            .with_question(Question::MultipleChoice {
                id: QuestionId::new("q2"),
                prompt: "What is 3+3?".into(),
                options: alloc::vec!["5".into(), "6".into(), "7".into()],
                correct: 1,
                explanation: "3+3=6".into(),
                points: 10,
            })
    }

    #[test]
    fn test_quiz_engine_creation() {
        let quiz = create_test_quiz();
        let engine = QuizEngine::new(quiz);

        assert_eq!(engine.state(), &QuizState::NotStarted);
        assert_eq!(engine.attempt_count(), 0);
    }

    #[test]
    fn test_start_quiz() {
        let quiz = create_test_quiz();
        let mut engine = QuizEngine::new(quiz);

        // Start and extract prompt immediately
        let prompt = engine.start().expect("Should start").prompt().to_string();

        assert!(matches!(engine.state(), QuizState::InProgress { .. }));
        assert_eq!(engine.attempt_count(), 1);
        assert!(prompt.contains("2+2"));
    }

    #[test]
    fn test_submit_correct_answer() {
        let quiz = create_test_quiz();
        let mut engine = QuizEngine::new(quiz);

        engine.start().expect("Should start");
        let feedback = engine
            .submit_answer(Answer::Choice(1))
            .expect("Should submit");

        assert!(feedback.correct);
        assert_eq!(feedback.points_earned, 10);
    }

    #[test]
    fn test_submit_incorrect_answer() {
        let quiz = create_test_quiz();
        let mut engine = QuizEngine::new(quiz);

        engine.start().expect("Should start");
        let feedback = engine
            .submit_answer(Answer::Choice(0))
            .expect("Should submit");

        assert!(!feedback.correct);
        assert_eq!(feedback.points_earned, 0);
    }

    #[test]
    fn test_next_question() {
        let quiz = create_test_quiz();
        let mut engine = QuizEngine::new(quiz);

        engine.start().expect("Should start");
        engine
            .submit_answer(Answer::Choice(1))
            .expect("Should submit");
        let next = engine.next_question().expect("Should advance");

        assert!(next.prompt().contains("3+3"));
    }

    #[test]
    fn test_finish_quiz() {
        let quiz = create_test_quiz();
        let mut engine = QuizEngine::new(quiz);

        engine.start().expect("Should start");
        engine
            .submit_answer(Answer::Choice(1))
            .expect("Should submit");
        engine.next_question().expect("Should advance");
        engine
            .submit_answer(Answer::Choice(1))
            .expect("Should submit");

        let score = engine.finish().expect("Should finish");
        assert!(score.passed);
        assert_eq!(score.correct_count, 2);
    }

    #[test]
    fn test_max_attempts() {
        let quiz = Quiz::new("test", "Test")
            .with_max_attempts(1)
            .with_question(Question::MultipleChoice {
                id: "q1".into(),
                prompt: "Q".into(),
                options: alloc::vec!["A".into()],
                correct: 0,
                explanation: "".into(),
                points: 10,
            });

        let mut engine = QuizEngine::new(quiz);

        engine.start().expect("First attempt should work");
        engine.submit_answer(Answer::Choice(0)).expect("Submit");
        engine.finish().expect("Finish");

        let result = engine.start();
        assert_eq!(result, Err(QuizError::MaxAttemptsReached));
    }

    #[test]
    fn test_progress() {
        let quiz = create_test_quiz();
        let mut engine = QuizEngine::new(quiz);

        assert_eq!(engine.progress(), 0.0);

        engine.start().expect("Start");
        assert_eq!(engine.progress(), 0.0);

        engine.submit_answer(Answer::Choice(1)).expect("Submit");
        assert!((engine.progress() - 0.5).abs() < f32::EPSILON);
    }
}
