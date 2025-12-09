//! Quiz grading logic.
//!
//! Provides automatic grading for all question types.

use profesor_core::{Answer, Feedback, Question, Quiz, Score};

/// Auto-grader for quizzes.
pub struct Grader;

impl Grader {
    /// Grade a single answer against a question.
    #[must_use]
    pub fn grade_answer(question: &Question, answer: &Answer) -> Feedback {
        let (correct, explanation, points) = match (question, answer) {
            (
                Question::MultipleChoice {
                    correct,
                    explanation,
                    points,
                    ..
                },
                Answer::Choice(idx),
            ) => (*correct == *idx, explanation.clone(), *points),

            (
                Question::MultipleSelect {
                    correct,
                    explanation,
                    points,
                    ..
                },
                Answer::MultiChoice(indices),
            ) => {
                let mut sorted_correct = correct.clone();
                let mut sorted_answer = indices.clone();
                sorted_correct.sort_unstable();
                sorted_answer.sort_unstable();
                (
                    sorted_correct == sorted_answer,
                    explanation.clone(),
                    *points,
                )
            }

            (
                Question::Ordering {
                    correct_order,
                    explanation,
                    points,
                    ..
                },
                Answer::Order(order),
            ) => (correct_order == order, explanation.clone(), *points),

            (
                Question::Matching {
                    correct_pairs,
                    points,
                    ..
                },
                Answer::Pairs(pairs),
            ) => {
                let mut sorted_correct = correct_pairs.to_vec();
                let mut sorted_answer = pairs.to_vec();
                sorted_correct.sort_unstable();
                sorted_answer.sort_unstable();
                (
                    sorted_correct == sorted_answer,
                    alloc::string::String::new(),
                    *points,
                )
            }

            (Question::CodeCompletion { blanks, points, .. }, Answer::Blanks(filled)) => {
                let all_correct = blanks.iter().all(|blank| {
                    filled
                        .iter()
                        .find(|(id, _)| id == &blank.id)
                        .is_some_and(|(_, ans)| blank.is_acceptable(ans))
                });
                (all_correct, alloc::string::String::new(), *points)
            }

            // FreeformCode requires execution - return pending for now
            (Question::FreeformCode { points, .. }, Answer::Code(_)) => {
                // Code execution handled separately
                (false, "Code execution required".into(), *points)
            }

            // Type mismatch
            _ => (false, "Invalid answer type".into(), 0),
        };

        if correct {
            Feedback::correct(explanation, points)
        } else {
            Feedback::incorrect(explanation)
        }
    }

    /// Calculate the final score for a quiz.
    #[must_use]
    pub fn calculate_score(quiz: &Quiz, feedback: &[Feedback]) -> Score {
        let points_earned: u32 = feedback.iter().map(|f| f.points_earned).sum();
        let points_possible = quiz.total_points();
        let correct_count = feedback.iter().filter(|f| f.correct).count();
        let total_questions = quiz.question_count();

        Score::calculate(
            points_earned,
            points_possible,
            quiz.passing_score,
            correct_count,
            total_questions,
        )
    }

    /// Check if an answer matches the correct answer for a question.
    #[must_use]
    pub fn is_correct(question: &Question, answer: &Answer) -> bool {
        Self::grade_answer(question, answer).correct
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use profesor_core::{Blank, QuestionId};

    #[test]
    fn test_grade_multiple_choice_correct() {
        let question = Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "Test".into(),
            options: alloc::vec!["A".into(), "B".into(), "C".into()],
            correct: 1,
            explanation: "B is correct".into(),
            points: 10,
        };

        let feedback = Grader::grade_answer(&question, &Answer::Choice(1));
        assert!(feedback.correct);
        assert_eq!(feedback.points_earned, 10);
    }

    #[test]
    fn test_grade_multiple_choice_incorrect() {
        let question = Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "Test".into(),
            options: alloc::vec!["A".into(), "B".into()],
            correct: 0,
            explanation: "A is correct".into(),
            points: 10,
        };

