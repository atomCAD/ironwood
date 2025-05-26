// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Tuple Composition and Container Views Example
//!
//! This example demonstrates the tuple composition system and container views
//! in Ironwood. It shows how to:
//!
//! 1. Use tuples to compose multiple views together
//! 2. Create VStack and HStack containers with spacing
//! 3. Build complex nested layouts
//! 4. Extract and verify the structure using the mock backend
//!
//! The example builds a complex UI hierarchy using only tuple composition
//! and container views, demonstrating the power and flexibility of this
//! approach for building user interfaces.

use ironwood::{backends::mock::MockBackend, prelude::*};

fn main() {
    println!("=== Tuple Composition and Container Views Example ===\n");

    // Example 1: Basic tuple composition
    println!("1. Basic Tuple Composition");
    basic_tuple_composition();
    println!();

    // Example 2: Container views with spacing
    println!("2. Container Views with Spacing");
    container_views_with_spacing();
    println!();

    // Example 3: Complex nested layout
    println!("3. Complex Nested Layout");
    complex_nested_layout();
    println!();

    // Example 4: Mixed content types
    println!("4. Mixed Content Types");
    mixed_content_types();
    println!();

    println!("=== Example Complete ===");
}

/// Demonstrates basic tuple composition with 2, 3, and 4-tuples
fn basic_tuple_composition() {
    let ctx = RenderContext::new();

    // 2-tuple composition
    let pair = (Text::new("First item"), Text::new("Second item"));

    let extracted_pair = MockBackend::extract(&pair, &ctx);
    println!("2-tuple extracted:");
    println!("  Item 1: '{}'", extracted_pair.0.content);
    println!("  Item 2: '{}'", extracted_pair.1.content);

    // 3-tuple composition
    let triple = (Text::new("Alpha"), Text::new("Beta"), Text::new("Gamma"));

    let extracted_triple = MockBackend::extract(&triple, &ctx);
    println!("3-tuple extracted:");
    println!("  Item 1: '{}'", extracted_triple.0.content);
    println!("  Item 2: '{}'", extracted_triple.1.content);
    println!("  Item 3: '{}'", extracted_triple.2.content);

    // 4-tuple composition
    let quad = (
        Text::new("North"),
        Text::new("South"),
        Text::new("East"),
        Text::new("West"),
    );

    let extracted_quad = MockBackend::extract(&quad, &ctx);
    println!("4-tuple extracted:");
    println!("  Item 1: '{}'", extracted_quad.0.content);
    println!("  Item 2: '{}'", extracted_quad.1.content);
    println!("  Item 3: '{}'", extracted_quad.2.content);
    println!("  Item 4: '{}'", extracted_quad.3.content);
}

/// Demonstrates VStack and HStack containers with configurable spacing
fn container_views_with_spacing() {
    let ctx = RenderContext::new();

    // VStack with spacing
    let vstack = VStack::new((
        Text::new("Top item"),
        Text::new("Middle item"),
        Text::new("Bottom item"),
    ))
    .spacing(16.0);

    let extracted_vstack = MockBackend::extract(&vstack, &ctx);
    println!("VStack with 16.0 spacing:");
    println!("  Spacing: {}", extracted_vstack.spacing);
    println!("  Item 1: '{}'", extracted_vstack.content.0.content);
    println!("  Item 2: '{}'", extracted_vstack.content.1.content);
    println!("  Item 3: '{}'", extracted_vstack.content.2.content);

    // HStack with spacing
    let hstack =
        HStack::new((Text::new("Left"), Text::new("Center"), Text::new("Right"))).spacing(8.0);

    let extracted_hstack = MockBackend::extract(&hstack, &ctx);
    println!("HStack with 8.0 spacing:");
    println!("  Spacing: {}", extracted_hstack.spacing);
    println!("  Item 1: '{}'", extracted_hstack.content.0.content);
    println!("  Item 2: '{}'", extracted_hstack.content.1.content);
    println!("  Item 3: '{}'", extracted_hstack.content.2.content);
}

