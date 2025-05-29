// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Integration tests for component hierarchy patterns
//!
//! These tests validate that components can be composed hierarchically,
//! with parent models containing child components and messages bubbling
//! up through the hierarchy in a type-safe way.

use ironwood::{backends::mock::MockBackend, prelude::*};

/// Test that demonstrates hierarchical component composition.
///
/// This validates the pattern where parent models contain child components
/// as fields, and parent messages wrap child messages for type-safe bubbling.
#[test]
fn component_hierarchy_integration() {
    // Define a parent model that contains multiple child components
    #[derive(Debug, Clone)]
    struct FormModel {
        submit_button: Button,
        cancel_button: Button,
        status_text: Text,
        form_data: String,
    }

    // Parent messages wrap child messages for type-safe message routing
    #[derive(Debug, Clone)]
    enum FormMessage {
        SubmitButton(ButtonMessage),
        CancelButton(ButtonMessage),
        UpdateData(String),
        FormSubmitted,
        FormCancelled,
    }

    impl Message for FormMessage {}

    impl FormModel {
        fn new() -> Self {
            Self {
                submit_button: Button::new("Submit")
                    .background_color(Color::GREEN)
                    .with_text(|text| text.color(Color::WHITE)),
                cancel_button: Button::new("Cancel")
                    .background_color(Color::RED)
                    .with_text(|text| text.color(Color::WHITE)),
                status_text: Text::new("Ready to submit").color(Color::BLACK),
                form_data: String::new(),
            }
        }
    }

    impl Model for FormModel {
        type Message = FormMessage;
        type View = VStack<(Text, HStack<(ButtonView, ButtonView)>)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                FormMessage::SubmitButton(button_msg) => {
                    match button_msg {
                        ButtonMessage::Clicked => {
                            // Handle submit button click - trigger form submission
                            Self {
                                submit_button: self.submit_button.update(button_msg),
                                status_text: Text::new("Form submitted!").color(Color::GREEN),
                                ..self
                            }
                            .update(FormMessage::FormSubmitted)
                        }
                        ButtonMessage::Interaction(_) => {
                            // Handle other button interactions (hover, focus, etc.)
                            Self {
                                submit_button: self.submit_button.update(button_msg),
                                ..self
                            }
                        }
                    }
                }
                FormMessage::CancelButton(button_msg) => {
                    match button_msg {
                        ButtonMessage::Clicked => {
                            // Handle cancel button click - reset form
                            Self {
                                cancel_button: self.cancel_button.update(button_msg),
                                status_text: Text::new("Form cancelled").color(Color::RED),
                                form_data: String::new(),
                                ..self
                            }
                            .update(FormMessage::FormCancelled)
                        }
                        ButtonMessage::Interaction(_) => {
                            // Handle other button interactions
                            Self {
                                cancel_button: self.cancel_button.update(button_msg),
                                ..self
                            }
                        }
                    }
                }
                FormMessage::UpdateData(data) => Self {
                    form_data: data,
                    status_text: Text::new("Data updated").color(Color::BLUE),
                    ..self
                },
                FormMessage::FormSubmitted => {
                    // Additional logic after form submission
                    Self {
                        submit_button: self.submit_button.disable(),
                        ..self
                    }
                }
                FormMessage::FormCancelled => {
                    // Additional logic after form cancellation
                    Self {
                        status_text: Text::new("Ready to submit").color(Color::BLACK),
                        ..self
                    }
                }
            }
        }

        fn view(&self) -> Self::View {
            VStack::new((
                self.status_text.clone(),
                HStack::new((self.submit_button.view(), self.cancel_button.view())).spacing(8.0),
            ))
            .spacing(16.0)
        }
    }

    // Test the complete hierarchy workflow
    let mut form = FormModel::new();

    // Verify initial state
    assert!(form.submit_button.is_enabled());
    assert!(form.cancel_button.is_enabled());
    assert_eq!(form.form_data, "");

    // Verify initial status text
    let ctx = RenderContext::new();
    let status_extracted = MockBackend::extract(&form.status_text, &ctx).unwrap();
    assert_eq!(status_extracted.content, "Ready to submit");

    // Test data update
    form = form.update(FormMessage::UpdateData("test data".to_string()));
    assert_eq!(form.form_data, "test data");

    // Test submit button interaction (non-click)
    form = form.update(FormMessage::SubmitButton(ButtonMessage::Interaction(
        InteractionMessage::HoverChanged(true),
    )));
    assert!(form.submit_button.is_hovered());
    assert!(form.submit_button.is_enabled()); // Should still be enabled

    // Test submit button click
    form = form.update(FormMessage::SubmitButton(ButtonMessage::Clicked));
    assert!(!form.submit_button.is_enabled()); // Should be disabled after submission

    // Test cancel button click on a fresh form
    let mut fresh_form = FormModel::new();
    fresh_form = fresh_form.update(FormMessage::UpdateData("some data".to_string()));
    fresh_form = fresh_form.update(FormMessage::CancelButton(ButtonMessage::Clicked));
    assert_eq!(fresh_form.form_data, ""); // Should be reset
}

