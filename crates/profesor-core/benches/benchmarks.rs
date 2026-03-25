//! Criterion benchmarks for profesor-core.
//!
//! Benchmarks quiz creation, question scoring, progress tracking,
//! and the Vec2 physics primitives used in simulations.

#![allow(clippy::unwrap_used)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use profesor_core::{Answer, Course, CourseId, CourseLevel, LearnerProgress, Question, Quiz, QuizId, QuestionId};

fn make_quiz(n_questions: usize) -> Quiz {
    let mut quiz = Quiz::new(QuizId::new("bench-quiz"), "Benchmark Quiz")
        .with_passing_score(0.7);
    for i in 0..n_questions {
        quiz = quiz.with_question(Question::MultipleChoice {
            id: QuestionId::new(format!("q{i}")),
            prompt: format!("Question {i}: What is {i} + {i}?"),
            options: vec![
                format!("{}", i * 2),
                format!("{}", i * 2 + 1),
                format!("{}", i * 3),
                format!("{}", i + 1),
            ],
            correct: 0,
            explanation: format!("{i} + {i} = {}", i * 2),
            points: 10,
        });
    }
    quiz
}

fn bench_quiz_creation(c: &mut Criterion) {
    c.bench_function("quiz_create_10_questions", |b| {
        b.iter(|| black_box(make_quiz(10)));
    });
}

fn bench_quiz_creation_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("quiz_creation");
    for count in [5, 10, 25, 50] {
        group.bench_with_input(
            BenchmarkId::new("questions", count),
            &count,
            |b, &count| {
                b.iter(|| black_box(make_quiz(count)));
            },
        );
    }
    group.finish();
}

fn bench_quiz_total_points(c: &mut Criterion) {
    let quiz = make_quiz(50);
    c.bench_function("quiz_total_points_50q", |b| {
        b.iter(|| black_box(quiz.total_points()));
    });
}

fn bench_quiz_question_count(c: &mut Criterion) {
    let quiz = make_quiz(100);
    c.bench_function("quiz_question_count", |b| {
        b.iter(|| black_box(quiz.question_count()));
    });
}

fn bench_learner_progress_create(c: &mut Criterion) {
    c.bench_function("learner_progress_create", |b| {
        b.iter(|| {
            black_box(LearnerProgress::new(black_box("student-42")));
        });
    });
}

fn bench_course_creation(c: &mut Criterion) {
    c.bench_function("course_create", |b| {
        b.iter(|| {
            black_box(
                Course::new(
                    CourseId::new(black_box("rust-101")),
                    black_box("Introduction to Rust"),
                )
                .with_level(CourseLevel::Beginner),
            );
        });
    });
}

fn bench_quiz_yaml_roundtrip(c: &mut Criterion) {
    let quiz = make_quiz(10);
    let yaml = serde_yaml::to_string(&quiz).unwrap();
    c.bench_function("quiz_yaml_deserialize_10q", |b| {
        b.iter(|| {
            let _: Quiz = serde_yaml::from_str(black_box(&yaml)).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_quiz_creation,
    bench_quiz_creation_scaling,
    bench_quiz_total_points,
    bench_quiz_question_count,
    bench_learner_progress_create,
    bench_course_creation,
    bench_quiz_yaml_roundtrip,
);
criterion_main!(benches);
