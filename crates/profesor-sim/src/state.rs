//! State machine simulations.
//!
//! Provides interactive state-based simulations for learning concepts.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use profesor_core::SimulationId;
use serde::{Deserialize, Serialize};

/// An interactive simulation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Simulation {
    /// Unique identifier
    pub id: SimulationId,
    /// Human-readable title
    pub title: String,
    /// Initial state
    pub initial_state: SimState,
    /// State transitions
    pub transitions: Vec<Transition>,
}

impl Simulation {
    /// Create a new simulation.
    #[must_use]
    pub fn new(id: impl Into<SimulationId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            initial_state: SimState::default(),
            transitions: Vec::new(),
        }
    }

    /// Set the initial state.
    #[must_use]
    pub fn with_initial_state(mut self, state: SimState) -> Self {
        self.initial_state = state;
        self
    }

    /// Add a transition.
    #[must_use]
    pub fn with_transition(mut self, transition: Transition) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Create a running instance of this simulation.
    #[must_use]
    pub fn instantiate(&self) -> SimulationInstance {
        SimulationInstance {
            simulation: self.clone(),
            current_state: self.initial_state.clone(),
            step_count: 0,
        }
    }
}

/// Current state of a simulation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SimState {
    /// Named variables
    pub variables: BTreeMap<String, Value>,
    /// Entities in the simulation
    pub entities: Vec<Entity>,
    /// Current time (simulation units)
    pub time: f64,
}

impl SimState {
    /// Create a new empty state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a variable.
    #[must_use]
    pub fn with_variable(mut self, name: impl Into<String>, value: Value) -> Self {
        self.variables.insert(name.into(), value);
        self
    }

    /// Add an entity.
    #[must_use]
    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    /// Get a variable value.
    #[must_use]
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    /// Set a variable value.
    pub fn set_variable(&mut self, name: impl Into<String>, value: Value) {
        self.variables.insert(name.into(), value);
    }

    /// Get an entity by ID.
    #[must_use]
    pub fn get_entity(&self, id: &str) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    /// Get a mutable entity by ID.
    pub fn get_entity_mut(&mut self, id: &str) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }
}

/// A value in the simulation state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    /// Boolean value
    Bool(bool),
    /// Integer value
    Int(i64),
    /// Floating point value
    Float(f64),
    /// String value
    String(String),
}

impl Value {
    /// Get as boolean.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as integer.
    #[must_use]
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Get as float.
    #[must_use]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
}

/// An entity in the simulation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    /// Unique ID
    pub id: String,
    /// Position
    pub position: Position,
    /// Custom properties
    pub properties: BTreeMap<String, Value>,
}

impl Entity {
    /// Create a new entity.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            position: Position::default(),
            properties: BTreeMap::new(),
        }
    }

    /// Set the position.
    #[must_use]
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = Position { x, y };
        self
    }

    /// Set a property.
    #[must_use]
    pub fn with_property(mut self, name: impl Into<String>, value: Value) -> Self {
        self.properties.insert(name.into(), value);
        self
    }
}

/// A 2D position.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub struct Position {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

/// A state transition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transition {
    /// What triggers this transition
    pub trigger: Trigger,
    /// Optional condition that must be true
    pub condition: Option<Condition>,
    /// Actions to perform
    pub actions: Vec<Action>,
}

impl Transition {
    /// Create a new transition.
    #[must_use]
    pub fn new(trigger: Trigger) -> Self {
        Self {
            trigger,
            condition: None,
            actions: Vec::new(),
        }
    }

    /// Add a condition.
    #[must_use]
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Add an action.
    #[must_use]
    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
}

/// Trigger types for transitions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Trigger {
    /// User clicks on a target
    UserClick {
        /// Target element ID
        target: String,
    },
    /// User drags an element
    UserDrag {
        /// Target element ID
        target: String,
    },
    /// Timer interval
    Timer {
        /// Interval in milliseconds
        interval_ms: u32,
    },
    /// Variable value changes
    StateChange {
        /// Variable name
        variable: String,
    },
}

/// Condition for a transition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Condition {
    /// Variable equals a value
    Equals {
        /// Variable name
        variable: String,
        /// Expected value
        value: Value,
    },
    /// Variable is greater than a value
    GreaterThan {
        /// Variable name
        variable: String,
        /// Threshold value
        value: f64,
    },
    /// Variable is less than a value
    LessThan {
        /// Variable name
        variable: String,
        /// Threshold value
        value: f64,
    },
}

/// Actions to perform on transition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    /// Set a variable
    SetVariable {
        /// Variable name
        name: String,
        /// New value
        value: Value,
    },
    /// Move an entity
    MoveEntity {
        /// Entity ID
        id: String,
        /// New position
        to: Position,
    },
    /// Show a message
    ShowMessage {
        /// Message text
        text: String,
    },
    /// Advance to next step
    AdvanceStep,
}

/// A running simulation instance.
#[derive(Debug, Clone)]
pub struct SimulationInstance {
    simulation: Simulation,
    current_state: SimState,
    step_count: u32,
}

impl SimulationInstance {
    /// Get the current state.
    #[must_use]
    pub fn state(&self) -> &SimState {
        &self.current_state
    }

    /// Get the step count.
    #[must_use]
    pub fn step_count(&self) -> u32 {
        self.step_count
    }

