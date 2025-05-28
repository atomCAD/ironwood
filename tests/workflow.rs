// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Integration tests for cross-module workflow patterns
//!
//! These tests validate complete user interaction flows across all framework
//! modules: Model → View → Extraction → Backend. They ensure that interaction
//! traits, styling, and component behavior work together seamlessly.

use ironwood::{backends::mock::MockBackend, prelude::*};

#[derive(Debug, Clone, Copy)]
enum ActionType {
    Increment,
    Decrement,
    Reset,
    ProgrammaticUpdate,
}

impl ActionType {
    fn as_str(self) -> &'static str {
        match self {
            ActionType::Increment => "increment",
            ActionType::Decrement => "decrement",
            ActionType::Reset => "reset",
            ActionType::ProgrammaticUpdate => "programmatic update",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Theme {
    Default,
    Dark,
    Light,
    HighContrast,
}

impl Theme {
    fn as_str(self) -> &'static str {
        match self {
            Theme::Default => "default",
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::HighContrast => "high_contrast",
        }
    }
}

/// Test complete user interaction workflow from model to backend extraction.
///
/// This validates the full cycle: user interaction → message → model update →
/// view creation → backend extraction, ensuring all modules work together.
#[test]
fn complete_user_interaction_workflow() {
    // Define an application model with interactive components
    #[derive(Debug, Clone)]
    struct AppModel {
        counter: i32,
        increment_button: Button,
        decrement_button: Button,
        reset_button: Button,
        display_text: Text,
        status_message: Text,
    }

    #[derive(Debug, Clone)]
    enum AppMessage {
        IncrementButton(ButtonMessage),
        DecrementButton(ButtonMessage),
        ResetButton(ButtonMessage),
        UpdateCounter(i32),
    }

    impl Message for AppMessage {}

    impl AppModel {
        fn new() -> Self {
            Self {
                counter: 0,
                increment_button: Button::new("+")
                    .background_color(Color::GREEN)
                    .with_text(|text| text.color(Color::WHITE).font_size(18.0)),
                decrement_button: Button::new("-")
                    .background_color(Color::RED)
                    .with_text(|text| text.color(Color::WHITE).font_size(18.0)),
                reset_button: Button::new("Reset")
                    .background_color(Color::BLUE)
                    .with_text(|text| text.color(Color::WHITE)),
                display_text: Text::new("Count: 0").font_size(24.0).color(Color::BLACK),
                status_message: Text::new("Ready")
                    .font_size(14.0)
                    .color(Color::rgb(0.5, 0.5, 0.5)),
            }
        }

        fn create_display_text(counter: i32) -> Text {
            Text::new(format!("Count: {}", counter))
                .font_size(24.0)
                .color(match counter.cmp(&0) {
                    std::cmp::Ordering::Greater => Color::GREEN,
                    std::cmp::Ordering::Less => Color::RED,
                    std::cmp::Ordering::Equal => Color::BLACK,
                })
        }

        fn create_status_message(action: ActionType) -> Text {
            Text::new(format!("Last action: {}", action.as_str()))
                .font_size(14.0)
                .color(Color::BLUE)
        }
    }

    impl Model for AppModel {
        type Message = AppMessage;
        type View = VStack<(Text, Text, HStack<(ButtonView, ButtonView, ButtonView)>)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                AppMessage::IncrementButton(button_msg) => match button_msg {
                    ButtonMessage::Clicked => {
                        let new_counter = self.counter.saturating_add(1);
                        Self {
                            counter: new_counter,
                            increment_button: self.increment_button.update(button_msg),
                            display_text: Self::create_display_text(new_counter),
                            status_message: Self::create_status_message(ActionType::Increment),
                            ..self
                        }
                    }
                    ButtonMessage::Interaction(_) => Self {
                        increment_button: self.increment_button.update(button_msg),
                        ..self
                    },
                },
                AppMessage::DecrementButton(button_msg) => match button_msg {
                    ButtonMessage::Clicked => {
                        let new_counter = self.counter.saturating_sub(1);
                        Self {
                            counter: new_counter,
                            decrement_button: self.decrement_button.update(button_msg),
                            display_text: Self::create_display_text(new_counter),
                            status_message: Self::create_status_message(ActionType::Decrement),
                            ..self
                        }
                    }
                    ButtonMessage::Interaction(_) => Self {
                        decrement_button: self.decrement_button.update(button_msg),
                        ..self
                    },
                },
                AppMessage::ResetButton(button_msg) => match button_msg {
                    ButtonMessage::Clicked => Self {
                        counter: 0,
                        reset_button: self.reset_button.update(button_msg),
                        display_text: Self::create_display_text(0),
                        status_message: Self::create_status_message(ActionType::Reset),
                        ..self
                    },
                    ButtonMessage::Interaction(_) => Self {
                        reset_button: self.reset_button.update(button_msg),
                        ..self
                    },
                },
                AppMessage::UpdateCounter(value) => Self {
                    counter: value,
                    display_text: Self::create_display_text(value),
                    status_message: Self::create_status_message(ActionType::ProgrammaticUpdate),
                    ..self
                },
            }
        }

