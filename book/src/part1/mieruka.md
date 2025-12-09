# Mieruka: Visual Management

Mieruka means "making things visible" - using visual tools to understand status at a glance.

## In Profesor

- Physics simulations visualize abstract concepts
- Progress bars show course completion
- Quiz results provide visual feedback
- Real-time rendering of simulation state

## Physics Visualization

```rust
let world = PhysicsWorld::new()
    .with_gravity(Vec2::new(0.0, 9.81))
    .with_bounds(0.0, 0.0, 800.0, 600.0);

world.add_body(RigidBody::new(100.0, 100.0));
world.step(); // Animate and visualize
```
