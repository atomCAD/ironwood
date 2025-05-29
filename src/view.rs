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

use std::{any::Any, fmt::Debug};

/// Marker trait for all view types in Ironwood.
///
/// Views are pure data structures that describe the UI hierarchy.
/// They contain no rendering logic - that's handled by backends through
/// the ViewExtractor pattern.
///
/// ## Dynamic View Support
///
/// The View trait now supports dynamic dispatch through type identification
/// methods. This enables runtime view composition and extraction without
/// knowing concrete types at compile time.
///
/// # Examples
///
/// ```
/// use std::any::Any;
/// use ironwood::prelude::*;
///
/// #[derive(Debug, Clone)]
/// struct CustomView {
///     content: String,
/// }
///
/// impl View for CustomView {
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
///
/// // Views can now be used dynamically
/// let view: Box<dyn View> = Box::new(CustomView { content: "Hello".to_string() });
/// let any = view.as_any();
/// let downcast_view = any.downcast_ref::<CustomView>().unwrap();
/// assert_eq!(downcast_view.content, "Hello");
/// ```
#[diagnostic::on_unimplemented(
    message = "the trait `View` is not implemented for `{Self}`",
    note = "if `{Self}` is a model (like Button), try calling `.view()` on it first",
    note = "models implement the `Model` trait and need `.view()` to get their view representation",
    note = "only view types (like ButtonView, Text, VStack, HStack) implement the `View` trait directly"
)]
pub trait View: Debug + Send + Sync + Any + 'static {
    /// Get a reference to this view as `&dyn Any`.
    ///
    /// This enables downcasting from trait objects back to concrete types,
    /// which is necessary for type-safe dynamic extraction. The type registry
    /// uses this method to convert `&dyn View` to `&dyn Any` for downcasting.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let text = Text::new("Hello");
    /// let view: &dyn View = &text;
    ///
    /// // Can downcast back to concrete type
    /// let any = view.as_any();
    /// let downcast_text = any.downcast_ref::<Text>().unwrap();
    /// assert_eq!(downcast_text.content, "Hello");
    /// ```
    fn as_any(&self) -> &dyn Any;
}

// Dynamic view collection implementation
impl View for Vec<Box<dyn View>> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Unit type implementation for utility types that don't have visual representation
impl View for () {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Optional view implementation - enables conditional rendering
///
/// When Some(view), the view is rendered; when None, nothing is rendered
impl<V: View> View for Option<V> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Tuple composition implementations - the core composition mechanism
///
/// This allows combining multiple views into a single composite view.
/// Supports up to 12-tuple arity for comprehensive composition capabilities.
/// Two-element tuple view composition
impl<V1: View, V2: View> View for (V1, V2) {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Three-element tuple view composition
impl<V1: View, V2: View, V3: View> View for (V1, V2, V3) {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Four-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View> View for (V1, V2, V3, V4) {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Five-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View> View for (V1, V2, V3, V4, V5) {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Six-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View, V6: View> View for (V1, V2, V3, V4, V5, V6) {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Seven-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View> View
    for (V1, V2, V3, V4, V5, V6, V7)
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Eight-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View, V8: View> View
    for (V1, V2, V3, V4, V5, V6, V7, V8)
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Nine-element tuple view composition
impl<V1: View, V2: View, V3: View, V4: View, V5: View, V6: View, V7: View, V8: View, V9: View> View
    for (V1, V2, V3, V4, V5, V6, V7, V8, V9)
{
    fn as_any(&self) -> &dyn Any {
        self
    }
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
    fn as_any(&self) -> &dyn Any {
        self
    }
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
    fn as_any(&self) -> &dyn Any {
        self
    }
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// End of File
