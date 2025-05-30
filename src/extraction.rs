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
//!
//! ## Dynamic Extraction
//!
//! The extraction system also provides a `ViewRegistry` for dynamic view extraction,
//! enabling runtime type dispatch for view extraction. This allows backends to
//! extract any registered view type from a `Box<dyn View>` without knowing the
//! concrete type at compile time.

use std::{
    any::{Any, TypeId, type_name, type_name_of_val},
    collections::HashMap,
    fmt::{Debug, Formatter, Result as FormatterResult},
};

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

/// A registry that maps view types to their extraction and conversion functions.
///
/// The `ViewRegistry` enables dynamic view extraction by storing type-erased
/// extraction functions that can be looked up at runtime based on the concrete
/// type of a view. This allows backends to extract any registered view type
/// from a `Box<dyn View>` without knowing the concrete type at compile time.
///
/// ## How It Works
///
/// 1. **Registration**: View types are registered with their extraction functions
/// 2. **Lookup**: At runtime, the registry uses `TypeId` to find the right extractor
/// 3. **Extraction**: The type-erased function is called to extract the view
/// 4. **Conversion**: Optional conversion functions transform extracted types
/// 5. **Type Safety**: All operations are type-safe despite the dynamic dispatch
///
/// ## Example Usage
///
/// ```rust
/// use ironwood::{prelude::*, backends::mock::MockBackend};
///
/// let mut registry = ViewRegistry::new();
///
/// // Register view types with their extractors
/// registry.register::<Text, MockBackend>();
/// registry.register::<ButtonView, MockBackend>();
///
/// // Now any Text or ButtonView can be extracted dynamically
/// let view: Box<dyn View> = Box::new(Text::new("Hello"));
/// let ctx = RenderContext::new();
/// let extracted = registry.extract_dynamic::<MockBackend>(view.as_ref(), &ctx);
/// ```
///
/// ## Thread Safety
///
/// The registry is designed to be thread-safe for read operations after initial
/// setup. Registration should typically happen during application initialization
/// before any concurrent access.
#[derive(Default)]
pub struct ViewRegistry {
    /// Maps TypeId to type-erased extraction functions
    ///
    /// Each function takes (&dyn Any, &RenderContext) and returns ExtractionResult&lt;Box&lt;dyn Any&gt;&gt;
    /// The Any types represent the view and extracted output respectively
    #[allow(clippy::type_complexity)]
    extractors: HashMap<
        TypeId,
        Box<dyn Fn(&dyn Any, &RenderContext) -> ExtractionResult<Box<dyn Any>> + Send + Sync>,
    >,

    /// Maps TypeId to type-erased conversion functions
    ///
    /// Each function takes Box&lt;dyn Any&gt; (extracted output) and returns ExtractionResult&lt;Box&lt;dyn Any&gt;&gt; (converted output)
    /// This enables backends to register custom conversion logic for their extracted types
    #[allow(clippy::type_complexity)]
    converters:
        HashMap<TypeId, Box<dyn Fn(Box<dyn Any>) -> ExtractionResult<Box<dyn Any>> + Send + Sync>>,
}

impl ViewRegistry {
    /// Create a new empty view registry.
    ///
    /// The registry starts empty and view types must be explicitly registered
    /// before they can be extracted dynamically.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ironwood::extraction::ViewRegistry;
    ///
    /// let registry = ViewRegistry::new();
    /// assert!(!registry.is_registered::<ironwood::elements::Text>());
    /// ```
    pub fn new() -> Self {
        Self {
            extractors: HashMap::new(),
            converters: HashMap::new(),
        }
    }

