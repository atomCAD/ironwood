// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Integration tests for threading and concurrency patterns
//!
//! These tests validate that the Ironwood framework components work correctly
//! in multi-threaded environments, including Send/Sync bounds verification,
//! message passing between threads, and concurrent model updates.

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use ironwood::{backends::mock::MockBackend, prelude::*};

#[derive(Debug, Clone, Copy)]
enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

impl Priority {
    fn as_str(self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Normal => "normal",
            Priority::High => "high",
            Priority::Critical => "critical",
        }
    }
}

/// Test that models and messages can be safely shared between threads.
///
/// This validates that the Model trait and Message implementations work
/// correctly when accessed from multiple threads simultaneously.
#[test]
fn models_and_messages_are_thread_safe() {
    #[derive(Debug, Clone)]
    struct SharedModel {
        counter: i32,
        status: Text,
        priority: Priority,
    }

    #[derive(Debug, Clone)]
    enum SharedMessage {
        Increment,
        Decrement,
        SetPriority(Priority),
        Reset,
    }

    impl Message for SharedMessage {}

    impl Model for SharedModel {
        type Message = SharedMessage;
        type View = VStack<(Text, Text)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                SharedMessage::Increment => {
                    let new_counter = self.counter.saturating_add(1);
                    Self {
                        counter: new_counter,
                        status: Text::new(format!("Count: {}", new_counter)).color(Color::GREEN),
                        ..self
                    }
                }
                SharedMessage::Decrement => {
                    let new_counter = self.counter.saturating_sub(1);
                    Self {
                        counter: new_counter,
                        status: Text::new(format!("Count: {}", new_counter)).color(Color::RED),
                        ..self
                    }
                }
                SharedMessage::SetPriority(priority) => Self {
                    priority,
                    status: Text::new(format!("Priority: {}", priority.as_str()))
                        .color(Color::BLUE),
                    ..self
                },
                SharedMessage::Reset => Self {
                    counter: 0,
                    status: Text::new("Reset").color(Color::BLACK),
                    priority: Priority::Normal,
                },
            }
        }

        fn view(&self) -> Self::View {
            VStack::new((
                Text::new(format!("Counter: {}", self.counter)),
                self.status.clone(),
            ))
            .spacing(8.0)
        }
    }

    let initial_model = SharedModel {
        counter: 0,
        status: Text::new("Initial").color(Color::BLACK),
        priority: Priority::Normal,
    };

    // Share model between threads using Arc<Mutex<>>
    let shared_model = Arc::new(Mutex::new(initial_model));
    let ctx = Arc::new(RenderContext::new());

    // Use barriers to synchronize thread execution phases
    let barrier1 = Arc::new(std::sync::Barrier::new(3)); // Wait for all threads to start
    let barrier2 = Arc::new(std::sync::Barrier::new(3)); // Wait before reset operations

    // Create multiple threads that update the model
    let mut handles = vec![];

    // Thread 1: Increment operations
    let model_clone = Arc::clone(&shared_model);
    let ctx_clone = Arc::clone(&ctx);
    let barrier1_clone = Arc::clone(&barrier1);
    let barrier2_clone = Arc::clone(&barrier2);
    let handle1 = thread::spawn(move || {
        barrier1_clone.wait(); // Wait for all threads to start

        for _ in 0..5 {
            let mut model = model_clone.lock().unwrap();
            *model = model.clone().update(SharedMessage::Increment);

            // Verify extraction works in thread
            let status_extracted = MockBackend::extract(&model.status, &ctx_clone).unwrap();
            assert!(status_extracted.content.starts_with("Count:"));

            thread::sleep(Duration::from_millis(1));
        }

        barrier2_clone.wait(); // Wait before reset phase
    });

    // Thread 2: Priority operations
    let model_clone = Arc::clone(&shared_model);
    let ctx_clone = Arc::clone(&ctx);
    let barrier1_clone = Arc::clone(&barrier1);
    let barrier2_clone = Arc::clone(&barrier2);
    let handle2 = thread::spawn(move || {
        barrier1_clone.wait(); // Wait for all threads to start

        let priorities = [
            Priority::High,
            Priority::Critical,
            Priority::Low,
            Priority::Normal,
        ];
        for priority in priorities {
            let mut model = model_clone.lock().unwrap();
            *model = model.clone().update(SharedMessage::SetPriority(priority));

            // Verify extraction works in thread
            let status_extracted = MockBackend::extract(&model.status, &ctx_clone).unwrap();
            assert!(status_extracted.content.starts_with("Priority:"));

            thread::sleep(Duration::from_millis(1));
        }

        barrier2_clone.wait(); // Wait before reset phase
    });

    // Thread 3: Mixed operations, including final reset
    let model_clone = Arc::clone(&shared_model);
    let barrier1_clone = Arc::clone(&barrier1);
    let barrier2_clone = Arc::clone(&barrier2);
    let handle3 = thread::spawn(move || {
        barrier1_clone.wait(); // Wait for all threads to start

        {
            let mut model = model_clone.lock().unwrap();
            *model = model.clone().update(SharedMessage::Decrement);
            *model = model.clone().update(SharedMessage::Increment);
        }

        barrier2_clone.wait(); // Wait for other threads to finish their operations

        // Now perform the reset operation after all other operations are complete
        {
            let mut model = model_clone.lock().unwrap();
            *model = model.clone().update(SharedMessage::Reset);

            // Verify final state
            assert_eq!(model.counter, 0);
            assert!(matches!(model.priority, Priority::Normal));
        }
    });

    handles.push(handle1);
    handles.push(handle2);
    handles.push(handle3);

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    // Verify final model state is consistent
    let final_model = shared_model.lock().unwrap();
    assert_eq!(final_model.counter, 0); // Reset was the last operation in thread 3
}

