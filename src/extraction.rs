// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! View extraction system for Ironwood UI Framework
//!
//! The extraction system defines the interface between view descriptions and
//! backend rendering implementations. Views are pure data structures that
//! describe what should be displayed, while the ViewExtractor pattern allows
//! different backends to process these descriptions in platform-specific ways.
//!
//! This separation enables the same view hierarchy to be rendered on different
//! platforms (GPU, native widgets, web, testing) without changing application code.

use std::any::TypeId;

use crate::view::View;

/// Errors that can occur during view extraction.
///
/// These errors represent various failure modes in the dynamic view extraction
/// system, providing detailed context for debugging and error handling.
#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    /// A view type is not registered in the view registry.
    ///
    /// This occurs when attempting to extract a view type that hasn't been
    /// registered with the backend's registry. The error includes both the
    /// human-readable type name and the TypeId for debugging.
    #[error("View type '{type_name}' is not registered in the view registry")]
    UnregisteredType {
        /// Human-readable name of the unregistered type
        type_name: &'static str,
        /// TypeId of the unregistered type for debugging
        type_id: TypeId,
    },

    /// Failed to downcast a view to the expected concrete type.
    ///
    /// This indicates a type registry invariant violation where the stored
    /// extraction function expects a different type than what was provided.
    #[error("Failed to downcast view to expected type '{expected_type}'")]
    DowncastFailed {
        /// The expected concrete type name
        expected_type: &'static str,
        /// The actual TypeId that was encountered
        actual_type_id: TypeId,
    },

    /// Failed to downcast extracted output to the expected type.
    ///
    /// This occurs when the extraction function returns a different type
    /// than expected, indicating a mismatch in the registry configuration.
    #[error("Failed to downcast extracted output to expected type '{expected_type}'")]
    OutputDowncastFailed {
        /// The expected output type name
        expected_type: &'static str,
    },
}

/// Result type for view extraction operations.
///
/// This type alias provides a convenient way to work with extraction results
/// throughout the codebase, ensuring consistent error handling.
pub type ExtractionResult<T> = Result<T, ExtractionError>;

/// Context provided to view extractors during rendering.
///
/// The render context contains platform-specific information that backends
/// need to properly extract and render views. This might include theme data,
/// font information, screen dimensions, or other rendering parameters.
///
/// For now, this is a placeholder that will be expanded as the framework grows.
#[derive(Debug, Clone)]
pub struct RenderContext {
    // Future: theme data, font registry, screen info, etc.
    _placeholder: (),
}

impl RenderContext {
    /// Create a new render context with default settings.
    ///
    /// This will be expanded to include actual context data as the framework develops.
    pub fn new() -> Self {
        Self { _placeholder: () }
    }
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for extracting view data into backend-specific representations.
///
/// The ViewExtractor pattern allows different backends to process the same
/// view types in different ways. For example, a GPU backend might extract
/// views into vertex buffers and textures, while a testing backend might
/// extract them into simple data structures for assertions.
///
/// Each backend implements ViewExtractor for the view types it supports,
/// defining how to convert the pure view data into whatever representation
/// the backend needs for rendering.
///
/// # Examples
///
/// ```
/// use ironwood::prelude::*;
///
/// // A simple backend that extracts text to strings
/// struct StringBackend;
///
/// impl ViewExtractor<Text> for StringBackend {
///     type Output = String;
///
///     fn extract(view: &Text, _ctx: &RenderContext) -> ExtractionResult<Self::Output> {
///         Ok(view.content.clone())
///     }
/// }
///
/// let text = Text::new("Hello, world!");
/// let ctx = RenderContext::new();
/// let result = StringBackend::extract(&text, &ctx).unwrap();
/// assert_eq!(result, "Hello, world!");
/// ```
pub trait ViewExtractor<V: View> {
    /// The backend-specific representation of the extracted view.
    ///
    /// This could be anything the backend needs: render commands, widget handles,
    /// test data structures, etc. The type is determined by what the backend
    /// needs to efficiently render or process the view.
    type Output;

    /// Extract a view into the backend's representation.
    ///
    /// This method takes a view and the current render context, and produces
    /// whatever output the backend needs to handle that view. The extraction
    /// process should be pure - it shouldn't have side effects or modify
    /// global state.
    ///
    /// # Arguments
    ///
    /// * `view` - The view to extract
    /// * `ctx` - The current render context with platform information
    ///
    /// # Returns
    ///
    /// The backend-specific representation of the view, or an error
    fn extract(view: &V, ctx: &RenderContext) -> ExtractionResult<Self::Output>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::Text;

    #[test]
    fn view_extractor_functionality() {
        // Test that ViewExtractor trait works correctly with custom backends
        struct StringBackend;

        impl ViewExtractor<Text> for StringBackend {
            type Output = String;

            fn extract(view: &Text, _ctx: &RenderContext) -> ExtractionResult<Self::Output> {
                Ok(view.content.clone())
            }
        }

        let text = Text::new("Hello, world!");
        let ctx = RenderContext::new();
        let result = StringBackend::extract(&text, &ctx).unwrap();
        assert_eq!(result, "Hello, world!");
    }
}

// End of File
