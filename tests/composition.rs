// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Integration tests for composition patterns
//!
//! These tests validate that tuple composition and layout containers work
//! correctly in real application scenarios, including complex nested
//! hierarchies, mixed composition patterns, and integration with the
//! Model/View/Message architecture.

use ironwood::{backends::mock::MockBackend, prelude::*};

/// Test that demonstrates tuple composition as the core composition mechanism.
///
/// This validates that tuples can be used to compose complex UIs and that
/// the extraction system properly handles nested tuple structures.
#[test]
fn tuple_composition_integration() {
    // Create a complex UI using tuple composition
    let header = (
        Text::new("My Application")
            .font_size(24.0)
            .color(Color::BLUE),
        Text::new("Version 1.0")
            .font_size(12.0)
            .color(Color::rgb(0.6, 0.6, 0.6)),
    );

    let toolbar = (
        Button::new("New").background_color(Color::GREEN).view(),
        Button::new("Open").background_color(Color::BLUE).view(),
        Button::new("Save")
            .background_color(Color::rgb(0.8, 0.8, 0.8))
            .view(),
        Button::new("Delete").background_color(Color::RED).view(),
    );

    let status_bar = (
        Text::new("Ready").color(Color::GREEN),
        Text::new("Items: 42").color(Color::BLACK),
        Text::new("Selected: 3").color(Color::BLUE),
    );

    // Compose the complete UI using nested tuples
    let complete_ui = (header, toolbar, status_bar);

    // Test extraction of the complete nested structure
    let ctx = RenderContext::new();
    let extracted = MockBackend::extract(&complete_ui, &ctx);

    // Verify the nested structure is preserved
    let (
        (header_title, header_version),
        (new_btn, open_btn, save_btn, delete_btn),
        (status_ready, status_items, status_selected),
    ) = extracted;

    // Verify header components
    assert_eq!(header_title.content, "My Application");
    assert_eq!(header_title.font_size, 24.0);
    assert_eq!(header_title.color, Color::BLUE);

    assert_eq!(header_version.content, "Version 1.0");
    assert_eq!(header_version.font_size, 12.0);
    assert_eq!(header_version.color, Color::rgb(0.6, 0.6, 0.6));

    // Verify toolbar buttons
    assert_eq!(new_btn.text, "New");
    assert_eq!(new_btn.background_color, Color::GREEN);

    assert_eq!(open_btn.text, "Open");
    assert_eq!(open_btn.background_color, Color::BLUE);

    assert_eq!(save_btn.text, "Save");
    assert_eq!(save_btn.background_color, Color::rgb(0.8, 0.8, 0.8));

    assert_eq!(delete_btn.text, "Delete");
    assert_eq!(delete_btn.background_color, Color::RED);

    // Verify status bar components
    assert_eq!(status_ready.content, "Ready");
    assert_eq!(status_ready.color, Color::GREEN);

    assert_eq!(status_items.content, "Items: 42");
    assert_eq!(status_items.color, Color::BLACK);

    assert_eq!(status_selected.content, "Selected: 3");
    assert_eq!(status_selected.color, Color::BLUE);
}