        fn view(&self) -> Self::View {
            VStack::new((
                self.display_text.clone(),
                self.status_message.clone(),
                HStack::new((
                    self.decrement_button.view(),
                    self.reset_button.view(),
                    self.increment_button.view(),
                ))
                .spacing(8.0),
            ))
            .spacing(16.0)
        }
    }

    // Test complete workflow: Model → Message → Update → View → Extraction
    let mut app = AppModel::new();
    let ctx = RenderContext::new();

    // 1. Initial state verification
    assert_eq!(app.counter, 0);
    let initial_display = MockBackend::extract(&app.display_text, &ctx);
    assert_eq!(initial_display.content, "Count: 0");
    assert_eq!(initial_display.color, Color::BLACK);

    // 2. User hovers over increment button (interaction without click)
    app = app.update(AppMessage::IncrementButton(ButtonMessage::Interaction(
        InteractionMessage::HoverChanged(true),
    )));

    // Verify button state changed but counter didn't
    assert_eq!(app.counter, 0);
    assert!(app.increment_button.is_hovered());
    let button_extracted = MockBackend::extract(&app.increment_button.view(), &ctx);
    assert!(button_extracted.interaction_state.is_hovered());

    // 3. User clicks increment button
    app = app.update(AppMessage::IncrementButton(ButtonMessage::Clicked));

    // Verify complete state update
    assert_eq!(app.counter, 1);
    let display_extracted = MockBackend::extract(&app.display_text, &ctx);
    assert_eq!(display_extracted.content, "Count: 1");
    assert_eq!(display_extracted.color, Color::GREEN); // Positive number = green

    let status_extracted = MockBackend::extract(&app.status_message, &ctx);
    assert_eq!(status_extracted.content, "Last action: increment");

    // 4. Multiple decrements to test negative styling
    app = app.update(AppMessage::DecrementButton(ButtonMessage::Clicked));
    app = app.update(AppMessage::DecrementButton(ButtonMessage::Clicked));
    app = app.update(AppMessage::DecrementButton(ButtonMessage::Clicked));

    // Verify negative styling
    assert_eq!(app.counter, -2);
    let negative_display = MockBackend::extract(&app.display_text, &ctx);
    assert_eq!(negative_display.content, "Count: -2");
    assert_eq!(negative_display.color, Color::RED); // Negative number = red

    // 5. Reset workflow
    app = app.update(AppMessage::ResetButton(ButtonMessage::Clicked));

    // Verify reset to neutral state
    assert_eq!(app.counter, 0);
    let reset_display = MockBackend::extract(&app.display_text, &ctx);
    assert_eq!(reset_display.content, "Count: 0");
    assert_eq!(reset_display.color, Color::BLACK); // Zero = black

    // 6. Programmatic update
    app = app.update(AppMessage::UpdateCounter(42));

    // Verify programmatic updates work through the same workflow
    assert_eq!(app.counter, 42);
    let programmatic_display = MockBackend::extract(&app.display_text, &ctx);
    assert_eq!(programmatic_display.content, "Count: 42");
    assert_eq!(programmatic_display.color, Color::GREEN);
}

