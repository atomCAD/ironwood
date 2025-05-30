// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Mock backend for testing view extraction
//!
//! The mock backend extracts views into simple, testable data structures.
//! This allows verification that views are being created correctly and
//! that their properties are accessible without needing a full rendering
//! pipeline.
//!
//! The mock backend is also useful for automated testing, as it produces
//! deterministic output that can be easily compared in assertions.

use std::{any::type_name, fmt::Debug};

use crate::{
    elements::{Alignment, HStack, Spacer, Text, VStack},
    extraction::{ExtractionError, ExtractionResult, RenderContext, ViewExtractor, ViewRegistry},
    interaction::InteractionState,
    style::{Color, TextStyle},
    view::View,
    widgets::ButtonView,
};

/// Mock backend for testing view extraction.
///
/// The MockBackend extracts views into simple, testable data structures.
/// This allows us to verify that views are being created correctly and
/// that their properties are accessible without needing a full rendering
/// pipeline.
///
/// The mock backend is also useful for automated testing, as it produces
/// deterministic output that can be easily compared in assertions.
///
/// # Examples
///
/// ```
/// use ironwood::{prelude::*, backends::mock::MockBackend, extraction::ViewExtractor};
///
/// let text = Text::new("Hello, world!");
/// let ctx = RenderContext::new();
/// let extracted = MockBackend::extract(&text, &ctx).unwrap();
/// assert_eq!(extracted.content, "Hello, world!");
///
/// // Or use the backend's dynamic extraction for trait objects
/// let backend = MockBackend::new();
/// let view: Box<dyn View> = Box::new(Text::new("Dynamic"));
/// let dynamic_extracted = backend.extract_dynamic(view.as_ref(), &ctx).unwrap();
/// ```
pub struct MockBackend {
    /// Type registry for dynamic view extraction
    registry: ViewRegistry,
}

/// Mock representation of extracted text for testing.
///
/// This captures all the essential information from a Text view in a format
/// that's easy to test against. The mock backend uses this to verify that
/// text views are being extracted correctly.
#[derive(Debug, Clone, PartialEq)]
pub struct MockText {
    /// The text content
    pub content: String,
    /// Font size in logical pixels
    pub font_size: f32,
    /// Text color
    pub color: Color,
}

impl MockBackend {
    /// Create a new MockBackend with a configured type registry.
    ///
    /// This sets up all the view types that the MockBackend can handle,
    /// including both static extraction and dynamic conversion functions.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::backends::mock::MockBackend;
    ///
    /// let backend = MockBackend::new();
    /// // Backend is ready to extract any registered view type
    /// ```
    pub fn new() -> Self {
        let mut registry = ViewRegistry::new();

        // Register view types with their extractors
        registry.register::<Text, MockBackend>();
        registry.register::<ButtonView, MockBackend>();
        registry.register::<Spacer, MockBackend>();
        registry.register::<VStack<Vec<Box<dyn View>>>, MockBackend>();
        registry.register::<HStack<Vec<Box<dyn View>>>, MockBackend>();

        // Register conversion functions for dynamic extraction
        registry.register_converter::<Text, MockText, MockDynamicChild, _>(MockDynamicChild::Text);

        registry.register_converter::<ButtonView, MockButton, MockDynamicChild, _>(
            MockDynamicChild::Button,
        );

        registry.register_converter::<Spacer, MockSpacer, MockDynamicChild, _>(
            MockDynamicChild::Spacer,
        );

        registry.register_converter::<
            VStack<Vec<Box<dyn View>>>,
            MockVStack<Vec<MockDynamicChild>>,
            MockDynamicChild,
            _,
        >(
            MockDynamicChild::VStack,
        );

        registry.register_converter::<
            HStack<Vec<Box<dyn View>>>,
            MockHStack<Vec<MockDynamicChild>>,
            MockDynamicChild,
            _,
        >(
            MockDynamicChild::HStack,
        );

        Self { registry }
    }

    /// Extract a view dynamically using the backend's type registry.
    ///
    /// This method can extract any view type that has been registered with
    /// the backend, returning the appropriate MockDynamicChild variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::{prelude::*, backends::mock::MockBackend};
    ///
    /// let backend = MockBackend::new();
    /// let view: Box<dyn View> = Box::new(Text::new("Hello"));
    /// let ctx = RenderContext::new();
    /// let extracted = backend.extract_dynamic(view.as_ref(), &ctx).unwrap();
    /// ```
    pub fn extract_dynamic(
        &self,
        view: &dyn View,
        context: &RenderContext,
    ) -> ExtractionResult<MockDynamicChild> {
        // Extract and convert using the registry
        let converted = self
            .registry
            .extract_and_convert::<MockBackend>(view, context)?;

        // The registry guarantees this will be a MockDynamicChild
        Ok(*converted.downcast::<MockDynamicChild>().map_err(|_| {
            ExtractionError::OutputDowncastFailed {
                expected_type: type_name::<MockDynamicChild>(),
            }
        })?)
    }
}

impl Default for MockBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewExtractor<Text> for MockBackend {
    type Output = MockText;

    fn extract(view: &Text, _ctx: &RenderContext) -> ExtractionResult<Self::Output> {
        // Extract all the essential data from the Text view
        // This demonstrates how backends can access view properties
        Ok(MockText {
            content: view.content.clone(),
            font_size: view.style.font_size,
            color: view.style.color,
        })
    }
}

