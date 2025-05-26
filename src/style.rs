// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Styling system for Ironwood UI Framework
//!
//! This module contains types and utilities for styling UI components.
//! Styling in Ironwood is declarative - you describe how things should look
//! rather than imperatively applying styles.
//!
//! The styling system is designed to be:
//! - **Simple**: Easy to understand and use
//! - **Consistent**: Uniform behavior across all components
//! - **Extensible**: Easy to add new styling properties
//! - **Platform-agnostic**: Works the same across different backends

/// Basic color representation for styling views.
///
/// Colors are represented as RGBA values with floating-point components
/// in the range [0.0, 1.0]. This provides sufficient precision for UI
/// rendering while being simple to work with.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// let red = Color::RED;
/// let custom = Color::rgba(0.5, 0.7, 0.9, 1.0);
/// let opaque_blue = Color::rgb(0.0, 0.0, 1.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red component (0.0 to 1.0)
    pub r: f32,
    /// Green component (0.0 to 1.0)
    pub g: f32,
    /// Blue component (0.0 to 1.0)
    pub b: f32,
    /// Alpha component (0.0 to 1.0)
    pub a: f32,
}

impl Color {
    /// Create a new color with the specified RGBA components.
    ///
    /// All components should be in the range [0.0, 1.0].
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let color = Color::rgba(0.5, 0.7, 0.9, 0.8);
    /// assert_eq!(color.r, 0.5);
    /// assert_eq!(color.a, 0.8);
    /// ```
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new opaque color with the specified RGB components.
    ///
    /// Alpha is set to 1.0 (fully opaque).
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let blue = Color::rgb(0.0, 0.0, 1.0);
    /// assert_eq!(blue.b, 1.0);
    /// assert_eq!(blue.a, 1.0);
    /// ```
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Pure black color
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);

    /// Pure white color
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);

    /// Pure red color
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);

    /// Pure green color
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);

    /// Pure blue color
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
}

/// Text styling properties for UI elements
///
/// `TextStyle` encapsulates all text-related styling properties including
/// color and font size. This provides a consistent way to style text across
/// different UI components.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// // Default text style (16px, black)
/// let default_style = TextStyle::default();
///
/// // Custom text style
/// let heading_style = TextStyle::new()
///     .font_size(24.0)
///     .color(Color::BLUE);
///
/// // Builder pattern
/// let warning_style = TextStyle::new()
///     .font_size(14.0)
///     .color(Color::RED);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStyle {
    /// Font size in logical pixels
    pub font_size: f32,
    /// Text color
    pub color: Color,
}

impl TextStyle {
    /// Create a new text style with default values.
    ///
    /// Default values are 16px font size and black color.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let style = TextStyle::new();
    /// assert_eq!(style.font_size, 16.0);
    /// assert_eq!(style.color, Color::BLACK);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the font size for this text style.
    ///
    /// # Arguments
    ///
    /// * `size` - Font size in logical pixels
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let style = TextStyle::new().font_size(24.0);
    /// assert_eq!(style.font_size, 24.0);
    /// ```
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the color for this text style.
    ///
    /// # Arguments
    ///
    /// * `color` - The text color
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let style = TextStyle::new().color(Color::RED);
    /// assert_eq!(style.color, Color::RED);
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for TextStyle {
    /// Create a default text style with 16px font size and black color.
    fn default() -> Self {
        Self {
            font_size: 16.0,
            color: Color::BLACK,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_style_functionality() {
        // Test default text style
        let default_style = TextStyle::default();
        assert_eq!(default_style.font_size, 16.0);
        assert_eq!(default_style.color, Color::BLACK);

        // Test new() method
        let new_style = TextStyle::new();
        assert_eq!(new_style, default_style);

        // Test builder pattern and method chaining
        let custom_style = TextStyle::new().font_size(24.0).color(Color::BLUE);
        assert_eq!(custom_style.font_size, 24.0);
        assert_eq!(custom_style.color, Color::BLUE);

        // Test that later calls override earlier ones
        let override_style = TextStyle::new()
            .font_size(18.0)
            .color(Color::RED)
            .font_size(20.0); // Should override previous font size

        assert_eq!(override_style.font_size, 20.0);
        assert_eq!(override_style.color, Color::RED);
    }

    #[test]
    fn color_edge_cases() {
        use crate::{
            backends::mock::MockBackend,
            elements::Text,
            extraction::{RenderContext, ViewExtractor},
        };

        let ctx = RenderContext::new();

        // Colors outside normal 0.0-1.0 range
        let over_range = Color::rgba(1.2, -0.1, 1.5, 0.5);
        let text = Text::new("Test").color(over_range);
        let extracted = MockBackend::extract(&text, &ctx);
        assert_eq!(extracted.color, over_range);

        // Fully transparent color
        let transparent = Color::rgba(1.0, 0.0, 0.0, 0.0);
        let text = Text::new("Test").color(transparent);
        let extracted = MockBackend::extract(&text, &ctx);
        assert_eq!(extracted.color, transparent);

        // Precise color values
        let precise = Color::rgba(0.123_456_8, 0.987_654_3, 0.555_555_6, 0.333_333_3);
        let text = Text::new("Test").color(precise);
        let extracted = MockBackend::extract(&text, &ctx);
        assert_eq!(extracted.color, precise);

        // Large display font with transparency
        let display_text = Text::new("Large Display")
            .font_size(72.0)
            .color(Color::rgba(0.0, 0.0, 0.0, 0.1));
        let extracted = MockBackend::extract(&display_text, &ctx);
        assert_eq!(extracted.font_size, 72.0);
        assert_eq!(extracted.color.a, 0.1);
    }
}

// End of File