/// Test that component hierarchy preserves individual component state.
///
/// This validates that child components maintain their own state correctly
/// even when embedded in parent models.
#[test]
fn component_state_preservation() {
    #[derive(Debug, Clone)]
    struct MultiButtonModel {
        button1: Button,
        button2: Button,
        button3: Button,
    }

    #[derive(Debug, Clone)]
    enum MultiButtonMessage {
        Button1(ButtonMessage),
        Button2(ButtonMessage),
        Button3(ButtonMessage),
    }

    impl Message for MultiButtonMessage {}

    impl Model for MultiButtonModel {
        type Message = MultiButtonMessage;
        type View = HStack<(ButtonView, ButtonView, ButtonView)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                MultiButtonMessage::Button1(msg) => Self {
                    button1: self.button1.update(msg),
                    ..self
                },
                MultiButtonMessage::Button2(msg) => Self {
                    button2: self.button2.update(msg),
                    ..self
                },
                MultiButtonMessage::Button3(msg) => Self {
                    button3: self.button3.update(msg),
                    ..self
                },
            }
        }

        fn view(&self) -> Self::View {
            HStack::new((
                self.button1.view(),
                self.button2.view(),
                self.button3.view(),
            ))
            .spacing(8.0)
        }
    }

    let mut model = MultiButtonModel {
        button1: Button::new("Button 1").enable(),
        button2: Button::new("Button 2").disable(),
        button3: Button::new("Button 3").enable(),
    };

    // Verify initial states
    assert!(model.button1.is_enabled());
    assert!(!model.button2.is_enabled());
    assert!(model.button3.is_enabled());

    // Update button1 state
    model = model.update(MultiButtonMessage::Button1(ButtonMessage::Interaction(
        InteractionMessage::FocusChanged(true),
    )));

    // Verify only button1 changed
    assert!(model.button1.is_focused());
    assert!(!model.button2.is_focused());
    assert!(!model.button3.is_focused());

    // Update button2 state (even though disabled, it can still receive some interactions)
    model = model.update(MultiButtonMessage::Button2(ButtonMessage::Interaction(
        InteractionMessage::HoverChanged(true),
    )));

    // Update button3 state
    model = model.update(MultiButtonMessage::Button3(ButtonMessage::Interaction(
        InteractionMessage::PressStateChanged(true),
    )));

    // Verify states are preserved independently
    assert!(model.button1.is_focused());
    assert!(!model.button1.is_pressed());
    assert!(!model.button2.is_enabled());
    assert!(model.button2.is_hovered()); // Disabled buttons can still be hovered
    assert!(!model.button3.is_focused());
    assert!(model.button3.is_pressed());
}

/// Test that component hierarchy works with view extraction.
///
/// This validates that hierarchical components can be extracted correctly
/// by backends, preserving all component states and styling.
#[test]
fn hierarchy_view_extraction() {
    #[derive(Debug, Clone)]
    struct DashboardModel {
        title: Text,
        save_button: Button,
        load_button: Button,
        status: Text,
    }

    let dashboard = DashboardModel {
        title: Text::new("Dashboard").font_size(24.0).color(Color::BLUE),
        save_button: Button::new("Save")
            .background_color(Color::GREEN)
            .with_text(|text| text.color(Color::WHITE))
            .focus(),
        load_button: Button::new("Load")
            .background_color(Color::BLUE)
            .with_text(|text| text.color(Color::WHITE))
            .disable(),
        status: Text::new("Ready").font_size(14.0).color(Color::BLACK),
    };

    let ctx = RenderContext::new();

    // Extract each component and verify properties are preserved
    let title_extracted = MockBackend::extract(&dashboard.title, &ctx).unwrap();
    assert_eq!(title_extracted.content, "Dashboard");
    assert_eq!(title_extracted.font_size, 24.0);
    assert_eq!(title_extracted.color, Color::BLUE);

    let save_extracted = MockBackend::extract(&dashboard.save_button.view(), &ctx).unwrap();
    assert_eq!(save_extracted.text, "Save");
    assert_eq!(save_extracted.background_color, Color::GREEN);
    assert_eq!(save_extracted.text_style.color, Color::WHITE);
    assert!(save_extracted.interaction_state.is_focused());
    assert!(save_extracted.interaction_state.is_enabled());

    let load_extracted = MockBackend::extract(&dashboard.load_button.view(), &ctx).unwrap();
    assert_eq!(load_extracted.text, "Load");
    assert_eq!(load_extracted.background_color, Color::BLUE);
    assert_eq!(load_extracted.text_style.color, Color::WHITE);
    assert!(!load_extracted.interaction_state.is_focused());
    assert!(!load_extracted.interaction_state.is_enabled());

    let status_extracted = MockBackend::extract(&dashboard.status, &ctx).unwrap();
    assert_eq!(status_extracted.content, "Ready");
    assert_eq!(status_extracted.font_size, 14.0);
    assert_eq!(status_extracted.color, Color::BLACK);
}

