//! Application state and event handling.
//!
//! Manages the overall application lifecycle and user interactions.

use alloc::string::String;
use alloc::vec::Vec;
use profesor_core::{Course, CourseId, LearnerProgress, Quiz};
use profesor_quiz::QuizEngine;
use serde::{Deserialize, Serialize};

/// Application state.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Available courses
    pub courses: Vec<Course>,
    /// Current learner progress
    pub progress: Option<LearnerProgress>,
    /// Currently active view
    pub current_view: View,
    /// Active quiz engine (if taking a quiz)
    pub active_quiz: Option<ActiveQuiz>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Create a new application state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            courses: Vec::new(),
            progress: None,
            current_view: View::CourseList,
            active_quiz: None,
        }
    }

    /// Load courses into the app.
    pub fn load_courses(&mut self, courses: Vec<Course>) {
        self.courses = courses;
    }

    /// Set the learner progress.
    pub fn set_progress(&mut self, progress: LearnerProgress) {
        self.progress = Some(progress);
    }

    /// Get a course by ID.
    #[must_use]
    pub fn get_course(&self, id: &CourseId) -> Option<&Course> {
        self.courses.iter().find(|c| &c.id == id)
    }

    /// Navigate to a view.
    pub fn navigate(&mut self, view: View) {
        self.current_view = view;
    }

    /// Start a quiz.
    pub fn start_quiz(&mut self, quiz: Quiz) -> Result<(), AppError> {
        let mut engine = QuizEngine::new(quiz);
        engine
            .start()
            .map_err(|_| AppError::QuizError("Failed to start quiz".into()))?;

        self.active_quiz = Some(ActiveQuiz {
            engine,
            start_time_ms: 0, // Would be set by JS interop
        });

        self.current_view = View::Quiz;
        Ok(())
    }

    /// Get the active quiz engine.
    #[must_use]
    pub fn quiz_engine(&self) -> Option<&QuizEngine> {
        self.active_quiz.as_ref().map(|aq| &aq.engine)
    }

    /// Get a mutable reference to the active quiz engine.
    pub fn quiz_engine_mut(&mut self) -> Option<&mut QuizEngine> {
        self.active_quiz.as_mut().map(|aq| &mut aq.engine)
    }
}

/// Active quiz session.
#[derive(Debug, Clone)]
pub struct ActiveQuiz {
    /// Quiz engine managing state
    pub engine: QuizEngine,
    /// When the quiz was started (ms since epoch)
    pub start_time_ms: u64,
}

/// Application views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum View {
    /// List of available courses
    #[default]
    CourseList,
    /// Single course detail
    CourseDetail,
    /// Module content
    Module,
    /// Lesson content
    Lesson,
    /// Taking a quiz
    Quiz,
    /// Doing a lab
    Lab,
    /// Viewing a simulation
    Simulation,
    /// Learner profile/progress
    Profile,
}

/// Application events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    /// Navigate to a course
    SelectCourse(CourseId),
    /// Navigate to a module
    SelectModule(profesor_core::ModuleId),
    /// Navigate to a lesson
    SelectLesson(profesor_core::LessonId),
    /// Start a quiz
    StartQuiz(profesor_core::QuizId),
    /// Submit a quiz answer
    SubmitAnswer(profesor_core::Answer),
    /// Go to next question
    NextQuestion,
    /// Go to previous question
    PreviousQuestion,
    /// Finish the quiz
    FinishQuiz,
    /// Start a lab
    StartLab(profesor_core::LabId),
    /// Run lab tests
    RunLabTests,
    /// Submit lab
    SubmitLab,
    /// Navigate back
    GoBack,
    /// View profile
    ViewProfile,
}

/// Application errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    /// Course not found
    CourseNotFound,
    /// Module not found
    ModuleNotFound,
    /// Quiz error
    QuizError(String),
    /// Lab error
    LabError(String),
    /// Invalid state
    InvalidState,
}

impl core::fmt::Display for AppError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CourseNotFound => write!(f, "Course not found"),
            Self::ModuleNotFound => write!(f, "Module not found"),
            Self::QuizError(e) => write!(f, "Quiz error: {}", e),
            Self::LabError(e) => write!(f, "Lab error: {}", e),
            Self::InvalidState => write!(f, "Invalid state for this operation"),
        }
    }
}

/// Main application controller.
#[derive(Debug, Default)]
pub struct App {
    state: AppState,
}

