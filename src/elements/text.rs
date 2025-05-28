// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Text component for displaying styled text content
//!
//! The Text component is a view that represents styled text content.
//! It's a pure data structure that describes how text should appear.

use crate::{
    style::{Color, TextStyle},
    view::View,
};

/// Text view for displaying styled text content.
///
/// Text views are pure data structures that describe how text should appear.
/// The actual rendering is handled by backends through the ViewExtractor pattern.
/// This separation allows the same text description to be rendered differently
/// on different platforms while maintaining consistent behavior.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// let text = Text::new("Hello, world!")
///     .font_size(24.0)
///     .color(Color::BLUE);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    /// The text content to display
    pub content: String,
    /// Text styling properties
    pub style: TextStyle,
}

impl Text {
    /// Create a new text view with the specified content.
    ///
    /// Uses default styling (16px black text) that can be customized
    /// using the builder methods.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let text = Text::new("Hello, world!");
    /// assert_eq!(text.content, "Hello, world!");
    /// ```
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: TextStyle::default(),
        }
    }

    /// Set the font size for this text.
    ///
    /// Font size is specified in logical pixels. The actual rendered size
    /// may vary based on the platform's DPI scaling.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let text = Text::new("Large text").font_size(24.0);
    /// assert_eq!(text.style.font_size, 24.0);
    /// ```
    pub fn font_size(mut self, size: f32) -> Self {
        self.style = self.style.font_size(size);
        self
    }

    /// Set the color for this text.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let text = Text::new("Red text").color(Color::RED);
    /// assert_eq!(text.style.color, Color::RED);
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.style = self.style.color(color);
        self
    }
}

impl View for Text {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_creation_and_styling() {
        // Test basic text creation
        let text = Text::new("Hello, world!");
        assert_eq!(text.content, "Hello, world!");
        assert_eq!(text.style.font_size, 16.0);
        assert_eq!(text.style.color, Color::BLACK);

        // Test font size styling
        let styled_text = Text::new("Styled").font_size(24.0);
        assert_eq!(styled_text.style.font_size, 24.0);

        // Test color styling
        let colored_text = Text::new("Colored").color(Color::RED);
        assert_eq!(colored_text.style.color, Color::RED);

        // Test chained styling
        let chained = Text::new("Chained").font_size(18.0).color(Color::BLUE);
        assert_eq!(chained.style.font_size, 18.0);
        assert_eq!(chained.style.color, Color::BLUE);
    }

    #[test]
    fn text_edge_cases() {
        use crate::{
            backends::mock::MockBackend,
            extraction::{RenderContext, ViewExtractor},
        };

        let ctx = RenderContext::new();

        // Empty string text
        let empty_text = Text::new("");
        let extracted = MockBackend::extract(&empty_text, &ctx);
        assert_eq!(extracted.content, "");

        // Very long text content
        let long_content = "a".repeat(10000);
        let long_text = Text::new(&long_content);
        let extracted = MockBackend::extract(&long_text, &ctx);
        assert_eq!(extracted.content.len(), 10000);

        // Very small font size
        let tiny_font = Text::new("Test").font_size(1.0);
        let extracted = MockBackend::extract(&tiny_font, &ctx);
        assert_eq!(extracted.font_size, 1.0);

        // Very large font size
        let huge_font = Text::new("Test").font_size(200.0);
        let extracted = MockBackend::extract(&huge_font, &ctx);
        assert_eq!(extracted.font_size, 200.0);
    }
}

// End of File
