// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! # Styling Example for Ironwood UI Framework
//!
//! This example demonstrates advanced styling patterns and design system techniques
//! for creating professional, visually consistent user interfaces.
//!
//! ## What This Example Shows
//!
//! 1. **Advanced Styling**: Comprehensive styling options and patterns
//! 2. **View Extraction**: How backends extract view data for rendering
//! 3. **Pure View Construction**: Creating views without application logic
//! 4. **Styling Patterns**: Common UI styling approaches and techniques

use ironwood::{backends::mock::MockBackend, prelude::*};

/// Example demonstrating advanced styling patterns and design systems
fn main() {
    println!("🎨 Ironwood Styling Example");
    println!("===========================");
    println!();

    // Create a render context for extraction
    let ctx = RenderContext::new();
    println!("📋 Created render context for view extraction");
    println!();

    // Demonstrate text views with different styling
    demonstrate_text_views(&ctx);
    println!();

    // Demonstrate button views with messages and styling
    demonstrate_button_views(&ctx);
    println!();

    // Demonstrate view extraction and backend processing
    demonstrate_view_extraction(&ctx);
    println!();

    // Demonstrate layout containers with styling
    demonstrate_layout_styling(&ctx);
    println!();

    println!("✅ Styling example completed successfully!");
}

/// Demonstrates advanced text styling patterns and techniques
fn demonstrate_text_views(ctx: &RenderContext) {
    println!("📝 Advanced Text Styling Demonstration");
    println!("--------------------------------------");

    // Typography hierarchy - common pattern for consistent text styling
    let heading_1 = Text::new("Main Heading")
        .font_size(32.0)
        .color(Color::rgb(0.1, 0.1, 0.1)); // Near black

    let heading_2 = Text::new("Section Heading")
        .font_size(24.0)
        .color(Color::rgb(0.2, 0.2, 0.2));

    let heading_3 = Text::new("Subsection Heading")
        .font_size(20.0)
        .color(Color::rgb(0.3, 0.3, 0.3));

    let body_text = Text::new("Body text with comfortable reading size")
        .font_size(16.0)
        .color(Color::rgb(0.15, 0.15, 0.15));

    let caption_text = Text::new("Small caption or metadata text")
        .font_size(12.0)
        .color(Color::rgb(0.5, 0.5, 0.5));

    // Semantic color patterns - using color to convey meaning
    let error_text = Text::new("🚨 Critical Error: System failure detected")
        .font_size(14.0)
        .color(Color::rgb(0.8, 0.1, 0.1)); // Strong red

    let warning_text = Text::new("⚠️ Warning: Check your input")
        .font_size(14.0)
        .color(Color::rgb(0.9, 0.6, 0.0)); // Orange

    let success_text = Text::new("✅ Success: Operation completed")
        .font_size(14.0)
        .color(Color::rgb(0.1, 0.7, 0.1)); // Green

    let info_text = Text::new("ℹ️ Info: Additional details available")
        .font_size(14.0)
        .color(Color::rgb(0.2, 0.5, 0.8)); // Blue

    // Advanced color techniques - transparency and subtle variations
    let overlay_text = Text::new("Overlay text with transparency")
        .font_size(18.0)
        .color(Color::rgba(1.0, 1.0, 1.0, 0.9)); // White with slight transparency

    let accent_text = Text::new("Accent text with brand color")
        .font_size(16.0)
        .color(Color::rgba(0.4, 0.2, 0.8, 1.0)); // Purple brand color

    println!("Typography Hierarchy:");
    let hierarchy = [
        ("H1", &heading_1),
        ("H2", &heading_2),
        ("H3", &heading_3),
        ("Body", &body_text),
        ("Caption", &caption_text),
    ];

    for (level, text) in hierarchy {
        let extracted = MockBackend::extract(text, ctx);
        println!(
            "  {}: \"{}\" ({}px, RGB({:.1}, {:.1}, {:.1}))",
            level,
            extracted.content,
            extracted.font_size,
            extracted.color.r,
            extracted.color.g,
            extracted.color.b
        );
    }

    println!("\nSemantic Colors:");
    let semantic = [
        ("Error", &error_text),
        ("Warning", &warning_text),
        ("Success", &success_text),
        ("Info", &info_text),
    ];

    for (type_name, text) in semantic {
        let extracted = MockBackend::extract(text, ctx);
        println!(
            "  {}: \"{}\" (RGB({:.1}, {:.1}, {:.1}))",
            type_name, extracted.content, extracted.color.r, extracted.color.g, extracted.color.b
        );
    }

    println!("\nAdvanced Techniques:");
    let advanced = [("Overlay", &overlay_text), ("Accent", &accent_text)];

    for (technique, text) in advanced {
        let extracted = MockBackend::extract(text, ctx);
        println!(
            "  {}: \"{}\" (RGBA({:.1}, {:.1}, {:.1}, {:.1}))",
            technique,
            extracted.content,
            extracted.color.r,
            extracted.color.g,
            extracted.color.b,
            extracted.color.a
        );
    }
}

