//! Physics simulations.
//!
//! Simple 2D physics for visual demonstrations.

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// A 2D vector.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub struct Vec2 {
    /// X component
    pub x: f32,
    /// Y component
    pub y: f32,
}

impl Vec2 {
    /// Zero vector.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// Create a new vector.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Calculate the magnitude (length) of the vector.
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        libm::sqrtf(self.x * self.x + self.y * self.y)
    }

    /// Normalize the vector (make it unit length).
    #[must_use]
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > f32::EPSILON {
            Self {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            Self::ZERO
        }
    }

    /// Dot product with another vector.
    #[must_use]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Scale the vector by a scalar.
    #[must_use]
    pub fn scale(&self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    /// Add another vector.
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtract another vector.
    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// A rigid body for physics simulation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RigidBody {
    /// Position
    pub position: Vec2,
    /// Velocity
    pub velocity: Vec2,
    /// Acceleration
    pub acceleration: Vec2,
    /// Mass in kg
    pub mass: f32,
    /// Restitution (bounciness, 0.0 - 1.0)
    pub restitution: f32,
    /// Radius for collision (circular body)
    pub radius: f32,
}

impl RigidBody {
    /// Create a new rigid body at a position.
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: 1.0,
            restitution: 0.8,
            radius: 10.0,
        }
    }

    /// Set the mass.
    #[must_use]
    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass.max(0.001); // Prevent zero/negative mass
        self
    }

    /// Set the restitution.
    #[must_use]
    pub fn with_restitution(mut self, restitution: f32) -> Self {
        self.restitution = restitution.clamp(0.0, 1.0);
        self
    }

    /// Set the radius.
    #[must_use]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius.max(0.1);
        self
    }

    /// Apply a force to the body.
    pub fn apply_force(&mut self, force: Vec2) {
        // F = ma, so a = F/m
        let accel = force.scale(1.0 / self.mass);
        self.acceleration = self.acceleration.add(&accel);
    }

    /// Apply an impulse (immediate velocity change).
    pub fn apply_impulse(&mut self, impulse: Vec2) {
        // J = m * delta_v, so delta_v = J/m
        let delta_v = impulse.scale(1.0 / self.mass);
        self.velocity = self.velocity.add(&delta_v);
    }

    /// Get kinetic energy.
    #[must_use]
    pub fn kinetic_energy(&self) -> f32 {
        let v_squared = self.velocity.dot(&self.velocity);
        0.5 * self.mass * v_squared
    }
}

/// A 2D physics world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsWorld {
    /// Bodies in the world
    pub bodies: Vec<RigidBody>,
    /// Gravity vector
    pub gravity: Vec2,
    /// Time step for simulation
    pub dt: f32,
    /// World bounds (min_x, min_y, max_x, max_y)
    pub bounds: Option<(f32, f32, f32, f32)>,
}