/// Demonstrates complex nested layout with multiple levels of containers
fn complex_nested_layout() {
    let ctx = RenderContext::new();

    // Build a complex layout: Header + Content Grid + Footer
    let header = Text::new("Application Header")
        .font_size(24.0)
        .color(Color::rgb(0.2, 0.2, 0.8));

    // Content grid: 2 rows of 2 columns each
    let row1 = HStack::new((
        Text::new("Cell 1,1").color(Color::RED),
        Text::new("Cell 1,2").color(Color::GREEN),
    ))
    .spacing(10.0);

    let row2 = HStack::new((
        Text::new("Cell 2,1").color(Color::BLUE),
        Text::new("Cell 2,2").color(Color::rgb(0.8, 0.4, 0.0)),
    ))
    .spacing(10.0);

    let content_grid = VStack::new((row1, row2)).spacing(5.0);

    let footer = Text::new("Footer Text")
        .font_size(12.0)
        .color(Color::rgb(0.5, 0.5, 0.5));

    // Main layout
    let main_layout = VStack::new((header, content_grid, footer)).spacing(20.0);

    let extracted = MockBackend::extract(&main_layout, &ctx);

    println!("Complex nested layout extracted:");
    println!("  Main spacing: {}", extracted.spacing);

    // Header
    println!("  Header: '{}'", extracted.content.0.content);
    println!("    Font size: {}", extracted.content.0.font_size);

    // Content grid
    println!("  Content grid spacing: {}", extracted.content.1.spacing);
    println!(
        "    Row 1 spacing: {}",
        extracted.content.1.content.0.spacing
    );
    println!(
        "      Cell 1,1: '{}'",
        extracted.content.1.content.0.content.0.content
    );
    println!(
        "      Cell 1,2: '{}'",
        extracted.content.1.content.0.content.1.content
    );
    println!(
        "    Row 2 spacing: {}",
        extracted.content.1.content.1.spacing
    );
    println!(
        "      Cell 2,1: '{}'",
        extracted.content.1.content.1.content.0.content
    );
    println!(
        "      Cell 2,2: '{}'",
        extracted.content.1.content.1.content.1.content
    );

    // Footer
    println!("  Footer: '{}'", extracted.content.2.content);
    println!("    Font size: {}", extracted.content.2.font_size);
}

/// Demonstrates mixing different view types in containers
fn mixed_content_types() {
    let ctx = RenderContext::new();

    // Mix text and buttons in containers
    let text_label = Text::new("Action Panel")
        .font_size(18.0)
        .color(Color::BLACK);

    let button_row = HStack::new((
        Button::new("Save").background_color(Color::GREEN),
        Button::new("Cancel").background_color(Color::RED),
        Button::new("Help").background_color(Color::BLUE),
    ))
    .spacing(12.0);

    let status_text = Text::new("Ready").color(Color::rgb(0.0, 0.6, 0.0));

    let panel = VStack::new((text_label, button_row, status_text)).spacing(15.0);

    let extracted = MockBackend::extract(&panel, &ctx);

    println!("Mixed content panel extracted:");
    println!("  Panel spacing: {}", extracted.spacing);

    // Label
    println!("  Label: '{}'", extracted.content.0.content);
    println!("    Font size: {}", extracted.content.0.font_size);

    // Button row
    println!("  Button row spacing: {}", extracted.content.1.spacing);
    println!("    Save button: '{}'", extracted.content.1.content.0.text);
    println!(
        "    Cancel button: '{}'",
        extracted.content.1.content.1.text
    );
    println!("    Help button: '{}'", extracted.content.1.content.2.text);

    // Status
    println!("  Status: '{}'", extracted.content.2.content);
    println!("    Color: {:?}", extracted.content.2.color);
}

// End of File