/// Test that demonstrates layout containers in real application scenarios.
///
/// This validates that VStack and HStack work correctly with complex content
/// and that spacing is properly handled.
#[test]
fn layout_containers_integration() {
    // Create a form layout using VStack and HStack
    let form_title = Text::new("User Registration")
        .font_size(20.0)
        .color(Color::BLACK);

    let name_row = HStack::new((
        Text::new("Name:").color(Color::BLACK),
        Text::new("[Input Field]").color(Color::rgb(0.5, 0.5, 0.5)),
    ))
    .spacing(10.0);

    let email_row = HStack::new((
        Text::new("Email:").color(Color::BLACK),
        Text::new("[Input Field]").color(Color::rgb(0.5, 0.5, 0.5)),
    ))
    .spacing(10.0);

    let button_row = HStack::new((
        Button::new("Cancel")
            .background_color(Color::rgb(0.8, 0.8, 0.8))
            .view(),
        Button::new("Register")
            .background_color(Color::GREEN)
            .view(),
    ))
    .spacing(15.0);

    let complete_form = VStack::new((form_title, name_row, email_row, button_row)).spacing(20.0);

    // Test extraction of the complete form
    let ctx = RenderContext::new();
    let extracted = MockBackend::extract(&complete_form, &ctx);

    // Verify the VStack structure
    assert_eq!(extracted.spacing, 20.0);
    let (title_extracted, name_row_extracted, email_row_extracted, button_row_extracted) =
        extracted.content;

    // Verify form title
    assert_eq!(title_extracted.content, "User Registration");
    assert_eq!(title_extracted.font_size, 20.0);

    // Verify name row (HStack)
    assert_eq!(name_row_extracted.spacing, 10.0);
    let (name_label, name_input) = name_row_extracted.content;
    assert_eq!(name_label.content, "Name:");
    assert_eq!(name_input.content, "[Input Field]");

    // Verify email row (HStack)
    assert_eq!(email_row_extracted.spacing, 10.0);
    let (email_label, email_input) = email_row_extracted.content;
    assert_eq!(email_label.content, "Email:");
    assert_eq!(email_input.content, "[Input Field]");

    // Verify button row (HStack)
    assert_eq!(button_row_extracted.spacing, 15.0);
    let (cancel_btn, register_btn) = button_row_extracted.content;
    assert_eq!(cancel_btn.text, "Cancel");
    assert_eq!(cancel_btn.background_color, Color::rgb(0.8, 0.8, 0.8));
    assert_eq!(register_btn.text, "Register");
    assert_eq!(register_btn.background_color, Color::GREEN);
}

/// Test that demonstrates mixed composition patterns.
///
/// This validates that tuple composition and layout containers can be
/// mixed together in complex hierarchies.
#[test]
fn mixed_composition_patterns() {
    // Create a dashboard using mixed composition patterns

    // Header using tuple composition
    let header = (
        Text::new("Dashboard").font_size(24.0).color(Color::BLUE),
        Button::new("Settings")
            .background_color(Color::rgb(0.7, 0.7, 0.7))
            .view(),
    );

    // Metrics section using VStack with tuple content
    let metrics = VStack::new((
        Text::new("Metrics").font_size(18.0).color(Color::BLACK),
        (
            Text::new("Users: 1,234").color(Color::GREEN),
            Text::new("Revenue: $5,678").color(Color::BLUE),
            Text::new("Growth: +12%").color(Color::GREEN),
        ),
    ))
    .spacing(10.0);

    // Actions section using HStack with mixed content
    let actions = HStack::new((
        VStack::new((
            Text::new("Quick Actions")
                .font_size(16.0)
                .color(Color::BLACK),
            Button::new("Export Data")
                .background_color(Color::BLUE)
                .view(),
        ))
        .spacing(5.0),
        (
            Button::new("Refresh").background_color(Color::GREEN).view(),
            Button::new("Help")
                .background_color(Color::rgb(0.8, 0.8, 0.8))
                .view(),
        ),
    ))
    .spacing(20.0);

    // Complete dashboard using tuple composition at the top level
    let dashboard = (header, metrics, actions);

    // Test extraction of the complete mixed structure
    let ctx = RenderContext::new();
    let extracted = MockBackend::extract(&dashboard, &ctx);

    let (header_extracted, metrics_extracted, actions_extracted) = extracted;

    // Verify header (tuple)
    let (dashboard_title, settings_btn) = header_extracted;
    assert_eq!(dashboard_title.content, "Dashboard");
    assert_eq!(dashboard_title.font_size, 24.0);
    assert_eq!(settings_btn.text, "Settings");

    // Verify metrics (VStack with tuple content)
    assert_eq!(metrics_extracted.spacing, 10.0);
    let (metrics_title, metrics_data) = metrics_extracted.content;
    assert_eq!(metrics_title.content, "Metrics");
    let (users_metric, revenue_metric, growth_metric) = metrics_data;
    assert_eq!(users_metric.content, "Users: 1,234");
    assert_eq!(revenue_metric.content, "Revenue: $5,678");
    assert_eq!(growth_metric.content, "Growth: +12%");

    // Verify actions (HStack with mixed content)
    assert_eq!(actions_extracted.spacing, 20.0);
    let (quick_actions_section, action_buttons) = actions_extracted.content;

    // Verify quick actions section (VStack)
    assert_eq!(quick_actions_section.spacing, 5.0);
    let (quick_actions_title, export_btn) = quick_actions_section.content;
    assert_eq!(quick_actions_title.content, "Quick Actions");
    assert_eq!(export_btn.text, "Export Data");

    // Verify action buttons (tuple)
    let (refresh_btn, help_btn) = action_buttons;
    assert_eq!(refresh_btn.text, "Refresh");
    assert_eq!(help_btn.text, "Help");
}

