//! Visualization configuration for simulations.
//!
//! Defines rendering configuration that the UI layer (Presentar) will use
//! to display simulations. This module is decoupled from the actual rendering
//! implementation - it only describes what should be rendered.
//!
//! Following Mieruka (visual management) principle: make the state visible at a glance.

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Configuration for rendering a simulation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenderConfig {
    /// Canvas width in logical pixels
    pub width: u32,
    /// Canvas height in logical pixels
    pub height: u32,
    /// Background color (hex)
    pub background_color: Color,
    /// Grid configuration
    pub grid: Option<GridConfig>,
    /// Entity rendering styles
    pub entity_styles: Vec<EntityStyle>,
    /// HUD elements to display
    pub hud_elements: Vec<HudElement>,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            background_color: Color::from_hex("#1a1a2e"),
            grid: None,
            entity_styles: Vec::new(),
            hud_elements: Vec::new(),
        }
    }
}

impl RenderConfig {
    /// Create a new render configuration with default values.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    /// Set the background color.
    #[must_use]
    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Add a grid overlay.
    #[must_use]
    pub fn with_grid(mut self, grid: GridConfig) -> Self {
        self.grid = Some(grid);
        self
    }

    /// Add an entity style.
    #[must_use]
    pub fn with_entity_style(mut self, style: EntityStyle) -> Self {
        self.entity_styles.push(style);
        self
    }

    /// Add a HUD element.
    #[must_use]
    pub fn with_hud(mut self, element: HudElement) -> Self {
        self.hud_elements.push(element);
        self
    }
}

/// Color representation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0-255)
    pub a: u8,
}

impl Color {
    /// Create a color from RGB values.
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a color from RGBA values.
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from hex string (e.g., "#ff0000" or "#ff0000ff").
    #[must_use]
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let len = hex.len();

        if len == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            Self::rgb(r, g, b)
        } else if len == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            Self::rgba(r, g, b, a)
        } else {
            Self::rgb(0, 0, 0)
        }
    }

    /// Convert to hex string.
    #[must_use]
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            alloc::format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            alloc::format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }

    /// Common colors
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    /// Black color
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    /// Red color
    pub const RED: Self = Self::rgb(255, 0, 0);
    /// Green color
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    /// Blue color
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    /// Transparent
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

/// Grid overlay configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridConfig {
    /// Cell size in pixels
    pub cell_size: u32,
    /// Grid line color
    pub color: Color,
    /// Line width
    pub line_width: f32,
    /// Show axis labels
    pub show_labels: bool,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            cell_size: 50,
            color: Color::rgba(255, 255, 255, 30),
            line_width: 1.0,
            show_labels: false,
        }
    }
}

/// Style for rendering an entity type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityStyle {
    /// Entity type/ID pattern to match
    pub pattern: String,
    /// Shape to render
    pub shape: Shape,
    /// Fill color
    pub fill: Color,
    /// Stroke color
    pub stroke: Color,
    /// Stroke width
    pub stroke_width: f32,
}

impl EntityStyle {
    /// Create a new entity style.
    #[must_use]
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            shape: Shape::Circle { radius: 10.0 },
            fill: Color::WHITE,
            stroke: Color::BLACK,
            stroke_width: 1.0,
        }
    }

    /// Set the shape.
    #[must_use]
    pub fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }

    /// Set the fill color.
    #[must_use]
    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill = color;
        self
    }

    /// Set the stroke.
    #[must_use]
    pub fn with_stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = color;
        self.stroke_width = width;
        self
    }
}

/// Shape definitions for entities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Shape {
    /// Circle with radius
    Circle {
        /// Radius in pixels
        radius: f32,
    },
    /// Rectangle with dimensions
    Rectangle {
        /// Width in pixels
        width: f32,
        /// Height in pixels
        height: f32,
    },
    /// Polygon with vertices (relative to center)
    Polygon {
        /// Vertex points
        vertices: Vec<(f32, f32)>,
    },
    /// Sprite/image reference
    Sprite {
        /// Image asset ID
        asset_id: String,
        /// Width
        width: f32,
        /// Height
        height: f32,
    },
}

