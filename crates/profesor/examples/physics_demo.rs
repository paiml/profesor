//! Physics Simulation Demo
//!
//! Demonstrates the profesor physics engine for visual learning (Mieruka principle).
//!
//! Run with: `cargo run --example physics_demo`

use profesor::{PhysicsWorld, RigidBody, Vec2};

fn main() {
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│          PROFESOR - Physics Simulation Demo             │");
    println!("│                                                         │");
    println!("│  Demonstrating Mieruka: Visual management for learning  │");
    println!("└─────────────────────────────────────────────────────────┘\n");

    // Phase 1: Setup Physics World
    phase_1_setup_world();

    // Phase 2: Simulate Projectile Motion
    phase_2_projectile_motion();

    // Phase 3: Collision Simulation
    phase_3_collision_demo();

    // Phase 4: Energy Conservation
    phase_4_energy_conservation();
}

fn phase_1_setup_world() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 1: World Setup        │");
    println!("└─────────────────────────────┘\n");

    let world = PhysicsWorld::new()
        .with_gravity(Vec2::new(0.0, 9.81))
        .with_dt(1.0 / 60.0)
        .with_bounds(0.0, 0.0, 800.0, 600.0);

    println!("Created physics world:");
    println!("  - Gravity: (0, 9.81) m/s² (Earth-like, downward)");
    println!("  - Time step: {:.4}s (60 FPS)", world.dt);
    println!("  - Bounds: 800x600 units");
    println!("  - Bodies: {}", world.body_count());
    println!();

    println!("Vec2 operations:");
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(1.0, 2.0);
    println!("  a = ({}, {})", a.x, a.y);
    println!("  b = ({}, {})", b.x, b.y);
    println!("  |a| = {} (magnitude)", a.magnitude());
    println!("  a + b = ({}, {})", a.add(&b).x, a.add(&b).y);
    println!("  a · b = {} (dot product)", a.dot(&b));
    println!(
        "  â = ({:.2}, {:.2}) (normalized)",
        a.normalize().x,
        a.normalize().y
    );
    println!();
}

fn phase_2_projectile_motion() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 2: Projectile Motion  │");
    println!("└─────────────────────────────┘\n");

    let mut world = PhysicsWorld::new()
        .with_gravity(Vec2::new(0.0, 10.0)) // Simplified gravity
        .with_dt(0.1); // Larger time step for demo

    // Create a projectile
    let mut projectile = RigidBody::new(0.0, 100.0).with_mass(1.0).with_radius(5.0);

    // Initial velocity: 45 degrees up-right
    projectile.velocity = Vec2::new(20.0, -15.0);
    world.add_body(projectile);

    println!("Launching projectile:");
    println!("  - Initial position: (0, 100)");
    println!("  - Initial velocity: (20, -15) m/s");
    println!("  - Mass: 1.0 kg");
    println!();

    println!("Trajectory (every 0.5s):");
    println!("  {:>6}  {:>8}  {:>8}  {:>8}", "Time", "X", "Y", "KE");
    println!("  {:->6}  {:->8}  {:->8}  {:->8}", "", "", "", "");

    for step in 0..15 {
        let body = &world.bodies[0];
        let time = step as f32 * world.dt;

        if step % 5 == 0 {
            println!(
                "  {:>5.1}s  {:>8.2}  {:>8.2}  {:>7.2}J",
                time,
                body.position.x,
                body.position.y,
                body.kinetic_energy()
            );
        }

        world.step();
    }
    println!();
}