/// Test that deeply nested component hierarchies work correctly.
///
/// This validates that components can be nested multiple levels deep
/// with proper message routing and state management.
#[test]
fn deep_component_nesting() {
    // Inner component
    #[derive(Debug, Clone)]
    struct InnerComponent {
        button: Button,
        text: Text,
    }

    #[derive(Debug, Clone)]
    enum InnerMessage {
        Button(ButtonMessage),
        UpdateText(String),
    }

    impl Message for InnerMessage {}

    impl Model for InnerComponent {
        type Message = InnerMessage;
        type View = VStack<(ButtonView, Text)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                InnerMessage::Button(msg) => Self {
                    button: self.button.update(msg),
                    ..self
                },
                InnerMessage::UpdateText(content) => Self {
                    text: Text::new(content),
                    ..self
                },
            }
        }

        fn view(&self) -> Self::View {
            VStack::new((self.button.view(), self.text.clone())).spacing(4.0)
        }
    }

    // Middle component containing inner component
    #[derive(Debug, Clone)]
    struct MiddleComponent {
        inner: InnerComponent,
        own_button: Button,
    }

    #[derive(Debug, Clone)]
    enum MiddleMessage {
        Inner(InnerMessage),
        OwnButton(ButtonMessage),
    }

    impl Message for MiddleMessage {}

    impl Model for MiddleComponent {
        type Message = MiddleMessage;
        type View = HStack<(<InnerComponent as Model>::View, ButtonView)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                MiddleMessage::Inner(msg) => Self {
                    inner: self.inner.update(msg),
                    ..self
                },
                MiddleMessage::OwnButton(msg) => Self {
                    own_button: self.own_button.update(msg),
                    ..self
                },
            }
        }

        fn view(&self) -> Self::View {
            HStack::new((self.inner.view(), self.own_button.view())).spacing(8.0)
        }
    }

    // Outer component containing middle component
    #[derive(Debug, Clone)]
    struct OuterComponent {
        middle: MiddleComponent,
        outer_text: Text,
    }

    #[derive(Debug, Clone)]
    enum OuterMessage {
        Middle(MiddleMessage),
        UpdateOuterText(String),
    }

    impl Message for OuterMessage {}

    impl Model for OuterComponent {
        type Message = OuterMessage;
        type View = VStack<(<MiddleComponent as Model>::View, Text)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                OuterMessage::Middle(msg) => Self {
                    middle: self.middle.update(msg),
                    ..self
                },
                OuterMessage::UpdateOuterText(content) => Self {
                    outer_text: Text::new(content),
                    ..self
                },
            }
        }

        fn view(&self) -> Self::View {
            VStack::new((self.middle.view(), self.outer_text.clone())).spacing(12.0)
        }
    }

    // Create nested structure
    let mut outer = OuterComponent {
        middle: MiddleComponent {
            inner: InnerComponent {
                button: Button::new("Inner Button").enable(),
                text: Text::new("Inner Text"),
            },
            own_button: Button::new("Middle Button").enable(),
        },
        outer_text: Text::new("Outer Text"),
    };

    // Test deep message routing: Outer -> Middle -> Inner -> Button
    outer = outer.update(OuterMessage::Middle(MiddleMessage::Inner(
        InnerMessage::Button(ButtonMessage::Interaction(
            InteractionMessage::HoverChanged(true),
        )),
    )));

    // Verify the message reached the deeply nested button
    assert!(outer.middle.inner.button.is_hovered());
    assert!(!outer.middle.own_button.is_hovered()); // Other buttons unaffected

    // Test middle component's own button
    outer = outer.update(OuterMessage::Middle(MiddleMessage::OwnButton(
        ButtonMessage::Interaction(InteractionMessage::FocusChanged(true)),
    )));

    // Verify middle button received the message
    assert!(outer.middle.own_button.is_focused());
    assert!(!outer.middle.inner.button.is_focused()); // Inner button unaffected

    // Test updating text at different levels
    outer = outer.update(OuterMessage::UpdateOuterText("Updated Outer".to_string()));
    outer = outer.update(OuterMessage::Middle(MiddleMessage::Inner(
        InnerMessage::UpdateText("Updated Inner".to_string()),
    )));

    // Verify updates reached correct levels
    let ctx = RenderContext::new();
    let outer_text_extracted = MockBackend::extract(&outer.outer_text, &ctx).unwrap();
    let inner_text_extracted = MockBackend::extract(&outer.middle.inner.text, &ctx).unwrap();

    assert_eq!(outer_text_extracted.content, "Updated Outer");
    assert_eq!(inner_text_extracted.content, "Updated Inner");
}

// End of File