/// Test interaction trait integration across different component types.
///
/// This validates that interaction traits work consistently across all
/// component types and that state changes are properly reflected in extraction.
#[test]
fn interaction_trait_integration() {
    // Create components with different interaction states
    let mut enabled_button = Button::new("Enabled").enable();
    let disabled_button = Button::new("Disabled").disable();
    let focused_button = Button::new("Focused").enable().focus();
    let hovered_button = Button::new("Hovered").enable().hover();
    let pressed_button = Button::new("Pressed").enable().press();

    let ctx = RenderContext::new();

    // Test that all interaction states are preserved through extraction
    let enabled_extracted = MockBackend::extract(&enabled_button.view(), &ctx);
    assert!(enabled_extracted.interaction_state.is_enabled());

    let disabled_extracted = MockBackend::extract(&disabled_button.view(), &ctx);
    assert!(!disabled_extracted.interaction_state.is_enabled());

    let focused_extracted = MockBackend::extract(&focused_button.view(), &ctx);
    assert!(focused_extracted.interaction_state.is_focused());
    assert!(focused_extracted.interaction_state.is_enabled());

    let hovered_extracted = MockBackend::extract(&hovered_button.view(), &ctx);
    assert!(hovered_extracted.interaction_state.is_hovered());
    assert!(hovered_extracted.interaction_state.is_enabled());

    let pressed_extracted = MockBackend::extract(&pressed_button.view(), &ctx);
    assert!(pressed_extracted.interaction_state.is_pressed());
    assert!(pressed_extracted.interaction_state.is_enabled());

    // Test interaction trait method chaining
    let complex_button = Button::new("Complex")
        .enable()
        .focus()
        .hover()
        .with_pressed(false); // Explicitly not pressed

    let complex_extracted = MockBackend::extract(&complex_button.view(), &ctx);
    assert!(complex_extracted.interaction_state.is_enabled());
    assert!(complex_extracted.interaction_state.is_focused());
    assert!(complex_extracted.interaction_state.is_hovered());
    assert!(!complex_extracted.interaction_state.is_pressed());

    // Test that interaction messages update state correctly
    enabled_button = enabled_button.update(ButtonMessage::Interaction(
        InteractionMessage::FocusChanged(true),
    ));

    let updated_extracted = MockBackend::extract(&enabled_button.view(), &ctx);
    assert!(updated_extracted.interaction_state.is_focused());

    // Test conditional interaction state
    let conditionally_enabled = Button::new("Conditional")
        .with_enabled(true)
        .with_focused(false)
        .with_hovered(true)
        .with_pressed(false);

    let conditional_extracted = MockBackend::extract(&conditionally_enabled.view(), &ctx);
    assert!(conditional_extracted.interaction_state.is_enabled());
    assert!(!conditional_extracted.interaction_state.is_focused());
    assert!(conditional_extracted.interaction_state.is_hovered());
    assert!(!conditional_extracted.interaction_state.is_pressed());
}

