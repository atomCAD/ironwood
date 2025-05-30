// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Layout container views for organizing child views
//!
//! Container views organize child views in specific layouts. They are pure
//! data structures that describe layout intent - the actual positioning
//! and sizing is handled by backends through the ViewExtractor pattern.

use std::any::Any;

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
    fn as_any(&self) -> &dyn Any {
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
    fn as_any(&self) -> &dyn Any {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Dynamic container implementations for Vec<Box<dyn View>>
// These provide the same API as the tuple-based containers but work with dynamic children

impl VStack<Vec<Box<dyn View>>> {
    /// Create a new empty dynamic vertical stack.
    ///
    /// This allows building VStack containers with a runtime-determined number
    /// of children of different types, enabling conditional rendering and loops.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Build a form dynamically based on conditions
    /// let show_optional_field = true;
    /// let mut form = VStack::dynamic()
    ///     .child(Box::new(Text::new("User Registration")))
    ///     .child(Box::new(Text::new("Name: _______")));
    ///
    /// if show_optional_field {
    ///     form = form.child(Box::new(Text::new("Optional: _______")));
    /// }
    ///
    /// form = form.child(Box::new(Button::new("Submit").view()));
    /// ```
    pub fn dynamic() -> Self {
        Self {
            content: Vec::new(),
            alignment: Alignment::Leading,
            spacing: 0.0,
        }
    }

    /// Set the children for this stack.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Create a menu from a list of items
    /// let menu_items = vec!["File", "Edit", "View", "Help"];
    /// let menu_views: Vec<Box<dyn View>> = menu_items
    ///     .into_iter()
    ///     .map(|item| Box::new(Button::new(item).view()) as Box<dyn View>)
    ///     .collect();
    ///
    /// let menu = VStack::dynamic().children(menu_views);
    /// ```
    pub fn children(mut self, children: Vec<Box<dyn View>>) -> Self {
        self.content = children;
        self
    }

    /// Add a single child to this stack.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Build a notification list incrementally
    /// let mut notifications = VStack::dynamic();
    ///
    /// for i in 1..=3 {
    ///     notifications = notifications.child(
    ///         Box::new(Text::new(format!("Notification {}", i)))
    ///     );
    /// }
    /// ```
    pub fn child(mut self, child: Box<dyn View>) -> Self {
        self.content.push(child);
        self
    }

    /// Add children conditionally based on a boolean condition.
    ///
    /// This is a convenience method for conditional rendering patterns.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Show admin controls only for admin users
    /// let is_admin = false;
    /// let page = VStack::dynamic()
    ///     .child(Box::new(Text::new("Welcome to the app")))
    ///     .conditional_children(is_admin, vec![
    ///         Box::new(Button::new("Admin Panel").view()),
    ///         Box::new(Button::new("User Management").view()),
    ///     ])
    ///     .child(Box::new(Button::new("Logout").view()));
    /// ```
    pub fn conditional_children(mut self, condition: bool, children: Vec<Box<dyn View>>) -> Self {
        if condition {
            self.content.extend(children);
        }
        self
    }

    /// Convenience for creating dynamic stacks from collections.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Create a todo list from data
    /// let todos = vec!["Buy groceries", "Walk the dog", "Read a book"];
    /// let todo_views: Vec<Box<dyn View>> = todos
    ///     .into_iter()
    ///     .map(|todo| Box::new(Text::new(format!("- {}", todo))) as Box<dyn View>)
    ///     .collect();
    ///
    /// let todo_list = VStack::from_children(todo_views)
    ///     .spacing(4.0);
    /// ```
    pub fn from_children<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn View>>,
    {
        Self::dynamic().children(iter.into_iter().collect())
    }
}

impl HStack<Vec<Box<dyn View>>> {
    /// Create a new empty dynamic horizontal stack.
    ///
    /// This allows building HStack containers with a runtime-determined number
    /// of children of different types, enabling conditional rendering and loops.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Create a responsive toolbar that adapts to screen size
    /// let is_mobile = false;
    /// let mut toolbar = HStack::dynamic();
    ///
    /// if !is_mobile {
    ///     toolbar = toolbar
    ///         .child(Box::new(Button::new("File").view()))
    ///         .child(Box::new(Button::new("Edit").view()));
    /// }
    ///
    /// toolbar = toolbar.child(Box::new(Button::new("⋮").view()));
    /// ```
    pub fn dynamic() -> Self {
        Self {
            content: Vec::new(),
            alignment: Alignment::Leading,
            spacing: 0.0,
        }
    }

    /// Set the children for this stack.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Create a button row from action names
    /// let actions = vec!["Save", "Cancel", "Delete"];
    /// let buttons: Vec<Box<dyn View>> = actions
    ///     .into_iter()
    ///     .map(|action| Box::new(Button::new(action).view()) as Box<dyn View>)
    ///     .collect();
    ///
    /// let button_row = HStack::dynamic()
    ///     .children(buttons)
    ///     .spacing(8.0);
    /// ```
    pub fn children(mut self, children: Vec<Box<dyn View>>) -> Self {
        self.content = children;
        self
    }

    /// Add a single child to this stack.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Build a navigation breadcrumb
    /// let path_segments = vec!["Home", "Products", "Electronics"];
    /// let mut breadcrumb = HStack::dynamic();
    ///
    /// for (i, segment) in path_segments.iter().enumerate() {
    ///     if i > 0 {
    ///         breadcrumb = breadcrumb.child(Box::new(Text::new(" > ")));
    ///     }
    ///     breadcrumb = breadcrumb.child(Box::new(Text::new(*segment)));
    /// }
    /// ```
    pub fn child(mut self, child: Box<dyn View>) -> Self {
        self.content.push(child);
        self
    }

    /// Add children conditionally based on a boolean condition.
    ///
    /// This is a convenience method for conditional rendering patterns.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Show different buttons based on user permissions
    /// let can_edit = true;
    /// let can_delete = false;
    ///
    /// let actions = HStack::dynamic()
    ///     .child(Box::new(Button::new("View").view()))
    ///     .conditional_children(can_edit, vec![
    ///         Box::new(Button::new("Edit").view())
    ///     ])
    ///     .conditional_children(can_delete, vec![
    ///         Box::new(Button::new("Delete").view())
    ///     ]);
    /// ```
    pub fn conditional_children(mut self, condition: bool, children: Vec<Box<dyn View>>) -> Self {
        if condition {
            self.content.extend(children);
        }
        self
    }

    /// Convenience for creating dynamic stacks from collections.
    ///
    /// ## Example
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// // Create a tag list from strings
    /// let tags = vec!["rust", "ui", "framework"];
    /// let tag_views: Vec<Box<dyn View>> = tags
    ///     .into_iter()
    ///     .map(|tag| Box::new(Text::new(format!("#{}", tag))) as Box<dyn View>)
    ///     .collect();
    ///
    /// let tag_row = HStack::from_children(tag_views)
    ///     .spacing(6.0)
    ///     .alignment(Alignment::Center);
    /// ```
    pub fn from_children<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn View>>,
    {
        Self::dynamic().children(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        backends::mock::MockBackend,
        elements::Text,
        extraction::{RenderContext, ViewExtractor},
        model::Model,
    };

    #[test]
    fn container_configuration_and_extraction() {
        let ctx = RenderContext::new();

        // Test spacer configuration
        let spacer = Spacer::min_size(100.0);
        assert_eq!(spacer.min_size, 100.0);

        // Test container spacing with extraction
        let vstack = VStack::new(Text::new("Test")).spacing(5.0);
        let extracted = MockBackend::extract(&vstack, &ctx).unwrap();
        assert_eq!(extracted.spacing, 5.0);

        // Test fractional values work correctly
        let fractional_spacing = VStack::new(Text::new("Test")).spacing(2.5);
        let extracted = MockBackend::extract(&fractional_spacing, &ctx).unwrap();
        assert_eq!(extracted.spacing, 2.5);
    }

    #[test]
    fn dynamic_container_patterns() {
        use crate::widgets::Button;
        let ctx = RenderContext::new();

        // Test creation, configuration, and extraction in one test
        let stack = VStack::dynamic()
            .child(Box::new(Text::new("Title")))
            .child(Box::new(Button::new("Action").view()))
            .spacing(12.0)
            .alignment(Alignment::Center);

        assert_eq!(stack.spacing, 12.0);
        assert_eq!(stack.alignment, Alignment::Center);

        // Test extraction preserves configuration
        let extracted = MockBackend::extract(&stack, &ctx).unwrap();
        assert_eq!(extracted.spacing, 12.0);
        assert_eq!(extracted.alignment, Alignment::Center);

        // Verify dynamic children are handled correctly
        use crate::backends::mock::MockDynamicChild;
        if let MockDynamicChild::Text(text) = &extracted.content[0] {
            assert_eq!(text.content, "Title");
        }
    }

    #[test]
    fn conditional_and_nested_patterns() {
        use crate::widgets::Button;
        let ctx = RenderContext::new();

        // Test conditional logic
        let conditional_stack = VStack::dynamic()
            .child(Box::new(Text::new("Always")))
            .conditional_children(true, vec![Box::new(Text::new("Shown"))])
            .conditional_children(false, vec![Box::new(Text::new("Hidden"))]);

        assert_eq!(conditional_stack.content.len(), 2); // Only true condition added

        // Test nested structures
        let inner = HStack::dynamic()
            .child(Box::new(Button::new("Cancel").view()))
            .child(Box::new(Button::new("OK").view()))
            .spacing(8.0);

        let form = VStack::dynamic()
            .child(Box::new(Text::new("Form")))
            .child(Box::new(inner))
            .spacing(16.0);

        let extracted = MockBackend::extract(&form, &ctx).unwrap();
        assert_eq!(extracted.spacing, 16.0);

        use crate::backends::mock::MockDynamicChild;
        if let MockDynamicChild::HStack(inner) = &extracted.content[1] {
            assert_eq!(inner.spacing, 8.0);
        }
    }

    #[test]
    fn dynamic_container_edge_cases() {
        let ctx = RenderContext::new();

        // Test empty dynamic containers
        let empty_vstack = VStack::dynamic();
        let extracted = MockBackend::extract(&empty_vstack, &ctx).unwrap();
        assert_eq!(extracted.content.len(), 0);

        // Test large collection (performance characteristic)
        let mut large_stack = VStack::dynamic();
        for i in 0..1000 {
            large_stack = large_stack.child(Box::new(Text::new(format!("Item {}", i))));
        }
        assert_eq!(large_stack.content.len(), 1000);

        // Framework should handle large collections without issues
        let extracted = MockBackend::extract(&large_stack, &ctx).unwrap();
        assert_eq!(extracted.content.len(), 1000);
    }

    #[test]
    fn mixed_static_dynamic_integration() {
        use crate::widgets::Button;
        let ctx = RenderContext::new();

        // Test mixing individual static views with dynamic containers
        let dynamic_body = VStack::dynamic()
            .child(Box::new(Text::new("Header")))
            .child(Box::new(Text::new("Subtitle")))
            .child(Box::new(Button::new("Action").view()))
            .spacing(16.0);

        // Framework should handle mixed patterns correctly
        let extracted = MockBackend::extract(&dynamic_body, &ctx).unwrap();
        assert_eq!(extracted.spacing, 16.0);

        use crate::backends::mock::MockDynamicChild;
        if let MockDynamicChild::Text(header) = &extracted.content[0] {
            assert_eq!(header.content, "Header");
        }
        if let MockDynamicChild::Text(subtitle) = &extracted.content[1] {
            assert_eq!(subtitle.content, "Subtitle");
        }
    }

    #[test]
    fn container_memory_safety() {
        use crate::widgets::Button;

        // Test that containers properly handle ownership
        let create_dynamic_content = || -> Vec<Box<dyn View>> {
            vec![
                Box::new(Text::new("Dynamic")),
                Box::new(Button::new("Test").view()),
            ]
        };

        let stack = VStack::dynamic()
            .children(create_dynamic_content())
            .spacing(8.0);

        // Memory should be properly managed
        assert_eq!(stack.content.len(), 2);

        // Test ownership transfer works correctly
        let moved_stack = stack;
        assert_eq!(moved_stack.content.len(), 2);
        assert_eq!(moved_stack.spacing, 8.0);
    }
}

// End of File