    /// Process a trigger and update state.
    pub fn process_trigger(&mut self, trigger: &Trigger) -> Vec<Action> {
        // First, collect actions to execute (avoiding borrow conflict)
        let actions_to_execute: Vec<Action> = self
            .simulation
            .transitions
            .iter()
            .filter(|t| self.trigger_matches(&t.trigger, trigger))
            .filter(|t| self.condition_satisfied(&t.condition))
            .flat_map(|t| t.actions.clone())
            .collect();

        // Now execute them
        for action in &actions_to_execute {
            self.execute_action(action);
        }

        actions_to_execute
    }

    /// Advance time in the simulation.
    pub fn advance_time(&mut self, delta: f64) {
        self.current_state.time += delta;
    }

    /// Reset to initial state.
    pub fn reset(&mut self) {
        self.current_state = self.simulation.initial_state.clone();
        self.step_count = 0;
    }

    fn trigger_matches(&self, defined: &Trigger, received: &Trigger) -> bool {
        match (defined, received) {
            (Trigger::UserClick { target: t1 }, Trigger::UserClick { target: t2 }) => t1 == t2,
            (Trigger::UserDrag { target: t1 }, Trigger::UserDrag { target: t2 }) => t1 == t2,
            (Trigger::Timer { .. }, Trigger::Timer { .. }) => true,
            (Trigger::StateChange { variable: v1 }, Trigger::StateChange { variable: v2 }) => {
                v1 == v2
            }
            _ => false,
        }
    }

    fn condition_satisfied(&self, condition: &Option<Condition>) -> bool {
        match condition {
            None => true,
            Some(Condition::Equals { variable, value }) => {
                self.current_state.get_variable(variable) == Some(value)
            }
            Some(Condition::GreaterThan { variable, value }) => self
                .current_state
                .get_variable(variable)
                .and_then(|v| v.as_float())
                .is_some_and(|v| v > *value),
            Some(Condition::LessThan { variable, value }) => self
                .current_state
                .get_variable(variable)
                .and_then(|v| v.as_float())
                .is_some_and(|v| v < *value),
        }
    }

    fn execute_action(&mut self, action: &Action) {
        match action {
            Action::SetVariable { name, value } => {
                self.current_state.set_variable(name.clone(), value.clone());
            }
            Action::MoveEntity { id, to } => {
                if let Some(entity) = self.current_state.get_entity_mut(id) {
                    entity.position = *to;
                }
            }
            Action::AdvanceStep => {
                self.step_count += 1;
            }
            Action::ShowMessage { .. } => {
                // UI would handle this
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_creation() {
        let sim = Simulation::new("sim-1", "Test Simulation");
        assert_eq!(sim.id.as_str(), "sim-1");
        assert_eq!(sim.title, "Test Simulation");
    }

    #[test]
    fn test_sim_state_variables() {
        let state = SimState::new()
            .with_variable("count", Value::Int(0))
            .with_variable("active", Value::Bool(true));

        assert_eq!(state.get_variable("count"), Some(&Value::Int(0)));
        assert_eq!(state.get_variable("active"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_entity() {
        let entity = Entity::new("player")
            .with_position(100.0, 200.0)
            .with_property("health", Value::Int(100));

        assert_eq!(entity.id, "player");
        assert_eq!(entity.position.x, 100.0);
    }

    #[test]
    fn test_simulation_instance() {
        let sim = Simulation::new("test", "Test")
            .with_initial_state(SimState::new().with_variable("counter", Value::Int(0)))
            .with_transition(
                Transition::new(Trigger::UserClick {
                    target: "button".into(),
                })
                .with_action(Action::SetVariable {
                    name: "counter".into(),
                    value: Value::Int(1),
                }),
            );

        let mut instance = sim.instantiate();

        // Process click
        let actions = instance.process_trigger(&Trigger::UserClick {
            target: "button".into(),
        });
        assert_eq!(actions.len(), 1);
        assert_eq!(
            instance.state().get_variable("counter"),
            Some(&Value::Int(1))
        );
    }

    #[test]
    fn test_conditional_transition() {
        let sim = Simulation::new("test", "Test")
            .with_initial_state(SimState::new().with_variable("score", Value::Int(100)))
            .with_transition(
                Transition::new(Trigger::UserClick {
                    target: "check".into(),
                })
                .with_condition(Condition::GreaterThan {
                    variable: "score".into(),
                    value: 50.0,
                })
                .with_action(Action::ShowMessage {
                    text: "Winner!".into(),
                }),
            );

        let mut instance = sim.instantiate();

        // Condition is met (100 > 50)
        let actions = instance.process_trigger(&Trigger::UserClick {
            target: "check".into(),
        });
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_value_conversions() {
        assert_eq!(Value::Bool(true).as_bool(), Some(true));
        assert_eq!(Value::Int(42).as_int(), Some(42));
        assert_eq!(Value::Float(3.14).as_float(), Some(3.14));
        assert_eq!(Value::Int(10).as_float(), Some(10.0));
    }

    #[test]
    fn test_advance_time() {
        let sim = Simulation::new("test", "Test");
        let mut instance = sim.instantiate();

        instance.advance_time(1.5);
        assert!((instance.state().time - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_reset() {
        let sim = Simulation::new("test", "Test")
            .with_initial_state(SimState::new().with_variable("x", Value::Int(0)));

        let mut instance = sim.instantiate();
        instance.current_state.set_variable("x", Value::Int(100));
        instance.step_count = 5;

        instance.reset();

        assert_eq!(instance.state().get_variable("x"), Some(&Value::Int(0)));
        assert_eq!(instance.step_count(), 0);
    }
}
