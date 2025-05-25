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

pub mod message;
pub mod model;
pub mod view;

pub use message::Message;
pub use model::Model;
pub use view::View;

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
    pub use crate::message::Message;
    pub use crate::model::Model;
    pub use crate::view::View;
}

// End of File