/// Demonstrates advanced button styling patterns and design systems
fn demonstrate_button_views(ctx: &RenderContext) {
    println!("🔘 Advanced Button Styling Demonstration");
    println!("----------------------------------------");

    // Design system approach - consistent button variants
    let primary_button = Button::new("Primary Action")
        .background_color(Color::rgb(0.2, 0.5, 0.9)) // Brand blue
        .with_text(|text| text.color(Color::WHITE).font_size(16.0));

    let secondary_button = Button::new("Secondary Action")
        .background_color(Color::rgb(0.95, 0.95, 0.95)) // Light gray
        .with_text(|text| text.color(Color::rgb(0.2, 0.2, 0.2)).font_size(16.0));

    let tertiary_button = Button::new("Tertiary Action")
        .background_color(Color::rgba(0.0, 0.0, 0.0, 0.0)) // Transparent
        .with_text(|text| text.color(Color::rgb(0.2, 0.5, 0.9)).font_size(16.0));

    // Semantic button colors - conveying meaning through color
    let success_button = Button::new("Confirm")
        .background_color(Color::rgb(0.1, 0.7, 0.1)) // Green
        .with_text(|text| text.color(Color::WHITE).font_size(16.0));

    let warning_button = Button::new("Proceed with Caution")
        .background_color(Color::rgb(0.9, 0.6, 0.0)) // Orange
        .with_text(|text| text.color(Color::WHITE).font_size(16.0));

    let danger_button = Button::new("Delete")
        .background_color(Color::rgb(0.8, 0.1, 0.1)) // Red
        .with_text(|text| text.color(Color::WHITE).font_size(16.0));

    // Size variations - different button sizes for hierarchy
    let large_button = Button::new("Large CTA")
        .background_color(Color::rgb(0.2, 0.5, 0.9))
        .with_text(|text| text.color(Color::WHITE).font_size(20.0));

    let small_button = Button::new("Small Action")
        .background_color(Color::rgb(0.2, 0.5, 0.9))
        .with_text(|text| text.color(Color::WHITE).font_size(12.0));

    // State demonstrations
    let disabled_button = Button::new("Disabled")
        .background_color(Color::rgb(0.7, 0.7, 0.7))
        .with_text(|text| text.color(Color::rgb(0.5, 0.5, 0.5)))
        .disable();

    // Advanced styling - subtle colors and transparency
    let glass_button = Button::new("Glass Effect")
        .background_color(Color::rgba(1.0, 1.0, 1.0, 0.2)) // Semi-transparent white
        .with_text(|text| text.color(Color::rgb(0.1, 0.1, 0.1)).font_size(16.0));

    println!("Design System Variants:");
    let variants = [
        ("Primary", &primary_button),
        ("Secondary", &secondary_button),
        ("Tertiary", &tertiary_button),
    ];

    for (variant, button) in variants {
        let extracted = MockBackend::extract(&button.view(), ctx);
        println!(
            "  {}: \"{}\" | BG: RGBA({:.1}, {:.1}, {:.1}, {:.1}) | Text: RGB({:.1}, {:.1}, {:.1})",
            variant,
            extracted.text,
            extracted.background_color.r,
            extracted.background_color.g,
            extracted.background_color.b,
            extracted.background_color.a,
            extracted.text_style.color.r,
            extracted.text_style.color.g,
            extracted.text_style.color.b
        );
    }

    println!("\nSemantic Colors:");
    let semantic = [
        ("Success", &success_button),
        ("Warning", &warning_button),
        ("Danger", &danger_button),
    ];

    for (meaning, button) in semantic {
        let extracted = MockBackend::extract(&button.view(), ctx);
        println!(
            "  {}: \"{}\" | BG: RGB({:.1}, {:.1}, {:.1})",
            meaning,
            extracted.text,
            extracted.background_color.r,
            extracted.background_color.g,
            extracted.background_color.b
        );
    }

    println!("\nSize Hierarchy:");
    let sizes = [
        ("Large", &large_button),
        ("Normal", &primary_button),
        ("Small", &small_button),
    ];

    for (size, button) in sizes {
        let extracted = MockBackend::extract(&button.view(), ctx);
        println!(
            "  {}: \"{}\" | Font: {}px",
            size, extracted.text, extracted.text_style.font_size
        );
    }

    println!("\nSpecial Effects:");
    let effects = [("Disabled", &disabled_button), ("Glass", &glass_button)];

    for (effect, button) in effects {
        let extracted = MockBackend::extract(&button.view(), ctx);
        let state = if extracted.interaction_state.is_enabled() {
            "Enabled"
        } else {
            "Disabled"
        };
        println!(
            "  {}: \"{}\" | {} | Alpha: {:.1}",
            effect, extracted.text, state, extracted.background_color.a
        );
    }
}