    /// Register a view type with its extraction function for a specific backend.
    ///
    /// This method creates a type-erased wrapper around the backend's ViewExtractor
    /// implementation and stores it in the registry for runtime lookup.
    ///
    /// ## Type Parameters
    ///
    /// - `V`: The view type to register (must implement View)
    /// - `B`: The backend type that can extract this view
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ironwood::{prelude::*, backends::mock::MockBackend};
    ///
    /// let mut registry = ViewRegistry::new();
    /// registry.register::<Text, MockBackend>();
    ///
    /// assert!(registry.is_registered::<Text>());
    /// ```
    pub fn register<V, B>(&mut self)
    where
        V: View + 'static,
        B: ViewExtractor<V>,
        B::Output: 'static,
    {
        let type_id = TypeId::of::<V>();

        // Create a type-erased extraction function
        let extractor = Box::new(
            move |view_any: &dyn Any, ctx: &RenderContext| -> ExtractionResult<Box<dyn Any>> {
                // Downcast the view from Any to the concrete type
                let view = view_any.downcast_ref::<V>().ok_or_else(|| {
                    ExtractionError::DowncastFailed {
                        expected_type: type_name::<V>(),
                        actual_type_id: (*view_any).type_id(),
                    }
                })?;

                // Extract using the backend's ViewExtractor implementation
                let extracted = B::extract(view, ctx)?;

                // Box the result as Any for type erasure
                Ok(Box::new(extracted))
            },
        );

        self.extractors.insert(type_id, extractor);
    }

    /// Register a conversion function for a view type.
    ///
    /// This allows backends to register custom logic for converting extracted types
    /// into backend-specific representations, eliminating the need for hardcoded
    /// type checking in backend implementations.
    ///
    /// ## Type Parameters
    ///
    /// - `V`: The original view type
    /// - `E`: The extracted type (output of ViewExtractor)
    /// - `C`: The converted type (backend-specific representation)
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ironwood::{prelude::*, backends::{MockBackend, MockText, MockDynamicChild}};
    ///
    /// let mut registry = ViewRegistry::new();
    /// registry.register::<Text, MockBackend>();
    /// registry.register_converter::<Text, MockText, MockDynamicChild, _>(
    ///     |extracted| MockDynamicChild::Text(extracted)
    /// );
    /// ```
    pub fn register_converter<V, E, C, F>(&mut self, converter: F)
    where
        V: View + 'static,
        E: 'static,
        C: 'static,
        F: Fn(E) -> C + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<V>();

        // Create a type-erased conversion function
        let type_erased_converter = Box::new(
            move |extracted_any: Box<dyn Any>| -> ExtractionResult<Box<dyn Any>> {
                // Downcast the extracted value to the expected type
                let extracted = extracted_any.downcast::<E>().map_err(|_| {
                    ExtractionError::OutputDowncastFailed {
                        expected_type: type_name::<E>(),
                    }
                })?;

                // Apply the conversion function
                let converted = converter(*extracted);

                // Box the result as Any for type erasure
                Ok(Box::new(converted))
            },
        );

