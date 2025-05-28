// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Integration tests for view extraction and backend integration
//!
//! These tests validate that the view extraction system works correctly,
//! including the ViewExtractor pattern and backend integration. This ensures
//! clean separation between view description and backend rendering.

use ironwood::{backends::mock::MockBackend, prelude::*};

/// Test that demonstrates the complete view creation and extraction workflow.
///
/// This validates that views can be created with various styling options,
/// that they store their data correctly, and that the mock backend can
/// extract all the information needed for rendering.
#[test]
fn complete_view_extraction_workflow() {
    // Create a render context for extraction
    let ctx = RenderContext::new();

    // Create various text views with different styling
    let title_text = Text::new("Application Title")
        .font_size(24.0)
        .color(Color::BLUE);

    let body_text = Text::new("This is some body text with default styling");

    let error_text = Text::new("Error: Something went wrong!")
        .font_size(14.0)
        .color(Color::RED);

    // Extract text views and verify all properties are preserved
    let title_extracted = MockBackend::extract(&title_text, &ctx);
    assert_eq!(title_extracted.content, "Application Title");
    assert_eq!(title_extracted.font_size, 24.0);
    assert_eq!(title_extracted.color, Color::BLUE);

    let body_extracted = MockBackend::extract(&body_text, &ctx);
    assert_eq!(
        body_extracted.content,
        "This is some body text with default styling"
    );
    assert_eq!(body_extracted.font_size, 16.0); // Default font size
    assert_eq!(body_extracted.color, Color::BLACK); // Default color

    let error_extracted = MockBackend::extract(&error_text, &ctx);
    assert_eq!(error_extracted.content, "Error: Something went wrong!");
    assert_eq!(error_extracted.font_size, 14.0);
    assert_eq!(error_extracted.color, Color::RED);

    // Create various button components with different styling and states
    let primary_button = Button::new("Save")
        .background_color(Color::BLUE)
        .with_text(|text| text.color(Color::WHITE));

    let secondary_button = Button::new("Load").background_color(Color::rgb(0.8, 0.8, 0.8));

    let danger_button = Button::new("Delete")
        .background_color(Color::RED)
        .with_text(|text| text.color(Color::WHITE));

    let disabled_button = Button::new("Disabled").disable();

    // Extract button components and verify all properties and state are preserved
    let primary_extracted = MockBackend::extract(&primary_button.view(), &ctx);
    assert_eq!(primary_extracted.text, "Save");
    assert_eq!(primary_extracted.background_color, Color::BLUE);
    assert_eq!(primary_extracted.text_style.color, Color::WHITE);
    assert!(primary_extracted.interaction_state.is_enabled());
    assert!(!primary_extracted.interaction_state.is_pressed());

    let secondary_extracted = MockBackend::extract(&secondary_button.view(), &ctx);
    assert_eq!(secondary_extracted.text, "Load");
    assert_eq!(
        secondary_extracted.background_color,
        Color::rgb(0.8, 0.8, 0.8)
    );
    assert!(secondary_extracted.interaction_state.is_enabled());

    let danger_extracted = MockBackend::extract(&danger_button.view(), &ctx);
    assert_eq!(danger_extracted.text, "Delete");
    assert_eq!(danger_extracted.background_color, Color::RED);
    assert_eq!(danger_extracted.text_style.color, Color::WHITE);
    assert!(danger_extracted.interaction_state.is_enabled());

    let disabled_extracted = MockBackend::extract(&disabled_button.view(), &ctx);
    assert_eq!(disabled_extracted.text, "Disabled");
    assert!(!disabled_extracted.interaction_state.is_enabled());
}

/// Test that view extraction is pure and doesn't modify the original views.
///
/// This is important for the framework's architecture - extraction should be
/// a read-only operation that doesn't have side effects.
#[test]
fn extraction_is_pure_operation() {
    let ctx = RenderContext::new();

    // Create original views
    let original_text = Text::new("Original Text")
        .font_size(20.0)
        .color(Color::GREEN);

    let original_button = Button::new("Original Button")
        .background_color(Color::BLUE)
        .enable();

    // Store original values for comparison
    let original_text_content = original_text.content.clone();
    let original_text_font_size = original_text.style.font_size;
    let original_text_color = original_text.style.color;

    let original_button_text = original_button.text.clone();
    let original_button_enabled = original_button.is_enabled();
    let original_button_bg = original_button.background_color;

    // Extract views multiple times
    let _extracted_text_1 = MockBackend::extract(&original_text, &ctx);
    let _extracted_text_2 = MockBackend::extract(&original_text, &ctx);
    let _extracted_button_1 = MockBackend::extract(&original_button.view(), &ctx);
    let _extracted_button_2 = MockBackend::extract(&original_button.view(), &ctx);

    // Verify original views are completely unchanged
    assert_eq!(original_text.content, original_text_content);
    assert_eq!(original_text.style.font_size, original_text_font_size);
    assert_eq!(original_text.style.color, original_text_color);

    assert_eq!(original_button.text, original_button_text);
    assert_eq!(original_button.is_enabled(), original_button_enabled);
    assert_eq!(original_button.background_color, original_button_bg);
}