/// Test that composition patterns work with the Model/View/Message architecture.
///
/// This validates that composed views can be used in models and that
/// message handling works correctly with composed structures.
#[test]
fn composition_with_model_integration() {
    #[derive(Debug, Clone)]
    struct AppModel {
        // Use tuple composition for the header
        header_text: Text,
        header_button: Button,
        // Use layout containers for the main content
        text: Text,
        button1: Button,
        button2: Button,
        // Use simple component for footer
        footer: Text,
    }

    #[derive(Debug, Clone)]
    enum AppMessage {
        HeaderButton,
        MainButton1,
        MainButton2,
        UpdateStatus(String),
    }

    impl Message for AppMessage {}

    impl Model for AppModel {
        type Message = AppMessage;
        type View = VStack<(
            (Text, ButtonView),
            VStack<(Text, HStack<(ButtonView, ButtonView)>)>,
            Text,
        )>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                AppMessage::HeaderButton => Self {
                    header_button: self.header_button.update(ButtonMessage::Clicked),
                    footer: Text::new("Header button clicked").color(Color::BLUE),
                    ..self
                },
                AppMessage::MainButton1 => Self {
                    button1: self.button1.update(ButtonMessage::Clicked),
                    footer: Text::new("Main button 1 clicked").color(Color::GREEN),
                    ..self
                },
                AppMessage::MainButton2 => Self {
                    button2: self.button2.update(ButtonMessage::Clicked),
                    footer: Text::new("Main button 2 clicked").color(Color::RED),
                    ..self
                },
                AppMessage::UpdateStatus(status) => Self {
                    footer: Text::new(status).color(Color::BLACK),
                    ..self
                },
            }
        }

        fn view(&self) -> Self::View {
            VStack::new((
                (self.header_text.clone(), self.header_button.view()),
                VStack::new((
                    self.text.clone(),
                    HStack::new((self.button1.view(), self.button2.view())).spacing(10.0),
                ))
                .spacing(15.0),
                self.footer.clone(),
            ))
            .spacing(20.0)
        }
    }

    // Create initial model with composed views
    let initial_model = AppModel {
        header_text: Text::new("My App").font_size(20.0).color(Color::BLUE),
        header_button: Button::new("Menu").background_color(Color::rgb(0.8, 0.8, 0.8)),
        text: Text::new("Welcome to the app!")
            .font_size(16.0)
            .color(Color::BLACK),
        button1: Button::new("Action 1").background_color(Color::GREEN),
        button2: Button::new("Action 2").background_color(Color::BLUE),
        footer: Text::new("Ready").color(Color::GREEN),
    };

    // Test message handling with composed views
    let mut model = initial_model;

    // Test header button click
    model = model.update(AppMessage::HeaderButton);
    assert_eq!(model.footer.content, "Header button clicked");

    // Test main button 1 click
    model = model.update(AppMessage::MainButton1);
    assert_eq!(model.footer.content, "Main button 1 clicked");

    // Test main button 2 click
    model = model.update(AppMessage::MainButton2);
    assert_eq!(model.footer.content, "Main button 2 clicked");

    // Test status update
    model = model.update(AppMessage::UpdateStatus(
        "All systems operational".to_string(),
    ));
    assert_eq!(model.footer.content, "All systems operational");

    // Test extraction of the final composed model
    let ctx = RenderContext::new();

    // Test extracting individual components from the model fields
    let header_text_extracted = MockBackend::extract(&model.header_text, &ctx);
    let header_button_extracted = MockBackend::extract(&model.header_button.view(), &ctx);
    let main_text_extracted = MockBackend::extract(&model.text, &ctx);
    let footer_extracted = MockBackend::extract(&model.footer, &ctx);

    // Verify individual components
    assert_eq!(header_text_extracted.content, "My App");
    assert_eq!(header_button_extracted.text, "Menu");
    assert_eq!(main_text_extracted.content, "Welcome to the app!");
    assert_eq!(footer_extracted.content, "All systems operational");

    // Extract the full composed view to test structure
    let full_view_extracted = MockBackend::extract(&model.view(), &ctx);
    assert_eq!(full_view_extracted.spacing, 20.0);

    let (header_tuple, main_vstack, footer_text) = full_view_extracted.content;

    // Verify header tuple structure
    let (title_extracted, menu_btn_extracted) = header_tuple;
    assert_eq!(title_extracted.content, "My App");
    assert_eq!(menu_btn_extracted.text, "Menu");

    // Verify main content VStack structure
    assert_eq!(main_vstack.spacing, 15.0);
    let (welcome_text, button_row) = main_vstack.content;
    assert_eq!(welcome_text.content, "Welcome to the app!");
    assert_eq!(button_row.spacing, 10.0);
    let (action1_btn, action2_btn) = button_row.content;
    assert_eq!(action1_btn.text, "Action 1");
    assert_eq!(action2_btn.text, "Action 2");

    // Verify footer
    assert_eq!(footer_text.content, "All systems operational");
}

