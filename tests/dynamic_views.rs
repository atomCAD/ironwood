// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Integration tests for dynamic view functionality
//!
//! These tests validate the dynamic view features introduced for runtime type checking,
//! including downcasting, trait object behavior, error handling, and integration patterns.

use std::any::TypeId;

use ironwood::{backends::mock::MockBackend, prelude::*};

/// Test dynamic downcasting functionality for core view types.
///
/// This validates that the `as_any()` method works correctly for downcasting
/// from `&dyn View` back to concrete types.
#[test]
fn test_dynamic_downcasting_core_types() {
    // Test Text downcasting
    let text = Text::new("Hello").color(Color::RED).font_size(20.0);
    let view: &dyn View = &text;
    let downcast_text = view.as_any().downcast_ref::<Text>().unwrap();
    assert_eq!(downcast_text.content, "Hello");
    assert_eq!(downcast_text.style.color, Color::RED);
    assert_eq!(downcast_text.style.font_size, 20.0);

    // Test ButtonView downcasting
    let button = Button::new("Click").background_color(Color::BLUE);
    let button_view = button.view();
    let view: &dyn View = &button_view;
    let downcast_button = view.as_any().downcast_ref::<ButtonView>().unwrap();
    assert_eq!(downcast_button.text.content, "Click");
    assert_eq!(downcast_button.background_color, Color::BLUE);

    // Test container downcasting
    let vstack = VStack::new((Text::new("A"), Text::new("B"))).spacing(10.0);
    let view: &dyn View = &vstack;
    let downcast_vstack = view
        .as_any()
        .downcast_ref::<VStack<(Text, Text)>>()
        .unwrap();
    assert_eq!(downcast_vstack.spacing, 10.0);
    assert_eq!(downcast_vstack.content.0.content, "A");
    assert_eq!(downcast_vstack.content.1.content, "B");
}

/// Test tuple and option composition downcasting.
///
/// This validates that composition types work correctly with dynamic dispatch.
#[test]
fn test_composition_downcasting() {
    // Test tuple composition
    let tuple = (Text::new("First"), Spacer::min_size(10.0));
    let view: &dyn View = &tuple;
    let downcast = view.as_any().downcast_ref::<(Text, Spacer)>().unwrap();
    assert_eq!(downcast.0.content, "First");
    assert_eq!(downcast.1.min_size, 10.0);

    // Test optional views
    let some_text = Some(Text::new("Present").color(Color::BLUE));
    let view: &dyn View = &some_text;
    let downcast = view.as_any().downcast_ref::<Option<Text>>().unwrap();
    assert!(downcast.is_some());
    assert_eq!(downcast.as_ref().unwrap().content, "Present");
    assert_eq!(downcast.as_ref().unwrap().style.color, Color::BLUE);

    let none_text: Option<Text> = None;
    let view: &dyn View = &none_text;
    let downcast = view.as_any().downcast_ref::<Option<Text>>().unwrap();
    assert!(downcast.is_none());
}

/// Test dynamic trait object behavior with hierarchical views.
///
/// This validates that view hierarchies work correctly when accessed through trait objects.
#[test]
fn test_dynamic_hierarchies() {
    // Create a simple but representative hierarchy
    let hierarchy = VStack::new((
        Text::new("Header").font_size(20.0),
        HStack::new((
            Button::new("Action").background_color(Color::GREEN).view(),
            Text::new("Label"),
        ))
        .spacing(8.0),
    ))
    .spacing(12.0);

    // Access through trait object and verify extraction works
    let view: &dyn View = &hierarchy;
    let ctx = RenderContext::new();
    let extracted = MockBackend::extract(&hierarchy, &ctx).unwrap();

    assert_eq!(extracted.spacing, 12.0);
    assert_eq!(extracted.content.0.content, "Header");
    assert_eq!(extracted.content.0.font_size, 20.0);
    assert_eq!(extracted.content.1.spacing, 8.0);
    assert_eq!(extracted.content.1.content.0.text, "Action");
    assert_eq!(extracted.content.1.content.0.background_color, Color::GREEN);
    assert_eq!(extracted.content.1.content.1.content, "Label");

    // Test downcasting works
    let downcast = view
        .as_any()
        .downcast_ref::<VStack<(Text, HStack<(ButtonView, Text)>)>>()
        .unwrap();
    assert_eq!(downcast.spacing, 12.0);
}