/// Test that button component state is properly handled in extraction.
///
/// This validates that the mock backend can extract button component state
/// including enabled/disabled and all interaction states.
#[test]
fn button_component_state_handling() {
    let ctx = RenderContext::new();

    // Test enabled button
    let enabled_button = Button::new("Enabled").enable();

    // Test disabled button
    let disabled_button = Button::new("Disabled").disable();

    // Test clicked button (using Model trait)
    let clicked_button = Button::new("Clicked").update(ButtonMessage::Clicked);

    // Test focused button
    let focused_button = Button::new("Focused").update(ButtonMessage::Interaction(
        InteractionMessage::FocusChanged(true),
    ));

    // Test pressed button
    let pressed_button = Button::new("Pressed").update(ButtonMessage::Interaction(
        InteractionMessage::PressStateChanged(true),
    ));

    // Test hovered button
    let hovered_button = Button::new("Hovered").update(ButtonMessage::Interaction(
        InteractionMessage::HoverChanged(true),
    ));

    // Extract and verify state
    let enabled_extracted = MockBackend::extract(&enabled_button.view(), &ctx);
    assert_eq!(enabled_extracted.text, "Enabled");
    assert!(enabled_extracted.interaction_state.is_enabled());
    assert!(!enabled_extracted.interaction_state.is_pressed());
    assert!(!enabled_extracted.interaction_state.is_focused());
    assert!(!enabled_extracted.interaction_state.is_hovered());

    let disabled_extracted = MockBackend::extract(&disabled_button.view(), &ctx);
    assert_eq!(disabled_extracted.text, "Disabled");
    assert!(!disabled_extracted.interaction_state.is_enabled());
    assert!(!disabled_extracted.interaction_state.is_pressed());
    assert!(!disabled_extracted.interaction_state.is_focused());
    assert!(!disabled_extracted.interaction_state.is_hovered());

    let clicked_extracted = MockBackend::extract(&clicked_button.view(), &ctx);
    assert_eq!(clicked_extracted.text, "Clicked");
    assert!(clicked_extracted.interaction_state.is_enabled());
    assert!(!clicked_extracted.interaction_state.is_pressed());
    assert!(!clicked_extracted.interaction_state.is_focused());
    assert!(!clicked_extracted.interaction_state.is_hovered());

    let focused_extracted = MockBackend::extract(&focused_button.view(), &ctx);
    assert_eq!(focused_extracted.text, "Focused");
    assert!(focused_extracted.interaction_state.is_enabled());
    assert!(!focused_extracted.interaction_state.is_pressed());
    assert!(focused_extracted.interaction_state.is_focused());
    assert!(!focused_extracted.interaction_state.is_hovered());

    let pressed_extracted = MockBackend::extract(&pressed_button.view(), &ctx);
    assert_eq!(pressed_extracted.text, "Pressed");
    assert!(pressed_extracted.interaction_state.is_enabled());
    assert!(pressed_extracted.interaction_state.is_pressed());
    assert!(!pressed_extracted.interaction_state.is_focused());
    assert!(!pressed_extracted.interaction_state.is_hovered());

    let hovered_extracted = MockBackend::extract(&hovered_button.view(), &ctx);
    assert_eq!(hovered_extracted.text, "Hovered");
    assert!(hovered_extracted.interaction_state.is_enabled());
    assert!(!hovered_extracted.interaction_state.is_pressed());
    assert!(!hovered_extracted.interaction_state.is_focused());
    assert!(hovered_extracted.interaction_state.is_hovered());

    // Test combined interaction states
    let complex_button = Button::new("Complex")
        .update(ButtonMessage::Interaction(
            InteractionMessage::FocusChanged(true),
        ))
        .update(ButtonMessage::Interaction(
            InteractionMessage::HoverChanged(true),
        ))
        .update(ButtonMessage::Interaction(
            InteractionMessage::PressStateChanged(true),
        ));

    let complex_extracted = MockBackend::extract(&complex_button.view(), &ctx);
    assert_eq!(complex_extracted.text, "Complex");
    assert!(complex_extracted.interaction_state.is_enabled());
    assert!(complex_extracted.interaction_state.is_pressed());
    assert!(complex_extracted.interaction_state.is_focused());
    assert!(complex_extracted.interaction_state.is_hovered());
}

