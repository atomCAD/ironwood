// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! View system for Ironwood UI Framework
//!
//! Views are pure data structures that describe the UI hierarchy.
//! They contain no rendering logic - that's handled by backends through
//! the ViewExtractor pattern.
//!
//! Views follow the principle of separation of concerns: they describe
//! what the UI should look like, while rendering backends determine how
//! to display that description on specific platforms.

use std::fmt::Debug;

/// Marker trait for all view types in Ironwood.
///
/// Views are pure data structures that describe the UI hierarchy.
/// They contain no rendering logic - that's handled by backends through
/// the ViewExtractor pattern.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// #[derive(Debug, Clone)]
/// struct CustomView {
///     content: String,
/// }
///
/// impl View for CustomView {}
/// ```
pub trait View: Debug + Clone {}

// End of File