/// Demonstrates the view extraction process and how backends process views
fn demonstrate_view_extraction(ctx: &RenderContext) {
    println!("🔄 View Extraction Demonstration");
    println!("--------------------------------");

    // Create a complex view with custom styling
    let complex_text = Text::new("Complex styled text with custom colors")
        .font_size(20.0)
        .color(Color::rgba(0.2, 0.6, 0.8, 0.9)); // Custom RGBA color

    let complex_button = Button::new("Complex Action")
        .background_color(Color::rgba(0.3, 0.7, 0.3, 1.0))
        .with_text(|text| text.color(Color::rgba(0.9, 0.9, 0.9, 1.0)));

    println!("  Original Text View:");
    println!("    Content: {:?}", complex_text.content);
    println!("    Font Size: {}", complex_text.style.font_size);
    println!(
        "    Color: RGBA({}, {}, {}, {})",
        complex_text.style.color.r,
        complex_text.style.color.g,
        complex_text.style.color.b,
        complex_text.style.color.a
    );

    println!();
    println!("  Original Button Component:");
    println!("    Text: {:?}", complex_button.text.content);
    println!("    Enabled: {}", complex_button.is_enabled());
    println!("    Pressed: {}", complex_button.is_pressed());
    println!(
        "    Background: RGBA({}, {}, {}, {})",
        complex_button.background_color.r,
        complex_button.background_color.g,
        complex_button.background_color.b,
        complex_button.background_color.a
    );

    println!();
    println!("  After MockBackend Extraction:");

    // Extract the views using the mock backend
    let text_extracted = MockBackend::extract(&complex_text, ctx);
    let button_extracted = MockBackend::extract(&complex_button.view(), ctx);

    println!("    Text -> MockText:");
    println!("      Content: {:?}", text_extracted.content);
    println!("      Font Size: {}", text_extracted.font_size);
    println!("      Color: {:?}", text_extracted.color);

    println!("    Button -> MockButton:");
    println!("      Text: {:?}", button_extracted.text);
    println!(
        "      Enabled: {}",
        button_extracted.interaction_state.is_enabled()
    );
    println!(
        "      Pressed: {}",
        button_extracted.interaction_state.is_pressed()
    );
    println!("      Background: {:?}", button_extracted.background_color);
    println!("      Text Color: {:?}", button_extracted.text_style.color);

    println!();
    println!("  🔍 Notice how all view data is perfectly preserved through extraction!");
    println!("     This demonstrates the ViewExtractor pattern's effectiveness.");
}