/// Test that view styling properties are correctly preserved through extraction.
///
/// This ensures that all styling information is available to backends for
/// proper rendering.
#[test]
fn view_styling_preservation() {
    let ctx = RenderContext::new();

    // Test all color constants
    let black_text = Text::new("Black").color(Color::BLACK);
    let white_text = Text::new("White").color(Color::WHITE);
    let red_text = Text::new("Red").color(Color::RED);
    let green_text = Text::new("Green").color(Color::GREEN);
    let blue_text = Text::new("Blue").color(Color::BLUE);

    // Test custom colors
    let custom_text = Text::new("Custom").color(Color::rgba(0.5, 0.7, 0.9, 0.8));

    // Extract and verify colors
    assert_eq!(MockBackend::extract(&black_text, &ctx).color, Color::BLACK);
    assert_eq!(MockBackend::extract(&white_text, &ctx).color, Color::WHITE);
    assert_eq!(MockBackend::extract(&red_text, &ctx).color, Color::RED);
    assert_eq!(MockBackend::extract(&green_text, &ctx).color, Color::GREEN);
    assert_eq!(MockBackend::extract(&blue_text, &ctx).color, Color::BLUE);
    assert_eq!(
        MockBackend::extract(&custom_text, &ctx).color,
        Color::rgba(0.5, 0.7, 0.9, 0.8)
    );

    // Test various font sizes
    let small_text = Text::new("Small").font_size(12.0);
    let large_text = Text::new("Large").font_size(32.0);

    assert_eq!(MockBackend::extract(&small_text, &ctx).font_size, 12.0);
    assert_eq!(MockBackend::extract(&large_text, &ctx).font_size, 32.0);

    // Test button styling combinations
    let styled_button = Button::new("Styled")
        .background_color(Color::rgba(0.2, 0.4, 0.6, 1.0))
        .with_text(|text| text.color(Color::rgba(0.9, 0.9, 0.9, 1.0)));

    let button_extracted = MockBackend::extract(&styled_button.view(), &ctx);
    assert_eq!(
        button_extracted.background_color,
        Color::rgba(0.2, 0.4, 0.6, 1.0)
    );
    assert_eq!(
        button_extracted.text_style.color,
        Color::rgba(0.9, 0.9, 0.9, 1.0)
    );
}

/// Test that the render context is properly passed to extractors.
///
/// While the current mock backend doesn't use the context, this test ensures
/// the extraction API is working correctly for future backend implementations.
#[test]
fn render_context_integration() {
    // Test that different context instances work the same way
    let ctx1 = RenderContext::new();
    let ctx2 = RenderContext::default();

    let text = Text::new("Context Test").font_size(18.0);
    let button = Button::new("Context Button").enable();

    // Extract with different contexts
    let text_extracted_1 = MockBackend::extract(&text, &ctx1);
    let text_extracted_2 = MockBackend::extract(&text, &ctx2);
    let button_extracted_1 = MockBackend::extract(&button.view(), &ctx1);
    let button_extracted_2 = MockBackend::extract(&button.view(), &ctx2);

    // Results should be identical regardless of context instance
    assert_eq!(text_extracted_1, text_extracted_2);
    assert_eq!(button_extracted_1, button_extracted_2);

    // Verify the extracted data is correct
    assert_eq!(text_extracted_1.content, "Context Test");
    assert_eq!(text_extracted_1.font_size, 18.0);
    assert_eq!(button_extracted_1.text, "Context Button");
    assert!(button_extracted_1.interaction_state.is_enabled());
}

