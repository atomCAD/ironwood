// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Button component for interactive UI elements
//!
//! The Button component provides interactive elements that can respond to user input
//! and maintain their own state (enabled/disabled, pressed, styling). Buttons are
//! models that contain state and behavior, and they create ButtonView instances
//! through their view() method to represent their visual state.

use crate::{
    elements::Text,
    interaction::{
        Enableable, Focusable, Hoverable, InteractionMessage, InteractionState, Interactive,
        Pressable,
    },
    message::Message,
    model::Model,
    style::Color,
    view::View,
};

/// View representation of a button's visual state.
///
/// This is a pure data structure that describes how a button should appear,
/// including its text content, styling, and current interaction state.
/// The actual rendering is handled by backends.
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonView {
    /// The text content of the button
    pub text: Text,
    /// Background color of the button
    pub background_color: Color,
    /// Current interaction state (enabled, pressed, focused, hovered)
    pub interaction_state: InteractionState,
}

impl View for ButtonView {}

/// Messages that represent user interactions with a Button component.
///
/// These messages combine button-specific interactions (like clicking) with
/// the standard interaction patterns provided by InteractionMessage.
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonMessage {
    /// Button was clicked/pressed by the user
    Clicked,
    /// Standard interaction (enabled, pressed, focused, hovered state changes)
    Interaction(InteractionMessage),
}

impl Message for ButtonMessage {}

/// Button component that maintains its own state and responds to user interactions.
///
/// Buttons have their styling configured at creation time and respond to user
/// interaction messages. The button's appearance is determined by its initial
/// configuration and current interaction state.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// // Create a styled button - styling set at creation time
/// let button = Button::new("Save Document")
///     .background_color(Color::BLUE)
///     .with_text(|text| text.color(Color::WHITE))
///     .enable();
///
/// // Handle user interactions
/// let clicked_button = button.clone().update(ButtonMessage::Clicked);
/// let pressed_button = button.clone().update(ButtonMessage::Interaction(
///     InteractionMessage::PressStateChanged(true),
/// ));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    /// The text content of the button
    pub text: Text,
    /// Background color of the button (set at creation)
    pub background_color: Color,
    /// Base interactive functionality (enabled, pressed, focused, hovered states)
    pub interactive: Interactive,
}

impl Button {
    /// Create a new button with the specified text.
    ///
    /// The button starts with default styling and is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Click me!");
    /// assert_eq!(button.text.content, "Click me!");
    /// assert!(button.is_enabled());
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Text::new(text),
            background_color: Color::rgb(0.9, 0.9, 0.9), // Light gray
            interactive: Interactive::new(),
        }
    }

    /// Set the background color for this button.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Action")
    ///     .background_color(Color::BLUE);
    /// assert_eq!(button.background_color, Color::BLUE);
    /// ```
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Configure the text content of this button.
    ///
    /// This method allows fluent configuration of the button's text styling
    /// by providing a closure that receives the current Text component and
    /// returns a modified version.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Action")
    ///     .with_text(|text| text.color(Color::WHITE).font_size(20.0));
    /// assert_eq!(button.text.style.color, Color::WHITE);
    /// assert_eq!(button.text.style.font_size, 20.0);
    /// ```
    pub fn with_text<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Text) -> Text,
    {
        self.text = f(self.text);
        self
    }
}

impl Model for Button {
    type Message = ButtonMessage;
    type View = ButtonView;

    /// Update the button's state based on the received message.
    ///
    /// This method handles all button interaction messages and returns a new
    /// button instance with updated state. The button follows Elm Architecture
    /// principles by being immutable and updating through explicit messages.
    ///
    /// # Arguments
    ///
    /// * `message` - The message describing what interaction occurred
    ///
    /// # Returns
    ///
    /// A new `Button` instance with updated state based on the message
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Click me");
    /// let clicked = button.clone().update(ButtonMessage::Clicked);
    /// assert_eq!(clicked, button);
    /// ```
    fn update(self, message: Self::Message) -> Self {
        match message {
            ButtonMessage::Clicked => {
                // Handle completed click interaction
                // The button itself doesn't change state when clicked
                // Application logic is handled when this message bubbles up to parent components
                self
            }
            ButtonMessage::Interaction(interaction_msg) => Self {
                interactive: self.interactive.update(interaction_msg),
                ..self
            },
        }
    }

    /// Create a view representation of this button's current state.
    ///
    /// This method creates a ButtonView that contains all the visual information
    /// needed to render the button, including its text, styling, and interaction state.
    fn view(&self) -> Self::View {
        ButtonView {
            text: self.text.clone(),
            background_color: self.background_color,
            interaction_state: self.interactive.state,
        }
    }
}

impl Enableable for Button {
    /// Check if this button is currently enabled for user interaction.
    ///
    /// Enabled buttons can receive clicks, focus, and other interactions.
    /// Disabled buttons typically appear grayed out and ignore user input.
    ///
    /// # Returns
    ///
    /// `true` if the button is enabled, `false` if disabled
    fn is_enabled(&self) -> bool {
        self.interactive.is_enabled()
    }