/// Test that deeply nested composition patterns work correctly.
///
/// This validates that the framework can handle arbitrarily deep
/// nesting of tuples and containers without issues.
#[test]
fn deep_composition_nesting() {
    // Create a deeply nested structure
    let level1 = (
        Text::new("Level 1").color(Color::BLACK),
        VStack::new((
            Text::new("Level 2").color(Color::BLUE),
            HStack::new((
                Text::new("Level 3a").color(Color::GREEN),
                (
                    Text::new("Level 4a").color(Color::RED),
                    VStack::new((
                        Text::new("Level 5").color(Color::rgb(0.5, 0.0, 0.5)),
                        (
                            Text::new("Level 6a").color(Color::rgb(0.8, 0.4, 0.0)),
                            Text::new("Level 6b").color(Color::rgb(0.0, 0.8, 0.8)),
                        ),
                    ))
                    .spacing(2.0),
                    Text::new("Level 4b").color(Color::rgb(0.6, 0.6, 0.0)),
                ),
                Text::new("Level 3b").color(Color::rgb(0.8, 0.0, 0.8)),
            ))
            .spacing(5.0),
        ))
        .spacing(10.0),
    );

    // Test extraction of the deeply nested structure
    let ctx = RenderContext::new();
    let extracted = MockBackend::extract(&level1, &ctx);

    // Navigate through the nested structure to verify it's preserved
    let (level1_text, level2_vstack) = extracted;
    assert_eq!(level1_text.content, "Level 1");
    assert_eq!(level2_vstack.spacing, 10.0);

    let (level2_text, level3_hstack) = level2_vstack.content;
    assert_eq!(level2_text.content, "Level 2");
    assert_eq!(level3_hstack.spacing, 5.0);

    let (level3a_text, level4_tuple, level3b_text) = level3_hstack.content;
    assert_eq!(level3a_text.content, "Level 3a");
    assert_eq!(level3b_text.content, "Level 3b");

    let (level4a_text, level5_vstack, level4b_text) = level4_tuple;
    assert_eq!(level4a_text.content, "Level 4a");
    assert_eq!(level4b_text.content, "Level 4b");
    assert_eq!(level5_vstack.spacing, 2.0);

    let (level5_text, level6_tuple) = level5_vstack.content;
    assert_eq!(level5_text.content, "Level 5");

    let (level6a_text, level6b_text) = level6_tuple;
    assert_eq!(level6a_text.content, "Level 6a");
    assert_eq!(level6b_text.content, "Level 6b");

    // Verify all colors are preserved through the deep nesting
    assert_eq!(level1_text.color, Color::BLACK);
    assert_eq!(level2_text.color, Color::BLUE);
    assert_eq!(level3a_text.color, Color::GREEN);
    assert_eq!(level3b_text.color, Color::rgb(0.8, 0.0, 0.8));
    assert_eq!(level4a_text.color, Color::RED);
    assert_eq!(level4b_text.color, Color::rgb(0.6, 0.6, 0.0));
    assert_eq!(level5_text.color, Color::rgb(0.5, 0.0, 0.5));
    assert_eq!(level6a_text.color, Color::rgb(0.8, 0.4, 0.0));
    assert_eq!(level6b_text.color, Color::rgb(0.0, 0.8, 0.8));
}

// End of File