/// Demonstrates layout containers with styling
fn demonstrate_layout_styling(ctx: &RenderContext) {
    println!("📐 Layout Container Styling Demonstration");
    println!("------------------------------------------");

    // Create styled text components for layout
    let title = Text::new("Application Dashboard")
        .font_size(24.0)
        .color(Color::rgb(0.1, 0.1, 0.1));

    let subtitle = Text::new("Real-time system status")
        .font_size(16.0)
        .color(Color::rgb(0.4, 0.4, 0.4));

    let status_good = Text::new("✅ All systems operational")
        .font_size(14.0)
        .color(Color::rgb(0.1, 0.7, 0.1));

    let status_warning = Text::new("⚠️ Minor issues detected")
        .font_size(14.0)
        .color(Color::rgb(0.9, 0.6, 0.0));

    // Create action buttons with consistent styling
    let primary_action = Button::new("Refresh Data")
        .background_color(Color::rgb(0.2, 0.5, 0.9))
        .with_text(|text| text.color(Color::WHITE).font_size(16.0));

    let secondary_action = Button::new("View Details")
        .background_color(Color::rgb(0.95, 0.95, 0.95))
        .with_text(|text| text.color(Color::rgb(0.2, 0.2, 0.2)).font_size(16.0));

    // Create a complete dashboard layout using containers
    let header_section = VStack::new((title, subtitle))
        .spacing(8.0)
        .alignment(Alignment::Leading);

    let status_section = VStack::new((status_good, status_warning))
        .spacing(4.0)
        .alignment(Alignment::Leading);

    let action_section = HStack::new((primary_action.view(), secondary_action.view()))
        .spacing(12.0)
        .alignment(Alignment::Center);

    let complete_dashboard = VStack::new((
        header_section,
        Spacer::min_size(20.0), // Add space between sections
        status_section,
        Spacer::min_size(16.0),
        action_section,
    ))
    .spacing(0.0) // No additional spacing since we use explicit spacers
    .alignment(Alignment::Leading);

    println!("Dashboard Layout Structure:");
    let dashboard_extracted = MockBackend::extract(&complete_dashboard, ctx);

    println!(
        "  Main VStack: {} spacing, {:?} alignment",
        dashboard_extracted.spacing, dashboard_extracted.alignment
    );

    // Extract header section
    let header_extracted = MockBackend::extract(&complete_dashboard.content.0, ctx);
    println!(
        "  Header Section: {} spacing, {:?} alignment",
        header_extracted.spacing, header_extracted.alignment
    );
    println!("    Title: '{}'", header_extracted.content.0.content);
    println!("    Subtitle: '{}'", header_extracted.content.1.content);

    // Extract spacer
    let spacer_extracted = MockBackend::extract(&complete_dashboard.content.1, ctx);
    println!("  Spacer: {} min_size", spacer_extracted.min_size);

    // Extract status section
    let status_extracted = MockBackend::extract(&complete_dashboard.content.2, ctx);
    println!(
        "  Status Section: {} spacing, {:?} alignment",
        status_extracted.spacing, status_extracted.alignment
    );
    println!("    Good status: '{}'", status_extracted.content.0.content);
    println!(
        "    Warning status: '{}'",
        status_extracted.content.1.content
    );

    // Extract action section
    let action_extracted = MockBackend::extract(&complete_dashboard.content.4, ctx);
    println!(
        "  Action Section: {} spacing, {:?} alignment",
        action_extracted.spacing, action_extracted.alignment
    );
    println!("    Primary: '{}'", action_extracted.content.0.text);
    println!("    Secondary: '{}'", action_extracted.content.1.text);

    println!();
    println!("Design System Patterns Demonstrated:");
    println!("  📝 Typography hierarchy (24px title, 16px subtitle, 14px status)");
    println!("  🎨 Semantic colors (green for success, orange for warning)");
    println!("  📐 Consistent spacing (8px, 12px, 16px, 20px)");
    println!("  🔘 Button styling patterns (primary vs secondary)");
    println!("  📋 Layout composition (nested VStack/HStack containers)");
    println!("  ↔️  Flexible spacing (using Spacer components)");
    println!();
    println!(
        "  🔍 This demonstrates how layout containers enable complex, maintainable UI designs!"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_views_compile_and_extract() {
        // Test that all the views in the example can be created and extracted
        let ctx = RenderContext::new();

        // Test text views
        let title = Text::new("Test Title").font_size(24.0).color(Color::BLUE);
        let extracted_text = MockBackend::extract(&title, &ctx);
        assert_eq!(extracted_text.content, "Test Title");
        assert_eq!(extracted_text.font_size, 24.0);
        assert_eq!(extracted_text.color, Color::BLUE);

        // Test button components
        let button = Button::new("Test Button")
            .background_color(Color::RED)
            .enable();
        let extracted_button = MockBackend::extract(&button.view(), &ctx);
        assert_eq!(extracted_button.text, "Test Button");
        assert!(extracted_button.interaction_state.is_enabled());
        assert_eq!(extracted_button.background_color, Color::RED);
    }

    #[test]
    fn button_component_states_work() {
        // Test that different button states work correctly
        let ctx = RenderContext::new();

        let enabled_button = Button::new("Enabled").enable();
        let disabled_button = Button::new("Disabled").disable();

        let enabled_extracted = MockBackend::extract(&enabled_button.view(), &ctx);
        let disabled_extracted = MockBackend::extract(&disabled_button.view(), &ctx);

        assert!(enabled_extracted.interaction_state.is_enabled());
        assert!(!disabled_extracted.interaction_state.is_enabled());
    }

    #[test]
    fn custom_colors_preserved() {
        // Test that custom RGBA colors are preserved through extraction
        let ctx = RenderContext::new();
        let custom_color = Color::rgba(0.1, 0.2, 0.3, 0.4);

        let text = Text::new("Custom").color(custom_color);
        let button = Button::new("Custom").background_color(custom_color);

        let text_extracted = MockBackend::extract(&text, &ctx);
        let button_extracted = MockBackend::extract(&button.view(), &ctx);

        assert_eq!(text_extracted.color, custom_color);
        assert_eq!(button_extracted.background_color, custom_color);
    }
}

// End of File