    /// Return a new button instance with enabled state set to true.
    ///
    /// This allows the button to receive user interactions like clicks,
    /// keyboard focus, and hover events.
    ///
    /// # Returns
    ///
    /// A new `Button` instance that is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Action").disable().enable();
    /// assert!(button.is_enabled());
    /// ```
    fn enable(self) -> Self {
        Self {
            interactive: self.interactive.enable(),
            ..self
        }
    }

    /// Return a new button instance with enabled state set to false.
    ///
    /// Disabled buttons cannot receive user interactions and typically
    /// appear grayed out to indicate their unavailable state.
    ///
    /// # Returns
    ///
    /// A new `Button` instance that is disabled
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Action").disable();
    /// assert!(!button.is_enabled());
    /// ```
    fn disable(self) -> Self {
        Self {
            interactive: self.interactive.disable(),
            ..self
        }
    }
}

impl Pressable for Button {
    /// Check if this button is currently in a pressed state.
    ///
    /// Pressed state typically provides visual feedback during user interaction,
    /// such as the button appearing "pushed down" while being clicked.
    ///
    /// # Returns
    ///
    /// `true` if the button is currently pressed, `false` otherwise
    fn is_pressed(&self) -> bool {
        self.interactive.is_pressed()
    }

    /// Return a new button instance with pressed state set to true.
    ///
    /// This is typically used to provide visual feedback during user
    /// interactions like mouse down events or touch start.
    ///
    /// # Returns
    ///
    /// A new `Button` instance that appears pressed
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Press me").press();
    /// assert!(button.is_pressed());
    /// ```
    fn press(self) -> Self {
        Self {
            interactive: self.interactive.press(),
            ..self
        }
    }

    /// Return a new button instance with pressed state set to false.
    ///
    /// This is typically used when user interaction ends, such as
    /// mouse up events or touch end.
    ///
    /// # Returns
    ///
    /// A new `Button` instance that appears released
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Press me").press().release();
    /// assert!(!button.is_pressed());
    /// ```
    fn release(self) -> Self {
        Self {
            interactive: self.interactive.release(),
            ..self
        }
    }
}

impl Focusable for Button {
    /// Check if this button currently has keyboard focus.
    ///
    /// Focused buttons typically display visual indicators (like outlines)
    /// and receive keyboard input events.
    ///
    /// # Returns
    ///
    /// `true` if the button currently has focus, `false` otherwise
    fn is_focused(&self) -> bool {
        self.interactive.is_focused()
    }

    /// Check if this button can receive keyboard focus.
    ///
    /// Buttons can receive focus when they are enabled. Disabled buttons
    /// cannot be focused via keyboard navigation.
    ///
    /// # Returns
    ///
    /// `true` if the button can receive focus, `false` if disabled
    fn can_receive_focus(&self) -> bool {
        self.interactive.can_receive_focus()
    }

    /// Return a new button instance with focus gained.
    ///
    /// Focused buttons typically display visual focus indicators and
    /// can receive keyboard events like Enter or Space.
    ///
    /// # Returns
    ///
    /// A new `Button` instance that has focus
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Focus me").focus();
    /// assert!(button.is_focused());
    /// ```
    fn focus(self) -> Self {
        Self {
            interactive: self.interactive.focus(),
            ..self
        }
    }

    /// Return a new button instance with focus lost.
    ///
    /// This removes the visual focus indicators and stops the button
    /// from receiving keyboard events.
    ///
    /// # Returns
    ///
    /// A new `Button` instance that does not have focus
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Focus me").focus().unfocus();
    /// assert!(!button.is_focused());
    /// ```
    fn unfocus(self) -> Self {
        Self {
            interactive: self.interactive.unfocus(),
            ..self
        }
    }
}

impl Hoverable for Button {
    /// Check if this button is currently being hovered by a pointer.
    ///
    /// Hovered state indicates that a mouse cursor, stylus, or other
    /// pointer device is positioned over the button.
    ///
    /// # Returns
    ///
    /// `true` if the button is currently hovered, `false` otherwise
    fn is_hovered(&self) -> bool {
        self.interactive.is_hovered()
    }

    /// Return a new button instance with hover state set to true.
    ///
    /// This is typically used when a pointer enters the button's area,
    /// often triggering visual feedback like color changes or highlights.
    ///
    /// # Returns
    ///
    /// A new `Button` instance that appears hovered
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Hover me").hover();
    /// assert!(button.is_hovered());
    /// ```
    fn hover(self) -> Self {
        Self {
            interactive: self.interactive.hover(),
            ..self
        }
    }