        self.converters.insert(type_id, type_erased_converter);
    }

    /// Check if a view type is registered in this registry.
    ///
    /// This is useful for debugging and validation to ensure all required
    /// view types have been registered before attempting dynamic extraction.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ironwood::{prelude::*, backends::MockBackend};
    ///
    /// let mut registry = ViewRegistry::new();
    /// assert!(!registry.is_registered::<Text>());
    ///
    /// registry.register::<Text, MockBackend>();
    /// assert!(registry.is_registered::<Text>());
    /// ```
    pub fn is_registered<V: View + 'static>(&self) -> bool {
        self.extractors.contains_key(&TypeId::of::<V>())
    }

    /// Extract a view dynamically using the registered extraction function.
    ///
    /// This method looks up the extraction function for the view's concrete type
    /// and calls it to extract the view. The result is returned as a boxed Any
    /// that can be downcast to the expected output type.
    ///
    /// ## Type Parameters
    ///
    /// - `B`: The backend type (used for type inference of the output)
    ///
    /// ## Errors
    ///
    /// Returns `ExtractionError::UnregisteredType` if the view type is not registered
    /// in this registry. Returns `ExtractionError::DowncastFailed` if type casting fails.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ironwood::{prelude::*, backends::MockBackend};
    ///
    /// let mut registry = ViewRegistry::new();
    /// registry.register::<Text, MockBackend>();
    ///
    /// let view: Box<dyn View> = Box::new(Text::new("Hello"));
    /// let ctx = RenderContext::new();
    /// let extracted = registry.extract_dynamic::<MockBackend>(view.as_ref(), &ctx);
    /// # Ok::<(), ironwood::extraction::ExtractionError>(())
    /// ```
    pub fn extract_dynamic<B>(
        &self,
        view: &dyn View,
        ctx: &RenderContext,
    ) -> ExtractionResult<Box<dyn Any>>
    where
        B: 'static,
    {
        let type_id = view.type_id();

        let extractor =
            self.extractors
                .get(&type_id)
                .ok_or_else(|| ExtractionError::UnregisteredType {
                    type_name: type_name_of_val(view),
                    type_id,
                })?;

        // Call the type-erased extraction function
        extractor(view.as_any(), ctx)
    }

    /// Extract and convert a view dynamically using registered functions.
    ///
    /// This method first extracts the view using the registered extraction function,
    /// then optionally applies a conversion function if one is registered for the view type.
    ///
    /// ## Type Parameters
    ///
    /// - `B`: The backend type (used for type inference of the output)
    ///
    /// ## Errors
    ///
    /// Returns the same errors as `extract_dynamic`, plus potential conversion errors.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ironwood::{prelude::*, backends::MockBackend};
    ///
    /// let mut registry = ViewRegistry::new();
    /// registry.register::<Text, MockBackend>();
    /// // ... register converter if needed
    ///
    /// let view: Box<dyn View> = Box::new(Text::new("Hello"));
    /// let ctx = RenderContext::new();
    /// let result = registry.extract_and_convert::<MockBackend>(view.as_ref(), &ctx)?;
    /// # Ok::<(), ironwood::extraction::ExtractionError>(())
    /// ```
    pub fn extract_and_convert<B>(
        &self,
        view: &dyn View,
        ctx: &RenderContext,
    ) -> ExtractionResult<Box<dyn Any>>
    where
        B: 'static,
    {
        let type_id = view.type_id();

        // First, extract the view
        let extracted = self.extract_dynamic::<B>(view, ctx)?;

        // Then, apply conversion if available
        if let Some(converter) = self.converters.get(&type_id) {
            converter(extracted)
        } else {
            Ok(extracted)
        }
    }

    /// Get the number of registered view types.
    ///
    /// This is primarily useful for debugging and testing to verify that
    /// the expected number of view types have been registered.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ironwood::{prelude::*, backends::MockBackend};
    ///
    /// let mut registry = ViewRegistry::new();
    /// assert_eq!(registry.len(), 0);
    ///
    /// registry.register::<Text, MockBackend>();
    /// assert_eq!(registry.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.extractors.len()
    }

    /// Check if the registry is empty (no view types registered).
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ironwood::extraction::ViewRegistry;
    ///
    /// let registry = ViewRegistry::new();
    /// assert!(registry.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.extractors.is_empty()
    }
}

impl Debug for ViewRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        f.debug_struct("ViewRegistry")
            .field("registered_types", &self.extractors.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::Text;

    #[test]
    fn basic_extraction() {
        struct TestBackend;

        impl ViewExtractor<Text> for TestBackend {
            type Output = String;
            fn extract(view: &Text, _ctx: &RenderContext) -> ExtractionResult<Self::Output> {
                Ok(view.content.clone())
            }
        }

        let text = Text::new("Hello");
        let ctx = RenderContext::new();
        let result = TestBackend::extract(&text, &ctx).unwrap();
        assert_eq!(result, "Hello");
    }
}

// End of File