/// Test style system integration across different component types.
///
/// This validates that styling works consistently across components and
/// that style information is properly preserved through extraction.
#[test]
fn style_system_integration() {
    let ctx = RenderContext::new();

    // Test text styling variations
    let title_text = Text::new("Title").font_size(32.0).color(Color::BLUE);

    let body_text = Text::new("Body text with default styling");

    let error_text = Text::new("Error message").font_size(12.0).color(Color::RED);

    let custom_text = Text::new("Custom styled text")
        .font_size(20.0)
        .color(Color::rgba(0.2, 0.8, 0.4, 0.9));

    // Test button styling variations
    let primary_button = Button::new("Primary")
        .background_color(Color::BLUE)
        .with_text(|text| text.color(Color::WHITE).font_size(16.0));

    let secondary_button = Button::new("Secondary")
        .background_color(Color::rgb(0.8, 0.8, 0.8))
        .with_text(|text| text.color(Color::BLACK).font_size(14.0));

    let danger_button = Button::new("Danger")
        .background_color(Color::RED)
        .with_text(|text| text.color(Color::WHITE).font_size(16.0));

    let custom_button = Button::new("Custom")
        .background_color(Color::rgba(0.3, 0.1, 0.7, 1.0))
        .with_text(|text| text.color(Color::rgba(0.9, 0.9, 0.9, 1.0)).font_size(18.0));

    // Extract and verify all text styles are preserved
    let title_extracted = MockBackend::extract(&title_text, &ctx);
    assert_eq!(title_extracted.font_size, 32.0);
    assert_eq!(title_extracted.color, Color::BLUE);

    let body_extracted = MockBackend::extract(&body_text, &ctx);
    assert_eq!(body_extracted.font_size, 16.0); // Default
    assert_eq!(body_extracted.color, Color::BLACK); // Default

    let error_extracted = MockBackend::extract(&error_text, &ctx);
    assert_eq!(error_extracted.font_size, 12.0);
    assert_eq!(error_extracted.color, Color::RED);

    let custom_text_extracted = MockBackend::extract(&custom_text, &ctx);
    assert_eq!(custom_text_extracted.font_size, 20.0);
    assert_eq!(custom_text_extracted.color, Color::rgba(0.2, 0.8, 0.4, 0.9));

    // Extract and verify all button styles are preserved
    let primary_extracted = MockBackend::extract(&primary_button.view(), &ctx);
    assert_eq!(primary_extracted.background_color, Color::BLUE);
    assert_eq!(primary_extracted.text_style.color, Color::WHITE);
    assert_eq!(primary_extracted.text_style.font_size, 16.0);

    let secondary_extracted = MockBackend::extract(&secondary_button.view(), &ctx);
    assert_eq!(
        secondary_extracted.background_color,
        Color::rgb(0.8, 0.8, 0.8)
    );
    assert_eq!(secondary_extracted.text_style.color, Color::BLACK);
    assert_eq!(secondary_extracted.text_style.font_size, 14.0);

    let danger_extracted = MockBackend::extract(&danger_button.view(), &ctx);
    assert_eq!(danger_extracted.background_color, Color::RED);
    assert_eq!(danger_extracted.text_style.color, Color::WHITE);
    assert_eq!(danger_extracted.text_style.font_size, 16.0);

    let custom_extracted = MockBackend::extract(&custom_button.view(), &ctx);
    assert_eq!(
        custom_extracted.background_color,
        Color::rgba(0.3, 0.1, 0.7, 1.0)
    );
    assert_eq!(
        custom_extracted.text_style.color,
        Color::rgba(0.9, 0.9, 0.9, 1.0)
    );
    assert_eq!(custom_extracted.text_style.font_size, 18.0);

    // Test that style changes through model updates are preserved
    #[derive(Debug, Clone)]
    struct StyledModel {
        dynamic_text: Text,
        theme: Theme,
    }

    #[derive(Debug, Clone)]
    enum StyledMessage {
        ChangeTheme(Theme),
    }

    impl Message for StyledMessage {}

    impl Model for StyledModel {
        type Message = StyledMessage;
        type View = Text;

        fn update(self, message: Self::Message) -> Self {
            match message {
                StyledMessage::ChangeTheme(theme) => {
                    let (color, size) = match theme {
                        Theme::Dark => (Color::WHITE, 18.0),
                        Theme::Light => (Color::BLACK, 16.0),
                        Theme::HighContrast => (Color::BLUE, 20.0),
                        Theme::Default => (Color::BLACK, 16.0),
                    };

                    Self {
                        dynamic_text: Text::new(format!("Theme: {}", theme.as_str()))
                            .color(color)
                            .font_size(size),
                        theme,
                    }
                }
            }
        }

        fn view(&self) -> Self::View {
            self.dynamic_text.clone()
        }
    }

    let mut styled_model = StyledModel {
        dynamic_text: Text::new("Theme: default")
            .color(Color::BLACK)
            .font_size(16.0),
        theme: Theme::Default,
    };

    // Test theme changes update styling correctly
    styled_model = styled_model.update(StyledMessage::ChangeTheme(Theme::Dark));
    let dark_extracted = MockBackend::extract(&styled_model.dynamic_text, &ctx);
    assert_eq!(dark_extracted.color, Color::WHITE);
    assert_eq!(dark_extracted.font_size, 18.0);
    assert_eq!(dark_extracted.content, "Theme: dark");

    styled_model = styled_model.update(StyledMessage::ChangeTheme(Theme::HighContrast));
    let contrast_extracted = MockBackend::extract(&styled_model.dynamic_text, &ctx);
    assert_eq!(contrast_extracted.color, Color::BLUE);
    assert_eq!(contrast_extracted.font_size, 20.0);
    assert_eq!(contrast_extracted.content, "Theme: high_contrast");

    styled_model = styled_model.update(StyledMessage::ChangeTheme(Theme::Light));
    let light_extracted = MockBackend::extract(&styled_model.dynamic_text, &ctx);
    assert_eq!(light_extracted.color, Color::BLACK);
    assert_eq!(light_extracted.font_size, 16.0);
    assert_eq!(light_extracted.content, "Theme: light");

    // Verify that the theme field is properly updated and can be accessed
    assert!(matches!(styled_model.theme, Theme::Light));

    // Test that we can query the current theme and use it for conditional logic
    let theme_dependent_text = match styled_model.theme {
        Theme::Dark => Text::new("Dark mode active").color(Color::WHITE),
        Theme::Light => Text::new("Light mode active").color(Color::BLACK),
        Theme::HighContrast => Text::new("High contrast mode active").color(Color::BLUE),
        Theme::Default => Text::new("Default theme active").color(Color::BLACK),
    };

    let theme_text_extracted = MockBackend::extract(&theme_dependent_text, &ctx);
    assert_eq!(theme_text_extracted.content, "Light mode active");
    assert_eq!(theme_text_extracted.color, Color::BLACK);
}