/// Test conditional dynamic view construction patterns.
///
/// This validates that views can be constructed conditionally at runtime.
#[test]
fn test_conditional_view_construction() {
    fn create_view(use_button: bool) -> Box<dyn View> {
        if use_button {
            Box::new(
                Button::new("Dynamic Button")
                    .background_color(Color::RED)
                    .view(),
            )
        } else {
            Box::new(Text::new("Dynamic Text").font_size(18.0))
        }
    }

    // Test button path
    let button_view = create_view(true);
    let button = button_view.as_any().downcast_ref::<ButtonView>().unwrap();
    assert_eq!(button.text.content, "Dynamic Button");
    assert_eq!(button.background_color, Color::RED);

    // Test text path
    let text_view = create_view(false);
    let text = text_view.as_any().downcast_ref::<Text>().unwrap();
    assert_eq!(text.content, "Dynamic Text");
    assert_eq!(text.style.font_size, 18.0);
}

/// Test framework dynamic patterns and integration.
///
/// This validates framework-specific dynamic view patterns work correctly.
#[test]
fn test_framework_dynamic_integration() {
    // Test dynamic view collections
    let views: Vec<Box<dyn View>> = vec![
        Box::new(Text::new("Title").font_size(20.0)),
        Box::new(Button::new("Submit").background_color(Color::BLUE).view()),
        Box::new(Spacer::min_size(10.0)),
    ];

    // Verify all views work as trait objects
    for view in &views {
        let _any = view.as_any(); // Should not panic
    }

    // Test mixed static/dynamic patterns
    let mixed_hierarchy = VStack::new((
        Text::new("Static header"),
        // Dynamic content would be inserted here in real usage
        Text::new("Static footer"),
    ))
    .spacing(5.0);

    let ctx = RenderContext::new();
    let extracted = MockBackend::extract(&mixed_hierarchy, &ctx).unwrap();
    assert_eq!(extracted.spacing, 5.0);
    assert_eq!(extracted.content.0.content, "Static header");
    assert_eq!(extracted.content.1.content, "Static footer");
}

/// Test dynamic form generation pattern.
///
/// This validates a real-world pattern of building UIs dynamically based on configuration.
#[test]
fn test_dynamic_form_generation() {
    fn create_form_field(field_type: &str, label: &str) -> Box<dyn View> {
        match field_type {
            "title" => Box::new(Text::new(label).font_size(18.0)),
            "input" => Box::new(Text::new(format!("{}:", label))),
            "button" => Box::new(Button::new(label).background_color(Color::GREEN).view()),
            _ => Box::new(Text::new("Unknown field")),
        }
    }

    // Generate form fields
    let form_fields: Vec<Box<dyn View>> = vec![
        create_form_field("title", "User Registration"),
        create_form_field("input", "Name"),
        create_form_field("input", "Email"),
        create_form_field("button", "Register"),
    ];

    let form = VStack::new(form_fields).spacing(8.0);

    // Test form structure
    let view: &dyn View = &form;
    assert_eq!(
        view.as_any().type_id(),
        TypeId::of::<VStack<Vec<Box<dyn View>>>>()
    );

    let downcast = view
        .as_any()
        .downcast_ref::<VStack<Vec<Box<dyn View>>>>()
        .unwrap();
    assert_eq!(downcast.spacing, 8.0);
    assert_eq!(downcast.content.len(), 4);

    // Verify field types
    assert!(
        downcast.content[0]
            .as_any()
            .downcast_ref::<Text>()
            .is_some()
    );
    assert!(
        downcast.content[3]
            .as_any()
            .downcast_ref::<ButtonView>()
            .is_some()
    );
}

// End of File