/// Test message passing between threads using channels.
///
/// This validates that messages can be sent between threads and processed
/// correctly, simulating real-world scenarios like UI event handling.
#[test]
fn message_passing_between_threads() {
    use std::sync::mpsc;

    #[derive(Debug, Clone)]
    struct EventModel {
        events: Vec<String>,
        button: Button,
        status_text: Text,
    }

    #[derive(Debug, Clone)]
    enum EventMessage {
        AddEvent(String),
        ButtonClicked,
        ClearEvents,
    }

    impl Message for EventMessage {}

    impl Model for EventModel {
        type Message = EventMessage;
        type View = VStack<(ButtonView, Text)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                EventMessage::AddEvent(event) => {
                    let mut new_events = self.events;
                    new_events.push(event.clone());
                    Self {
                        events: new_events.clone(),
                        status_text: Text::new(format!("Events: {}", new_events.len()))
                            .color(Color::BLUE),
                        ..self
                    }
                }
                EventMessage::ButtonClicked => {
                    let mut new_events = self.events;
                    new_events.push("Button clicked".to_string());
                    Self {
                        events: new_events.clone(),
                        button: self.button.update(ButtonMessage::Clicked),
                        status_text: Text::new(format!(
                            "Button clicked! Total events: {}",
                            new_events.len()
                        ))
                        .color(Color::GREEN),
                    }
                }
                EventMessage::ClearEvents => Self {
                    events: vec![],
                    status_text: Text::new("Events cleared").color(Color::RED),
                    ..self
                },
            }
        }

        fn view(&self) -> Self::View {
            VStack::new((self.button.view(), self.status_text.clone())).spacing(12.0)
        }
    }

    let initial_model = EventModel {
        events: vec![],
        button: Button::new("Click Me").background_color(Color::BLUE),
        status_text: Text::new("Ready").color(Color::BLACK),
    };

    // Create channel for message passing
    let (sender, receiver) = mpsc::channel::<EventMessage>();
    let model = Arc::new(Mutex::new(initial_model));
    let ctx = RenderContext::new();

    // Spawn producer threads that send messages
    let sender1 = sender.clone();
    let producer1 = thread::spawn(move || {
        for i in 0..3 {
            sender1
                .send(EventMessage::AddEvent(format!("Event {}", i)))
                .expect("Should send message");
            thread::sleep(Duration::from_millis(10));
        }
    });

    let sender2 = sender.clone();
    let producer2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(15));
        sender2
            .send(EventMessage::ButtonClicked)
            .expect("Should send message");
        sender2
            .send(EventMessage::AddEvent("Final event".to_string()))
            .expect("Should send message");
    });

    // Producer 3: Clear events to test the ClearEvents variant
    let sender3 = sender.clone();
    let producer3 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(25));
        sender3
            .send(EventMessage::ClearEvents)
            .expect("Should send message");
        sender3
            .send(EventMessage::AddEvent("After clear".to_string()))
            .expect("Should send message");
    });

    // Drop original sender to close channel when producers finish
    drop(sender);

    // Consumer thread that processes messages
    let model_clone = Arc::clone(&model);
    let consumer = thread::spawn(move || {
        let mut message_count = 0;
        while let Ok(message) = receiver.recv() {
            let mut current_model = model_clone.lock().unwrap();
            *current_model = current_model.clone().update(message);
            message_count += 1;
        }
        message_count
    });

    // Wait for all threads to complete
    producer1.join().expect("Producer 1 should complete");
    producer2.join().expect("Producer 2 should complete");
    producer3.join().expect("Producer 3 should complete");
    let total_messages = consumer.join().expect("Consumer should complete");

    // Verify final state
    let final_model = model.lock().unwrap();
    assert_eq!(total_messages, 7); // 3 AddEvent + 1 ButtonClicked + 1 AddEvent + 1 ClearEvents + 1 AddEvent

    // Due to thread scheduling, we can't guarantee exact message ordering.
    // What we can verify is that all messages were processed and the final state is valid.
    // The events list should contain some subset of the expected events.
    assert!(!final_model.events.is_empty()); // At least "After clear" should be present
    assert!(final_model.events.len() <= 6); // At most all events before clear + "After clear"

    // "After clear" should always be present since it's sent last by producer3
    assert!(final_model.events.contains(&"After clear".to_string()));

    // Verify extraction still works after threading
    let status_extracted = MockBackend::extract(&final_model.status_text, &ctx).unwrap();
    // The status should reflect the current number of events
    assert!(
        status_extracted
            .content
            .contains(&format!("Events: {}", final_model.events.len()))
    );
    assert_eq!(status_extracted.color, Color::BLUE);
}

