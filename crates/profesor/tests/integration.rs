//! Integration tests for the Profesor LMS.
//!
//! Tests end-to-end workflows across multiple crates.

#![allow(clippy::expect_used)]

use profesor::{
    Answer, Course, CourseLevel, Lesson, LessonContent, Module, PhysicsWorld, Question, QuestionId,
    Quiz, QuizEngine, QuizState, RigidBody, Vec2,
};

#[test]
fn test_complete_quiz_workflow() {
    // Create a quiz
    let quiz = Quiz::new("integration-test", "Integration Test Quiz")
        .with_passing_score(0.5)
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "What is 1 + 1?".into(),
            options: vec!["1".into(), "2".into(), "3".into()],
            correct: 1,
            explanation: "1 + 1 = 2".into(),
            points: 10,
        })
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q2"),
            prompt: "What is 2 + 2?".into(),
            options: vec!["3".into(), "4".into(), "5".into()],
            correct: 1,
            explanation: "2 + 2 = 4".into(),
            points: 10,
        });

    // Create engine and take quiz
    let mut engine = QuizEngine::new(quiz);

    // Start
    assert!(matches!(engine.state(), QuizState::NotStarted));
    let _ = engine.start().expect("Should start");
    assert!(matches!(engine.state(), QuizState::InProgress { .. }));

    // Answer first question correctly
    let feedback1 = engine
        .submit_answer(Answer::Choice(1))
        .expect("Should submit");
    assert!(feedback1.correct);
    assert_eq!(feedback1.points_earned, 10);

    // Move to next question
    let _ = engine.next_question().expect("Should advance");

    // Answer second question correctly
    let feedback2 = engine
        .submit_answer(Answer::Choice(1))
        .expect("Should submit");
    assert!(feedback2.correct);

    // Finish quiz
    let score = engine.finish().expect("Should finish");
    assert!(matches!(engine.state(), QuizState::Completed { .. }));
    assert_eq!(score.correct_count, 2);
    assert_eq!(score.points_earned, 20);
    assert!(score.passed);
}

#[test]
fn test_physics_simulation_workflow() {
    // Create world with gravity
    let mut world = PhysicsWorld::new()
        .with_gravity(Vec2::new(0.0, 10.0))
        .with_dt(0.1)
        .with_bounds(0.0, 0.0, 100.0, 100.0);

    // Add a body
    let body = RigidBody::new(50.0, 10.0).with_mass(1.0).with_radius(5.0);
    world.add_body(body);

    // Simulate
    let initial_y = world.bodies[0].position.y;
    for _ in 0..10 {
        world.step();
    }
    let final_y = world.bodies[0].position.y;

    // Body should have fallen due to gravity
    assert!(final_y > initial_y);
}

#[test]
fn test_course_structure_workflow() {
    // Create a course with modules and lessons
    let course = Course::new("test-course", "Test Course")
        .with_level(CourseLevel::Beginner)
        .with_description("A test course")
        .with_module(
            Module::new("module-1", "Module 1")
                .with_lesson(
                    Lesson::new("lesson-1", "Lesson 1")
                        .with_duration(15)
                        .with_content(LessonContent::Text("Content here".into())),
                )
                .with_lesson(
                    Lesson::new("lesson-2", "Lesson 2")
                        .with_duration(20)
                        .with_content(LessonContent::Text("More content".into())),
                ),
        )
        .with_module(Module::new("module-2", "Module 2"));

    // Verify structure
    assert_eq!(course.modules.len(), 2);
    assert_eq!(course.total_lessons(), 2);
    assert_eq!(course.modules[0].lessons.len(), 2);
    assert_eq!(course.modules[0].total_duration_minutes(), 35);
}

#[test]
fn test_quiz_grading_edge_cases() {
    // Quiz with zero passing score
    let quiz = Quiz::new("edge-case", "Edge Case Quiz")
        .with_passing_score(0.0)
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "Any question".into(),
            options: vec!["A".into(), "B".into()],
            correct: 0,
            explanation: "A is correct".into(),
            points: 10,
        });

    let mut engine = QuizEngine::new(quiz);
    engine.start().expect("Should start");

    // Answer incorrectly
    let feedback = engine
        .submit_answer(Answer::Choice(1))
        .expect("Should submit");
    assert!(!feedback.correct);

    let score = engine.finish().expect("Should finish");
    // Should still pass because passing score is 0
    assert!(score.passed);
}

#[test]
fn test_physics_body_movement() {
    let mut world = PhysicsWorld::new().with_gravity(Vec2::ZERO).with_dt(0.1);

    // Single body with velocity
    let mut body = RigidBody::new(0.0, 0.0).with_radius(10.0);
    body.velocity = Vec2::new(10.0, 0.0);
    world.add_body(body);

    // Initial position
    let initial_x = world.bodies[0].position.x;

    // Simulate for 10 steps
    for _ in 0..10 {
        world.step();
    }

    // Body should have moved right
    let final_x = world.bodies[0].position.x;
    assert!(
        final_x > initial_x,
        "Body should have moved right: {} -> {}",
        initial_x,
        final_x
    );

    // Velocity * time = 10 * 0.1 * 10 = 10 units
    assert!(
        (final_x - 10.0).abs() < 0.01,
        "Body should have moved ~10 units: {}",
        final_x
    );
}