/// Mock representation of extracted button for testing.
///
/// This captures the information from a Button component that's relevant for
/// display and rendering, including visual states like pressed/focused that
/// affect how the button should appear on screen.
#[derive(Debug, Clone, PartialEq)]
pub struct MockButton {
    /// The button text
    pub text: String,
    /// Background color
    pub background_color: Color,
    /// Text styling properties
    pub text_style: TextStyle,
    /// The interaction state of the button
    pub interaction_state: InteractionState,
}

impl ViewExtractor<ButtonView> for MockBackend {
    type Output = MockButton;

    fn extract(view: &ButtonView, _ctx: &RenderContext) -> ExtractionResult<Self::Output> {
        // Extract button component display information for testing
        Ok(MockButton {
            text: view.text.content.clone(),
            background_color: view.background_color,
            text_style: view.text.style,
            interaction_state: view.interaction_state,
        })
    }
}

/// Mock representation of extracted spacer for testing.
///
/// This captures the spacer properties that affect layout calculations.
#[derive(Debug, Clone, PartialEq)]
pub struct MockSpacer {
    /// Minimum size for the spacer in logical pixels
    pub min_size: f32,
}

impl ViewExtractor<Spacer> for MockBackend {
    type Output = MockSpacer;

    fn extract(view: &Spacer, _ctx: &RenderContext) -> ExtractionResult<Self::Output> {
        Ok(MockSpacer {
            min_size: view.min_size,
        })
    }
}

// Optional view extraction - returns Some(extracted) or None
impl<V> ViewExtractor<Option<V>> for MockBackend
where
    V: View,
    Self: ViewExtractor<V>,
{
    type Output = Option<<Self as ViewExtractor<V>>::Output>;

    fn extract(view: &Option<V>, context: &RenderContext) -> ExtractionResult<Self::Output> {
        view.as_ref()
            .map(|inner| Self::extract(inner, context))
            .transpose()
    }
}

// Tuple extraction implementations - return tuples of extracted outputs
// For simplicity and to avoid type recursion issues, we'll implement a few key arities
impl<V1, V2> ViewExtractor<(V1, V2)> for MockBackend
where
    V1: View,
    V2: View,
    Self: ViewExtractor<V1> + ViewExtractor<V2>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
    );

    fn extract(view: &(V1, V2), context: &RenderContext) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
        ))
    }
}

impl<V1, V2, V3> ViewExtractor<(V1, V2, V3)> for MockBackend
where
    V1: View,
    V2: View,
    V3: View,
    Self: ViewExtractor<V1> + ViewExtractor<V2> + ViewExtractor<V3>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
    );

    fn extract(view: &(V1, V2, V3), context: &RenderContext) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
        ))
    }
}

impl<V1, V2, V3, V4> ViewExtractor<(V1, V2, V3, V4)> for MockBackend
where
    V1: View,
    V2: View,
    V3: View,
    V4: View,
    Self: ViewExtractor<V1> + ViewExtractor<V2> + ViewExtractor<V3> + ViewExtractor<V4>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
        <Self as ViewExtractor<V4>>::Output,
    );

    fn extract(view: &(V1, V2, V3, V4), context: &RenderContext) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
            Self::extract(&view.3, context)?,
        ))
    }
}

impl<V1, V2, V3, V4, V5> ViewExtractor<(V1, V2, V3, V4, V5)> for MockBackend
where
    V1: View,
    V2: View,
    V3: View,
    V4: View,
    V5: View,
    Self: ViewExtractor<V1>
        + ViewExtractor<V2>
        + ViewExtractor<V3>
        + ViewExtractor<V4>
        + ViewExtractor<V5>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
        <Self as ViewExtractor<V4>>::Output,
        <Self as ViewExtractor<V5>>::Output,
    );

    fn extract(
        view: &(V1, V2, V3, V4, V5),
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
            Self::extract(&view.3, context)?,
            Self::extract(&view.4, context)?,
        ))
    }
}

impl<V1, V2, V3, V4, V5, V6> ViewExtractor<(V1, V2, V3, V4, V5, V6)> for MockBackend
where
    V1: View,
    V2: View,
    V3: View,
    V4: View,
    V5: View,
    V6: View,
    Self: ViewExtractor<V1>
        + ViewExtractor<V2>
        + ViewExtractor<V3>
        + ViewExtractor<V4>
        + ViewExtractor<V5>
        + ViewExtractor<V6>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
        <Self as ViewExtractor<V4>>::Output,
        <Self as ViewExtractor<V5>>::Output,
        <Self as ViewExtractor<V6>>::Output,
    );

    fn extract(
        view: &(V1, V2, V3, V4, V5, V6),
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
            Self::extract(&view.3, context)?,
            Self::extract(&view.4, context)?,
            Self::extract(&view.5, context)?,
        ))
    }
}

