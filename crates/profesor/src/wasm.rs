//! WASM exports for browser integration.
//!
//! Provides `#[no_mangle]` exports that can be called from the browser
//! via the WebAssembly API. No JavaScript glue code required.
//!
//! ## Memory Management
//!
//! WASM linear memory is used for passing data. The browser allocates
//! memory via `alloc_bytes` and frees it via `free_bytes`.
//!
//! ## Usage from Browser
//!
//! ```javascript
//! // Pure WebAssembly API - no wasm-bindgen
//! const { instance } = await WebAssembly.instantiateStreaming(
//!     fetch('profesor.wasm'),
//!     { env: { /* imports if needed */ } }
//! );
//!
//! const { exports } = instance;
//! const version = exports.get_version();
//! ```
//!
//! ## Safety
//!
//! All FFI functions that take raw pointers require the caller to ensure
//! the pointers are valid. This is documented on each function.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::{App, Quiz, QuizEngine};

// =============================================================================
// Memory Management
// =============================================================================

/// Allocate bytes in WASM memory.
///
/// Returns a pointer to the allocated memory.
#[no_mangle]
pub extern "C" fn alloc_bytes(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

/// Free previously allocated bytes.
///
/// # Safety
///
/// The caller must ensure `ptr` was allocated by `alloc_bytes` with `len` bytes.
#[no_mangle]
pub extern "C" fn free_bytes(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        // SAFETY: Caller guarantees ptr was allocated by alloc_bytes
        let _ = unsafe { Vec::from_raw_parts(ptr, 0, len) };
    }
}

// =============================================================================
// Version Info
// =============================================================================

/// Get the library version as a static string pointer.
///
/// Returns: pointer to null-terminated version string
#[no_mangle]
pub extern "C" fn get_version() -> *const u8 {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr()
}

/// Check if this is a WASM build.
///
/// Returns: 1 if WASM, 0 otherwise
#[no_mangle]
pub extern "C" fn is_wasm_build() -> u32 {
    if cfg!(target_arch = "wasm32") {
        1
    } else {
        0
    }
}

// =============================================================================
// Application State
// =============================================================================

/// Application handle (opaque pointer).
pub type AppHandle = *mut App;

/// Create a new application instance.
///
/// Returns: handle to the app (must be freed with `app_destroy`)
#[no_mangle]
pub extern "C" fn app_create() -> AppHandle {
    Box::into_raw(Box::new(App::new()))
}

/// Destroy an application instance.
///
/// # Safety
///
/// Handle must have been created by `app_create`.
#[no_mangle]
pub extern "C" fn app_destroy(handle: AppHandle) {
    if !handle.is_null() {
        // SAFETY: Caller guarantees handle is valid
        let _ = unsafe { Box::from_raw(handle) };
    }
}

/// Get the current view ID.
///
/// Returns: view ID (0=CourseList, 1=CourseDetail, etc.)
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn app_get_view(handle: AppHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }

    // SAFETY: Caller guarantees handle is valid
    let app = unsafe { &*handle };

    match app.state().current_view {
        crate::app::View::CourseList => 0,
        crate::app::View::CourseDetail => 1,
        crate::app::View::Module => 2,
        crate::app::View::Lesson => 3,
        crate::app::View::Quiz => 4,
        crate::app::View::Lab => 5,
        crate::app::View::Simulation => 6,
        crate::app::View::Profile => 7,
    }
}

/// Get the number of loaded courses.
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn app_course_count(handle: AppHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }

    // SAFETY: Caller guarantees handle is valid
    let app = unsafe { &*handle };
    app.state().courses.len() as u32
}

/// Navigate to a view.
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn app_navigate(handle: AppHandle, view_id: u32) {
    if handle.is_null() {
        return;
    }

    // SAFETY: Caller guarantees handle is valid
    let app = unsafe { &mut *handle };

    let view = match view_id {
        0 => crate::app::View::CourseList,
        1 => crate::app::View::CourseDetail,
        2 => crate::app::View::Module,
        3 => crate::app::View::Lesson,
        4 => crate::app::View::Quiz,
        5 => crate::app::View::Lab,
        6 => crate::app::View::Simulation,
        7 => crate::app::View::Profile,
        _ => return,
    };

    app.state_mut().navigate(view);
}

// =============================================================================
// Quiz Engine
// =============================================================================

/// Quiz engine handle.
pub type QuizHandle = *mut QuizEngine;