/// Test that complex workflows with multiple components maintain consistency.
///
/// This validates that when multiple components are updated simultaneously,
/// all state and styling changes are properly coordinated and extracted.
#[test]
fn complex_multi_component_workflow() {
    #[derive(Debug, Clone)]
    struct ComplexAppModel {
        primary_button: Button,
        secondary_button: Button,
        status_text: Text,
        counter_text: Text,
        counter: i32,
        last_action: String,
    }

    #[derive(Debug, Clone)]
    enum ComplexAppMessage {
        PrimaryAction(ButtonMessage),
        SecondaryAction(ButtonMessage),
        BulkUpdate { counter: i32, action: String },
    }

    impl Message for ComplexAppMessage {}

    impl Model for ComplexAppModel {
        type Message = ComplexAppMessage;
        type View = VStack<(Text, Text, HStack<(ButtonView, ButtonView)>)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                ComplexAppMessage::PrimaryAction(button_msg) => match button_msg {
                    ButtonMessage::Clicked => {
                        let new_counter = self.counter + 10;
                        Self {
                            primary_button: self.primary_button.update(button_msg),
                            counter: new_counter,
                            last_action: "primary".to_string(),
                            counter_text: Text::new(format!("Count: {}", new_counter))
                                .font_size(20.0)
                                .color(Color::GREEN),
                            status_text: Text::new("Primary action executed").color(Color::BLUE),
                            ..self
                        }
                    }
                    ButtonMessage::Interaction(_) => Self {
                        primary_button: self.primary_button.update(button_msg),
                        ..self
                    },
                },
                ComplexAppMessage::SecondaryAction(button_msg) => match button_msg {
                    ButtonMessage::Clicked => {
                        let new_counter = self.counter - 5;
                        Self {
                            secondary_button: self.secondary_button.update(button_msg),
                            counter: new_counter,
                            last_action: "secondary".to_string(),
                            counter_text: Text::new(format!("Count: {}", new_counter))
                                .font_size(20.0)
                                .color(if new_counter < 0 {
                                    Color::RED
                                } else {
                                    Color::BLACK
                                }),
                            status_text: Text::new("Secondary action executed")
                                .color(Color::rgb(0.8, 0.4, 0.0)),
                            ..self
                        }
                    }
                    ButtonMessage::Interaction(_) => Self {
                        secondary_button: self.secondary_button.update(button_msg),
                        ..self
                    },
                },
                ComplexAppMessage::BulkUpdate { counter, action } => Self {
                    counter,
                    last_action: action.clone(),
                    counter_text: Text::new(format!("Count: {}", counter))
                        .font_size(22.0)
                        .color(Color::rgba(0.5, 0.0, 0.8, 1.0)),
                    status_text: Text::new(format!("Bulk update: {}", action))
                        .color(Color::rgba(0.0, 0.6, 0.6, 1.0)),
                    ..self
                },
            }
        }

        fn view(&self) -> Self::View {
            VStack::new((
                self.counter_text.clone(),
                self.status_text.clone(),
                HStack::new((self.primary_button.view(), self.secondary_button.view()))
                    .spacing(12.0),
            ))
            .spacing(16.0)
        }
    }

    let mut app = ComplexAppModel {
        primary_button: Button::new("Primary (+10)")
            .background_color(Color::GREEN)
            .with_text(|text| text.color(Color::WHITE)),
        secondary_button: Button::new("Secondary (-5)")
            .background_color(Color::RED)
            .with_text(|text| text.color(Color::WHITE)),
        status_text: Text::new("Ready").color(Color::BLACK),
        counter_text: Text::new("Count: 0").font_size(20.0).color(Color::BLACK),
        counter: 0,
        last_action: "none".to_string(),
    };

    let ctx = RenderContext::new();

    // Test complex interaction sequence
    app = app.update(ComplexAppMessage::PrimaryAction(ButtonMessage::Clicked));
    app = app.update(ComplexAppMessage::SecondaryAction(ButtonMessage::Clicked));
    app = app.update(ComplexAppMessage::PrimaryAction(ButtonMessage::Clicked));

    // Verify final state: +10 -5 +10 = 15
    assert_eq!(app.counter, 15);
    assert_eq!(app.last_action, "primary");

    // Extract and verify all components reflect the final state
    let counter_extracted = MockBackend::extract(&app.counter_text, &ctx);
    assert_eq!(counter_extracted.content, "Count: 15");
    assert_eq!(counter_extracted.color, Color::GREEN); // Positive = green from primary action

    let status_extracted = MockBackend::extract(&app.status_text, &ctx);
    assert_eq!(status_extracted.content, "Primary action executed");
    assert_eq!(status_extracted.color, Color::BLUE);

    // Test bulk update affects all components
    app = app.update(ComplexAppMessage::BulkUpdate {
        counter: -100,
        action: "system reset".to_string(),
    });

    assert_eq!(app.counter, -100);
    assert_eq!(app.last_action, "system reset");

    let bulk_counter_extracted = MockBackend::extract(&app.counter_text, &ctx);
    assert_eq!(bulk_counter_extracted.content, "Count: -100");
    assert_eq!(
        bulk_counter_extracted.color,
        Color::rgba(0.5, 0.0, 0.8, 1.0)
    );
    assert_eq!(bulk_counter_extracted.font_size, 22.0);

    let bulk_status_extracted = MockBackend::extract(&app.status_text, &ctx);
    assert_eq!(bulk_status_extracted.content, "Bulk update: system reset");
    assert_eq!(bulk_status_extracted.color, Color::rgba(0.0, 0.6, 0.6, 1.0));

    // Verify buttons maintain their styling through all updates
    let primary_extracted = MockBackend::extract(&app.primary_button.view(), &ctx);
    assert_eq!(primary_extracted.background_color, Color::GREEN);
    assert_eq!(primary_extracted.text_style.color, Color::WHITE);

    let secondary_extracted = MockBackend::extract(&app.secondary_button.view(), &ctx);
    assert_eq!(secondary_extracted.background_color, Color::RED);
    assert_eq!(secondary_extracted.text_style.color, Color::WHITE);
}

// End of File
