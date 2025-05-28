// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! User interaction traits and components for UI elements
//!
//! This module provides both the interaction traits that define common patterns
//! across UI components and the concrete types for managing interaction state.
//! The `InteractionState` bitflags and `Interactive` component provide a reusable
//! foundation that eliminates boilerplate code and ensures consistent behavior.
//!
//! All transformation methods take `self` by value and return a new instance,
//! ensuring components remain immutable and updates are explicit.

use crate::{message::Message, model::Model};
use bitflags::bitflags;

bitflags! {
    /// Bitflags representing the interactive state of a UI component.
    ///
    /// This compact representation uses a single byte to track all common
    /// interaction states, making it efficient for components that need to
    /// manage multiple interaction states simultaneously.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let state = InteractionState::ENABLED | InteractionState::FOCUSED;
    /// assert!(state.contains(InteractionState::ENABLED));
    /// assert!(state.contains(InteractionState::FOCUSED));
    /// assert!(!state.contains(InteractionState::PRESSED));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InteractionState: u8 {
        /// Component is enabled and can receive user interactions
        const ENABLED = 0b0001;
        /// Component is currently being pressed (mouse down, touch active)
        const PRESSED = 0b0010;
        /// Component currently has keyboard focus
        const FOCUSED = 0b0100;
        /// Component is currently being hovered by a pointer
        const HOVERED = 0b1000;
    }
}

impl Default for InteractionState {
    /// Create a default interaction state with only ENABLED set.
    ///
    /// This represents a newly created interactive component that is
    /// enabled but not currently being interacted with.
    fn default() -> Self {
        Self::ENABLED
    }
}

impl Enableable for InteractionState {
    /// Check if this interaction state includes the enabled flag.
    fn is_enabled(&self) -> bool {
        self.contains(Self::ENABLED)
    }

    /// Return a new interaction state with enabled flag set to true.
    fn enable(self) -> Self {
        self | Self::ENABLED
    }

    /// Return a new interaction state with enabled flag set to false.
    fn disable(self) -> Self {
        self & !Self::ENABLED
    }
}

impl Pressable for InteractionState {
    /// Check if this interaction state includes the pressed flag.
    fn is_pressed(&self) -> bool {
        self.contains(Self::PRESSED)
    }

    /// Return a new interaction state with pressed flag set to true.
    fn press(self) -> Self {
        self | Self::PRESSED
    }

    /// Return a new interaction state with pressed flag set to false.
    fn release(self) -> Self {
        self & !Self::PRESSED
    }
}

impl Focusable for InteractionState {
    /// Check if this interaction state includes the focused flag.
    fn is_focused(&self) -> bool {
        self.contains(Self::FOCUSED)
    }

    /// Check if this interaction state can receive focus (when enabled).
    fn can_receive_focus(&self) -> bool {
        self.contains(Self::ENABLED)
    }

    /// Return a new interaction state with focused flag set to true.
    fn focus(self) -> Self {
        self | Self::FOCUSED
    }

    /// Return a new interaction state with focused flag set to false.
    fn unfocus(self) -> Self {
        self & !Self::FOCUSED
    }
}

impl Hoverable for InteractionState {
    /// Check if this interaction state includes the hovered flag.
    fn is_hovered(&self) -> bool {
        self.contains(Self::HOVERED)
    }

    /// Return a new interaction state with hovered flag set to true.
    fn hover(self) -> Self {
        self | Self::HOVERED
    }

    /// Return a new interaction state with hovered flag set to false.
    fn unhover(self) -> Self {
        self & !Self::HOVERED
    }
}

/// Messages for controlling the state of interactive components.
///
/// These messages represent user interactions and programmatic state changes
/// that can occur on any interactive component.
#[derive(Debug, Clone, PartialEq)]
pub enum InteractionMessage {
    /// Component enabled state changed
    EnabledChanged(bool),
    /// Component press state changed (mouse down/up, touch start/end)
    PressStateChanged(bool),
    /// Component focus changed (gained/lost focus via keyboard navigation)
    FocusChanged(bool),
    /// Component hover state changed (mouse enter/leave)
    HoverChanged(bool),
}

impl Message for InteractionMessage {}

