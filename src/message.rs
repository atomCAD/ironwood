// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Message system for Ironwood UI Framework
//!
//! Messages represent user interactions and state changes in the application.
//! They follow the Elm architecture pattern where all state changes flow
//! through explicit message types.
//!
//! By requiring all state changes to go through explicit message types, we gain:
//!
//! - **Predictability**: Every possible state change is visible in the message enum
//! - **Debuggability**: Messages can be logged, replayed, and inspected
//! - **Testability**: Business logic becomes pure functions that are easy to test
//! - **Time-travel debugging**: Previous states can be reconstructed by replaying messages
//! - **Undo/Redo**: Message history provides natural undo/redo functionality
//!
//! The trait bounds ensure messages can be sent across threads (for async operations),
//! cloned efficiently (for message queuing), and debugged during development.

use std::fmt::Debug;

/// Marker trait for all message types in Ironwood.
///
/// Messages must be debuggable, cloneable, and safe to send across threads.
/// They represent discrete events or commands that can update application state.
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
///     Reset,
/// }
///
/// impl Message for AppMessage {}
/// ```
pub trait Message: Debug + Clone + Send + Sync + 'static {}

// End of File