        let feedback = Grader::grade_answer(&question, &Answer::Choice(1));
        assert!(!feedback.correct);
        assert_eq!(feedback.points_earned, 0);
    }

    #[test]
    fn test_grade_multiple_select() {
        let question = Question::MultipleSelect {
            id: QuestionId::new("q1"),
            prompt: "Select all".into(),
            options: alloc::vec!["A".into(), "B".into(), "C".into()],
            correct: alloc::vec![0, 2],
            explanation: "A and C".into(),
            points: 20,
        };

        // Correct (order doesn't matter)
        let feedback = Grader::grade_answer(&question, &Answer::MultiChoice(alloc::vec![2, 0]));
        assert!(feedback.correct);

        // Incorrect
        let feedback = Grader::grade_answer(&question, &Answer::MultiChoice(alloc::vec![0, 1]));
        assert!(!feedback.correct);
    }

    #[test]
    fn test_grade_ordering() {
        let question = Question::Ordering {
            id: QuestionId::new("q1"),
            prompt: "Order these".into(),
            items: alloc::vec!["First".into(), "Second".into(), "Third".into()],
            correct_order: alloc::vec![0, 1, 2],
            explanation: "Correct order".into(),
            points: 15,
        };

        let feedback = Grader::grade_answer(&question, &Answer::Order(alloc::vec![0, 1, 2]));
        assert!(feedback.correct);

        let feedback = Grader::grade_answer(&question, &Answer::Order(alloc::vec![2, 1, 0]));
        assert!(!feedback.correct);
    }

    #[test]
    fn test_grade_matching() {
        let question = Question::Matching {
            id: QuestionId::new("q1"),
            prompt: "Match".into(),
            left: alloc::vec!["A".into(), "B".into()],
            right: alloc::vec!["1".into(), "2".into()],
            correct_pairs: alloc::vec![(0, 1), (1, 0)],
            points: 20,
        };

        let feedback = Grader::grade_answer(&question, &Answer::Pairs(alloc::vec![(1, 0), (0, 1)]));
        assert!(feedback.correct);
    }

    #[test]
    fn test_grade_code_completion() {
        let question = Question::CodeCompletion {
            id: QuestionId::new("q1"),
            prompt: "Fill in".into(),
            code_template: "fn add(a: i32, b: i32) -> i32 { {{blank1}} }".into(),
            blanks: alloc::vec![Blank::new("blank1")
                .with_answer("a + b")
                .with_answer("return a + b;"),],
            test_cases: alloc::vec![],
            points: 25,
        };

        let feedback = Grader::grade_answer(
            &question,
            &Answer::Blanks(alloc::vec![("blank1".into(), "a + b".into())]),
        );
        assert!(feedback.correct);

        let feedback = Grader::grade_answer(
            &question,
            &Answer::Blanks(alloc::vec![("blank1".into(), "a - b".into())]),
        );
        assert!(!feedback.correct);
    }

    #[test]
    fn test_calculate_score_passing() {
        let quiz = Quiz::new("test", "Test")
            .with_passing_score(0.7)
            .with_question(Question::MultipleChoice {
                id: "q1".into(),
                prompt: "Q1".into(),
                options: alloc::vec!["A".into()],
                correct: 0,
                explanation: "".into(),
                points: 10,
            })
            .with_question(Question::MultipleChoice {
                id: "q2".into(),
                prompt: "Q2".into(),
                options: alloc::vec!["A".into()],
                correct: 0,
                explanation: "".into(),
                points: 10,
            });

        let feedback = alloc::vec![Feedback::correct("", 10), Feedback::correct("", 10),];

        let score = Grader::calculate_score(&quiz, &feedback);
        assert!(score.passed);
        assert_eq!(score.points_earned, 20);
        assert_eq!(score.correct_count, 2);
    }

    #[test]
    fn test_calculate_score_failing() {
        let quiz = Quiz::new("test", "Test")
            .with_passing_score(0.7)
            .with_question(Question::MultipleChoice {
                id: "q1".into(),
                prompt: "Q1".into(),
                options: alloc::vec!["A".into()],
                correct: 0,
                explanation: "".into(),
                points: 10,
            })
            .with_question(Question::MultipleChoice {
                id: "q2".into(),
                prompt: "Q2".into(),
                options: alloc::vec!["A".into()],
                correct: 0,
                explanation: "".into(),
                points: 10,
            });

        let feedback = alloc::vec![Feedback::correct("", 10), Feedback::incorrect(""),];

        let score = Grader::calculate_score(&quiz, &feedback);
        assert!(!score.passed);
        assert_eq!(score.points_earned, 10);
    }

    #[test]
    fn test_type_mismatch() {
        let question = Question::MultipleChoice {
            id: "q1".into(),
            prompt: "Q".into(),
            options: alloc::vec!["A".into()],
            correct: 0,
            explanation: "".into(),
            points: 10,
        };

        // Wrong answer type
        let feedback = Grader::grade_answer(&question, &Answer::Code("code".into()));
        assert!(!feedback.correct);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use profesor_core::QuestionId;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_correct_index_always_grades_correct(
            correct_idx in 0usize..10,
            num_options in 2usize..10
        ) {
            let correct_idx = correct_idx % num_options;
            let options: Vec<_> = (0..num_options).map(|i| alloc::format!("Option {}", i)).collect();

            let question = Question::MultipleChoice {
                id: QuestionId::new("q1"),
                prompt: "Test".into(),
                options,
                correct: correct_idx,
                explanation: "".into(),
                points: 10,
            };

            let feedback = Grader::grade_answer(&question, &Answer::Choice(correct_idx));
            prop_assert!(feedback.correct);
        }

        #[test]
        fn test_wrong_index_grades_incorrect(
            correct_idx in 0usize..10,
            wrong_idx in 0usize..10,
            num_options in 2usize..10
        ) {
            let correct_idx = correct_idx % num_options;
            let wrong_idx = wrong_idx % num_options;

            prop_assume!(correct_idx != wrong_idx);

            let options: Vec<_> = (0..num_options).map(|i| alloc::format!("Option {}", i)).collect();

            let question = Question::MultipleChoice {
                id: QuestionId::new("q1"),
                prompt: "Test".into(),
                options,
                correct: correct_idx,
                explanation: "".into(),
                points: 10,
            };

            let feedback = Grader::grade_answer(&question, &Answer::Choice(wrong_idx));
            prop_assert!(!feedback.correct);
        }
    }
}