impl PhysicsWorld {
    /// Create a new physics world.
    #[must_use]
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            gravity: Vec2::new(0.0, 9.81),
            dt: 1.0 / 60.0, // 60 fps
            bounds: None,
        }
    }

    /// Set the gravity.
    #[must_use]
    pub fn with_gravity(mut self, gravity: Vec2) -> Self {
        self.gravity = gravity;
        self
    }

    /// Set the time step.
    #[must_use]
    pub fn with_dt(mut self, dt: f32) -> Self {
        self.dt = dt.max(0.001);
        self
    }

    /// Set world bounds.
    #[must_use]
    pub fn with_bounds(mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        self.bounds = Some((min_x, min_y, max_x, max_y));
        self
    }

    /// Add a body to the world.
    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    /// Step the simulation forward.
    pub fn step(&mut self) {
        // Apply gravity and integrate
        for body in &mut self.bodies {
            // Apply gravity
            let gravity_force = self.gravity.scale(body.mass);
            body.apply_force(gravity_force);

            // Integrate velocity
            body.velocity = body.velocity.add(&body.acceleration.scale(self.dt));

            // Integrate position
            body.position = body.position.add(&body.velocity.scale(self.dt));

            // Reset acceleration
            body.acceleration = Vec2::ZERO;
        }

        // Handle bounds collisions
        if let Some((min_x, min_y, max_x, max_y)) = self.bounds {
            for body in &mut self.bodies {
                // Left bound
                if body.position.x - body.radius < min_x {
                    body.position.x = min_x + body.radius;
                    body.velocity.x = -body.velocity.x * body.restitution;
                }
                // Right bound
                if body.position.x + body.radius > max_x {
                    body.position.x = max_x - body.radius;
                    body.velocity.x = -body.velocity.x * body.restitution;
                }
                // Top bound
                if body.position.y - body.radius < min_y {
                    body.position.y = min_y + body.radius;
                    body.velocity.y = -body.velocity.y * body.restitution;
                }
                // Bottom bound
                if body.position.y + body.radius > max_y {
                    body.position.y = max_y - body.radius;
                    body.velocity.y = -body.velocity.y * body.restitution;
                }
            }
        }

        // Body-body collisions (simplified)
        self.resolve_collisions();
    }

    /// Resolve collisions between bodies.
    fn resolve_collisions(&mut self) {
        let len = self.bodies.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if self.check_collision(i, j) {
                    self.resolve_collision(i, j);
                }
            }
        }
    }

    /// Check if two bodies are colliding.
    fn check_collision(&self, i: usize, j: usize) -> bool {
        let a = &self.bodies[i];
        let b = &self.bodies[j];

        let diff = a.position.sub(&b.position);
        let dist_sq = diff.dot(&diff);
        let min_dist = a.radius + b.radius;

        dist_sq < min_dist * min_dist
    }

    /// Resolve a collision between two bodies.
    fn resolve_collision(&mut self, i: usize, j: usize) {
        // Get positions and velocities
        let (a_pos, a_vel, a_mass, a_rest, a_radius) = {
            let a = &self.bodies[i];
            (a.position, a.velocity, a.mass, a.restitution, a.radius)
        };
        let (b_pos, b_vel, b_mass, b_rest, b_radius) = {
            let b = &self.bodies[j];
            (b.position, b.velocity, b.mass, b.restitution, b.radius)
        };

        // Normal vector from a to b
        let normal = b_pos.sub(&a_pos).normalize();

        // Relative velocity
        let rel_vel = a_vel.sub(&b_vel);
        let vel_along_normal = rel_vel.dot(&normal);

        // Don't resolve if velocities are separating
        if vel_along_normal > 0.0 {
            return;
        }

        // Combined restitution
        let e = (a_rest + b_rest) / 2.0;

        // Impulse magnitude
        let impulse_mag = -(1.0 + e) * vel_along_normal / (1.0 / a_mass + 1.0 / b_mass);

        // Apply impulses
        let impulse = normal.scale(impulse_mag);

        self.bodies[i].velocity = a_vel.sub(&impulse.scale(1.0 / a_mass));
        self.bodies[j].velocity = b_vel.add(&impulse.scale(1.0 / b_mass));

        // Separate overlapping bodies
        let diff = b_pos.sub(&a_pos);
        let dist = diff.magnitude();
        let overlap = (a_radius + b_radius) - dist;

        if overlap > 0.0 {
            let separation = normal.scale(overlap / 2.0);
            self.bodies[i].position = a_pos.sub(&separation);
            self.bodies[j].position = b_pos.add(&separation);
        }
    }

    /// Get the number of bodies.
    #[must_use]
    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }

    /// Get total kinetic energy in the system.
    #[must_use]
    pub fn total_kinetic_energy(&self) -> f32 {
        self.bodies.iter().map(|b| b.kinetic_energy()).sum()
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_operations() {
        let a = Vec2::new(3.0, 4.0);
        let b = Vec2::new(1.0, 2.0);

        assert!((a.magnitude() - 5.0).abs() < f32::EPSILON);
        assert_eq!(a.add(&b), Vec2::new(4.0, 6.0));
        assert_eq!(a.sub(&b), Vec2::new(2.0, 2.0));
        assert_eq!(a.scale(2.0), Vec2::new(6.0, 8.0));
        assert_eq!(a.dot(&b), 11.0);
    }

    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalize();
        assert!((n.magnitude() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_rigid_body_creation() {
        let body = RigidBody::new(10.0, 20.0)
            .with_mass(2.0)
            .with_restitution(0.5);

        assert_eq!(body.position, Vec2::new(10.0, 20.0));
        assert_eq!(body.mass, 2.0);
        assert_eq!(body.restitution, 0.5);
    }

    #[test]
    fn test_apply_force() {
        let mut body = RigidBody::new(0.0, 0.0).with_mass(2.0);
        body.apply_force(Vec2::new(10.0, 0.0));

        // F = ma, so a = F/m = 10/2 = 5
        assert_eq!(body.acceleration.x, 5.0);
    }

    #[test]
    fn test_kinetic_energy() {
        let mut body = RigidBody::new(0.0, 0.0).with_mass(2.0);
        body.velocity = Vec2::new(3.0, 4.0); // |v| = 5

        // KE = 0.5 * m * v^2 = 0.5 * 2 * 25 = 25
        assert!((body.kinetic_energy() - 25.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_physics_world_step() {
        let mut world = PhysicsWorld::new()
            .with_gravity(Vec2::new(0.0, 10.0))
            .with_dt(0.1);

        world.add_body(RigidBody::new(0.0, 0.0));

        let initial_y = world.bodies[0].position.y;
        world.step();
        let after_y = world.bodies[0].position.y;

        // Body should have moved down due to gravity
        assert!(after_y > initial_y);
    }

    #[test]
    fn test_bounds_collision() {
        let mut world = PhysicsWorld::new()
            .with_gravity(Vec2::ZERO)
            .with_dt(0.1)
            .with_bounds(0.0, 0.0, 100.0, 100.0);

        let mut body = RigidBody::new(5.0, 50.0).with_radius(10.0);
        body.velocity = Vec2::new(-100.0, 0.0);
        world.add_body(body);

        world.step();

        // Body should have bounced off left wall
        assert!(world.bodies[0].velocity.x > 0.0);
    }

    #[test]
    fn test_total_kinetic_energy() {
        let mut world = PhysicsWorld::new();

        let mut body1 = RigidBody::new(0.0, 0.0).with_mass(1.0);
        body1.velocity = Vec2::new(2.0, 0.0);

        let mut body2 = RigidBody::new(50.0, 0.0).with_mass(1.0);
        body2.velocity = Vec2::new(3.0, 0.0);

        world.add_body(body1);
        world.add_body(body2);

        // KE = 0.5*1*4 + 0.5*1*9 = 2 + 4.5 = 6.5
        assert!((world.total_kinetic_energy() - 6.5).abs() < 0.001);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_normalize_gives_unit_length(x in -100.0f32..100.0, y in -100.0f32..100.0) {
            prop_assume!(x.abs() > 0.01 || y.abs() > 0.01);
            let v = Vec2::new(x, y);
            let n = v.normalize();
            prop_assert!((n.magnitude() - 1.0).abs() < 0.001);
        }

        #[test]
        fn test_dot_product_commutative(
            x1 in -10.0f32..10.0, y1 in -10.0f32..10.0,
            x2 in -10.0f32..10.0, y2 in -10.0f32..10.0
        ) {
            let a = Vec2::new(x1, y1);
            let b = Vec2::new(x2, y2);
            prop_assert!((a.dot(&b) - b.dot(&a)).abs() < 0.0001);
        }

        #[test]
        fn test_kinetic_energy_non_negative(
            mass in 0.1f32..100.0,
            vx in -10.0f32..10.0,
            vy in -10.0f32..10.0
        ) {
            let mut body = RigidBody::new(0.0, 0.0).with_mass(mass);
            body.velocity = Vec2::new(vx, vy);
            prop_assert!(body.kinetic_energy() >= 0.0);
        }
    }
}
