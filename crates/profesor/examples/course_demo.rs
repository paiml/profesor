//! Course Structure Demo
//!
//! Demonstrates the profesor course management system (Kaizen principle).
//!
//! Run with: `cargo run --example course_demo`

use profesor::{
    Course, CourseLevel, Difficulty, Hint, Lab, LabStep, Language, Lesson, LessonContent, Module,
    Question, QuestionId, Quiz, StarterFile, StepValidation, TestCase, TestSuite,
};

fn main() {
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│          PROFESOR - Course Structure Demo               │");
    println!("│                                                         │");
    println!("│  Demonstrating Kaizen: Continuous improvement paths     │");
    println!("└─────────────────────────────────────────────────────────┘\n");

    // Phase 1: Create Course Structure
    phase_1_course_structure();

    // Phase 2: Show Module Organization
    phase_2_module_organization();

    // Phase 3: Learning Path
    phase_3_learning_path();
}

fn phase_1_course_structure() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 1: Course Structure   │");
    println!("└─────────────────────────────┘\n");

    let course = create_rust_course();

    println!("Course: {}", course.title);
    println!("  - ID: {}", course.id.as_str());
    println!("  - Level: {:?}", course.level);
    println!("  - Modules: {}", course.modules.len());
    println!("  - Total Lessons: {}", course.total_lessons());
    println!("  - Total Quizzes: {}", course.total_quizzes());
    println!();

    println!("Course Hierarchy:");
    println!("  Course");
    println!("    └── Modules (topic groupings)");
    println!("          └── Lessons (content + exercises)");
    println!("          └── Quiz (knowledge check)");
    println!("          └── Lab (hands-on practice)");
    println!();
}

fn phase_2_module_organization() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 2: Module Structure   │");
    println!("└─────────────────────────────┘\n");

    let course = create_rust_course();

    for (i, module) in course.modules.iter().enumerate() {
        println!("Module {}: {}", i + 1, module.title);
        println!("  ID: {}", module.id.as_str());
        println!("  Duration: {} min", module.total_duration_minutes());
        println!();

        // Show lessons
        if !module.lessons.is_empty() {
            println!("  Lessons:");
            for (j, lesson) in module.lessons.iter().enumerate() {
                println!(
                    "    {}. {} ({} min)",
                    j + 1,
                    lesson.title,
                    lesson.duration_minutes
                );
                match &lesson.content {
                    LessonContent::Text(t) => {
                        let preview = if t.len() > 50 { &t[..50] } else { t.as_str() };
                        println!("       Content: \"{}...\"", preview);
                    }
                    LessonContent::InteractiveCode { language, .. } => {
                        println!("       Interactive: {:?}", language);
                    }
                    LessonContent::Video {
                        duration_seconds, ..
                    } => {
                        println!("       Duration: {}s", duration_seconds);
                    }
                    LessonContent::Simulation { sim_id } => {
                        println!("       Simulation: {}", sim_id.as_str());
                    }
                }
            }
            println!();
        }

        // Show quiz
        if let Some(quiz) = &module.quiz {
            println!("  Quiz:");
            println!(
                "    - {} ({} questions, {} pts)",
                quiz.title,
                quiz.question_count(),
                quiz.total_points()
            );
            println!();
        }

        // Show lab
        if let Some(lab) = &module.lab {
            println!("  Lab:");
            println!(
                "    - {} ({} steps, {:?})",
                lab.title,
                lab.step_count(),
                lab.language
            );
            println!();
        }
    }
}

fn phase_3_learning_path() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 3: Learning Path      │");
    println!("└─────────────────────────────┘\n");

    println!("Recommended progression:");
    println!();
    println!("  1. Read Lesson (gain knowledge)");
    println!("       │");
    println!("       ▼");
    println!("  2. Take Quiz (verify understanding)");
    println!("       │ ◄─── Jidoka: Immediate feedback on errors");
    println!("       ▼");
    println!("  3. Complete Lab (apply skills)");
    println!("       │ ◄─── Poka-Yoke: Guided steps prevent mistakes");
    println!("       ▼");
    println!("  4. Next Module (progressive complexity)");
    println!("       │ ◄─── Kaizen: Continuous improvement");
    println!("       ▼");
    println!("  5. Course Completion!");
    println!();

    println!("Toyota Way Principles:");
    println!("  - Jidoka: Automatic quality checks at each step");
    println!("  - Poka-Yoke: Lab structure prevents common errors");
    println!("  - Kaizen: Small steps build to mastery");
    println!("  - Mieruka: Progress visualization");
    println!();

    println!("Content Types:");
    println!("  ┌──────────────┬───────────────────────────────┐");
    println!("  │ Type         │ Purpose                       │");
    println!("  ├──────────────┼───────────────────────────────┤");
    println!("  │ Lesson       │ Knowledge transfer            │");
    println!("  │ Quiz         │ Knowledge verification        │");
    println!("  │ Lab          │ Skill application             │");
    println!("  │ Simulation   │ Visual concept exploration    │");
    println!("  └──────────────┴───────────────────────────────┘");
    println!();

    println!("Demo complete!");
}