/// Base component providing common interactive functionality.
///
/// `Interactive` encapsulates the standard interaction patterns that most
/// UI components need: enabled/disabled state, press feedback, keyboard focus,
/// and hover effects. Components can embed an `Interactive` to get this
/// functionality without boilerplate.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// let interactive = Interactive::new();
/// assert!(interactive.is_enabled());
/// assert!(!interactive.is_pressed());
///
/// let pressed = interactive.press();
/// assert!(pressed.is_pressed());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Interactive {
    /// The current interaction state of this component
    pub state: InteractionState,
}

impl Interactive {
    /// Create a new interactive component with default state (enabled only).
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let interactive = Interactive::new();
    /// assert!(interactive.is_enabled());
    /// assert!(!interactive.is_pressed());
    /// assert!(!interactive.is_focused());
    /// assert!(!interactive.is_hovered());
    /// ```
    pub fn new() -> Self {
        Self {
            state: InteractionState::default(),
        }
    }

    /// Create an interactive component with the specified initial state.
    ///
    /// # Arguments
    ///
    /// * `state` - The initial interaction state
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let state = InteractionState::ENABLED | InteractionState::FOCUSED;
    /// let interactive = Interactive::with_state(state);
    /// assert!(interactive.is_enabled());
    /// assert!(interactive.is_focused());
    /// ```
    pub fn with_state(state: InteractionState) -> Self {
        Self { state }
    }
}

impl Default for Interactive {
    fn default() -> Self {
        Self::new()
    }
}

impl Model for Interactive {
    type Message = InteractionMessage;
    type View = ();

    /// Update the component's state based on the received message.
    ///
    /// This handles all standard interaction messages and updates the
    /// interaction state accordingly.
    ///
    /// # Arguments
    ///
    /// * `message` - The interaction message to process
    ///
    /// # Returns
    ///
    /// A new `Interactive` with updated state
    fn update(self, message: Self::Message) -> Self {
        let mut new_state = self.state;

        match message {
            InteractionMessage::EnabledChanged(enabled) => {
                new_state.set(InteractionState::ENABLED, enabled);
            }
            InteractionMessage::PressStateChanged(pressed) => {
                new_state.set(InteractionState::PRESSED, pressed);
            }
            InteractionMessage::FocusChanged(focused) => {
                new_state.set(InteractionState::FOCUSED, focused);
            }
            InteractionMessage::HoverChanged(hovered) => {
                new_state.set(InteractionState::HOVERED, hovered);
            }
        }

        Self { state: new_state }
    }

    /// Interactive is a utility type for managing interaction state and doesn't
    /// have a visual representation of its own. It returns a unit type.
    fn view(&self) -> Self::View {}
}

/// Trait for components that can be enabled or disabled.
///
/// Enableable components can be in an enabled state (accepting user interaction)
/// or disabled state (rejecting user interaction and typically appearing grayed out).
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// let button = Button::new("Click me")
///     .enable()
///     .disable()
///     .with_enabled(true);
///
/// assert!(button.is_enabled());
/// ```
pub trait Enableable {
    /// Check if the component is currently enabled.
    ///
    /// Enabled components can receive user interaction, while disabled
    /// components typically appear grayed out and ignore input.
    fn is_enabled(&self) -> bool;

    /// Return a new instance with enabled state set to true.
    ///
    /// This is equivalent to `with_enabled(true)` but more expressive.
    fn enable(self) -> Self;

    /// Return a new instance with enabled state set to false.
    ///
    /// This is equivalent to `with_enabled(false)` but more expressive.
    fn disable(self) -> Self;

    /// Return a new instance with the specified enabled state.
    ///
    /// This method provides a convenient way to conditionally enable or
    /// disable a component based on application logic.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether the component should be enabled
    fn with_enabled(self, enabled: bool) -> Self
    where
        Self: Sized,
    {
        if enabled {
            self.enable()
        } else {
            self.disable()
        }
    }
}

impl Enableable for Interactive {
    /// Check if this component is currently enabled for user interaction.
    fn is_enabled(&self) -> bool {
        self.state.is_enabled()
    }

    /// Return a new component instance with enabled state set to true.
    fn enable(self) -> Self {
        Self {
            state: self.state.enable(),
        }
    }

