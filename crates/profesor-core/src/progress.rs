//! Progress tracking types.
//!
//! Tracks learner progress across courses, quizzes, and labs.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::ids::{CourseId, LabId, ModuleId, QuizId};

/// A unique identifier for a learner.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LearnerId(alloc::string::String);

impl LearnerId {
    /// Create a new learner ID.
    #[must_use]
    pub fn new(id: impl Into<alloc::string::String>) -> Self {
        Self(id.into())
    }

    /// Get the inner string value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Timestamp in milliseconds since Unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Create a timestamp from milliseconds.
    #[must_use]
    pub const fn from_millis(millis: u64) -> Self {
        Self(millis)
    }

    /// Get the value in milliseconds.
    #[must_use]
    pub const fn as_millis(&self) -> u64 {
        self.0
    }

    /// Zero timestamp.
    pub const ZERO: Self = Self(0);
}

/// Learner progress across all courses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LearnerProgress {
    /// Learner identifier
    pub learner_id: LearnerId,
    /// Progress for each course (course_id -> progress)
    pub courses: BTreeMap<alloc::string::String, CourseProgress>,
    /// Total experience points earned
    pub total_xp: u64,
    /// Current streak in days
    pub streak_days: u32,
    /// Timestamp of last activity
    pub last_activity: Timestamp,
}

impl LearnerProgress {
    /// Create a new learner progress record.
    #[must_use]
    pub fn new(learner_id: impl Into<LearnerId>) -> Self {
        Self {
            learner_id: learner_id.into(),
            courses: BTreeMap::new(),
            total_xp: 0,
            streak_days: 0,
            last_activity: Timestamp::ZERO,
        }
    }

    /// Get progress for a specific course.
    #[must_use]
    pub fn course_progress(&self, course_id: &CourseId) -> Option<&CourseProgress> {
        self.courses.get(course_id.as_str())
    }

    /// Start a new course.
    pub fn start_course(&mut self, course_id: CourseId, now: Timestamp) {
        let progress = CourseProgress::new(course_id.clone(), now);
        self.courses.insert(course_id.as_str().into(), progress);
        self.last_activity = now;
    }

    /// Add XP and update last activity.
    pub fn add_xp(&mut self, xp: u64, now: Timestamp) {
        self.total_xp = self.total_xp.saturating_add(xp);
        self.last_activity = now;
    }

    /// Get the number of courses in progress or completed.
    #[must_use]
    pub fn course_count(&self) -> usize {
        self.courses.len()
    }

    /// Get the number of completed courses.
    #[must_use]
    pub fn completed_courses(&self) -> usize {
        self.courses
            .values()
            .filter(|p| p.status == CourseStatus::Completed)
            .count()
    }
}

impl From<alloc::string::String> for LearnerId {
    fn from(s: alloc::string::String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for LearnerId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Progress within a single course.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseProgress {
    /// Course identifier
    pub course_id: CourseId,
    /// Current status
    pub status: CourseStatus,
    /// Completed module IDs
    pub modules_completed: Vec<ModuleId>,
    /// Current module being worked on
    pub current_module: Option<ModuleId>,
    /// Quiz attempts (quiz_id -> attempts)
    pub quiz_scores: BTreeMap<alloc::string::String, Vec<QuizAttempt>>,
    /// Lab completions (lab_id -> completion)
    pub lab_completions: BTreeMap<alloc::string::String, LabCompletion>,
    /// When the course was started
    pub started_at: Timestamp,
    /// When the course was completed (if completed)
    pub completed_at: Option<Timestamp>,
}

impl CourseProgress {
    /// Create a new course progress record.
    #[must_use]
    pub fn new(course_id: CourseId, started_at: Timestamp) -> Self {
        Self {
            course_id,
            status: CourseStatus::InProgress,
            modules_completed: Vec::new(),
            current_module: None,
            quiz_scores: BTreeMap::new(),
            lab_completions: BTreeMap::new(),
            started_at,
            completed_at: None,
        }
    }

    /// Mark a module as completed.
    pub fn complete_module(&mut self, module_id: ModuleId) {
        if !self.modules_completed.contains(&module_id) {
            self.modules_completed.push(module_id);
        }
    }

    /// Set the current module.
    pub fn set_current_module(&mut self, module_id: ModuleId) {
        self.current_module = Some(module_id);
    }

    /// Record a quiz attempt.
    pub fn record_quiz_attempt(&mut self, quiz_id: &QuizId, attempt: QuizAttempt) {
        self.quiz_scores
            .entry(quiz_id.as_str().into())
            .or_default()
            .push(attempt);
    }

    /// Record a lab completion.
    pub fn record_lab_completion(&mut self, lab_id: &LabId, completion: LabCompletion) {
        self.lab_completions
            .insert(lab_id.as_str().into(), completion);
    }

    /// Get the best score for a quiz.
    #[must_use]
    pub fn best_quiz_score(&self, quiz_id: &QuizId) -> Option<f32> {
        self.quiz_scores.get(quiz_id.as_str()).and_then(|attempts| {
            attempts
                .iter()
                .map(|a| a.score)
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
        })
    }

    /// Mark the course as completed.
    pub fn complete(&mut self, now: Timestamp) {
        self.status = CourseStatus::Completed;
        self.completed_at = Some(now);
    }

    /// Get the percentage of modules completed.
    #[must_use]
    pub fn completion_percentage(&self, total_modules: usize) -> f32 {
        if total_modules == 0 {
            return 0.0;
        }
        self.modules_completed.len() as f32 / total_modules as f32
    }
}

/// Status of a course.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CourseStatus {
    /// Not started yet
    #[default]
    NotStarted,
    /// Currently in progress
    InProgress,
    /// Completed successfully
    Completed,
    /// Abandoned by the learner
    Abandoned,
}

impl CourseStatus {
    /// Check if the course is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    /// Check if the course is finished (completed or abandoned).
    #[must_use]
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Completed | Self::Abandoned)
    }
}

