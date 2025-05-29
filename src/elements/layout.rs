// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Layout container views for organizing child views
//!
//! Container views organize child views in specific layouts. They are pure
//! data structures that describe layout intent - the actual positioning
//! and sizing is handled by backends through the ViewExtractor pattern.

use crate::view::View;

/// Alignment options for layout containers.
///
/// Determines how child views are aligned within their container.
/// The actual alignment behavior is implemented by backends during extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Align to the leading edge (left in LTR, right in RTL, top in vertical)
    Leading,
    /// Center alignment
    Center,
    /// Align to the trailing edge (right in LTR, left in RTL, bottom in vertical)
    Trailing,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Leading
    }
}

/// A flexible space that expands to fill available space.
///
/// Spacer is useful for pushing elements apart in stacks, creating flexible
/// layouts, and centering content. The actual space calculation is performed
/// by backends during extraction.
///
/// # Examples
///
/// ```
/// use ironwood::{HStack, Text, Spacer};
///
/// // Push button to the right side
/// let toolbar = HStack::new((
///     Text::new("Title"),
///     Spacer::new(),
///     Text::new("Button"),
/// ));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Spacer {
    /// Minimum size for the spacer in logical pixels
    pub min_size: f32,
}

impl Spacer {
    /// Creates a new spacer with no minimum size.
    ///
    /// The spacer will expand to fill all available space.
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::Spacer;
    ///
    /// let spacer = Spacer::new();
    /// ```
    pub fn new() -> Self {
        Self { min_size: 0.0 }
    }

    /// Creates a spacer with a minimum size.
    ///
    /// The spacer will take at least the specified size, but can expand further
    /// if more space is available.
    ///
    /// # Arguments
    ///
    /// * `min_size` - The minimum size in logical pixels
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::Spacer;
    ///
    /// let spacer = Spacer::min_size(20.0);
    /// ```
    pub fn min_size(min_size: f32) -> Self {
        Self { min_size }
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Spacer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Vertical stack container that arranges children vertically.
///
/// VStack arranges its children in a vertical column with configurable spacing
/// between elements. The actual layout calculations are performed by backends
/// during extraction.
///
/// # Examples
///
/// ```
/// use ironwood::{VStack, Text};
///
/// let stack = VStack::new((
///     Text::new("First item"),
///     Text::new("Second item"),
/// )).spacing(8.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VStack<T> {
    /// The child views to arrange vertically
    pub content: T,
    /// Horizontal alignment of child views
    pub alignment: Alignment,
    /// Spacing between child views in logical pixels
    pub spacing: f32,
}

impl<T: View> VStack<T> {
    /// Creates a new vertical stack with the given content.
    ///
    /// # Arguments
    ///
    /// * `content` - The child views to arrange vertically
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::{VStack, Text};
    ///
    /// let stack = VStack::new((
    ///     Text::new("Top"),
    ///     Text::new("Bottom"),
    /// ));
    /// ```
    pub fn new(content: T) -> Self {
        Self {
            content,
            alignment: Alignment::default(),
            spacing: 0.0,
        }
    }

    /// Sets the spacing between child views.
    ///
    /// # Arguments
    ///
    /// * `spacing` - The spacing in logical pixels
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::{VStack, Text};
    ///
    /// let stack = VStack::new((
    ///     Text::new("Top"),
    ///     Text::new("Bottom"),
    /// )).spacing(16.0);
    /// ```
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the horizontal alignment of child views.
    ///
    /// # Arguments
    ///
    /// * `alignment` - The alignment option for child views
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::{VStack, Text, Alignment};
    ///
    /// let stack = VStack::new((
    ///     Text::new("Centered"),
    ///     Text::new("Content"),
    /// )).alignment(Alignment::Center);
    /// ```
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl<T: View> View for VStack<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Horizontal stack container that arranges children horizontally.
///
/// HStack arranges its children in a horizontal row with configurable spacing
/// between elements. The actual layout calculations are performed by backends
/// during extraction.
///
/// # Examples
///
/// ```
/// use ironwood::{HStack, Text};
///
/// let stack = HStack::new((
///     Text::new("Left"),
///     Text::new("Right"),
/// )).spacing(8.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HStack<T> {
    /// The child views to arrange horizontally
    pub content: T,
    /// Vertical alignment of child views
    pub alignment: Alignment,
    /// Spacing between child views in logical pixels
    pub spacing: f32,
}

impl<T: View> HStack<T> {
    /// Creates a new horizontal stack with the given content.
    ///
    /// # Arguments
    ///
    /// * `content` - The child views to arrange horizontally
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::{HStack, Text};
    ///
    /// let stack = HStack::new((
    ///     Text::new("Left"),
    ///     Text::new("Right"),
    /// ));
    /// ```
    pub fn new(content: T) -> Self {
        Self {
            content,
            alignment: Alignment::default(),
            spacing: 0.0,
        }
    }

    /// Sets the spacing between child views.
    ///
    /// # Arguments
    ///
    /// * `spacing` - The spacing in logical pixels
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::{HStack, Text};
    ///
    /// let stack = HStack::new((
    ///     Text::new("Left"),
    ///     Text::new("Right"),
    /// )).spacing(16.0);
    /// ```
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the vertical alignment of child views.
    ///
    /// # Arguments
    ///
    /// * `alignment` - The alignment option for child views
    ///
    /// # Examples
    ///
    /// ```
    /// use ironwood::{HStack, Text, Alignment};
    ///
    /// let stack = HStack::new((
    ///     Text::new("Left"),
    ///     Text::new("Right"),
    /// )).alignment(Alignment::Center);
    /// ```
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl<T: View> View for HStack<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        backends::mock::MockBackend,
        elements::Text,
        extraction::{RenderContext, ViewExtractor},
    };

    #[test]
    fn spacer_edge_cases() {
        let ctx = RenderContext::new();

        // Zero min_size spacer
        let flexible_spacer = Spacer::min_size(0.0);
        let extracted = MockBackend::extract(&flexible_spacer, &ctx).unwrap();
        assert_eq!(extracted.min_size, 0.0);

        // Large spacer size
        let large_spacer = Spacer::min_size(1000.0);
        let extracted = MockBackend::extract(&large_spacer, &ctx).unwrap();
        assert_eq!(extracted.min_size, 1000.0);

        // Fractional spacer size
        let fractional_spacer = Spacer::min_size(0.5);
        let extracted = MockBackend::extract(&fractional_spacer, &ctx).unwrap();
        assert_eq!(extracted.min_size, 0.5);
    }

    #[test]
    fn container_edge_cases() {
        let ctx = RenderContext::new();

        // Zero spacing
        let tight_spacing = VStack::new((Text::new("A"), Text::new("B"))).spacing(0.0);
        let extracted = MockBackend::extract(&tight_spacing, &ctx).unwrap();
        assert_eq!(extracted.spacing, 0.0);

        // Large spacing
        let large_spacing = HStack::new((Text::new("A"), Text::new("B"))).spacing(100.0);
        let extracted = MockBackend::extract(&large_spacing, &ctx).unwrap();
        assert_eq!(extracted.spacing, 100.0);

        // Fractional spacing
        let fractional_spacing = VStack::new((Text::new("A"), Text::new("B"))).spacing(2.5);
        let extracted = MockBackend::extract(&fractional_spacing, &ctx).unwrap();
        assert_eq!(extracted.spacing, 2.5);
    }
}

// End of File