impl<V1, V2, V3, V4, V5, V6, V7> ViewExtractor<(V1, V2, V3, V4, V5, V6, V7)> for MockBackend
where
    V1: View,
    V2: View,
    V3: View,
    V4: View,
    V5: View,
    V6: View,
    V7: View,
    Self: ViewExtractor<V1>
        + ViewExtractor<V2>
        + ViewExtractor<V3>
        + ViewExtractor<V4>
        + ViewExtractor<V5>
        + ViewExtractor<V6>
        + ViewExtractor<V7>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
        <Self as ViewExtractor<V4>>::Output,
        <Self as ViewExtractor<V5>>::Output,
        <Self as ViewExtractor<V6>>::Output,
        <Self as ViewExtractor<V7>>::Output,
    );

    fn extract(
        view: &(V1, V2, V3, V4, V5, V6, V7),
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
            Self::extract(&view.3, context)?,
            Self::extract(&view.4, context)?,
            Self::extract(&view.5, context)?,
            Self::extract(&view.6, context)?,
        ))
    }
}

impl<V1, V2, V3, V4, V5, V6, V7, V8> ViewExtractor<(V1, V2, V3, V4, V5, V6, V7, V8)> for MockBackend
where
    V1: View,
    V2: View,
    V3: View,
    V4: View,
    V5: View,
    V6: View,
    V7: View,
    V8: View,
    Self: ViewExtractor<V1>
        + ViewExtractor<V2>
        + ViewExtractor<V3>
        + ViewExtractor<V4>
        + ViewExtractor<V5>
        + ViewExtractor<V6>
        + ViewExtractor<V7>
        + ViewExtractor<V8>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
        <Self as ViewExtractor<V4>>::Output,
        <Self as ViewExtractor<V5>>::Output,
        <Self as ViewExtractor<V6>>::Output,
        <Self as ViewExtractor<V7>>::Output,
        <Self as ViewExtractor<V8>>::Output,
    );

    fn extract(
        view: &(V1, V2, V3, V4, V5, V6, V7, V8),
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
            Self::extract(&view.3, context)?,
            Self::extract(&view.4, context)?,
            Self::extract(&view.5, context)?,
            Self::extract(&view.6, context)?,
            Self::extract(&view.7, context)?,
        ))
    }
}

impl<V1, V2, V3, V4, V5, V6, V7, V8, V9> ViewExtractor<(V1, V2, V3, V4, V5, V6, V7, V8, V9)>
    for MockBackend
where
    V1: View,
    V2: View,
    V3: View,
    V4: View,
    V5: View,
    V6: View,
    V7: View,
    V8: View,
    V9: View,
    Self: ViewExtractor<V1>
        + ViewExtractor<V2>
        + ViewExtractor<V3>
        + ViewExtractor<V4>
        + ViewExtractor<V5>
        + ViewExtractor<V6>
        + ViewExtractor<V7>
        + ViewExtractor<V8>
        + ViewExtractor<V9>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
        <Self as ViewExtractor<V4>>::Output,
        <Self as ViewExtractor<V5>>::Output,
        <Self as ViewExtractor<V6>>::Output,
        <Self as ViewExtractor<V7>>::Output,
        <Self as ViewExtractor<V8>>::Output,
        <Self as ViewExtractor<V9>>::Output,
    );

    fn extract(
        view: &(V1, V2, V3, V4, V5, V6, V7, V8, V9),
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
            Self::extract(&view.3, context)?,
            Self::extract(&view.4, context)?,
            Self::extract(&view.5, context)?,
            Self::extract(&view.6, context)?,
            Self::extract(&view.7, context)?,
            Self::extract(&view.8, context)?,
        ))
    }
}

impl<V1, V2, V3, V4, V5, V6, V7, V8, V9, V10>
    ViewExtractor<(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10)> for MockBackend
where
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
    Self: ViewExtractor<V1>
        + ViewExtractor<V2>
        + ViewExtractor<V3>
        + ViewExtractor<V4>
        + ViewExtractor<V5>
        + ViewExtractor<V6>
        + ViewExtractor<V7>
        + ViewExtractor<V8>
        + ViewExtractor<V9>
        + ViewExtractor<V10>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
        <Self as ViewExtractor<V4>>::Output,
        <Self as ViewExtractor<V5>>::Output,
        <Self as ViewExtractor<V6>>::Output,
        <Self as ViewExtractor<V7>>::Output,
        <Self as ViewExtractor<V8>>::Output,
        <Self as ViewExtractor<V9>>::Output,
        <Self as ViewExtractor<V10>>::Output,
    );

    fn extract(
        view: &(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10),
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
            Self::extract(&view.3, context)?,
            Self::extract(&view.4, context)?,
            Self::extract(&view.5, context)?,
            Self::extract(&view.6, context)?,
            Self::extract(&view.7, context)?,
            Self::extract(&view.8, context)?,
            Self::extract(&view.9, context)?,
        ))
    }
}

impl<V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11>
    ViewExtractor<(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11)> for MockBackend