/// Test extraction of edge cases and boundary conditions.
///
/// This validates that the extraction system handles edge cases correctly,
/// such as empty text, extreme styling values, and complex state combinations.
#[test]
fn extraction_edge_cases() {
    let ctx = RenderContext::new();

    // Test empty text
    let empty_text = Text::new("");
    let empty_extracted = MockBackend::extract(&empty_text, &ctx);
    assert_eq!(empty_extracted.content, "");
    assert_eq!(empty_extracted.font_size, 16.0); // Default font size
    assert_eq!(empty_extracted.color, Color::BLACK); // Default color

    // Test text with extreme font sizes
    let tiny_text = Text::new("Tiny").font_size(1.0);
    let huge_text = Text::new("Huge").font_size(1000.0);

    let tiny_extracted = MockBackend::extract(&tiny_text, &ctx);
    let huge_extracted = MockBackend::extract(&huge_text, &ctx);

    assert_eq!(tiny_extracted.font_size, 1.0);
    assert_eq!(huge_extracted.font_size, 1000.0);

    // Test transparent colors
    let transparent_text = Text::new("Transparent").color(Color::rgba(1.0, 0.0, 0.0, 0.0));
    let transparent_extracted = MockBackend::extract(&transparent_text, &ctx);
    assert_eq!(transparent_extracted.color, Color::rgba(1.0, 0.0, 0.0, 0.0));

    // Test button with empty text
    let empty_button = Button::new("");
    let empty_button_extracted = MockBackend::extract(&empty_button.view(), &ctx);
    assert_eq!(empty_button_extracted.text, "");
    assert!(empty_button_extracted.interaction_state.is_enabled());

    // Test button state transitions (disabled then enabled)
    let state_button = Button::new("State").disable().enable(); // Should end up enabled

    let state_extracted = MockBackend::extract(&state_button.view(), &ctx);
    assert_eq!(state_extracted.text, "State");
    assert!(state_extracted.interaction_state.is_enabled());
}

/// Test extraction of deeply nested structures.
///
/// This validates that the extraction system can handle complex nested
/// structures without stack overflow or performance issues.
#[test]
fn deep_nesting_extraction() {
    let ctx = RenderContext::new();

    // Create deeply nested tuple structure
    let deeply_nested = (
        (
            (
                (Text::new("Level 1"), Text::new("Level 2")),
                (Text::new("Level 3"), Text::new("Level 4")),
            ),
            (
                (Text::new("Level 5"), Text::new("Level 6")),
                (Text::new("Level 7"), Text::new("Level 8")),
            ),
        ),
        VStack::new((
            HStack::new((Text::new("Nested"), Text::new("Stack"))),
            VStack::new((Text::new("Deep"), Text::new("Nesting"))),
        )),
    );

    // Should be able to extract without stack overflow
    let _extracted = MockBackend::extract(&deeply_nested, &ctx);
    // If we get here, the test passed
}

/// Test extraction performance with large collections.
///
/// This validates that the extraction system can handle large numbers
/// of components efficiently.
#[test]
fn large_collection_extraction() {
    let ctx = RenderContext::new();

    // Create many small components
    let mut components = Vec::new();
    for i in 0..1000 {
        components.push(Text::new(format!("Component {}", i)));
    }

    // Extract all of them
    let extracted: Vec<_> = components
        .iter()
        .map(|comp| MockBackend::extract(comp, &ctx))
        .collect();

    assert_eq!(extracted.len(), 1000);
    assert_eq!(extracted[0].content, "Component 0");
    assert_eq!(extracted[999].content, "Component 999");

    // Create large tuple (test tuple extraction limits)
    let large_tuple = (
        Text::new("1"),
        Text::new("2"),
        Text::new("3"),
        Text::new("4"),
        Text::new("5"),
        Text::new("6"),
        Text::new("7"),
        Text::new("8"),
        Text::new("9"),
        Text::new("10"),
        Text::new("11"),
        Text::new("12"),
    );
    let extracted_tuple = MockBackend::extract(&large_tuple, &ctx);
    assert_eq!(extracted_tuple.0.content, "1");
    assert_eq!(extracted_tuple.11.content, "12");
}

/// Test extraction of optional values.
///
/// This validates that the extraction system correctly handles Option types
/// including nested options and options within containers.
#[test]
fn option_extraction() {
    let ctx = RenderContext::new();

    // Nested options
    let nested_some: Option<Option<Text>> = Some(Some(Text::new("Nested")));
    let extracted = MockBackend::extract(&nested_some, &ctx);
    assert!(extracted.is_some());
    assert!(extracted.as_ref().unwrap().is_some());
    assert_eq!(extracted.unwrap().unwrap().content, "Nested");

    let nested_none: Option<Option<Text>> = Some(None);
    let extracted = MockBackend::extract(&nested_none, &ctx);
    assert!(extracted.is_some());
    assert!(extracted.unwrap().is_none());

    let outer_none: Option<Option<Text>> = None;
    let extracted = MockBackend::extract(&outer_none, &ctx);
    assert!(extracted.is_none());

    // Option in containers
    let optional_stack = VStack::new((
        Some(Text::new("Present")),
        None::<Text>,
        Some(Text::new("Also present")),
    ));
    let extracted = MockBackend::extract(&optional_stack, &ctx);
    assert!(extracted.content.0.is_some());
    assert!(extracted.content.1.is_none());
    assert!(extracted.content.2.is_some());
}

// End of File