/// HUD element to display.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HudElement {
    /// Unique ID
    pub id: String,
    /// Position on screen
    pub position: HudPosition,
    /// Element type
    pub element_type: HudElementType,
}

impl HudElement {
    /// Create a new HUD element.
    #[must_use]
    pub fn new(id: impl Into<String>, element_type: HudElementType) -> Self {
        Self {
            id: id.into(),
            position: HudPosition::TopLeft,
            element_type,
        }
    }

    /// Set the position.
    #[must_use]
    pub fn with_position(mut self, position: HudPosition) -> Self {
        self.position = position;
        self
    }
}

/// Position for HUD elements.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum HudPosition {
    /// Top-left corner
    #[default]
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
    /// Top center
    TopCenter,
    /// Bottom center
    BottomCenter,
}

/// Types of HUD elements.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HudElementType {
    /// Display a variable value
    VariableDisplay {
        /// Variable name to display
        variable: String,
        /// Label text
        label: String,
        /// Format string (use {} for value)
        format: String,
    },
    /// Progress bar
    ProgressBar {
        /// Variable for current value
        variable: String,
        /// Maximum value
        max: f64,
        /// Bar width
        width: u32,
        /// Bar color
        color: Color,
    },
    /// Text label
    Label {
        /// Static text
        text: String,
        /// Font size
        font_size: u32,
        /// Text color
        color: Color,
    },
    /// Timer display
    Timer {
        /// Format (e.g., "mm:ss")
        format: String,
    },
    /// Step counter
    StepCounter {
        /// Label
        label: String,
    },
}

/// A frame of animation data ready for rendering.
#[derive(Debug, Clone)]
pub struct RenderFrame {
    /// Entities to render
    pub entities: Vec<RenderEntity>,
    /// Current simulation time
    pub time: f64,
    /// Current step
    pub step: u32,
    /// Variable values for HUD
    pub variables: Vec<(String, String)>,
}

/// Entity data ready for rendering.
#[derive(Debug, Clone)]
pub struct RenderEntity {
    /// Entity ID
    pub id: String,
    /// X position
    pub x: f64,
    /// Y position
    pub y: f64,
    /// Rotation angle (radians)
    pub rotation: f64,
    /// Scale factor
    pub scale: f64,
}