where
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
    Self: ViewExtractor<V1>
        + ViewExtractor<V2>
        + ViewExtractor<V3>
        + ViewExtractor<V4>
        + ViewExtractor<V5>
        + ViewExtractor<V6>
        + ViewExtractor<V7>
        + ViewExtractor<V8>
        + ViewExtractor<V9>
        + ViewExtractor<V10>
        + ViewExtractor<V11>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
        <Self as ViewExtractor<V4>>::Output,
        <Self as ViewExtractor<V5>>::Output,
        <Self as ViewExtractor<V6>>::Output,
        <Self as ViewExtractor<V7>>::Output,
        <Self as ViewExtractor<V8>>::Output,
        <Self as ViewExtractor<V9>>::Output,
        <Self as ViewExtractor<V10>>::Output,
        <Self as ViewExtractor<V11>>::Output,
    );

    fn extract(
        view: &(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11),
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
            Self::extract(&view.3, context)?,
            Self::extract(&view.4, context)?,
            Self::extract(&view.5, context)?,
            Self::extract(&view.6, context)?,
            Self::extract(&view.7, context)?,
            Self::extract(&view.8, context)?,
            Self::extract(&view.9, context)?,
            Self::extract(&view.10, context)?,
        ))
    }
}

impl<V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12>
    ViewExtractor<(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12)> for MockBackend
where
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
    Self: ViewExtractor<V1>
        + ViewExtractor<V2>
        + ViewExtractor<V3>
        + ViewExtractor<V4>
        + ViewExtractor<V5>
        + ViewExtractor<V6>
        + ViewExtractor<V7>
        + ViewExtractor<V8>
        + ViewExtractor<V9>
        + ViewExtractor<V10>
        + ViewExtractor<V11>
        + ViewExtractor<V12>,
{
    type Output = (
        <Self as ViewExtractor<V1>>::Output,
        <Self as ViewExtractor<V2>>::Output,
        <Self as ViewExtractor<V3>>::Output,
        <Self as ViewExtractor<V4>>::Output,
        <Self as ViewExtractor<V5>>::Output,
        <Self as ViewExtractor<V6>>::Output,
        <Self as ViewExtractor<V7>>::Output,
        <Self as ViewExtractor<V8>>::Output,
        <Self as ViewExtractor<V9>>::Output,
        <Self as ViewExtractor<V10>>::Output,
        <Self as ViewExtractor<V11>>::Output,
        <Self as ViewExtractor<V12>>::Output,
    );

    fn extract(
        view: &(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12),
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        Ok((
            Self::extract(&view.0, context)?,
            Self::extract(&view.1, context)?,
            Self::extract(&view.2, context)?,
            Self::extract(&view.3, context)?,
            Self::extract(&view.4, context)?,
            Self::extract(&view.5, context)?,
            Self::extract(&view.6, context)?,
            Self::extract(&view.7, context)?,
            Self::extract(&view.8, context)?,
            Self::extract(&view.9, context)?,
            Self::extract(&view.10, context)?,
            Self::extract(&view.11, context)?,
        ))
    }
}

/// Mock representation of a VStack for testing and debugging
#[derive(Debug, Clone, PartialEq)]
pub struct MockVStack<T> {
    /// The extracted content of the VStack
    pub content: T,
    /// The horizontal alignment of child views
    pub alignment: Alignment,
    /// The spacing between child views
    pub spacing: f32,
}

/// Statically typed VStack container extraction
impl<T> ViewExtractor<VStack<T>> for MockBackend
where
    T: View,
    Self: ViewExtractor<T>,
{
    type Output = MockVStack<<Self as ViewExtractor<T>>::Output>;

    fn extract(view: &VStack<T>, context: &RenderContext) -> ExtractionResult<Self::Output> {
        Ok(MockVStack {
            content: Self::extract(&view.content, context)?,
            alignment: view.alignment,
            spacing: view.spacing,
        })
    }
}

/// Dynamically typed VStack container extraction
impl ViewExtractor<VStack<Vec<Box<dyn View>>>> for MockBackend {
    type Output = MockVStack<Vec<MockDynamicChild>>;

    fn extract(
        view: &VStack<Vec<Box<dyn View>>>,
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        // Create a backend instance for dynamic extraction
        let backend = MockBackend::new();

        // Extract each child dynamically using the backend's registry
        let extracted_children: Result<Vec<MockDynamicChild>, _> = view
            .content
            .iter()
            .map(|child| {
                MockDynamicChild::extract_from_view_with_backend(child.as_ref(), context, &backend)
            })
            .collect();

        Ok(MockVStack {
            content: extracted_children?,
            alignment: view.alignment,
            spacing: view.spacing,
        })
    }
}

/// Mock representation of an HStack for testing and debugging
#[derive(Debug, Clone, PartialEq)]
pub struct MockHStack<T> {
    /// The extracted content of the HStack
    pub content: T,
    /// The vertical alignment of child views
    pub alignment: Alignment,
    /// The spacing between child views
    pub spacing: f32,
}

/// Statically typed HStack container extraction
impl<T> ViewExtractor<HStack<T>> for MockBackend
where
    T: View,
    Self: ViewExtractor<T>,
{
    type Output = MockHStack<<Self as ViewExtractor<T>>::Output>;

    fn extract(view: &HStack<T>, context: &RenderContext) -> ExtractionResult<Self::Output> {
        Ok(MockHStack {
            content: Self::extract(&view.content, context)?,
            alignment: view.alignment,
            spacing: view.spacing,
        })
    }
}

/// Dynamically typed HStack container extraction
impl ViewExtractor<HStack<Vec<Box<dyn View>>>> for MockBackend {
    type Output = MockHStack<Vec<MockDynamicChild>>;

