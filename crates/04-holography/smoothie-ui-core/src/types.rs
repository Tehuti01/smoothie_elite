/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x55495479 | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-ui-core/src/types.rs                │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Core UI primitives and layout types.                        │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

/// RGBA color representation using f32 components.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}

/// A 2D point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// A 2D rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Spacing primitive for padding and margins.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Spacing {
    pub const fn uniform(v: f32) -> Self {
        Self { top: v, right: v, bottom: v, left: v }
    }
}

pub type Padding = Spacing;
pub type Margin = Spacing;

/// UI Alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

/// Font size representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontSize(pub f32);

/// Global UI Theme configuration.
#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub text: Color,
    pub font_size_normal: FontSize,
}

/// Seraphic Dark Theme (Standard)
pub const DARK_THEME: Theme = Theme {
    background: Color::rgb(0.05, 0.05, 0.05),
    primary: Color::rgb(0.1, 0.1, 0.1),
    secondary: Color::rgb(0.2, 0.2, 0.2),
    accent: Color::rgb(0.0, 0.8, 1.0),
    text: Color::rgb(0.9, 0.9, 0.9),
    font_size_normal: FontSize(14.0),
};

/// Seraphic Light Theme (Elite Edition)
pub const LIGHT_THEME: Theme = Theme {
    background: Color::rgb(0.95, 0.95, 0.95),
    primary: Color::rgb(0.9, 0.9, 0.9),
    secondary: Color::rgb(0.8, 0.8, 0.8),
    accent: Color::rgb(0.0, 0.5, 0.8),
    text: Color::rgb(0.1, 0.1, 0.1),
    font_size_normal: FontSize(14.0),
};
