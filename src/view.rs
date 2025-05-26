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

/// Unit type implementation for utility types that don't have visual representation
impl View for () {}

/// Optional view implementation - enables conditional rendering
///
/// When Some(view), the view is rendered; when None, nothing is rendered
impl<V: View> View for Option<V> {}

/// Tuple composition implementations - the core composition mechanism
///
/// This allows combining multiple views into a single composite view.
/// Supports up to 12-tuple arity for comprehensive composition capabilities.
/// Two-element tuple view composition
impl<V1: View, V2: View> View for (V1, V2) {}

/// Three-element tuple view composition
impl<V1: View, V2: View, V3: View> View for (V1, V2, V3) {}

/// Four-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View> View for (V1, V2, V3, V4) {}

/// Five-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View> View for (V1, V2, V3, V4, V5) {}

/// Six-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View, V6: View> View for (V1, V2, V3, V4, V5, V6) {}

/// Seven-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View> View
    for (V1, V2, V3, V4, V5, V6, V7)
{
}

/// Eight-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View, V8: View> View
    for (V1, V2, V3, V4, V5, V6, V7, V8)
{
}

/// Nine-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View, V8: View, V9: View> View
    for (V1, V2, V3, V4, V5, V6, V7, V8, V9)
{
}

/// Ten-element tuple view composition
impl<
    V1: View,
    V2: View,
    V3: View,
    V4: View,
    V5: View,
    V6: View,
    V7: View,
    V8: View,
    V9: View,
    V10: View,
> View for (V1, V2, V3, V4, V5, V6, V7, V8, V9, V10)
{
}

/// Eleven-element tuple view composition
impl<
    V1: View,
    V2: View,
    V3: View,
    V4: View,
    V5: View,
    V6: View,
    V7: View,
    V8: View,
    V9: View,
    V10: View,
    V11: View,
> View for (V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11)
{
}

/// Twelve-element tuple view composition
impl<
    V1: View,
    V2: View,
    V3: View,
    V4: View,
    V5: View,
    V6: View,
    V7: View,
    V8: View,
    V9: View,
    V10: View,
    V11: View,
    V12: View,
> View for (V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12)
{
}

// End of File
