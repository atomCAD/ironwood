// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Ironwood UI Framework
//!
//! A Rust-native UI framework that combines Elm's unidirectional data flow
//! with SwiftUI's declarative syntax and Rust's zero-cost abstractions.
//!
//! Ironwood follows the Elm Architecture to ensure predictable state management
//! and eliminate entire classes of bugs common in traditional UI frameworks.
//! By enforcing unidirectional data flow through explicit messages, applications
//! become easier to reason about, test, and debug.
//!
//! ## Component Hierarchy Pattern
//!
//! Components in Ironwood follow a hierarchical composition pattern where each
//! component can be both a Model (with its own state and messages) and a View
//! (with rendering data). This enables building complex UIs from simple, reusable parts.
//!
//! Components are designed to be embedded in parent models as fields, with
//! their messages bubbling up through the component hierarchy in a type-safe way:
//!
//! ```rust
//! use ironwood::prelude::*;
//!
//! // Parent model contains child components as fields
//! #[derive(Clone, Debug)]
//! struct FormModel {
//!     submit_button: Button,
//!     cancel_button: Button,
//! }
//!
//! // Parent messages wrap child messages for type-safe bubbling
//! #[derive(Debug, Clone)]
//! enum FormMessage {
//!     SubmitButton(ButtonMessage),
//!     CancelButton(ButtonMessage),
//!     FormSubmitted,
//! }
//!
//! impl Message for FormMessage {}
//! ```
//!
//! This enables:
//! - **Reusability**: Components can be used in any parent context
//! - **Encapsulation**: Each component manages its own state
//! - **Type Safety**: Message routing is checked at compile time
//! - **Testability**: Components can be tested in isolation
//! - **Composability**: Complex UIs built from simple parts
//!
//! ## Framework Organization
//!
//! - **[`backends`]** - Concrete backend implementations
//! - **[`elements`]** - Basic display building blocks with no state
//! - **[`extraction`]** - Backend abstraction for rendering views
//! - **[`interaction`]** - Traits and types for user interaction handling
//! - **[`message`]** - Message trait and types for state changes
//! - **[`model`]** - Model trait and types for application state
//! - **[`style`]** - Styling types for colors, fonts, and layout
//! - **[`view`]** - View trait and types for rendering views
//! - **[`widgets`]** - Interactive components with state and behavior

pub mod backends;
pub mod elements;
pub mod extraction;
pub mod interaction;
pub mod message;
pub mod model;
pub mod style;
pub mod view;
pub mod widgets;

pub use elements::Text;
pub use extraction::{RenderContext, ViewExtractor};
pub use interaction::{
    Enableable, Focusable, Hoverable, InteractionMessage, InteractionState, Interactive, Pressable,
};
pub use message::Message;
pub use model::Model;
pub use style::{Color, TextStyle};
pub use view::View;
pub use widgets::{Button, ButtonMessage};

/// Prelude module for Ironwood UI Framework
///
/// This module re-exports the most commonly used types and traits from Ironwood,
/// allowing users to import everything they need with a single `use` statement.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// #[derive(Debug, Clone)]
/// enum AppMessage {
///     Increment,
///     Decrement,
/// }
///
/// impl Message for AppMessage {}
///
/// #[derive(Clone, Debug)]
/// struct AppModel {
///     count: i32,
/// }
///
/// impl Model for AppModel {
///     type Message = AppMessage;
///
///     fn update(self, message: Self::Message) -> Self {
///         match message {
///             AppMessage::Increment => Self { count: self.count + 1 },
///             AppMessage::Decrement => Self { count: self.count - 1 },
///         }
///     }
/// }
///
/// #[derive(Debug, Clone)]
/// struct TextView {
///     text: String,
/// }
///
/// impl View for TextView {}
/// ```
pub mod prelude {
    // Re-export the core traits that users will need in almost every Ironwood application
    pub use crate::elements::Text;
    pub use crate::extraction::{RenderContext, ViewExtractor};
    pub use crate::interaction::{
        Enableable, Focusable, Hoverable, InteractionMessage, InteractionState, Interactive,
        Pressable,
    };
    pub use crate::message::Message;
    pub use crate::model::Model;
    pub use crate::style::{Color, TextStyle};
    pub use crate::view::View;
    pub use crate::widgets::{Button, ButtonMessage};
}

// End of File