/// Create a quiz engine with a sample quiz.
///
/// Returns: handle to quiz engine (must be freed with `quiz_destroy`)
#[no_mangle]
pub extern "C" fn quiz_create_sample() -> QuizHandle {
    use crate::{Question, QuestionId};

    let quiz = Quiz::new("sample", "Sample Quiz")
        .with_passing_score(0.7)
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q1"),
            prompt: "What is 2 + 2?".into(),
            options: alloc::vec!["3".into(), "4".into(), "5".into()],
            correct: 1,
            explanation: "2 + 2 = 4".into(),
            points: 10,
        })
        .with_question(Question::MultipleChoice {
            id: QuestionId::new("q2"),
            prompt: "What is the capital of France?".into(),
            options: alloc::vec!["London".into(), "Berlin".into(), "Paris".into()],
            correct: 2,
            explanation: "Paris is the capital of France.".into(),
            points: 10,
        });

    Box::into_raw(Box::new(QuizEngine::new(quiz)))
}

/// Destroy a quiz engine.
///
/// # Safety
///
/// Handle must have been created by `quiz_create_*`.
#[no_mangle]
pub extern "C" fn quiz_destroy(handle: QuizHandle) {
    if !handle.is_null() {
        // SAFETY: Caller guarantees handle is valid
        let _ = unsafe { Box::from_raw(handle) };
    }
}

/// Start the quiz.
///
/// Returns: 0 on success, error code on failure
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn quiz_start(handle: QuizHandle) -> i32 {
    if handle.is_null() {
        return -1;
    }

    // SAFETY: Caller guarantees handle is valid
    let engine = unsafe { &mut *handle };

    match engine.start() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Get the current question index (0-based).
///
/// Returns: question index, or -1 if not in progress
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn quiz_current_index(handle: QuizHandle) -> i32 {
    if handle.is_null() {
        return -1;
    }

    // SAFETY: Caller guarantees handle is valid
    let engine = unsafe { &*handle };

    match engine.state() {
        crate::QuizState::InProgress {
            current_question, ..
        } => *current_question as i32,
        _ => -1,
    }
}

/// Get the total number of questions.
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn quiz_question_count(handle: QuizHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }

    // SAFETY: Caller guarantees handle is valid
    let engine = unsafe { &*handle };
    engine.quiz().question_count() as u32
}

/// Submit an answer (choice index for multiple choice).
///
/// Returns: 1 if correct, 0 if incorrect, -1 on error
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn quiz_submit_choice(handle: QuizHandle, choice: u32) -> i32 {
    if handle.is_null() {
        return -1;
    }

    // SAFETY: Caller guarantees handle is valid
    let engine = unsafe { &mut *handle };

    match engine.submit_answer(crate::Answer::Choice(choice as usize)) {
        Ok(feedback) => {
            if feedback.correct {
                1
            } else {
                0
            }
        }
        Err(_) => -1,
    }
}

/// Move to the next question.
///
/// Returns: 0 on success, -1 on error
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn quiz_next(handle: QuizHandle) -> i32 {
    if handle.is_null() {
        return -1;
    }

    // SAFETY: Caller guarantees handle is valid
    let engine = unsafe { &mut *handle };

    match engine.next_question() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Finish the quiz and get the score percentage.
///
/// Returns: score as percentage (0-100), or -1 on error
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn quiz_finish(handle: QuizHandle) -> i32 {
    if handle.is_null() {
        return -1;
    }

    // SAFETY: Caller guarantees handle is valid
    let engine = unsafe { &mut *handle };

    match engine.finish() {
        Ok(score) => (score.percentage * 100.0) as i32,
        Err(_) => -1,
    }
}

/// Get quiz progress (0.0 - 1.0 as fixed point * 100).
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn quiz_progress(handle: QuizHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }

    // SAFETY: Caller guarantees handle is valid
    let engine = unsafe { &*handle };
    (engine.progress() * 100.0) as u32
}

// =============================================================================
// Physics Simulation
// =============================================================================

/// Physics world handle.
pub type PhysicsHandle = *mut crate::PhysicsWorld;

/// Create a physics world.
///
/// Returns: handle (must be freed with `physics_destroy`)
#[no_mangle]
pub extern "C" fn physics_create() -> PhysicsHandle {
    Box::into_raw(Box::new(crate::PhysicsWorld::new()))
}

/// Destroy a physics world.
///
/// # Safety
///
/// Handle must have been created by `physics_create`.
#[no_mangle]
pub extern "C" fn physics_destroy(handle: PhysicsHandle) {
    if !handle.is_null() {
        // SAFETY: Caller guarantees handle is valid
        let _ = unsafe { Box::from_raw(handle) };
    }
}

/// Set world bounds.
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn physics_set_bounds(
    handle: PhysicsHandle,
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
) {
    if handle.is_null() {
        return;
    }

    // SAFETY: Caller guarantees handle is valid
    let world = unsafe { &mut *handle };
    world.bounds = Some((min_x, min_y, max_x, max_y));
}