    /// Return a new button instance with hover state set to false.
    ///
    /// This is typically used when a pointer leaves the button's area,
    /// removing any hover-specific visual effects.
    ///
    /// # Returns
    ///
    /// A new `Button` instance that is not hovered
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let button = Button::new("Hover me").hover().unhover();
    /// assert!(!button.is_hovered());
    /// ```
    fn unhover(self) -> Self {
        Self {
            interactive: self.interactive.unhover(),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_creation_and_styling() {
        // Test basic button creation
        let button = Button::new("Click me");
        assert_eq!(button.text.content, "Click me");
        assert_eq!(button.background_color, Color::rgb(0.9, 0.9, 0.9));
        assert_eq!(button.text.style.color, Color::BLACK);
        assert!(button.is_enabled());
        assert!(!button.is_pressed());
        assert!(!button.is_focused());
        assert!(!button.is_hovered());
    }

    #[test]
    fn button_styling() {
        // Test button color customization
        let styled_button = Button::new("Styled")
            .background_color(Color::BLUE)
            .with_text(|text| text.color(Color::WHITE))
            .disable();

        assert_eq!(styled_button.background_color, Color::BLUE);
        assert_eq!(styled_button.text.style.color, Color::WHITE);
        assert!(!styled_button.is_enabled());
    }

    #[test]
    fn button_interaction_handling() {
        let button = Button::new("Test");

        // Test click on enabled button
        let clicked_button = button.clone().update(ButtonMessage::Clicked);
        assert_eq!(clicked_button, button); // Button unchanged after click

        // Test click on disabled button
        let disabled_button = button.clone().disable();
        let clicked_disabled = disabled_button.clone().update(ButtonMessage::Clicked);
        assert_eq!(clicked_disabled, disabled_button); // Button unchanged after click

        // Test enable/disable changes
        let disabled = button.clone().update(ButtonMessage::Interaction(
            InteractionMessage::EnabledChanged(false),
        ));
        assert!(!disabled.is_enabled());
        let re_enabled = disabled.update(ButtonMessage::Interaction(
            InteractionMessage::EnabledChanged(true),
        ));
        assert!(re_enabled.is_enabled());

        // Test press state changes
        let press_down = button.clone().update(ButtonMessage::Interaction(
            InteractionMessage::PressStateChanged(true),
        ));
        assert!(press_down.is_pressed());
        let press_up = press_down.update(ButtonMessage::Interaction(
            InteractionMessage::PressStateChanged(false),
        ));
        assert!(!press_up.is_pressed());

        // Test focus changes
        let focused_button = button.clone().update(ButtonMessage::Interaction(
            InteractionMessage::FocusChanged(true),
        ));
        assert!(focused_button.is_focused());
        let unfocused_button = focused_button.update(ButtonMessage::Interaction(
            InteractionMessage::FocusChanged(false),
        ));
        assert!(!unfocused_button.is_focused());

        // Test hover changes
        let hovered_button = button.clone().update(ButtonMessage::Interaction(
            InteractionMessage::HoverChanged(true),
        ));
        assert!(hovered_button.is_hovered());
        let unhovered_button = hovered_button.update(ButtonMessage::Interaction(
            InteractionMessage::HoverChanged(false),
        ));
        assert!(!unhovered_button.is_hovered());
    }

    #[test]
    fn button_builder_pattern() {
        // Test fluent builder pattern
        let button = Button::new("Builder Test")
            .background_color(Color::GREEN)
            .with_text(|text| text.color(Color::BLACK))
            .enable();

        assert_eq!(button.text.content, "Builder Test");
        assert_eq!(button.background_color, Color::GREEN);
        assert_eq!(button.text.style.color, Color::BLACK);
        assert!(button.is_enabled());
    }

    #[test]
    fn button_with_text_method() {
        // Test the new with_text method
        let button = Button::new("Test").with_text(|text| text.color(Color::RED).font_size(20.0));

        assert_eq!(button.text.content, "Test");
        assert_eq!(button.text.style.color, Color::RED);
        assert_eq!(button.text.style.font_size, 20.0);
    }

    #[test]
    fn button_model_trait_implementation() {
        // Test that Button properly implements Model trait
        let button = Button::new("Model Test");
        let updated = button.clone().update(ButtonMessage::Clicked);

        // Verify the update worked (button unchanged for click)
        assert_eq!(updated, button);

        // Verify immutability - original button unchanged
        let original = Button::new("Model Test");
        assert!(!original.is_pressed());
    }

    #[test]
    fn view_trait_implementation() {
        // Test that Button implements View trait
        let button = Button::new("View Test");

        // This should compile - Button implements View
        fn accepts_view(_view: impl View) {}
        accepts_view(button.view());
    }

    #[test]
    fn trait_method_chaining() {
        // Test that trait methods can be chained together
        let button = Button::new("Chaining Test")
            .enable()
            .press()
            .focus()
            .hover()
            .disable()
            .release()
            .unfocus()
            .unhover();

        assert!(!button.is_enabled());
        assert!(!button.is_pressed());
        assert!(!button.is_focused());
        assert!(!button.is_hovered());
    }
}

// End of File
