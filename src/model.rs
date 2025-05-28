// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Model system for Ironwood UI Framework
//!
//! Models represent application state and define how that state changes
//! in response to messages. They follow the Elm architecture pattern
//! of immutable updates through pure functions.
//!
//! Models are separate from Views - a Model contains state and behavior,
//! while a View is a pure data structure describing how that state should
//! be rendered. Models implement a `view()` method that creates the
//! appropriate View representation of their current state.
//!
//! The `update` method consumes the old model and returns a new one because:
//!
//! - **Eliminates data races**: No shared mutable state means no race conditions
//! - **Enables time-travel debugging**: Immutable updates make state history possible
//! - **Simplifies reasoning**: State changes are explicit and predictable
//! - **Supports undo/redo**: Immutable updates enable straightforward undo/redo implementation
//! - **Enables structural sharing**: Rust's move semantics make this efficient
//!
//! While this might seem inefficient, Rust's ownership system and compiler
//! optimizations make immutable updates as fast as mutation in most cases,
//! while providing much stronger guarantees about program correctness.

use std::fmt::Debug;

use crate::{message::Message, view::View};

/// Trait for application models in Ironwood.
///
/// Models are the single source of truth for application state.
/// They must be cloneable for efficient updates and debuggable for development.
/// The `update` method defines how the model changes in response to messages,
/// and the `view` method creates the visual representation of the current state.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// #[derive(Clone, Debug)]
/// struct AppModel {
///     count: i32,
/// }
///
/// #[derive(Debug, Clone)]
/// enum AppMessage {
///     Increment,
///     Decrement,
/// }
///
/// impl Message for AppMessage {}
///
/// impl Model for AppModel {
///     type Message = AppMessage;
///     type View = Text;
///
///     fn update(self, message: Self::Message) -> Self {
///         match message {
///             AppMessage::Increment => Self { count: self.count + 1 },
///             AppMessage::Decrement => Self { count: self.count - 1 },
///         }
///     }
///
///     fn view(&self) -> Self::View {
///         Text::new(format!("Count: {}", self.count))
///     }
/// }
/// ```
pub trait Model: Clone + Debug + Send + Sync + 'static {
    /// The message type that can update this model
    type Message: Message;

    /// The view type that represents this model's visual state
    type View: View;

    /// Update the model with a message, consuming the old model and returning a new one.
    ///
    /// This follows functional programming principles - the old model is consumed
    /// and a new model is returned, ensuring immutable updates.
    fn update(self, message: Self::Message) -> Self;

    /// Create a view representation of this model's current state.
    ///
    /// This method creates a pure data structure that describes how the model
    /// should be rendered, without containing any rendering logic itself.
    fn view(&self) -> Self::View;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::Text;

    #[test]
    fn model_trait_pattern() {
        // Verify trait can be implemented with realistic patterns
        #[derive(Debug, Clone)]
        struct TestModel {
            value: i32,
        }

        #[derive(Debug, Clone)]
        enum TestMessage {
            SetValue(i32),
            Reset,
        }

        impl Message for TestMessage {}

        impl Model for TestModel {
            type Message = TestMessage;
            type View = Text;

            fn update(self, message: Self::Message) -> Self {
                match message {
                    TestMessage::SetValue(value) => Self { value },
                    TestMessage::Reset => Self { value: 0 },
                }
            }

            fn view(&self) -> Self::View {
                Text::new(format!("Value: {}", self.value))
            }
        }

        let model = TestModel { value: 5 };
        let updated = model.update(TestMessage::SetValue(10));
        assert_eq!(updated.value, 10);

        let reset = updated.update(TestMessage::Reset);
        assert_eq!(reset.value, 0);

        let _debug_str = format!("{:?}", reset);
    }

    #[test]
    fn model_immutability() {
        // Demonstrate that immutable updates preserve previous states for debugging/undo
        #[derive(Debug, Clone, PartialEq)]
        struct TestModel {
            data: String,
        }

        #[derive(Debug, Clone)]
        enum TestMessage {
            UpdateData(String),
        }

        impl Message for TestMessage {}

        impl Model for TestModel {
            type Message = TestMessage;
            type View = Text;

            fn update(self, message: Self::Message) -> Self {
                match message {
                    TestMessage::UpdateData(data) => Self { data },
                }
            }

            fn view(&self) -> Self::View {
                Text::new(format!("Data: {}", self.data))
            }
        }

        let original = TestModel {
            data: "original".to_string(),
        };
        let original_data = original.data.clone();

        let updated = original
            .clone()
            .update(TestMessage::UpdateData("updated".to_string()));

        // Immutability enables time-travel debugging and undo/redo when history is maintained
        assert_eq!(original.data, original_data);
        assert_eq!(updated.data, "updated");
        assert_ne!(original, updated);
    }
}

// End of File