/// Test concurrent view extraction from multiple threads.
///
/// This validates that view extraction is thread-safe and produces
/// consistent results when called from multiple threads simultaneously.
#[test]
fn concurrent_view_extraction() {
    // Create a complex component with multiple styled elements
    let complex_button = Button::new("Complex Button")
        .background_color(Color::rgba(0.2, 0.4, 0.8, 1.0))
        .with_text(|text| text.color(Color::WHITE).font_size(16.0))
        .enable()
        .focus()
        .hover();

    let styled_text = Text::new("Styled Text Content")
        .font_size(20.0)
        .color(Color::rgb(0.1, 0.7, 0.3));

    // Share components and context between threads
    let shared_button = Arc::new(complex_button);
    let shared_text = Arc::new(styled_text);
    let shared_ctx = Arc::new(RenderContext::new());

    // Create multiple threads that extract views concurrently
    let mut handles = vec![];

    for thread_id in 0..5 {
        let button_clone = Arc::clone(&shared_button);
        let text_clone = Arc::clone(&shared_text);
        let ctx_clone = Arc::clone(&shared_ctx);

        let handle = thread::spawn(move || {
            // Perform multiple extractions in each thread
            let mut results = vec![];

            for extraction_id in 0..10 {
                // Extract button
                let button_extracted =
                    MockBackend::extract(&button_clone.view(), &ctx_clone).unwrap();

                // Extract text
                let text_extracted = MockBackend::extract(text_clone.as_ref(), &ctx_clone).unwrap();

                // Verify consistency
                assert_eq!(button_extracted.text, "Complex Button");
                assert_eq!(
                    button_extracted.background_color,
                    Color::rgba(0.2, 0.4, 0.8, 1.0)
                );
                assert_eq!(button_extracted.text_style.color, Color::WHITE);
                assert_eq!(button_extracted.text_style.font_size, 16.0);
                assert!(button_extracted.interaction_state.is_enabled());
                assert!(button_extracted.interaction_state.is_focused());
                assert!(button_extracted.interaction_state.is_hovered());

                assert_eq!(text_extracted.content, "Styled Text Content");
                assert_eq!(text_extracted.font_size, 20.0);
                assert_eq!(text_extracted.color, Color::rgb(0.1, 0.7, 0.3));

                results.push((thread_id, extraction_id, button_extracted, text_extracted));

                // Small delay to increase chance of concurrent access
                thread::sleep(Duration::from_millis(1));
            }

            results
        });

        handles.push(handle);
    }

    // Collect all results
    let mut all_results = vec![];
    for handle in handles {
        let thread_results = handle.join().expect("Thread should complete successfully");
        all_results.extend(thread_results);
    }

    // Verify we got results from all threads and extractions
    assert_eq!(all_results.len(), 5 * 10); // 5 threads * 10 extractions each

    // Verify all extractions produced identical results
    let first_button = &all_results[0].2;
    let first_text = &all_results[0].3;

    for (thread_id, extraction_id, button_result, text_result) in &all_results {
        // All button extractions should be identical
        assert_eq!(button_result.text, first_button.text);
        assert_eq!(
            button_result.background_color,
            first_button.background_color
        );
        assert_eq!(
            button_result.text_style.color,
            first_button.text_style.color
        );
        assert_eq!(
            button_result.text_style.font_size,
            first_button.text_style.font_size
        );
        assert_eq!(
            button_result.interaction_state.is_enabled(),
            first_button.interaction_state.is_enabled()
        );
        assert_eq!(
            button_result.interaction_state.is_focused(),
            first_button.interaction_state.is_focused()
        );
        assert_eq!(
            button_result.interaction_state.is_hovered(),
            first_button.interaction_state.is_hovered()
        );

        // All text extractions should be identical
        assert_eq!(text_result.content, first_text.content);
        assert_eq!(text_result.font_size, first_text.font_size);
        assert_eq!(text_result.color, first_text.color);

        // Verify thread and extraction IDs are in expected ranges
        assert!(*thread_id < 5);
        assert!(*extraction_id < 10);
    }
}