    /// Return a new component instance with enabled state set to false.
    fn disable(self) -> Self {
        Self {
            state: self.state.disable(),
        }
    }
}

/// Trait for components that can be pressed or unpressed.
///
/// Pressable components maintain a pressed state that typically corresponds
/// to visual feedback during user interaction (like a button being held down).
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// let button = Button::new("Press me")
///     .press()
///     .release()
///     .with_pressed(true);
///
/// assert!(button.is_pressed());
/// ```
pub trait Pressable {
    /// Check if the component is currently pressed.
    ///
    /// Pressed state typically provides visual feedback during user interaction,
    /// such as a button appearing "pushed down" while being clicked.
    fn is_pressed(&self) -> bool;

    /// Return a new instance with pressed state set to true.
    ///
    /// This is equivalent to `with_pressed(true)` but more expressive.
    fn press(self) -> Self;

    /// Return a new instance with pressed state set to false.
    ///
    /// This is equivalent to `with_pressed(false)` but more expressive.
    fn release(self) -> Self;

    /// Return a new instance with the specified pressed state.
    ///
    /// This method provides a convenient way to set pressed state based
    /// on user interaction events.
    ///
    /// # Arguments
    ///
    /// * `pressed` - Whether the component should be pressed
    fn with_pressed(self, pressed: bool) -> Self
    where
        Self: Sized,
    {
        if pressed {
            self.press()
        } else {
            self.release()
        }
    }
}

impl Pressable for Interactive {
    /// Check if this component is currently in a pressed state.
    fn is_pressed(&self) -> bool {
        self.state.is_pressed()
    }

    /// Return a new component instance with pressed state set to true.
    fn press(self) -> Self {
        Self {
            state: self.state.press(),
        }
    }

    /// Return a new component instance with pressed state set to false.
    fn release(self) -> Self {
        Self {
            state: self.state.release(),
        }
    }
}

/// Trait for components that can receive keyboard focus.
///
/// Focusable components can be navigated to via keyboard (typically Tab key)
/// and may display visual focus indicators like outlines or highlights.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// let button = Button::new("Focus me")
///     .focus()
///     .unfocus()
///     .with_focused(true);
///
/// assert!(button.is_focused());
/// assert!(button.can_receive_focus());
/// ```
pub trait Focusable {
    /// Check if the component currently has keyboard focus.
    ///
    /// Focused components typically display visual indicators and receive
    /// keyboard input events.
    fn is_focused(&self) -> bool;

    /// Check if the component can receive keyboard focus.
    ///
    /// Most interactive components can receive focus, but some (like disabled
    /// components) may not be focusable. The default implementation returns true.
    fn can_receive_focus(&self) -> bool {
        true
    }

    /// Return a new instance with focus gained.
    ///
    /// This is equivalent to `with_focused(true)` but more expressive.
    fn focus(self) -> Self;

    /// Return a new instance with focus lost.
    ///
    /// This is equivalent to `with_focused(false)` but more expressive.
    fn unfocus(self) -> Self;

    /// Return a new instance with the specified focus state.
    ///
    /// This method provides a convenient way to set focus state based
    /// on keyboard navigation events.
    ///
    /// # Arguments
    ///
    /// * `focused` - Whether the component should have focus
    fn with_focused(self, focused: bool) -> Self
    where
        Self: Sized,
    {
        if focused {
            self.focus()
        } else {
            self.unfocus()
        }
    }
}

impl Focusable for Interactive {
    /// Check if this component currently has keyboard focus.
    fn is_focused(&self) -> bool {
        self.state.is_focused()
    }

    /// Check if this component can receive keyboard focus.
    ///
    /// Components can receive focus when they are enabled.
    fn can_receive_focus(&self) -> bool {
        self.state.can_receive_focus()
    }

    /// Return a new component instance with focus gained.
    fn focus(self) -> Self {
        Self {
            state: self.state.focus(),
        }
    }

    /// Return a new component instance with focus lost.
    fn unfocus(self) -> Self {
        Self {
            state: self.state.unfocus(),
        }
    }
}

