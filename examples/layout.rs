// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Layout Example
//!
//! This example demonstrates the layout features in Ironwood:
//! - Option<V> for conditional rendering
//! - Spacer for flexible spacing
//! - Alignment for container alignment

use ironwood::{backends::mock::MockBackend, prelude::*};

fn main() {
    println!("=== Layout Example ===\n");

    // Create render context
    let ctx = RenderContext::new();

    // 1. Optional Views
    println!("1. Optional Views");

    // Some view
    let some_text = Some(Text::new("This text is present"));
    let some_extracted = MockBackend::extract(&some_text, &ctx).unwrap();
    println!("Some(Text): {:?}", some_extracted);

    // None view
    let none_text: Option<Text> = None;
    let none_extracted = MockBackend::extract(&none_text, &ctx).unwrap();
    println!("None: {:?}", none_extracted);

    println!();

    // 2. Spacer Views
    println!("2. Spacer Views");

    // Default spacer
    let default_spacer = Spacer::new();
    let default_extracted = MockBackend::extract(&default_spacer, &ctx).unwrap();
    println!("Default spacer: min_size = {}", default_extracted.min_size);

    // Spacer with minimum size
    let sized_spacer = Spacer::min_size(50.0);
    let sized_extracted = MockBackend::extract(&sized_spacer, &ctx).unwrap();
    println!("Sized spacer: min_size = {}", sized_extracted.min_size);

    println!();

    // 3. Alignment in Containers
    println!("3. Container Alignment");

    // VStack with different alignments
    let leading_vstack = VStack::new((Text::new("Leading aligned"), Text::new("content")))
        .alignment(Alignment::Leading);
    let leading_extracted = MockBackend::extract(&leading_vstack, &ctx).unwrap();
    println!(
        "VStack leading alignment: {:?}",
        leading_extracted.alignment
    );

    let center_vstack = VStack::new((Text::new("Center aligned"), Text::new("content")))
        .alignment(Alignment::Center);
    let center_extracted = MockBackend::extract(&center_vstack, &ctx).unwrap();
    println!("VStack center alignment: {:?}", center_extracted.alignment);

    let trailing_vstack = VStack::new((Text::new("Trailing aligned"), Text::new("content")))
        .alignment(Alignment::Trailing);
    let trailing_extracted = MockBackend::extract(&trailing_vstack, &ctx).unwrap();
    println!(
        "VStack trailing alignment: {:?}",
        trailing_extracted.alignment
    );

    // HStack with alignment
    let center_hstack =
        HStack::new((Text::new("Left"), Text::new("Right"))).alignment(Alignment::Center);
    let hstack_extracted = MockBackend::extract(&center_hstack, &ctx).unwrap();
    println!("HStack center alignment: {:?}", hstack_extracted.alignment);

    println!();

    // 4. Practical Layout Example
    println!("4. Practical Layout with All Features");

    // Create a toolbar layout using all new features
    let show_help = true; // Conditional flag
    let toolbar = HStack::new((
        Text::new("App Title"),
        Spacer::new(), // Push everything to the sides
        VStack::new((
            Text::new("Status: Ready"),
            if show_help {
                Some(Text::new("Help available"))
            } else {
                None
            },
        ))
        .alignment(Alignment::Trailing),
    ))
    .spacing(8.0)
    .alignment(Alignment::Center);

    let toolbar_extracted = MockBackend::extract(&toolbar, &ctx).unwrap();
    println!("Toolbar layout extracted:");
    println!("  Spacing: {}", toolbar_extracted.spacing);
    println!("  Alignment: {:?}", toolbar_extracted.alignment);
    println!("  Title: '{}'", toolbar_extracted.content.0.content);
    println!(
        "  Spacer min_size: {}",
        toolbar_extracted.content.1.min_size
    );
    println!(
        "  Status panel alignment: {:?}",
        toolbar_extracted.content.2.alignment
    );
    println!(
        "  Status: '{}'",
        toolbar_extracted.content.2.content.0.content
    );

    // Check if help text is present
    match &toolbar_extracted.content.2.content.1 {
        Some(help_text) => println!("  Help: '{}'", help_text.content),
        None => println!("  Help: Not shown"),
    }

    println!("\n=== Example Complete ===");
}

// End of File