impl App {
    /// Create a new application.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    /// Get the current state.
    #[must_use]
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Get a mutable reference to the state.
    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    /// Process an event and update state.
    pub fn handle_event(&mut self, event: AppEvent) -> Result<(), AppError> {
        match event {
            AppEvent::SelectCourse(id) => {
                if self.state.get_course(&id).is_some() {
                    self.state.navigate(View::CourseDetail);
                    Ok(())
                } else {
                    Err(AppError::CourseNotFound)
                }
            }

            AppEvent::StartQuiz(_quiz_id) => {
                // Would look up quiz and start it
                self.state.navigate(View::Quiz);
                Ok(())
            }

            AppEvent::SubmitAnswer(answer) => {
                if let Some(engine) = self.state.quiz_engine_mut() {
                    engine
                        .submit_answer(answer)
                        .map_err(|e| AppError::QuizError(alloc::format!("{}", e)))?;
                    Ok(())
                } else {
                    Err(AppError::InvalidState)
                }
            }

            AppEvent::NextQuestion => {
                if let Some(engine) = self.state.quiz_engine_mut() {
                    engine
                        .next_question()
                        .map_err(|e| AppError::QuizError(alloc::format!("{}", e)))?;
                    Ok(())
                } else {
                    Err(AppError::InvalidState)
                }
            }

            AppEvent::PreviousQuestion => {
                if let Some(engine) = self.state.quiz_engine_mut() {
                    engine
                        .previous_question()
                        .map_err(|e| AppError::QuizError(alloc::format!("{}", e)))?;
                    Ok(())
                } else {
                    Err(AppError::InvalidState)
                }
            }

            AppEvent::FinishQuiz => {
                if let Some(engine) = self.state.quiz_engine_mut() {
                    engine
                        .finish()
                        .map_err(|e| AppError::QuizError(alloc::format!("{}", e)))?;
                    Ok(())
                } else {
                    Err(AppError::InvalidState)
                }
            }

            AppEvent::GoBack => {
                self.state.navigate(View::CourseList);
                Ok(())
            }

            AppEvent::ViewProfile => {
                self.state.navigate(View::Profile);
                Ok(())
            }

            // Other events would be handled similarly
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use profesor_core::{LearnerProgress, Question, QuestionId, Quiz};

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert_eq!(app.state().current_view, View::CourseList);
    }

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert_eq!(state.current_view, View::CourseList);
        assert!(state.courses.is_empty());
        assert!(state.progress.is_none());
        assert!(state.active_quiz.is_none());
    }

    #[test]
    fn test_load_courses() {
        let mut app = App::new();
        let courses = alloc::vec![Course::new("c1", "Course 1"), Course::new("c2", "Course 2"),];

        app.state_mut().load_courses(courses);
        assert_eq!(app.state().courses.len(), 2);
    }

    #[test]
    fn test_set_progress() {
        let mut app = App::new();
        let progress = LearnerProgress::new("learner-1");
        app.state_mut().set_progress(progress);
        assert!(app.state().progress.is_some());
    }

    #[test]
    fn test_get_course() {
        let mut app = App::new();
        let courses = alloc::vec![Course::new("c1", "Course 1"), Course::new("c2", "Course 2"),];
        app.state_mut().load_courses(courses);

        let found = app.state().get_course(&CourseId::new("c1"));
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Course 1");

        let not_found = app.state().get_course(&CourseId::new("nonexistent"));
        assert!(not_found.is_none());
    }

    #[test]
    fn test_navigate() {
        let mut app = App::new();
        app.state_mut().navigate(View::Profile);
        assert_eq!(app.state().current_view, View::Profile);
    }

    #[test]
    fn test_start_quiz() {
        let mut app = App::new();
        let quiz = Quiz::new("test-quiz", "Test Quiz").with_question(Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "Test?".into(),
            options: alloc::vec!["A".into(), "B".into()],
            correct: 0,
            explanation: "A is correct".into(),
            points: 10,
        });