/// Trait for components that can be hovered by a pointer.
///
/// Hoverable components can detect when a mouse cursor or other pointer
/// is positioned over them, typically providing visual feedback.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// let button = Button::new("Hover me")
///     .hover()
///     .unhover()
///     .with_hovered(true);
///
/// assert!(button.is_hovered());
/// ```
pub trait Hoverable {
    /// Check if the component is currently being hovered.
    ///
    /// Hovered state indicates that a pointer (mouse cursor, stylus, etc.)
    /// is currently positioned over the component.
    fn is_hovered(&self) -> bool;

    /// Return a new instance with hover state set to true.
    ///
    /// This is equivalent to `with_hovered(true)` but more expressive.
    fn hover(self) -> Self;

    /// Return a new instance with hover state set to false.
    ///
    /// This is equivalent to `with_hovered(false)` but more expressive.
    fn unhover(self) -> Self;

    /// Return a new instance with the specified hover state.
    ///
    /// This method provides a convenient way to set hover state based
    /// on pointer movement events.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the component should be hovered
    fn with_hovered(self, hovered: bool) -> Self
    where
        Self: Sized,
    {
        if hovered {
            self.hover()
        } else {
            self.unhover()
        }
    }
}

impl Hoverable for Interactive {
    /// Check if this component is currently being hovered by a pointer.
    fn is_hovered(&self) -> bool {
        self.state.is_hovered()
    }

    /// Return a new component instance with hover state set to true.
    fn hover(self) -> Self {
        Self {
            state: self.state.hover(),
        }
    }

