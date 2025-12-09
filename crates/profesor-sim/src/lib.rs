//! # Profesor Simulations
//!
//! Interactive simulations for visual learning.
//!
//! Provides state machine and physics-based simulations
//! for hands-on understanding of complex concepts.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

mod physics;
mod render;
mod state;

pub use physics::{PhysicsWorld, RigidBody, Vec2};
pub use render::{
    Color, EntityStyle, GridConfig, HudElement, HudElementType, HudPosition, RenderConfig,
    RenderEntity, RenderFrame, Shape,
};
pub use state::{Action, SimState, Simulation, Transition, Trigger};