/// Add a body to the world.
///
/// Returns: body index
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn physics_add_body(handle: PhysicsHandle, x: f32, y: f32, radius: f32) -> u32 {
    if handle.is_null() {
        return u32::MAX;
    }

    // SAFETY: Caller guarantees handle is valid
    let world = unsafe { &mut *handle };

    let body = crate::RigidBody::new(x, y).with_radius(radius);
    world.add_body(body);

    (world.body_count() - 1) as u32
}

/// Step the physics simulation.
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn physics_step(handle: PhysicsHandle) {
    if handle.is_null() {
        return;
    }

    // SAFETY: Caller guarantees handle is valid
    let world = unsafe { &mut *handle };
    world.step();
}

/// Get body X position.
///
/// # Safety
///
/// Handle and index must be valid.
#[no_mangle]
pub extern "C" fn physics_body_x(handle: PhysicsHandle, index: u32) -> f32 {
    if handle.is_null() {
        return 0.0;
    }

    // SAFETY: Caller guarantees handle is valid
    let world = unsafe { &*handle };

    world
        .bodies
        .get(index as usize)
        .map(|b| b.position.x)
        .unwrap_or(0.0)
}

/// Get body Y position.
///
/// # Safety
///
/// Handle and index must be valid.
#[no_mangle]
pub extern "C" fn physics_body_y(handle: PhysicsHandle, index: u32) -> f32 {
    if handle.is_null() {
        return 0.0;
    }

    // SAFETY: Caller guarantees handle is valid
    let world = unsafe { &*handle };

    world
        .bodies
        .get(index as usize)
        .map(|b| b.position.y)
        .unwrap_or(0.0)
}

/// Get number of bodies.
///
/// # Safety
///
/// Handle must be valid.
#[no_mangle]
pub extern "C" fn physics_body_count(handle: PhysicsHandle) -> u32 {
    if handle.is_null() {
        return 0;
    }

    // SAFETY: Caller guarantees handle is valid
    let world = unsafe { &*handle };
    world.body_count() as u32
}

// =============================================================================
// Tests (only compiled for native tests, not WASM)
// =============================================================================

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let ptr = get_version();
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_app_lifecycle() {
        let handle = app_create();
        assert!(!handle.is_null());

        assert_eq!(app_get_view(handle), 0); // CourseList
        assert_eq!(app_course_count(handle), 0);

        app_navigate(handle, 7); // Profile
        assert_eq!(app_get_view(handle), 7);

        app_destroy(handle);
    }

    #[test]
    fn test_quiz_lifecycle() {
        let handle = quiz_create_sample();
        assert!(!handle.is_null());

        assert_eq!(quiz_question_count(handle), 2);
        assert_eq!(quiz_current_index(handle), -1); // Not started

        assert_eq!(quiz_start(handle), 0);
        assert_eq!(quiz_current_index(handle), 0);

        // Submit correct answer (index 1 = "4")
        let result = quiz_submit_choice(handle, 1);
        assert_eq!(result, 1); // Correct

        // Move to next
        assert_eq!(quiz_next(handle), 0);
        assert_eq!(quiz_current_index(handle), 1);

        // Submit correct answer (index 2 = "Paris")
        let result = quiz_submit_choice(handle, 2);
        assert_eq!(result, 1); // Correct

        // Finish
        let score = quiz_finish(handle);
        assert_eq!(score, 100); // 100%

        quiz_destroy(handle);
    }

    #[test]
    fn test_physics_lifecycle() {
        let handle = physics_create();
        assert!(!handle.is_null());

        physics_set_bounds(handle, 0.0, 0.0, 800.0, 600.0);

        let idx = physics_add_body(handle, 100.0, 100.0, 10.0);
        assert_eq!(idx, 0);
        assert_eq!(physics_body_count(handle), 1);

        let x = physics_body_x(handle, 0);
        assert!((x - 100.0).abs() < f32::EPSILON);

        physics_step(handle);

        // Body should have moved due to gravity
        let y_after = physics_body_y(handle, 0);
        assert!(y_after > 100.0);

        physics_destroy(handle);
    }

    #[test]
    fn test_memory_alloc_free() {
        let ptr = alloc_bytes(1024);
        assert!(!ptr.is_null());
        free_bytes(ptr, 1024);
    }

    #[test]
    fn test_null_handles() {
        // These should not panic
        app_destroy(core::ptr::null_mut());
        quiz_destroy(core::ptr::null_mut());
        physics_destroy(core::ptr::null_mut());

        assert_eq!(app_get_view(core::ptr::null_mut()), 0);
        assert_eq!(quiz_current_index(core::ptr::null_mut()), -1);
        assert_eq!(physics_body_count(core::ptr::null_mut()), 0);
    }
}
