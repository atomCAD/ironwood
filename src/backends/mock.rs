// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Mock backend for testing view extraction
//!
//! The mock backend extracts views into simple, testable data structures.
//! This allows verification that views are being created correctly and
//! that their properties are accessible without needing a full rendering
//! pipeline.
//!
//! The mock backend is also useful for automated testing, as it produces
//! deterministic output that can be easily compared in assertions.

use crate::{
    elements::Text,
    extraction::{RenderContext, ViewExtractor},
    interaction::InteractionState,
    style::{Color, TextStyle},
    widgets::Button,
};

/// Mock backend for testing view extraction.
///
/// The MockBackend extracts views into simple, testable data structures.
/// This allows us to verify that views are being created correctly and
/// that their properties are accessible without needing a full rendering
/// pipeline.
///
/// The mock backend is also useful for automated testing, as it produces
/// deterministic output that can be easily compared in assertions.
///
/// # Examples
///
/// ```
/// use ironwood::{prelude::*, backends::mock::MockBackend};
///
/// let text = Text::new("Hello, world!");
/// let ctx = RenderContext::new();
/// let extracted = MockBackend::extract(&text, &ctx);
/// assert_eq!(extracted.content, "Hello, world!");
/// ```
pub struct MockBackend;

/// Mock representation of extracted text for testing.
///
/// This captures all the essential information from a Text view in a format
/// that's easy to test against. The mock backend uses this to verify that
/// text views are being extracted correctly.
#[derive(Debug, Clone, PartialEq)]
pub struct MockText {
    /// The text content
    pub content: String,
    /// Font size in logical pixels
    pub font_size: f32,
    /// Text color
    pub color: Color,
}

impl ViewExtractor<Text> for MockBackend {
    type Output = MockText;

    fn extract(view: &Text, _ctx: &RenderContext) -> Self::Output {
        // Extract all the essential data from the Text view
        // This demonstrates how backends can access view properties
        MockText {
            content: view.content.clone(),
            font_size: view.style.font_size,
            color: view.style.color,
        }
    }
}

/// Mock representation of extracted button for testing.
///
/// This captures the information from a Button component that's relevant for
/// display and rendering, including visual states like pressed/focused that
/// affect how the button should appear on screen.
#[derive(Debug, Clone, PartialEq)]
pub struct MockButton {
    /// The button text
    pub text: String,
    /// Background color
    pub background_color: Color,
    /// Text styling properties
    pub text_style: TextStyle,
    /// The interaction state of the button
    pub interaction_state: InteractionState,
}

impl ViewExtractor<Button> for MockBackend {
    type Output = MockButton;

    fn extract(view: &Button, _ctx: &RenderContext) -> Self::Output {
        // Extract button component display information for testing
        MockButton {
            text: view.text.content.clone(),
            background_color: view.background_color,
            text_style: view.text.style,
            interaction_state: view.interactive.state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        interaction::{Enableable, Focusable, Hoverable, InteractionMessage, Pressable},
        model::Model,
        widgets::ButtonMessage,
    };

    #[test]
    fn text_extraction() {
        // Test extracting a basic text view
        let text = Text::new("Hello, world!");
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&text, &ctx);

        assert_eq!(extracted.content, "Hello, world!");
        assert_eq!(extracted.font_size, 16.0);
        assert_eq!(extracted.color, Color::BLACK);
    }

    #[test]
    fn styled_text_extraction() {
        // Test extracting a styled text view
        let text = Text::new("Styled text").font_size(24.0).color(Color::RED);
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&text, &ctx);