    fn extract(
        view: &HStack<Vec<Box<dyn View>>>,
        context: &RenderContext,
    ) -> ExtractionResult<Self::Output> {
        // Create a backend instance for dynamic extraction
        let backend = MockBackend::new();

        // Extract each child dynamically using the backend's registry
        let extracted_children: Result<Vec<MockDynamicChild>, _> = view
            .content
            .iter()
            .map(|child| {
                MockDynamicChild::extract_from_view_with_backend(child.as_ref(), context, &backend)
            })
            .collect();

        Ok(MockHStack {
            content: extracted_children?,
            alignment: view.alignment,
            spacing: view.spacing,
        })
    }
}

/// A type-erased representation of extracted dynamic children.
///
/// This allows the mock backend to handle different types of extracted views
/// in a uniform way while preserving type information for testing.
#[derive(Debug, Clone, PartialEq)]
pub enum MockDynamicChild {
    Text(MockText),
    Button(MockButton),
    Spacer(MockSpacer),
    VStack(MockVStack<Vec<MockDynamicChild>>),
    HStack(MockHStack<Vec<MockDynamicChild>>),
}

impl MockDynamicChild {
    /// Extract a view dynamically into a MockDynamicChild using a backend instance.
    ///
    /// This method uses the backend's type registry to eliminate hardcoded type checking.
    /// All type dispatch is handled by the backend's registry.
    pub fn extract_from_view_with_backend(
        view: &dyn View,
        context: &RenderContext,
        backend: &MockBackend,
    ) -> ExtractionResult<Self> {
        backend.extract_dynamic(view, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        elements::Text,
        interaction::{Enableable, Focusable, Hoverable, InteractionMessage, Pressable},
        model::Model,
        widgets::Button,
        widgets::ButtonMessage,
    };

    #[test]
    fn text_extraction() {
        // Test extracting a basic text view
        let text = Text::new("Hello, world!");
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&text, &ctx).unwrap();

        assert_eq!(extracted.content, "Hello, world!");
        assert_eq!(extracted.font_size, 16.0);
        assert_eq!(extracted.color, Color::BLACK);
    }

    #[test]
    fn styled_text_extraction() {
        // Test extracting a styled text view
        let text = Text::new("Styled text").font_size(24.0).color(Color::RED);
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&text, &ctx).unwrap();