    /// Return a new component instance with hover state set to false.
    fn unhover(self) -> Self {
        Self {
            state: self.state.unhover(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interaction_state_enableable() {
        let state = InteractionState::default();
        assert!(state.is_enabled());

        let disabled = state.disable();
        assert!(!disabled.is_enabled());

        let enabled = disabled.enable();
        assert!(enabled.is_enabled());
    }

    #[test]
    fn interaction_state_pressable() {
        let state = InteractionState::default();
        assert!(!state.is_pressed());

        let pressed = state.press();
        assert!(pressed.is_pressed());

        let released = pressed.release();
        assert!(!released.is_pressed());
    }

    #[test]
    fn interaction_state_focusable() {
        let state = InteractionState::default();
        assert!(!state.is_focused());
        assert!(state.can_receive_focus());

        let focused = state.focus();
        assert!(focused.is_focused());

        let unfocused = focused.unfocus();
        assert!(!unfocused.is_focused());

        // Disabled state cannot receive focus
        let disabled = state.disable();
        assert!(!disabled.can_receive_focus());
    }

    #[test]
    fn interaction_state_hoverable() {
        let state = InteractionState::default();
        assert!(!state.is_hovered());

        let hovered = state.hover();
        assert!(hovered.is_hovered());

        let unhovered = hovered.unhover();
        assert!(!unhovered.is_hovered());
    }

    #[test]
    fn interaction_state_trait_chaining() {
        let state = InteractionState::default()
            .press()
            .focus()
            .hover()
            .disable();

        assert!(!state.is_enabled());
        assert!(state.is_pressed());
        assert!(state.is_focused());
        assert!(state.is_hovered());
    }

    #[test]
    fn interactive_creation() {
        let interactive = Interactive::new();
        assert!(interactive.is_enabled());
        assert!(!interactive.is_pressed());
        assert!(!interactive.is_focused());
        assert!(!interactive.is_hovered());

        let state = InteractionState::ENABLED | InteractionState::FOCUSED;
        let interactive = Interactive::with_state(state);
        assert!(interactive.is_enabled());
        assert!(interactive.is_focused());
    }

    #[test]
    fn interactive_enableable() {
        // Test Enableable trait methods
        let interactive = Interactive::new();

        // Test initial state
        assert!(interactive.is_enabled());

        // Test disable
        let disabled = interactive.clone().disable();
        assert!(!disabled.is_enabled());

        // Test enable
        let enabled = disabled.enable();
        assert!(enabled.is_enabled());

        // Test with_enabled
        let conditionally_enabled = interactive.clone().with_enabled(false);
        assert!(!conditionally_enabled.is_enabled());
        let conditionally_enabled = conditionally_enabled.with_enabled(true);
        assert!(conditionally_enabled.is_enabled());
    }

    #[test]
    fn interactive_pressable() {
        // Test Pressable trait methods
        let interactive = Interactive::new();

        // Test initial state
        assert!(!interactive.is_pressed());

        // Test press
        let pressed = interactive.clone().press();
        assert!(pressed.is_pressed());

        // Test release
        let released = pressed.release();
        assert!(!released.is_pressed());

        // Test with_pressed
        let conditionally_pressed = interactive.clone().with_pressed(true);
        assert!(conditionally_pressed.is_pressed());
        let conditionally_pressed = conditionally_pressed.with_pressed(false);
        assert!(!conditionally_pressed.is_pressed());
    }

    #[test]
    fn interactive_focusable() {
        // Test Focusable trait methods
        let interactive = Interactive::new();

        // Test initial state
        assert!(!interactive.is_focused());
        assert!(interactive.can_receive_focus());

        // Test focus
        let focused = interactive.clone().focus();
        assert!(focused.is_focused());

        // Test unfocus
        let unfocused = focused.unfocus();
        assert!(!unfocused.is_focused());

        // Test with_focused
        let conditionally_focused = interactive.clone().with_focused(true);
        assert!(conditionally_focused.is_focused());
        let conditionally_focused = conditionally_focused.with_focused(false);
        assert!(!conditionally_focused.is_focused());

        // Test disabled button cannot receive focus
        let disabled_button = interactive.disable();
        assert!(!disabled_button.can_receive_focus());
    }

    #[test]
    fn interactive_hoverable() {
        // Test Hoverable trait methods
        let interactive = Interactive::new();

        // Test initial state
        assert!(!interactive.is_hovered());

        // Test hover
        let hovered = interactive.clone().hover();
        assert!(hovered.is_hovered());

        // Test unhover
        let unhovered = hovered.unhover();
        assert!(!unhovered.is_hovered());

        // Test with_hovered
        let conditionally_hovered = interactive.clone().with_hovered(true);
        assert!(conditionally_hovered.is_hovered());
        let conditionally_hovered = conditionally_hovered.with_hovered(false);
        assert!(!conditionally_hovered.is_hovered());
    }

    #[test]
    fn interactive_message_handling() {
        let interactive = Interactive::new();

        // Test state changes
        let enabled = interactive
            .clone()
            .update(InteractionMessage::EnabledChanged(false));
        assert!(!enabled.is_enabled());

        let pressed = interactive
            .clone()
            .update(InteractionMessage::PressStateChanged(true));
        assert!(pressed.is_pressed());

        let focused = interactive
            .clone()
            .update(InteractionMessage::FocusChanged(true));
        assert!(focused.is_focused());

        let hovered = interactive
            .clone()
            .update(InteractionMessage::HoverChanged(true));
        assert!(hovered.is_hovered());
    }

    #[test]
    fn interactive_trait_chaining() {
        let interactive = Interactive::new().press().focus().hover().disable();

        assert!(!interactive.is_enabled());
        assert!(interactive.is_pressed());
        assert!(interactive.is_focused());
        assert!(interactive.is_hovered());
    }

    #[test]
    fn interaction_edge_cases() {
        // Test rapid state changes
        let mut interactive = Interactive::new();

        // Rapidly toggle states
        for _ in 0..1000 {
            interactive = interactive
                .press()
                .release()
                .focus()
                .unfocus()
                .hover()
                .unhover();
        }

        // Should end up in default state
        assert!(interactive.is_enabled());
        assert!(!interactive.is_pressed());
        assert!(!interactive.is_focused());
        assert!(!interactive.is_hovered());
        assert!(interactive.can_receive_focus());

        // Test state combinations that can occur together
        let combined_states = Interactive::new()
            .disable()
            .focus() // Disabled but focused (can happen during state transitions)
            .press() // Disabled but pressed (can happen during state transitions)
            .hover(); // Disabled but hovered (normal - for tooltips, etc.)

        assert!(!combined_states.is_enabled());
        assert!(combined_states.is_focused());
        assert!(combined_states.is_pressed());
        assert!(combined_states.is_hovered());
        assert!(!combined_states.can_receive_focus()); // But can't receive new focus
    }
}

// End of File
