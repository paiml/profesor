//! Course structure types.
//!
//! Defines the hierarchical structure of courses: Course → Module → Lesson.

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::ids::{CourseId, LessonId, ModuleId, SimulationId};
use crate::lab::Lab;
use crate::quiz::Quiz;

/// A complete course (e.g., "Rust Fundamentals").
///
/// Courses are the top-level container for learning content, consisting
/// of multiple modules that are completed sequentially.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Course {
    /// Unique identifier for this course
    pub id: CourseId,
    /// Human-readable title
    pub title: String,
    /// Course description
    pub description: String,
    /// Difficulty level
    pub level: CourseLevel,
    /// Ordered list of modules
    pub modules: Vec<Module>,
    /// Courses that must be completed before this one
    pub prerequisites: Vec<CourseId>,
    /// Estimated time to complete in hours
    pub estimated_hours: u32,
}

impl Course {
    /// Create a new course with required fields.
    #[must_use]
    pub fn new(id: impl Into<CourseId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: String::new(),
            level: CourseLevel::Beginner,
            modules: Vec::new(),
            prerequisites: Vec::new(),
            estimated_hours: 0,
        }
    }

    /// Set the course description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the course level.
    #[must_use]
    pub fn with_level(mut self, level: CourseLevel) -> Self {
        self.level = level;
        self
    }

    /// Add a module to the course.
    #[must_use]
    pub fn with_module(mut self, module: Module) -> Self {
        self.modules.push(module);
        self
    }

    /// Set the estimated hours.
    #[must_use]
    pub fn with_estimated_hours(mut self, hours: u32) -> Self {
        self.estimated_hours = hours;
        self
    }

    /// Get the total number of lessons in the course.
    #[must_use]
    pub fn total_lessons(&self) -> usize {
        self.modules.iter().map(|m| m.lessons.len()).sum()
    }

    /// Get the total number of quizzes in the course.
    #[must_use]
    pub fn total_quizzes(&self) -> usize {
        self.modules.iter().filter(|m| m.quiz.is_some()).count()
    }
}

/// Course difficulty level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CourseLevel {
    /// For beginners with no prior experience
    #[default]
    Beginner,
    /// For those with some background knowledge
    Intermediate,
    /// For experienced practitioners
    Advanced,
    /// For domain experts seeking deep knowledge
    Expert,
}

impl CourseLevel {
    /// Get a human-readable label for the level.
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

/// A module within a course (e.g., "Week 1: Ownership").
///
/// Modules are designed to be ~2-4 hours each for consistent pacing
/// (Heijunka principle from Toyota Way).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Module {
    /// Unique identifier for this module
    pub id: ModuleId,
    /// Human-readable title
    pub title: String,
    /// Ordered list of lessons
    pub lessons: Vec<Lesson>,
    /// Optional quiz for this module
    pub quiz: Option<Quiz>,
    /// Optional hands-on lab
    pub lab: Option<Lab>,
    /// Criteria to unlock this module
    pub unlock_criteria: UnlockCriteria,
}

impl Module {
    /// Create a new module with required fields.
    #[must_use]
    pub fn new(id: impl Into<ModuleId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            lessons: Vec::new(),
            quiz: None,
            lab: None,
            unlock_criteria: UnlockCriteria::None,
        }
    }

    /// Add a lesson to this module.
    #[must_use]
    pub fn with_lesson(mut self, lesson: Lesson) -> Self {
        self.lessons.push(lesson);
        self
    }

    /// Set the quiz for this module.
    #[must_use]
    pub fn with_quiz(mut self, quiz: Quiz) -> Self {
        self.quiz = Some(quiz);
        self
    }

    /// Set the lab for this module.
    #[must_use]
    pub fn with_lab(mut self, lab: Lab) -> Self {
        self.lab = Some(lab);
        self
    }

    /// Set the unlock criteria.
    #[must_use]
    pub fn with_unlock_criteria(mut self, criteria: UnlockCriteria) -> Self {
        self.unlock_criteria = criteria;
        self
    }

    /// Get the total duration of all lessons in minutes.
    #[must_use]
    pub fn total_duration_minutes(&self) -> u32 {
        self.lessons.iter().map(|l| l.duration_minutes).sum()
    }
}