/// A single quiz attempt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuizAttempt {
    /// Score as a percentage (0.0 - 1.0)
    pub score: f32,
    /// Duration in seconds
    pub duration_secs: u32,
    /// When the attempt was made
    pub attempted_at: Timestamp,
    /// Whether the attempt passed
    pub passed: bool,
}

impl QuizAttempt {
    /// Create a new quiz attempt.
    #[must_use]
    pub fn new(score: f32, duration_secs: u32, attempted_at: Timestamp, passed: bool) -> Self {
        Self {
            score: score.clamp(0.0, 1.0),
            duration_secs,
            attempted_at,
            passed,
        }
    }
}

/// A lab completion record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LabCompletion {
    /// Whether all tests passed
    pub all_tests_passed: bool,
    /// Number of tests passed
    pub tests_passed: u32,
    /// Total number of tests
    pub tests_total: u32,
    /// Time spent in seconds
    pub time_spent_secs: u32,
    /// When the lab was completed
    pub completed_at: Timestamp,
}

impl LabCompletion {
    /// Create a new lab completion.
    #[must_use]
    pub fn new(
        tests_passed: u32,
        tests_total: u32,
        time_spent_secs: u32,
        completed_at: Timestamp,
    ) -> Self {
        Self {
            all_tests_passed: tests_passed == tests_total,
            tests_passed,
            tests_total,
            time_spent_secs,
            completed_at,
        }
    }