        assert_eq!(extracted.content, "Styled text");
        assert_eq!(extracted.font_size, 24.0);
        assert_eq!(extracted.color, Color::RED);
    }

    #[test]
    fn button_extraction_basic() {
        // Test extracting a basic button component
        let button = Button::new("Click me");
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&button, &ctx);

        assert_eq!(extracted.text, "Click me");
        assert_eq!(extracted.background_color, Color::rgb(0.9, 0.9, 0.9));
        assert_eq!(extracted.text_style.color, Color::BLACK);
        assert!(extracted.interaction_state.is_enabled());
        assert!(!extracted.interaction_state.is_pressed());
        assert!(!extracted.interaction_state.is_focused());
        assert!(!extracted.interaction_state.is_hovered());
    }

    #[test]
    fn button_extraction_disabled() {
        // Test extracting a disabled button
        let button = Button::new("Disabled").disable();
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&button, &ctx);

        assert_eq!(extracted.text, "Disabled");
        assert!(!extracted.interaction_state.is_enabled());
        assert!(!extracted.interaction_state.is_pressed());
        assert!(!extracted.interaction_state.is_focused());
        assert!(!extracted.interaction_state.is_hovered());
    }

    #[test]
    fn button_extraction_visual_states() {
        // Test extracting button visual states that affect rendering
        let ctx = RenderContext::new();

        // Test clicked button (no state change)
        let clicked_button = Button::new("Clicked").update(ButtonMessage::Clicked);
        let clicked_extracted = MockBackend::extract(&clicked_button, &ctx);
        assert_eq!(clicked_extracted.text, "Clicked");
        assert!(clicked_extracted.interaction_state.is_enabled());
        assert!(!clicked_extracted.interaction_state.is_pressed());
        assert!(!clicked_extracted.interaction_state.is_focused());
        assert!(!clicked_extracted.interaction_state.is_hovered());

        // Test focused button
        let focused_button = Button::new("Focused").update(ButtonMessage::Interaction(
            InteractionMessage::FocusChanged(true),
        ));
        let focused_extracted = MockBackend::extract(&focused_button, &ctx);
        assert_eq!(focused_extracted.text, "Focused");
        assert!(focused_extracted.interaction_state.is_enabled());
        assert!(!focused_extracted.interaction_state.is_pressed());
        assert!(focused_extracted.interaction_state.is_focused());
        assert!(!focused_extracted.interaction_state.is_hovered());

        // Test pressed button (via press state change)
        let pressed_button = Button::new("Pressed").update(ButtonMessage::Interaction(
            InteractionMessage::PressStateChanged(true),
        ));
        let pressed_extracted = MockBackend::extract(&pressed_button, &ctx);
        assert_eq!(pressed_extracted.text, "Pressed");
        assert!(pressed_extracted.interaction_state.is_enabled());
        assert!(pressed_extracted.interaction_state.is_pressed());
        assert!(!pressed_extracted.interaction_state.is_focused());
        assert!(!pressed_extracted.interaction_state.is_hovered());

        // Test hovered button
        let hovered_button = Button::new("Hovered").update(ButtonMessage::Interaction(
            InteractionMessage::HoverChanged(true),
        ));
        let hovered_extracted = MockBackend::extract(&hovered_button, &ctx);
        assert_eq!(hovered_extracted.text, "Hovered");
        assert!(hovered_extracted.interaction_state.is_enabled());
        assert!(!hovered_extracted.interaction_state.is_pressed());
        assert!(!hovered_extracted.interaction_state.is_focused());
        assert!(hovered_extracted.interaction_state.is_hovered());
    }

    #[test]
    fn styled_button_extraction() {
        // Test extracting a styled button
        let button = Button::new("Styled")
            .background_color(Color::BLUE)
            .with_text(|text| text.color(Color::WHITE))
            .enable();
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&button, &ctx);

        assert_eq!(extracted.text, "Styled");
        assert_eq!(extracted.background_color, Color::BLUE);
        assert_eq!(extracted.text_style.color, Color::WHITE);
        assert!(extracted.interaction_state.is_enabled());
    }

    #[test]
    fn extraction_preserves_view_data() {
        // Test that extraction doesn't modify the original view
        let original_text = Text::new("Original").font_size(20.0);
        let original_button = Button::new("Original").enable();
        let ctx = RenderContext::new();

        // Extract views
        let _extracted_text = MockBackend::extract(&original_text, &ctx);
        let _extracted_button = MockBackend::extract(&original_button, &ctx);

        // Original views should be unchanged
        assert_eq!(original_text.content, "Original");
        assert_eq!(original_text.style.font_size, 20.0);
        assert_eq!(original_button.text.content, "Original");
        assert!(original_button.is_enabled());
    }

    #[test]
    fn multiple_extractions_same_result() {
        // Test that extracting the same view multiple times gives the same result
        let text = Text::new("Consistent").color(Color::GREEN);
        let ctx = RenderContext::new();

        let extracted1 = MockBackend::extract(&text, &ctx);
        let extracted2 = MockBackend::extract(&text, &ctx);

        assert_eq!(extracted1, extracted2);
    }
}

// End of File