/// Test that interaction state updates work correctly across threads.
///
/// This validates that interaction state changes are properly synchronized
/// and that the framework maintains consistency in multi-threaded scenarios.
#[test]
fn concurrent_interaction_state_updates() {
    #[derive(Debug, Clone)]
    struct InteractiveModel {
        primary_button: Button,
        secondary_button: Button,
        interaction_count: i32,
        last_interaction: String,
    }

    #[derive(Debug, Clone)]
    enum InteractionMessage {
        PrimaryHover(bool),
        PrimaryFocus(bool),
        PrimaryPress(bool),
        SecondaryHover(bool),
        SecondaryFocus(bool),
        SecondaryPress(bool),
        Click(String),
    }

    impl Message for InteractionMessage {}

    impl Model for InteractiveModel {
        type Message = InteractionMessage;
        type View = VStack<(HStack<(ButtonView, ButtonView)>, Text, Text)>;

        fn update(self, message: Self::Message) -> Self {
            match message {
                InteractionMessage::PrimaryHover(hovered) => Self {
                    primary_button: if hovered {
                        self.primary_button.hover()
                    } else {
                        self.primary_button.unhover()
                    },
                    interaction_count: self.interaction_count + 1,
                    last_interaction: format!("Primary hover: {}", hovered),
                    ..self
                },
                InteractionMessage::PrimaryFocus(focused) => Self {
                    primary_button: if focused {
                        self.primary_button.focus()
                    } else {
                        self.primary_button.unfocus()
                    },
                    interaction_count: self.interaction_count + 1,
                    last_interaction: format!("Primary focus: {}", focused),
                    ..self
                },
                InteractionMessage::PrimaryPress(pressed) => Self {
                    primary_button: if pressed {
                        self.primary_button.press()
                    } else {
                        self.primary_button.release()
                    },
                    interaction_count: self.interaction_count + 1,
                    last_interaction: format!("Primary press: {}", pressed),
                    ..self
                },
                InteractionMessage::SecondaryHover(hovered) => Self {
                    secondary_button: if hovered {
                        self.secondary_button.hover()
                    } else {
                        self.secondary_button.unhover()
                    },
                    interaction_count: self.interaction_count + 1,
                    last_interaction: format!("Secondary hover: {}", hovered),
                    ..self
                },
                InteractionMessage::SecondaryFocus(focused) => Self {
                    secondary_button: if focused {
                        self.secondary_button.focus()
                    } else {
                        self.secondary_button.unfocus()
                    },
                    interaction_count: self.interaction_count + 1,
                    last_interaction: format!("Secondary focus: {}", focused),
                    ..self
                },
                InteractionMessage::SecondaryPress(pressed) => Self {
                    secondary_button: if pressed {
                        self.secondary_button.press()
                    } else {
                        self.secondary_button.release()
                    },
                    interaction_count: self.interaction_count + 1,
                    last_interaction: format!("Secondary press: {}", pressed),
                    ..self
                },
                InteractionMessage::Click(button_name) => Self {
                    interaction_count: self.interaction_count + 1,
                    last_interaction: format!("Clicked: {}", button_name),
                    ..self
                },
            }
        }

        fn view(&self) -> Self::View {
            VStack::new((
                HStack::new((self.primary_button.view(), self.secondary_button.view()))
                    .spacing(8.0),
                Text::new(format!("Interactions: {}", self.interaction_count)),
                Text::new(format!("Last: {}", self.last_interaction)),
            ))
            .spacing(12.0)
        }
    }

    let initial_model = InteractiveModel {
        primary_button: Button::new("Primary")
            .background_color(Color::BLUE)
            .enable(),
        secondary_button: Button::new("Secondary")
            .background_color(Color::GREEN)
            .enable(),
        interaction_count: 0,
        last_interaction: "None".to_string(),
    };

    let shared_model = Arc::new(Mutex::new(initial_model));
    let ctx = RenderContext::new();

    // Create threads that simulate different types of interactions
    let mut handles = vec![];

    // Thread 1: Primary button interactions
    let model_clone = Arc::clone(&shared_model);
    let handle1 = thread::spawn(move || {
        let interactions = [
            InteractionMessage::PrimaryHover(true),
            InteractionMessage::PrimaryFocus(true),
            InteractionMessage::PrimaryPress(true),
            InteractionMessage::Click("Primary".to_string()),
            InteractionMessage::PrimaryPress(false),
            InteractionMessage::PrimaryFocus(false),
            InteractionMessage::PrimaryHover(false),
        ];

        for interaction in interactions {
            let mut model = model_clone.lock().unwrap();
            *model = model.clone().update(interaction);
            thread::sleep(Duration::from_millis(2));
        }
    });

    // Thread 2: Secondary button interactions
    let model_clone = Arc::clone(&shared_model);
    let handle2 = thread::spawn(move || {
        let interactions = [
            InteractionMessage::SecondaryHover(true),
            InteractionMessage::SecondaryFocus(true),
            InteractionMessage::SecondaryPress(true),
            InteractionMessage::Click("Secondary".to_string()),
            InteractionMessage::SecondaryPress(false),
            InteractionMessage::SecondaryFocus(false),
            InteractionMessage::SecondaryHover(false),
        ];

        for interaction in interactions {
            let mut model = model_clone.lock().unwrap();
            *model = model.clone().update(interaction);
            thread::sleep(Duration::from_millis(2));
        }
    });

    handles.push(handle1);
    handles.push(handle2);

    // Wait for all interactions to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    // Verify final state
    let final_model = shared_model.lock().unwrap();
    assert_eq!(final_model.interaction_count, 14); // 7 interactions per thread

    // Verify that both buttons are in their final states (not hovered, focused, or pressed)
    let primary_extracted = MockBackend::extract(&final_model.primary_button.view(), &ctx).unwrap();
    assert!(primary_extracted.interaction_state.is_enabled());
    assert!(!primary_extracted.interaction_state.is_hovered());
    assert!(!primary_extracted.interaction_state.is_focused());
    assert!(!primary_extracted.interaction_state.is_pressed());

    let secondary_extracted =
        MockBackend::extract(&final_model.secondary_button.view(), &ctx).unwrap();
    assert!(secondary_extracted.interaction_state.is_enabled());
    assert!(!secondary_extracted.interaction_state.is_hovered());
    assert!(!secondary_extracted.interaction_state.is_focused());
    assert!(!secondary_extracted.interaction_state.is_pressed());

    // Verify that the last interaction was recorded
    assert!(
        final_model.last_interaction.contains("Secondary")
            || final_model.last_interaction.contains("Primary")
    );
}

// End of File
