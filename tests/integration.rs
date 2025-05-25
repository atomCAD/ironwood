// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! Integration tests for Ironwood UI Framework
//!
//! These tests validate that the core traits (Message, Model, View) work together
//! correctly to implement the Elm Architecture pattern.

use ironwood::prelude::*;

/// Test that the core traits work together to implement the Elm Architecture.
///
/// This validates that Message, Model, and their trait bounds compose correctly
/// for the complete Model-Update cycle that forms the foundation of Ironwood applications.
#[test]
fn framework_integration() {
    // Simple model representing application state
    #[derive(Debug, Clone, PartialEq)]
    struct SimpleModel {
        value: i32,
    }

    // Messages representing all possible state changes
    #[derive(Debug, Clone)]
    enum SimpleMessage {
        Increment,
        SetValue(i32),
    }

    impl Message for SimpleMessage {}

    impl Model for SimpleModel {
        type Message = SimpleMessage;

        fn update(self, message: Self::Message) -> Self {
            match message {
                SimpleMessage::Increment => Self {
                    value: self.value + 1,
                },
                SimpleMessage::SetValue(value) => Self { value },
            }
        }
    }

    // Test complete workflow: Model → Message → Update → New Model
    let mut model = SimpleModel { value: 0 };
    assert_eq!(model.value, 0);

    model = model.update(SimpleMessage::Increment);
    assert_eq!(model.value, 1);

    model = model.update(SimpleMessage::SetValue(42));
    assert_eq!(model.value, 42);

    // Ensure trait bounds support required operations
    let _debug_str = format!("{:?}", model);
    let _cloned = model.clone();
}

// End of File
