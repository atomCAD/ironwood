// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Text component for displaying styled text content
//!
//! The Text component is a pure data structure that describes how text should
//! appear in the UI. It contains no rendering logic - that's handled by backends
//! through the ViewExtractor pattern.

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

        // Test text styling
        let styled_text = Text::new("Styled text").font_size(24.0).color(Color::RED);

        assert_eq!(styled_text.content, "Styled text");
        assert_eq!(styled_text.style.font_size, 24.0);
        assert_eq!(styled_text.style.color, Color::RED);
    }
}

// End of File