/// Criteria for unlocking a module.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum UnlockCriteria {
    /// No requirements (first module)
    #[default]
    None,
    /// Must complete a specific module
    ModuleCompleted(ModuleId),
    /// Must achieve a minimum score on a quiz
    QuizScore {
        /// The quiz that must be completed
        quiz_id: crate::ids::QuizId,
        /// Minimum score (0.0 - 1.0)
        min_score: f32,
    },
}

/// A single lesson within a module.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lesson {
    /// Unique identifier for this lesson
    pub id: LessonId,
    /// Human-readable title
    pub title: String,
    /// The lesson content
    pub content: LessonContent,
    /// Estimated duration in minutes
    pub duration_minutes: u32,
}

impl Lesson {
    /// Create a new lesson with required fields.
    #[must_use]
    pub fn new(id: impl Into<LessonId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: LessonContent::Text(String::new()),
            duration_minutes: 0,
        }
    }

    /// Set the lesson content.
    #[must_use]
    pub fn with_content(mut self, content: LessonContent) -> Self {
        self.content = content;
        self
    }

    /// Set the duration.
    #[must_use]
    pub fn with_duration(mut self, minutes: u32) -> Self {
        self.duration_minutes = minutes;
        self
    }
}

/// Types of lesson content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LessonContent {
    /// Markdown text content
    Text(String),
    /// Interactive code example
    InteractiveCode {
        /// The code to display
        code: String,
        /// Programming language
        language: crate::lab::Language,
    },
    /// Embedded simulation
    Simulation {
        /// ID of the simulation to embed
        sim_id: SimulationId,
    },
    /// Video reference (external URL)
    Video {
        /// URL of the video
        url: String,
        /// Duration in seconds
        duration_seconds: u32,
    },
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::*;
    use crate::lab::Language;
    use crate::quiz::Quiz;

    #[test]
    fn test_course_creation() {
        let course = Course::new("rust-101", "Rust Fundamentals")
            .with_description("Learn Rust from scratch")
            .with_level(CourseLevel::Beginner)
            .with_estimated_hours(40);

        assert_eq!(course.id.as_str(), "rust-101");
        assert_eq!(course.title, "Rust Fundamentals");
        assert_eq!(course.level, CourseLevel::Beginner);
        assert_eq!(course.estimated_hours, 40);
    }

    #[test]
    fn test_course_total_lessons() {
        let course = Course::new("test", "Test")
            .with_module(
                Module::new("mod-1", "Module 1")
                    .with_lesson(Lesson::new("l1", "Lesson 1"))
                    .with_lesson(Lesson::new("l2", "Lesson 2")),
            )
            .with_module(
                Module::new("mod-2", "Module 2").with_lesson(Lesson::new("l3", "Lesson 3")),
            );

        assert_eq!(course.total_lessons(), 3);
    }

    #[test]
    fn test_course_total_quizzes() {
        let course = Course::new("test", "Test")
            .with_module(Module::new("mod-1", "Module 1").with_quiz(Quiz::new("q1", "Quiz 1")))
            .with_module(Module::new("mod-2", "Module 2"))
            .with_module(Module::new("mod-3", "Module 3").with_quiz(Quiz::new("q2", "Quiz 2")));

        assert_eq!(course.total_quizzes(), 2);
    }

    #[test]
    fn test_module_duration() {
        let module = Module::new("mod-1", "Module 1")
            .with_lesson(Lesson::new("l1", "L1").with_duration(30))
            .with_lesson(Lesson::new("l2", "L2").with_duration(45));

        assert_eq!(module.total_duration_minutes(), 75);
    }

    #[test]
    fn test_module_with_quiz() {
        let module = Module::new("mod-1", "Module 1").with_quiz(Quiz::new("q1", "Quiz 1"));
        assert!(module.quiz.is_some());
        assert_eq!(module.quiz.as_ref().unwrap().id.as_str(), "q1");
    }

    #[test]
    fn test_module_with_lab() {
        let lab = crate::lab::Lab::new("lab-1", "Lab 1");
        let module = Module::new("mod-1", "Module 1").with_lab(lab);
        assert!(module.lab.is_some());
        assert_eq!(module.lab.as_ref().unwrap().id.as_str(), "lab-1");
    }

    #[test]
    fn test_module_with_unlock_criteria() {
        let module = Module::new("mod-1", "Module 1").with_unlock_criteria(
            UnlockCriteria::ModuleCompleted(crate::ids::ModuleId::new("mod-0")),
        );
        assert!(matches!(
            module.unlock_criteria,
            UnlockCriteria::ModuleCompleted(_)
        ));
    }

    #[test]
    fn test_course_level_label() {
        assert_eq!(CourseLevel::Beginner.label(), "Beginner");
        assert_eq!(CourseLevel::Intermediate.label(), "Intermediate");
        assert_eq!(CourseLevel::Advanced.label(), "Advanced");
        assert_eq!(CourseLevel::Expert.label(), "Expert");
    }

    #[test]
    fn test_course_level_default() {
        let level = CourseLevel::default();
        assert_eq!(level, CourseLevel::Beginner);
    }

    #[test]
    fn test_unlock_criteria_default() {
        let criteria = UnlockCriteria::default();
        assert_eq!(criteria, UnlockCriteria::None);
    }

    #[test]
    fn test_unlock_criteria_quiz_score() {
        let criteria = UnlockCriteria::QuizScore {
            quiz_id: crate::ids::QuizId::new("q1"),
            min_score: 0.8,
        };
        if let UnlockCriteria::QuizScore { quiz_id, min_score } = criteria {
            assert_eq!(quiz_id.as_str(), "q1");
            assert!((min_score - 0.8).abs() < f32::EPSILON);
        } else {
            panic!("Expected QuizScore variant");
        }
    }

    #[test]
    fn test_lesson_with_content() {
        let lesson = Lesson::new("l1", "Lesson 1")
            .with_content(LessonContent::Text("Hello world".into()))
            .with_duration(15);
        assert_eq!(lesson.duration_minutes, 15);
        if let LessonContent::Text(text) = &lesson.content {
            assert_eq!(text, "Hello world");
        } else {
            panic!("Expected Text content");
        }
    }

    #[test]
    fn test_lesson_content_types() {
        let text = LessonContent::Text("Hello".into());
        assert!(matches!(text, LessonContent::Text(_)));

        let video = LessonContent::Video {
            url: "https://example.com".into(),
            duration_seconds: 300,
        };
        assert!(matches!(video, LessonContent::Video { .. }));

        let interactive_code = LessonContent::InteractiveCode {
            code: "fn main() {}".into(),
            language: Language::Rust,
        };
        if let LessonContent::InteractiveCode { code, language } = &interactive_code {
            assert_eq!(code, "fn main() {}");
            assert_eq!(*language, Language::Rust);
        } else {
            panic!("Expected InteractiveCode content");
        }

        let simulation = LessonContent::Simulation {
            sim_id: crate::ids::SimulationId::new("sim-1"),
        };
        if let LessonContent::Simulation { sim_id } = &simulation {
            assert_eq!(sim_id.as_str(), "sim-1");
        } else {
            panic!("Expected Simulation content");
        }
    }

    #[test]
    fn test_course_with_level_intermediate_advanced() {
        let course_int = Course::new("test", "Test").with_level(CourseLevel::Intermediate);
        assert_eq!(course_int.level, CourseLevel::Intermediate);

        let course_adv = Course::new("test", "Test").with_level(CourseLevel::Advanced);
        assert_eq!(course_adv.level, CourseLevel::Advanced);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_course_lessons_count_matches_modules(
            num_modules in 1usize..10,
            lessons_per_module in 1usize..5
        ) {
            let mut course = Course::new("test", "Test");
            for i in 0..num_modules {
                let mut module = Module::new(format!("mod-{}", i), format!("Module {}", i));
                for j in 0..lessons_per_module {
                    module = module.with_lesson(
                        Lesson::new(format!("l-{}-{}", i, j), format!("Lesson {}-{}", i, j))
                    );
                }
                course = course.with_module(module);
            }
            prop_assert_eq!(course.total_lessons(), num_modules * lessons_per_module);
        }

        #[test]
        fn test_module_duration_is_sum(
            durations in prop::collection::vec(0u32..120, 1..10)
        ) {
            let mut module = Module::new("test", "Test");
            let expected_total: u32 = durations.iter().sum();

            for (i, &duration) in durations.iter().enumerate() {
                module = module.with_lesson(
                    Lesson::new(format!("l-{}", i), format!("L{}", i))
                        .with_duration(duration)
                );
            }

            prop_assert_eq!(module.total_duration_minutes(), expected_total);
        }
    }
}