fn create_rust_course() -> Course {
    Course::new("rust-fundamentals", "Rust Programming Fundamentals")
        .with_level(CourseLevel::Beginner)
        .with_description("Learn Rust from scratch with hands-on exercises")
        .with_estimated_hours(40)
        .with_module(create_module_1())
        .with_module(create_module_2())
}

fn create_module_1() -> Module {
    Module::new("getting-started", "Getting Started with Rust")
        .with_lesson(
            Lesson::new("hello-rust", "Hello, Rust!")
                .with_duration(15)
                .with_content(LessonContent::Text(
                    "Rust is a systems programming language focused on safety, speed, and \
                     concurrency. In this lesson, you'll learn to write your first Rust \
                     program and understand the basic structure of Rust code."
                        .into(),
                )),
        )
        .with_lesson(
            Lesson::new("variables", "Variables and Mutability")
                .with_duration(20)
                .with_content(LessonContent::InteractiveCode {
                    code: "let x = 5; // immutable\nlet mut y = 10; // mutable".into(),
                    language: Language::Rust,
                }),
        )
        .with_quiz(create_basics_quiz())
        .with_lab(create_hello_lab())
}

fn create_module_2() -> Module {
    Module::new("ownership", "Ownership and Borrowing")
        .with_lesson(
            Lesson::new("ownership-rules", "The Three Rules of Ownership")
                .with_duration(30)
                .with_content(LessonContent::Text(
                    "1. Each value has a single owner\n\
                     2. When the owner goes out of scope, the value is dropped\n\
                     3. Ownership can be transferred (moved) or borrowed"
                        .into(),
                )),
        )
        .with_lesson(
            Lesson::new("borrowing", "References and Borrowing")
                .with_duration(25)
                .with_content(LessonContent::InteractiveCode {
                    code: "let s = String::from(\"hello\");\nlet r = &s; // borrow".into(),
                    language: Language::Rust,
                }),
        )
}

fn create_basics_quiz() -> Quiz {
    Quiz::new("basics-quiz", "Rust Basics Quiz")
        .with_passing_score(0.7)
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "What keyword makes a variable mutable?".into(),
            options: vec!["var".into(), "mut".into(), "let".into(), "mutable".into()],
            correct: 1,
            explanation: "The 'mut' keyword is used to make a variable mutable.".into(),
            points: 10,
        })
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q2"),
            prompt: "Are variables in Rust mutable by default?".into(),
            options: vec!["Yes".into(), "No".into()],
            correct: 1,
            explanation: "Variables are immutable by default. Use 'mut' for mutability.".into(),
            points: 10,
        })
}

fn create_hello_lab() -> Lab {
    Lab::new("hello-lab", "Your First Rust Program")
        .with_language(Language::Rust)
        .with_difficulty(Difficulty::Beginner)
        .with_estimated_minutes(30)
        .with_description("Write your first Rust program and learn the basics of the language")
        .with_step(
            LabStep::new(1, "Create main function")
                .with_description("Create a main function that prints 'Hello, World!'")
                .with_validation(StepValidation::FunctionExists {
                    name: "main".into(),
                }),
        )
        .with_step(
            LabStep::new(2, "Add a variable")
                .with_description("Create a variable 'name' and print a greeting with it")
                .with_validation(StepValidation::OutputMatches {
                    expected: "Hello".into(),
                }),
        )
        .with_starter_file(StarterFile::new(
            "src/main.rs",
            "fn main() {\n    // Your code here\n}\n",
        ))
        .with_test_suite(
            TestSuite::new()
                .with_test(TestCase::new("test_hello"))
                .with_test(TestCase::new("test_greeting")),
        )
        .with_hint(Hint::new(
            1,
            "Use the println! macro to print to the console",
        ))
        .with_hint(Hint::new(
            2,
            "Use format string: println!(\"Hello, {}!\", name)",
        ))
}