impl RenderEntity {
    /// Create a new render entity.
    #[must_use]
    pub fn new(id: impl Into<String>, x: f64, y: f64) -> Self {
        Self {
            id: id.into(),
            x,
            y,
            rotation: 0.0,
            scale: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_config_default() {
        let config = RenderConfig::default();

        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
    }

    #[test]
    fn test_render_config_builder() {
        let config = RenderConfig::new(1024, 768)
            .with_background(Color::BLACK)
            .with_grid(GridConfig::default())
            .with_entity_style(EntityStyle::new("player"))
            .with_hud(HudElement::new(
                "score",
                HudElementType::Label {
                    text: "Score: 0".into(),
                    font_size: 16,
                    color: Color::WHITE,
                },
            ));

        assert_eq!(config.width, 1024);
        assert_eq!(config.height, 768);
        assert_eq!(config.background_color, Color::BLACK);
        assert!(config.grid.is_some());
        assert_eq!(config.entity_styles.len(), 1);
        assert_eq!(config.hud_elements.len(), 1);
    }

    #[test]
    fn test_color_rgb() {
        let color = Color::rgb(255, 128, 0);

        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_rgba() {
        let color = Color::rgba(255, 128, 0, 128);

        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#ff8000");

        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_color_from_hex_with_alpha() {
        let color = Color::from_hex("#ff800080");

        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::rgb(255, 128, 0);

        assert_eq!(color.to_hex(), "#ff8000");
    }

    #[test]
    fn test_color_to_hex_with_alpha() {
        let color = Color::rgba(255, 128, 0, 128);

        assert_eq!(color.to_hex(), "#ff800080");
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(Color::WHITE, Color::rgb(255, 255, 255));
        assert_eq!(Color::BLACK, Color::rgb(0, 0, 0));
        assert_eq!(Color::RED, Color::rgb(255, 0, 0));
        assert_eq!(Color::GREEN, Color::rgb(0, 255, 0));
        assert_eq!(Color::BLUE, Color::rgb(0, 0, 255));
    }

    #[test]
    fn test_grid_config_default() {
        let grid = GridConfig::default();

        assert_eq!(grid.cell_size, 50);
        assert_eq!(grid.line_width, 1.0);
        assert!(!grid.show_labels);
    }

    #[test]
    fn test_entity_style_builder() {
        let style = EntityStyle::new("ball")
            .with_shape(Shape::Circle { radius: 20.0 })
            .with_fill(Color::RED)
            .with_stroke(Color::BLACK, 2.0);

        assert_eq!(style.pattern, "ball");
        assert!(
            matches!(style.shape, Shape::Circle { radius } if (radius - 20.0).abs() < f32::EPSILON)
        );
        assert_eq!(style.fill, Color::RED);
        assert_eq!(style.stroke, Color::BLACK);
        assert!((style.stroke_width - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hud_element() {
        let element = HudElement::new(
            "timer",
            HudElementType::Timer {
                format: "mm:ss".into(),
            },
        )
        .with_position(HudPosition::TopRight);

        assert_eq!(element.id, "timer");
        assert_eq!(element.position, HudPosition::TopRight);
    }

    #[test]
    fn test_render_entity() {
        let entity = RenderEntity::new("player", 100.0, 200.0);

        assert_eq!(entity.id, "player");
        assert!((entity.x - 100.0).abs() < f64::EPSILON);
        assert!((entity.y - 200.0).abs() < f64::EPSILON);
        assert!((entity.rotation - 0.0).abs() < f64::EPSILON);
        assert!((entity.scale - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_shape_variants() {
        let circle = Shape::Circle { radius: 10.0 };
        let rect = Shape::Rectangle {
            width: 20.0,
            height: 30.0,
        };
        let polygon = Shape::Polygon {
            vertices: alloc::vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)],
        };
        let sprite = Shape::Sprite {
            asset_id: "player.png".into(),
            width: 32.0,
            height: 32.0,
        };

        assert!(matches!(circle, Shape::Circle { .. }));
        assert!(matches!(rect, Shape::Rectangle { .. }));
        assert!(matches!(polygon, Shape::Polygon { .. }));
        assert!(matches!(sprite, Shape::Sprite { .. }));
    }

    #[test]
    fn test_hud_variable_display() {
        let element = HudElementType::VariableDisplay {
            variable: "score".into(),
            label: "Score".into(),
            format: "Score: {}".into(),
        };

        assert!(matches!(element, HudElementType::VariableDisplay { .. }));
    }

    #[test]
    fn test_hud_progress_bar() {
        let element = HudElementType::ProgressBar {
            variable: "health".into(),
            max: 100.0,
            width: 200,
            color: Color::GREEN,
        };

        assert!(matches!(element, HudElementType::ProgressBar { .. }));
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_color_hex_roundtrip(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let original = Color::rgb(r, g, b);
            let hex = original.to_hex();
            let parsed = Color::from_hex(&hex);

            prop_assert_eq!(original, parsed);
        }

        #[test]
        fn test_render_entity_position(x in -1000.0f64..1000.0, y in -1000.0f64..1000.0) {
            let entity = RenderEntity::new("test", x, y);

            prop_assert!((entity.x - x).abs() < f64::EPSILON);
            prop_assert!((entity.y - y).abs() < f64::EPSILON);
        }
    }
}