        app.state_mut().start_quiz(quiz).expect("Should start quiz");
        assert_eq!(app.state().current_view, View::Quiz);
        assert!(app.state().active_quiz.is_some());
        assert!(app.state().quiz_engine().is_some());
    }

    #[test]
    fn test_quiz_engine_mut() {
        let mut app = App::new();
        let quiz = Quiz::new("test-quiz", "Test Quiz").with_question(Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "Test?".into(),
            options: alloc::vec!["A".into(), "B".into()],
            correct: 0,
            explanation: "A is correct".into(),
            points: 10,
        });

        app.state_mut().start_quiz(quiz).expect("Should start quiz");
        let engine = app.state_mut().quiz_engine_mut();
        assert!(engine.is_some());
    }

    #[test]
    fn test_select_existing_course() {
        let mut app = App::new();
        let courses = alloc::vec![Course::new("c1", "Course 1")];
        app.state_mut().load_courses(courses);

        let result = app.handle_event(AppEvent::SelectCourse(CourseId::new("c1")));
        assert!(result.is_ok());
        assert_eq!(app.state().current_view, View::CourseDetail);
    }

    #[test]
    fn test_select_nonexistent_course() {
        let mut app = App::new();
        let result = app.handle_event(AppEvent::SelectCourse(CourseId::new("nonexistent")));
        assert_eq!(result, Err(AppError::CourseNotFound));
    }

    #[test]
    fn test_start_quiz_event() {
        let mut app = App::new();
        let result = app.handle_event(AppEvent::StartQuiz(profesor_core::QuizId::new("quiz1")));
        assert!(result.is_ok());
        assert_eq!(app.state().current_view, View::Quiz);
    }

    #[test]
    fn test_submit_answer_no_quiz() {
        let mut app = App::new();
        let result = app.handle_event(AppEvent::SubmitAnswer(profesor_core::Answer::Choice(0)));
        assert_eq!(result, Err(AppError::InvalidState));
    }

    #[test]
    fn test_submit_answer_with_quiz() {
        let mut app = App::new();
        let quiz = Quiz::new("test-quiz", "Test Quiz").with_question(Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "Test?".into(),
            options: alloc::vec!["A".into(), "B".into()],
            correct: 0,
            explanation: "A is correct".into(),
            points: 10,
        });

        app.state_mut().start_quiz(quiz).expect("Should start quiz");
        let result = app.handle_event(AppEvent::SubmitAnswer(profesor_core::Answer::Choice(0)));
        assert!(result.is_ok());
    }

    #[test]
    fn test_previous_question_no_quiz() {
        let mut app = App::new();
        let result = app.handle_event(AppEvent::PreviousQuestion);
        assert_eq!(result, Err(AppError::InvalidState));
    }

    #[test]
    fn test_finish_quiz_no_quiz() {
        let mut app = App::new();
        let result = app.handle_event(AppEvent::FinishQuiz);
        assert_eq!(result, Err(AppError::InvalidState));
    }

    #[test]
    fn test_finish_quiz_with_quiz() {
        let mut app = App::new();
        let quiz = Quiz::new("test-quiz", "Test Quiz").with_question(Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "Test?".into(),
            options: alloc::vec!["A".into(), "B".into()],
            correct: 0,
            explanation: "A is correct".into(),
            points: 10,
        });

        app.state_mut().start_quiz(quiz).expect("Should start quiz");
        app.handle_event(AppEvent::SubmitAnswer(profesor_core::Answer::Choice(0)))
            .expect("Should submit");
        let result = app.handle_event(AppEvent::FinishQuiz);
        assert!(result.is_ok());
    }

    #[test]
    fn test_go_back() {
        let mut app = App::new();
        app.state_mut().navigate(View::Profile);
        app.handle_event(AppEvent::GoBack).expect("Should go back");
        assert_eq!(app.state().current_view, View::CourseList);
    }

    #[test]
    fn test_view_profile() {
        let mut app = App::new();
        app.handle_event(AppEvent::ViewProfile)
            .expect("Should view profile");
        assert_eq!(app.state().current_view, View::Profile);
    }

    #[test]
    fn test_invalid_quiz_operation() {
        let mut app = App::new();
        let result = app.handle_event(AppEvent::NextQuestion);
        assert_eq!(result, Err(AppError::InvalidState));
    }

    #[test]
    fn test_unhandled_events() {
        let mut app = App::new();
        // These events are handled by the catch-all pattern
        let result = app.handle_event(AppEvent::SelectModule(profesor_core::ModuleId::new("m1")));
        assert!(result.is_ok());
        let result = app.handle_event(AppEvent::SelectLesson(profesor_core::LessonId::new("l1")));
        assert!(result.is_ok());
        let result = app.handle_event(AppEvent::StartLab(profesor_core::LabId::new("lab1")));
        assert!(result.is_ok());
        let result = app.handle_event(AppEvent::RunLabTests);
        assert!(result.is_ok());
        let result = app.handle_event(AppEvent::SubmitLab);
        assert!(result.is_ok());
    }

    #[test]
    fn test_app_error_display() {
        assert_eq!(
            alloc::format!("{}", AppError::CourseNotFound),
            "Course not found"
        );
        assert_eq!(
            alloc::format!("{}", AppError::ModuleNotFound),
            "Module not found"
        );
        assert_eq!(
            alloc::format!("{}", AppError::QuizError("test".into())),
            "Quiz error: test"
        );
        assert_eq!(
            alloc::format!("{}", AppError::LabError("test".into())),
            "Lab error: test"
        );
        assert_eq!(
            alloc::format!("{}", AppError::InvalidState),
            "Invalid state for this operation"
        );
    }

    #[test]
    fn test_view_default() {
        let view = View::default();
        assert_eq!(view, View::CourseList);
    }
}