        assert_eq!(extracted.content, "Styled text");
        assert_eq!(extracted.font_size, 24.0);
        assert_eq!(extracted.color, Color::RED);
    }

    #[test]
    fn button_extraction_basic() {
        // Test extracting a basic button component
        let button = Button::new("Click me");
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&button.view(), &ctx).unwrap();

        assert_eq!(extracted.text, "Click me");
        assert_eq!(extracted.background_color, Color::rgb(0.9, 0.9, 0.9));
        assert_eq!(extracted.text_style.color, Color::BLACK);
        assert!(extracted.interaction_state.is_enabled());
        assert!(!extracted.interaction_state.is_pressed());
        assert!(!extracted.interaction_state.is_focused());
        assert!(!extracted.interaction_state.is_hovered());
    }

    #[test]
    fn button_extraction_disabled() {
        // Test extracting a disabled button
        let button = Button::new("Disabled").disable();
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&button.view(), &ctx).unwrap();

        assert_eq!(extracted.text, "Disabled");
        assert!(!extracted.interaction_state.is_enabled());
        assert!(!extracted.interaction_state.is_pressed());
        assert!(!extracted.interaction_state.is_focused());
        assert!(!extracted.interaction_state.is_hovered());
    }

    #[test]
    fn button_extraction_visual_states() {
        // Test extracting button visual states that affect rendering
        let ctx = RenderContext::new();

        // Test clicked button (no state change)
        let clicked_button = Button::new("Clicked").update(ButtonMessage::Clicked);
        let clicked_extracted = MockBackend::extract(&clicked_button.view(), &ctx).unwrap();
        assert_eq!(clicked_extracted.text, "Clicked");
        assert!(clicked_extracted.interaction_state.is_enabled());
        assert!(!clicked_extracted.interaction_state.is_pressed());
        assert!(!clicked_extracted.interaction_state.is_focused());
        assert!(!clicked_extracted.interaction_state.is_hovered());

        // Test focused button
        let focused_button = Button::new("Focused").update(ButtonMessage::Interaction(
            InteractionMessage::FocusChanged(true),
        ));
        let focused_extracted = MockBackend::extract(&focused_button.view(), &ctx).unwrap();
        assert_eq!(focused_extracted.text, "Focused");
        assert!(focused_extracted.interaction_state.is_enabled());
        assert!(!focused_extracted.interaction_state.is_pressed());
        assert!(focused_extracted.interaction_state.is_focused());
        assert!(!focused_extracted.interaction_state.is_hovered());

        // Test pressed button (via press state change)
        let pressed_button = Button::new("Pressed").update(ButtonMessage::Interaction(
            InteractionMessage::PressStateChanged(true),
        ));
        let pressed_extracted = MockBackend::extract(&pressed_button.view(), &ctx).unwrap();
        assert_eq!(pressed_extracted.text, "Pressed");
        assert!(pressed_extracted.interaction_state.is_enabled());
        assert!(pressed_extracted.interaction_state.is_pressed());
        assert!(!pressed_extracted.interaction_state.is_focused());
        assert!(!pressed_extracted.interaction_state.is_hovered());

        // Test hovered button
        let hovered_button = Button::new("Hovered").update(ButtonMessage::Interaction(
            InteractionMessage::HoverChanged(true),
        ));
        let hovered_extracted = MockBackend::extract(&hovered_button.view(), &ctx).unwrap();
        assert_eq!(hovered_extracted.text, "Hovered");
        assert!(hovered_extracted.interaction_state.is_enabled());
        assert!(!hovered_extracted.interaction_state.is_pressed());
        assert!(!hovered_extracted.interaction_state.is_focused());
        assert!(hovered_extracted.interaction_state.is_hovered());
    }

    #[test]
    fn styled_button_extraction() {
        // Test extracting a styled button
        let button = Button::new("Styled")
            .background_color(Color::BLUE)
            .with_text(|text| text.color(Color::WHITE))
            .enable();
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&button.view(), &ctx).unwrap();

        assert_eq!(extracted.text, "Styled");
        assert_eq!(extracted.background_color, Color::BLUE);
        assert_eq!(extracted.text_style.color, Color::WHITE);
        assert!(extracted.interaction_state.is_enabled());
    }

    #[test]
    fn extraction_preserves_view_data() {
        // Test that extraction doesn't modify the original view
        let original_text = Text::new("Original").font_size(20.0);
        let original_button = Button::new("Original").enable();
        let ctx = RenderContext::new();

        // Extract views
        let _extracted_text = MockBackend::extract(&original_text, &ctx).unwrap();
        let _extracted_button = MockBackend::extract(&original_button.view(), &ctx).unwrap();

        // Original views should be unchanged
        assert_eq!(original_text.content, "Original");
        assert_eq!(original_text.style.font_size, 20.0);
        assert_eq!(original_button.text.content, "Original");
        assert!(original_button.is_enabled());
    }

    #[test]
    fn spacer_extraction() {
        // Test extracting spacer views
        let ctx = RenderContext::new();

        // Test default spacer
        let spacer = Spacer::new();
        let extracted = MockBackend::extract(&spacer, &ctx).unwrap();
        assert_eq!(extracted.min_size, 0.0);

        // Test spacer with minimum size
        let sized_spacer = Spacer::min_size(20.0);
        let sized_extracted = MockBackend::extract(&sized_spacer, &ctx).unwrap();
        assert_eq!(sized_extracted.min_size, 20.0);
    }

    #[test]
    fn option_extraction() {
        // Test extracting optional views
        let ctx = RenderContext::new();

        // Test Some(view)
        let some_text = Some(Text::new("Present"));
        let some_extracted = MockBackend::extract(&some_text, &ctx).unwrap();
        assert!(some_extracted.is_some());
        assert_eq!(some_extracted.unwrap().content, "Present");

        // Test None
        let none_text: Option<Text> = None;
        let none_extracted = MockBackend::extract(&none_text, &ctx).unwrap();
        assert!(none_extracted.is_none());
    }

    #[test]
    fn backend_owns_registry_architecture() {
        // This test demonstrates that the registry is now properly part of the backend object
        // This allows for better encapsulation and potentially different backend configurations
        let ctx = RenderContext::new();

        // Create a backend instance - the registry is part of the backend
        let backend = MockBackend::new();

        // The backend can extract views dynamically using its own registry
        let text_view = Text::new("Backend Test");
        let button_view = Button::new("Backend Button").view();

        // Extract using the backend's registry
        let text_extracted = backend.extract_dynamic(&text_view, &ctx).unwrap();
        let button_extracted = backend.extract_dynamic(&button_view, &ctx).unwrap();

        // Verify the extractions worked correctly
        assert!(
            matches!(text_extracted, MockDynamicChild::Text(text) if text.content == "Backend Test")
        );

        assert!(
            matches!(button_extracted, MockDynamicChild::Button(button) if button.text == "Backend Button")
        );

        // The backend encapsulates its registry - no global state needed
        // Different backend instances could potentially have different registries
        let another_backend = MockBackend::new();
        let another_extracted = another_backend.extract_dynamic(&text_view, &ctx).unwrap();

        assert!(
            matches!(another_extracted, MockDynamicChild::Text(text) if text.content == "Backend Test")
        );
    }

    #[test]
    fn tuple_extraction_comprehensive() {
        let ctx = RenderContext::new();

        // Test 2-tuple extraction
        let tuple2 = (Text::new("First"), Text::new("Second"));
        let extracted2 = MockBackend::extract(&tuple2, &ctx).unwrap();
        assert_eq!(extracted2.0.content, "First");
        assert_eq!(extracted2.1.content, "Second");

        // Test 3-tuple extraction
        let tuple3 = (Text::new("One"), Text::new("Two"), Text::new("Three"));
        let extracted3 = MockBackend::extract(&tuple3, &ctx).unwrap();
        assert_eq!(extracted3.0.content, "One");
        assert_eq!(extracted3.1.content, "Two");
        assert_eq!(extracted3.2.content, "Three");

        // Test 4-tuple extraction
        let tuple4 = (
            Text::new("A"),
            Text::new("B"),
            Text::new("C"),
            Text::new("D"),
        );
        let extracted4 = MockBackend::extract(&tuple4, &ctx).unwrap();
        assert_eq!(extracted4.0.content, "A");
        assert_eq!(extracted4.1.content, "B");
        assert_eq!(extracted4.2.content, "C");
        assert_eq!(extracted4.3.content, "D");
    }

    #[test]
    fn tuple_mixed_types_extraction() {
        // Test tuple with mixed view types
        let text = Text::new("Hello").color(Color::RED);
        let button = Button::new("Click me").background_color(Color::BLUE);
        let tuple = (text, button.view());
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&tuple, &ctx).unwrap();

        assert_eq!(extracted.0.content, "Hello");
        assert_eq!(extracted.0.color, Color::RED);
        assert_eq!(extracted.1.text, "Click me");
        assert_eq!(extracted.1.background_color, Color::BLUE);
    }

    #[test]
    fn vstack_extraction_basic() {
        // Test basic VStack extraction
        let text1 = Text::new("Top");
        let text2 = Text::new("Bottom");
        let vstack = VStack::new((text1, text2));
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&vstack, &ctx).unwrap();

        assert_eq!(extracted.spacing, 0.0);
        assert_eq!(extracted.content.0.content, "Top");
        assert_eq!(extracted.content.1.content, "Bottom");
    }

    #[test]
    fn vstack_extraction_with_spacing() {
        // Test VStack with spacing
        let text1 = Text::new("Top");
        let text2 = Text::new("Bottom");
        let vstack = VStack::new((text1, text2)).spacing(16.0);
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&vstack, &ctx).unwrap();

        assert_eq!(extracted.spacing, 16.0);
        assert_eq!(extracted.alignment, Alignment::Leading);
        assert_eq!(extracted.content.0.content, "Top");
        assert_eq!(extracted.content.1.content, "Bottom");
    }

    #[test]
    fn vstack_extraction_with_alignment() {
        // Test VStack with alignment
        let text1 = Text::new("Centered");
        let text2 = Text::new("Content");
        let vstack = VStack::new((text1, text2)).alignment(Alignment::Center);
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&vstack, &ctx).unwrap();

        assert_eq!(extracted.spacing, 0.0);
        assert_eq!(extracted.alignment, Alignment::Center);
        assert_eq!(extracted.content.0.content, "Centered");
        assert_eq!(extracted.content.1.content, "Content");
    }

    #[test]
    fn hstack_extraction_basic() {
        // Test basic HStack extraction
        let text1 = Text::new("Left");
        let text2 = Text::new("Right");
        let hstack = HStack::new((text1, text2));
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&hstack, &ctx).unwrap();

        assert_eq!(extracted.spacing, 0.0);
        assert_eq!(extracted.content.0.content, "Left");
        assert_eq!(extracted.content.1.content, "Right");
    }

    #[test]
    fn hstack_extraction_with_spacing() {
        // Test HStack with spacing
        let text1 = Text::new("Left");
        let text2 = Text::new("Right");
        let hstack = HStack::new((text1, text2)).spacing(8.0);
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&hstack, &ctx).unwrap();

        assert_eq!(extracted.spacing, 8.0);
        assert_eq!(extracted.alignment, Alignment::Leading);
        assert_eq!(extracted.content.0.content, "Left");
        assert_eq!(extracted.content.1.content, "Right");
    }

    #[test]
    fn hstack_extraction_with_alignment() {
        // Test HStack with alignment
        let text1 = Text::new("Left");
        let text2 = Text::new("Right");
        let hstack = HStack::new((text1, text2)).alignment(Alignment::Trailing);
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&hstack, &ctx).unwrap();

        assert_eq!(extracted.spacing, 0.0);
        assert_eq!(extracted.alignment, Alignment::Trailing);
        assert_eq!(extracted.content.0.content, "Left");
        assert_eq!(extracted.content.1.content, "Right");
    }

    #[test]
    fn nested_containers_extraction() {
        // Test nested container extraction
        let inner_text1 = Text::new("Inner 1");
        let inner_text2 = Text::new("Inner 2");
        let inner_hstack = HStack::new((inner_text1, inner_text2)).spacing(4.0);

        let outer_text = Text::new("Outer");
        let outer_vstack = VStack::new((inner_hstack, outer_text)).spacing(12.0);

        let ctx = RenderContext::new();
        let extracted = MockBackend::extract(&outer_vstack, &ctx).unwrap();

        assert_eq!(extracted.spacing, 12.0);
        assert_eq!(extracted.content.0.spacing, 4.0);
        assert_eq!(extracted.content.0.content.0.content, "Inner 1");
        assert_eq!(extracted.content.0.content.1.content, "Inner 2");
        assert_eq!(extracted.content.1.content, "Outer");
    }

    #[test]
    fn container_with_mixed_content() {
        // Test container with mixed content types
        let text = Text::new("Label").color(Color::GREEN);
        let button = Button::new("Action").background_color(Color::RED);
        let vstack = VStack::new((text, button.view())).spacing(10.0);

        let ctx = RenderContext::new();
        let extracted = MockBackend::extract(&vstack, &ctx).unwrap();

        assert_eq!(extracted.spacing, 10.0);
        assert_eq!(extracted.content.0.content, "Label");
        assert_eq!(extracted.content.0.color, Color::GREEN);
        assert_eq!(extracted.content.1.text, "Action");
        assert_eq!(extracted.content.1.background_color, Color::RED);
    }

    #[test]
    fn large_tuple_extraction() {
        // Test larger tuple (5-tuple) to verify higher arity works
        let texts = (
            Text::new("1"),
            Text::new("2"),
            Text::new("3"),
            Text::new("4"),
            Text::new("5"),
        );
        let ctx = RenderContext::new();

        let extracted = MockBackend::extract(&texts, &ctx).unwrap();

        assert_eq!(extracted.0.content, "1");
        assert_eq!(extracted.1.content, "2");
        assert_eq!(extracted.2.content, "3");
        assert_eq!(extracted.3.content, "4");
        assert_eq!(extracted.4.content, "5");
    }

    #[test]
    fn complex_nested_hierarchy() {
        // Test complex nested hierarchy with multiple levels
        let header = Text::new("Header").font_size(24.0);

        let row1 = HStack::new((Text::new("Col 1"), Text::new("Col 2"))).spacing(5.0);

        let row2 = HStack::new((
            Button::new("Button 1").view(),
            Button::new("Button 2").view(),
        ))
        .spacing(5.0);

        let content = VStack::new((row1, row2)).spacing(8.0);
        let footer = Text::new("Footer").color(Color::BLUE);

        let main_layout = VStack::new((header, content, footer)).spacing(16.0);

        let ctx = RenderContext::new();
        let extracted = MockBackend::extract(&main_layout, &ctx).unwrap();

        // Verify structure
        assert_eq!(extracted.spacing, 16.0);

        // Header
        assert_eq!(extracted.content.0.content, "Header");
        assert_eq!(extracted.content.0.font_size, 24.0);

        // Content (nested VStack)
        assert_eq!(extracted.content.1.spacing, 8.0);

        // Row 1 (HStack with texts)
        assert_eq!(extracted.content.1.content.0.spacing, 5.0);
        assert_eq!(extracted.content.1.content.0.content.0.content, "Col 1");
        assert_eq!(extracted.content.1.content.0.content.1.content, "Col 2");

        // Row 2 (HStack with buttons)
        assert_eq!(extracted.content.1.content.1.spacing, 5.0);
        assert_eq!(extracted.content.1.content.1.content.0.text, "Button 1");
        assert_eq!(extracted.content.1.content.1.content.1.text, "Button 2");

        // Footer
        assert_eq!(extracted.content.2.content, "Footer");
        assert_eq!(extracted.content.2.color, Color::BLUE);
    }

    #[test]
    fn registry_based_dynamic_extraction_no_hardcoding() {
        // This test demonstrates that the registry-based approach works
        // without any hardcoded type checking in the extraction logic
        let ctx = RenderContext::new();

        // Create a dynamic container with mixed view types
        let dynamic_views: Vec<Box<dyn View>> = vec![
            Box::new(Text::new("Hello")),
            Box::new(Button::new("Click me").view()),
            Box::new(Spacer::min_size(10.0)),
        ];

        let dynamic_vstack = VStack::dynamic().children(dynamic_views).spacing(8.0);

        // Extract the dynamic container
        let extracted = MockBackend::extract(&dynamic_vstack, &ctx).unwrap();

        // Verify the structure was extracted correctly
        assert_eq!(extracted.spacing, 8.0);
        assert_eq!(extracted.content.len(), 3);

        // Verify each child was converted correctly by the registry
        assert!(
            matches!(&extracted.content[0], MockDynamicChild::Text(text) if text.content == "Hello")
        );

        assert!(
            matches!(&extracted.content[1], MockDynamicChild::Button(button) if button.text == "Click me")
        );

        assert!(
            matches!(&extracted.content[2], MockDynamicChild::Spacer(spacer) if spacer.min_size == 10.0)
        );
    }

    #[test]
    fn nested_dynamic_containers_registry_based() {
        // Test nested dynamic containers to ensure the registry handles
        // recursive extraction correctly
        let ctx = RenderContext::new();

        // Create nested dynamic structure
        let inner_hstack = HStack::dynamic()
            .child(Box::new(Text::new("Left")))
            .child(Box::new(Text::new("Right")))
            .spacing(4.0);

        let outer_vstack = VStack::dynamic()
            .child(Box::new(Text::new("Header")))
            .child(Box::new(inner_hstack))
            .child(Box::new(Button::new("Footer Button").view()))
            .spacing(12.0);

        // Extract the nested structure
        let extracted = MockBackend::extract(&outer_vstack, &ctx).unwrap();

        // Verify the nested structure
        assert_eq!(extracted.spacing, 12.0);
        assert_eq!(extracted.content.len(), 3);

        // Check header
        assert!(
            matches!(&extracted.content[0], MockDynamicChild::Text(text) if text.content == "Header")
        );

        // Check nested HStack
        assert!(
            matches!(&extracted.content[1], MockDynamicChild::HStack(hstack) if hstack.spacing == 4.0 && hstack.content.len() == 2)
        );

        if let MockDynamicChild::HStack(hstack) = &extracted.content[1] {
            assert!(
                matches!(&hstack.content[0], MockDynamicChild::Text(text) if text.content == "Left")
            );
            assert!(
                matches!(&hstack.content[1], MockDynamicChild::Text(text) if text.content == "Right")
            );
        }

        // Check footer button
        assert!(
            matches!(&extracted.content[2], MockDynamicChild::Button(button) if button.text == "Footer Button")
        );
    }
}

// End of File