    /// Get the test pass rate.
    #[must_use]
    pub fn pass_rate(&self) -> f32 {
        if self.tests_total == 0 {
            return 0.0;
        }
        self.tests_passed as f32 / self.tests_total as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learner_progress_creation() {
        let progress = LearnerProgress::new("user-123");
        assert_eq!(progress.learner_id.as_str(), "user-123");
        assert_eq!(progress.total_xp, 0);
        assert_eq!(progress.course_count(), 0);
    }

    #[test]
    fn test_learner_id_from_string() {
        let id: LearnerId = alloc::string::String::from("user-456").into();
        assert_eq!(id.as_str(), "user-456");
    }

    #[test]
    fn test_timestamp() {
        let ts = Timestamp::from_millis(12345);
        assert_eq!(ts.as_millis(), 12345);
        assert_eq!(Timestamp::ZERO.as_millis(), 0);
        assert_eq!(Timestamp::default().as_millis(), 0);
    }

    #[test]
    fn test_start_course() {
        let mut progress = LearnerProgress::new("user-1");
        let course_id = CourseId::new("rust-101");
        let now = Timestamp::from_millis(1000);

        progress.start_course(course_id.clone(), now);

        assert_eq!(progress.course_count(), 1);
        assert!(progress.course_progress(&course_id).is_some());
        assert_eq!(progress.last_activity.as_millis(), 1000);
    }

    #[test]
    fn test_add_xp() {
        let mut progress = LearnerProgress::new("user-1");
        let now = Timestamp::from_millis(1000);

        progress.add_xp(100, now);
        progress.add_xp(50, now);

        assert_eq!(progress.total_xp, 150);
    }

    #[test]
    fn test_completed_courses() {
        let mut progress = LearnerProgress::new("user-1");
        let now = Timestamp::from_millis(1000);

        progress.start_course(CourseId::new("course-1"), now);
        progress.start_course(CourseId::new("course-2"), now);

        assert_eq!(progress.completed_courses(), 0);

        // Complete one course
        if let Some(cp) = progress.courses.get_mut("course-1") {
            cp.complete(now);
        }

        assert_eq!(progress.completed_courses(), 1);
    }

    #[test]
    fn test_course_progress_modules() {
        let mut cp = CourseProgress::new(CourseId::new("test"), Timestamp::from_millis(0));

        cp.complete_module(ModuleId::new("mod-1"));
        cp.complete_module(ModuleId::new("mod-2"));
        cp.complete_module(ModuleId::new("mod-1")); // Duplicate should be ignored

        assert_eq!(cp.modules_completed.len(), 2);
    }

    #[test]
    fn test_set_current_module() {
        let mut cp = CourseProgress::new(CourseId::new("test"), Timestamp::from_millis(0));
        assert!(cp.current_module.is_none());

        cp.set_current_module(ModuleId::new("mod-1"));
        assert_eq!(cp.current_module.as_ref().unwrap().as_str(), "mod-1");
    }

    #[test]
    fn test_quiz_attempt_recording() {
        let mut cp = CourseProgress::new(CourseId::new("test"), Timestamp::from_millis(0));
        let quiz_id = QuizId::new("quiz-1");

        cp.record_quiz_attempt(
            &quiz_id,
            QuizAttempt::new(0.6, 300, Timestamp::from_millis(1000), false),
        );
        cp.record_quiz_attempt(
            &quiz_id,
            QuizAttempt::new(0.8, 250, Timestamp::from_millis(2000), true),
        );

        assert_eq!(cp.best_quiz_score(&quiz_id), Some(0.8));
    }

    #[test]
    fn test_best_quiz_score_none() {
        let cp = CourseProgress::new(CourseId::new("test"), Timestamp::from_millis(0));
        let quiz_id = QuizId::new("nonexistent");
        assert!(cp.best_quiz_score(&quiz_id).is_none());
    }

    #[test]
    fn test_record_lab_completion() {
        let mut cp = CourseProgress::new(CourseId::new("test"), Timestamp::from_millis(0));
        let lab_id = LabId::new("lab-1");
        let completion = LabCompletion::new(10, 10, 1800, Timestamp::from_millis(5000));

        cp.record_lab_completion(&lab_id, completion);

        assert!(cp.lab_completions.contains_key("lab-1"));
        assert!(cp.lab_completions.get("lab-1").unwrap().all_tests_passed);
    }

    #[test]
    fn test_course_progress_complete() {
        let mut cp = CourseProgress::new(CourseId::new("test"), Timestamp::from_millis(0));
        assert_eq!(cp.status, CourseStatus::InProgress);
        assert!(cp.completed_at.is_none());

        let now = Timestamp::from_millis(10000);
        cp.complete(now);

        assert_eq!(cp.status, CourseStatus::Completed);
        assert_eq!(cp.completed_at.unwrap().as_millis(), 10000);
    }

    #[test]
    fn test_lab_completion() {
        let completion = LabCompletion::new(8, 10, 1800, Timestamp::from_millis(5000));

        assert!(!completion.all_tests_passed);
        assert!((completion.pass_rate() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_lab_completion_all_passed() {
        let completion = LabCompletion::new(10, 10, 1800, Timestamp::from_millis(5000));
        assert!(completion.all_tests_passed);
        assert!((completion.pass_rate() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_lab_completion_zero_tests() {
        let completion = LabCompletion::new(0, 0, 100, Timestamp::from_millis(0));
        assert!((completion.pass_rate()).abs() < f32::EPSILON);
    }

    #[test]
    fn test_completion_percentage() {
        let mut cp = CourseProgress::new(CourseId::new("test"), Timestamp::from_millis(0));
        cp.complete_module(ModuleId::new("mod-1"));
        cp.complete_module(ModuleId::new("mod-2"));

        let pct = cp.completion_percentage(4);
        assert!((pct - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_completion_percentage_zero_modules() {
        let cp = CourseProgress::new(CourseId::new("test"), Timestamp::from_millis(0));
        assert!((cp.completion_percentage(0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_course_status() {
        assert!(CourseStatus::InProgress.is_active());
        assert!(!CourseStatus::Completed.is_active());
        assert!(CourseStatus::Completed.is_finished());
        assert!(CourseStatus::Abandoned.is_finished());
    }

    #[test]
    fn test_course_status_default() {
        let status = CourseStatus::default();
        assert_eq!(status, CourseStatus::NotStarted);
        assert!(!status.is_active());
        assert!(!status.is_finished());
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_xp_addition_is_monotonic(
            xp1 in 0u64..1000,
            xp2 in 0u64..1000
        ) {
            let mut progress = LearnerProgress::new("test");
            let now = Timestamp::from_millis(1000);

            progress.add_xp(xp1, now);
            let after_first = progress.total_xp;

            progress.add_xp(xp2, now);
            let after_second = progress.total_xp;

            prop_assert!(after_second >= after_first);
        }

        #[test]
        fn test_quiz_score_clamped(score in -1.0f32..2.0) {
            let attempt = QuizAttempt::new(score, 100, Timestamp::from_millis(0), false);
            prop_assert!(attempt.score >= 0.0);
            prop_assert!(attempt.score <= 1.0);
        }

        #[test]
        fn test_lab_pass_rate_in_range(
            passed in 0u32..100,
            total in 1u32..100
        ) {
            let completion = LabCompletion::new(
                passed.min(total),
                total,
                100,
                Timestamp::from_millis(0)
            );
            prop_assert!(completion.pass_rate() >= 0.0);
            prop_assert!(completion.pass_rate() <= 1.0);
        }
    }
}