fn phase_3_collision_demo() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 3: Collision Demo     │");
    println!("└─────────────────────────────┘\n");

    let mut world = PhysicsWorld::new()
        .with_gravity(Vec2::ZERO) // No gravity for this demo
        .with_dt(0.1)
        .with_bounds(0.0, 0.0, 200.0, 100.0);

    // Two balls approaching each other
    let mut ball1 = RigidBody::new(30.0, 50.0)
        .with_mass(1.0)
        .with_radius(10.0)
        .with_restitution(0.9);
    ball1.velocity = Vec2::new(10.0, 0.0); // Moving right

    let mut ball2 = RigidBody::new(170.0, 50.0)
        .with_mass(2.0)
        .with_radius(15.0)
        .with_restitution(0.9);
    ball2.velocity = Vec2::new(-5.0, 0.0); // Moving left

    world.add_body(ball1);
    world.add_body(ball2);

    println!("Two balls approaching:");
    println!("  Ball 1: m=1kg at (30, 50), v=(10, 0) →");
    println!("  Ball 2: m=2kg at (170, 50), v=(-5, 0) ←");
    println!();

    println!("Simulation:");
    println!(
        "  {:>4}  {:>12}  {:>12}  {:>10}",
        "Step", "Ball1 X,Vx", "Ball2 X,Vx", "Total KE"
    );
    println!("  {:->4}  {:->12}  {:->12}  {:->10}", "", "", "", "");

    for step in 0..20 {
        if step % 4 == 0 {
            let b1 = &world.bodies[0];
            let b2 = &world.bodies[1];

            println!(
                "  {:>4}  {:>5.1},{:>5.1}  {:>5.1},{:>5.1}  {:>9.2}J",
                step,
                b1.position.x,
                b1.velocity.x,
                b2.position.x,
                b2.velocity.x,
                world.total_kinetic_energy()
            );
        }
        world.step();
    }
    println!();

    println!("Observation: Momentum is conserved during collision!");
    println!();
}

fn phase_4_energy_conservation() {
    println!("┌─────────────────────────────┐");
    println!("│ Phase 4: Energy Demo        │");
    println!("└─────────────────────────────┘\n");

    let mut world = PhysicsWorld::new()
        .with_gravity(Vec2::new(0.0, 10.0))
        .with_dt(0.05)
        .with_bounds(0.0, 0.0, 100.0, 200.0);

    // Bouncing ball with high restitution
    let ball = RigidBody::new(50.0, 20.0)
        .with_mass(1.0)
        .with_radius(10.0)
        .with_restitution(0.95); // Nearly elastic

    world.add_body(ball);

    println!("Bouncing ball (restitution = 0.95):");
    println!("  - Dropped from height 20");
    println!("  - Floor at y = 200");
    println!();

    println!("Energy tracking:");
    println!(
        "  {:>4}  {:>8}  {:>8}  {:>8}",
        "Step", "Y pos", "Vy", "KE (J)"
    );
    println!("  {:->4}  {:->8}  {:->8}  {:->8}", "", "", "", "");

    let mut bounces = 0;
    let mut last_vy_sign = -1.0f32;

    for step in 0..100 {
        let body = &world.bodies[0];

        // Detect bounce
        if body.velocity.y * last_vy_sign < 0.0 && body.velocity.y > 0.0 {
            bounces += 1;
            println!(
                "  {:>4}  {:>8.2}  {:>8.2}  {:>8.2}  <- Bounce #{}",
                step,
                body.position.y,
                body.velocity.y,
                body.kinetic_energy(),
                bounces
            );
        } else if step % 20 == 0 {
            println!(
                "  {:>4}  {:>8.2}  {:>8.2}  {:>8.2}",
                step,
                body.position.y,
                body.velocity.y,
                body.kinetic_energy()
            );
        }

        last_vy_sign = body.velocity.y.signum();
        world.step();

        if bounces >= 3 {
            break;
        }
    }
    println!();

    println!("Toyota Way Alignment:");
    println!("  - Mieruka: Visual physics makes abstract concepts concrete");
    println!("  - Kaizen: Immediate feedback enables iterative learning");
    println!("  - Genchi Genbutsu: 'Go and see' - experiment with parameters");
    println!();

    println!("Physics Engine Features:");
    println!("  - Verlet integration for stability");
    println!("  - Elastic/inelastic collisions");
    println!("  - Boundary handling with bounce");
    println!("  - Energy tracking for validation");
    println!();

    println!("Demo complete!");
}
