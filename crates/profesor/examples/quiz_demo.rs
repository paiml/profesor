//! Quiz Engine Demo
//!
//! Demonstrates the profesor quiz engine with immediate feedback (Jidoka principle).
//!
//! Run with: `cargo run --example quiz_demo`

use profesor::{Answer, Feedback, Question, QuestionId, Quiz, QuizEngine, QuizState};

fn main() {
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│           PROFESOR - Quiz Engine Demo                   │");
    println!("│                                                         │");
    println!("│  Demonstrating Jidoka: Immediate feedback on errors     │");
    println!("└─────────────────────────────────────────────────────────┘\n");

    // Phase 1: Create Quiz
    phase_1_create_quiz();

    // Phase 2: Take Quiz
    phase_2_take_quiz();

    // Phase 3: Review Results
    phase_3_review_results();
}

fn phase_1_create_quiz() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 1: Quiz Creation      │");
    println!("└─────────────────────────────┘\n");

    let quiz = create_rust_fundamentals_quiz();

    println!("Created quiz: {}", quiz.title);
    println!("  - ID: {}", quiz.id.as_str());
    println!("  - Questions: {}", quiz.question_count());
    println!("  - Total Points: {}", quiz.total_points());
    println!(
        "  - Passing Score: {}%",
        (quiz.passing_score * 100.0) as u32
    );
    println!("  - Max Attempts: {:?}", quiz.max_attempts);
    println!();

    println!("Questions:");
    for (i, q) in quiz.questions.iter().enumerate() {
        println!("  {}. {} ({} pts)", i + 1, q.prompt(), q.points());
    }
    println!();
}

fn phase_2_take_quiz() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 2: Taking the Quiz    │");
    println!("└─────────────────────────────┘\n");

    let quiz = create_rust_fundamentals_quiz();
    let mut engine = QuizEngine::new(quiz);

    println!("Starting quiz attempt...\n");

    // Start the quiz
    let first_prompt = engine
        .start()
        .expect("Failed to start quiz")
        .prompt()
        .to_string();
    println!("State: {:?}", state_name(engine.state()));
    println!("Attempt #: {}", engine.attempt_count());
    println!();

    // Question 1: Correct answer
    println!("Q1: {}", first_prompt);
    println!("    Options: let, var, const, mut");
    println!("    Answering: 'let' (index 0)");

    let feedback1 = engine
        .submit_answer(Answer::Choice(0))
        .expect("Failed to submit");
    print_feedback(&feedback1);

    // Move to next question
    let q2_prompt = engine
        .next_question()
        .expect("Failed to advance")
        .prompt()
        .to_string();
    println!("\nQ2: {}", q2_prompt);
    println!("    Options: i32, bool, String, char");
    println!("    Answering: 'String' (index 2) - CORRECT");

    let feedback2 = engine
        .submit_answer(Answer::Choice(2))
        .expect("Failed to submit");
    print_feedback(&feedback2);

    // Move to next question
    let q3_prompt = engine
        .next_question()
        .expect("Failed to advance")
        .prompt()
        .to_string();
    println!("\nQ3: {}", q3_prompt);
    println!("    Options: It's an error, It's a macro, It's a reference, It's mutable");
    println!("    Answering: 'It's an error' (index 0) - WRONG");

    let feedback3 = engine
        .submit_answer(Answer::Choice(0))
        .expect("Failed to submit");
    print_feedback(&feedback3);

    // Finish quiz
    println!("\n--- Finishing Quiz ---\n");
    let score = engine.finish().expect("Failed to finish");

    println!("Final Score:");
    println!(
        "  - Points: {}/{}",
        score.points_earned, score.points_possible
    );
    println!("  - Percentage: {}%", (score.percentage * 100.0) as u32);
    println!(
        "  - Correct: {}/{}",
        score.correct_count, score.total_questions
    );
    println!(
        "  - Result: {}",
        if score.passed { "PASSED" } else { "FAILED" }
    );
    println!();
}

fn phase_3_review_results() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 3: Results Review     │");
    println!("└─────────────────────────────┘\n");

    println!("Jidoka Principle in Action:");
    println!("  - Each answer received IMMEDIATE feedback");
    println!("  - Wrong answers explained WHY they were wrong");
    println!("  - Learner can correct mental model in real-time");
    println!();

    println!("Quiz Engine Features:");
    println!("  - State machine: NotStarted -> InProgress -> Completed");
    println!("  - Multiple question types supported");
    println!("  - Automatic grading with explanations");
    println!("  - Progress tracking (answered/total)");
    println!("  - Attempt limiting for assessments");
    println!();

    println!("Toyota Way Alignment:");
    println!("  - Jidoka: Stop the line on errors (immediate feedback)");
    println!("  - Poka-Yoke: Structured formats prevent mistakes");
    println!("  - Kaizen: Explanations enable continuous improvement");
    println!();

    println!("Demo complete!");
}

fn create_rust_fundamentals_quiz() -> Quiz {
    Quiz::new("rust-basics", "Rust Fundamentals Quiz")
        .with_passing_score(0.7)
        .with_max_attempts(3)
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "What keyword declares an immutable variable in Rust?".into(),
            options: vec!["let".into(), "var".into(), "const".into(), "mut".into()],
            correct: 0,
            explanation: "In Rust, 'let' declares a variable. Variables are immutable by default."
                .into(),
            points: 10,
        })
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q2"),
            prompt: "Which is NOT a scalar type in Rust?".into(),
            options: vec!["i32".into(), "bool".into(), "String".into(), "char".into()],
            correct: 2,
            explanation:
                "String is a compound type (heap-allocated). Scalar types are i32, bool, char, f64."
                    .into(),
            points: 10,
        })
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q3"),
            prompt: "What does the '!' in println! indicate?".into(),
            options: vec![
                "It's an error".into(),
                "It's a macro".into(),
                "It's a reference".into(),
                "It's mutable".into(),
            ],
            correct: 1,
            explanation: "The '!' indicates a macro call. Macros are expanded at compile time."
                .into(),
            points: 10,
        })
}

fn print_feedback(feedback: &Feedback) {
    if feedback.correct {
        println!(
            "    [CORRECT] +{} points - {}",
            feedback.points_earned, feedback.explanation
        );
    } else {
        println!("    [WRONG] +0 points - {}", feedback.explanation);
    }
}

fn state_name(state: &QuizState) -> &'static str {
    match state {
        QuizState::NotStarted => "NotStarted",
        QuizState::InProgress { .. } => "InProgress",
        QuizState::Reviewing { .. } => "Reviewing",
        QuizState::Completed { .. } => "Completed",
    }
}
