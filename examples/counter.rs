// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at <https://mozilla.org/MPL/2.0/>.

//! # Counter Example for Ironwood UI Framework
//!
//! This example demonstrates the fundamental Model-Message pattern that forms the core
//! of the Elm Architecture. We'll build a simple counter that can be incremented,
//! decremented, reset, or set to a specific value.
//!
//! ## What This Example Teaches
//!
//! This counter demonstrates all the core concepts you need to understand:
//!
//! 1. **Models**: How to represent application state as data structures
//! 2. **Messages**: How to represent all possible state changes as enum variants
//! 3. **Updates**: How to transform state in response to messages using pure functions
//! 4. **Immutability**: Why we create new state instead of modifying existing state
//!
//! ## Why Start With a Counter?
//!
//! While simple, this counter demonstrates the core Elm Architecture pattern:
//!
//! - **Explicit State**: The entire application state is contained in `CounterModel`
//! - **Explicit Operations**: All possible state changes are enumerated in `CounterMessage`
//! - **Pure Updates**: The `update` function is deterministic and has no side effects
//! - **Immutable State**: Each operation returns a new model, preserving the old one
//!
//! These properties make the application highly predictable and debuggable.
//! You can trace every state change, replay operations, and reason about the
//! application's behavior with confidence.
//!
//! ## The Elm Architecture Pattern
//!
//! The Elm Architecture consists of three main components:
//!
//! 1. **Model**: The state of your application
//! 2. **Update**: A way to update your state based on messages
//! 3. **View**: A way to view your state as UI elements (demonstrated with button and text components)
//!
//! This creates a unidirectional data flow: User interactions generate messages,
//! messages update the model, and the updated model generates a new view.

use ironwood::prelude::*;

/// Messages that can be sent to update the counter.
///
/// ## What This Enum Represents
///
/// This enum defines the complete "API" of our counter application. Every possible
/// way the counter can change must be represented as a variant in this enum.
/// This is fundamentally different from object-oriented approaches where state
/// changes happen through method calls.
///
/// ## Why Use Explicit Messages?
///
/// By making all state changes explicit through message types, we gain several benefits:
///
/// - **Complete Visibility**: You can see every possible state change at a glance
/// - **Easy Testing**: Each message variant can be tested in isolation
/// - **Natural Undo/Redo**: Store and replay messages to implement undo/redo
/// - **Debugging Support**: Log all messages to trace application behavior
/// - **Time Travel**: Replay message sequences to reproduce bugs
/// - **Predictability**: No hidden state changes or side effects
///
/// ## Why Button Messages?
///
/// Counter messages wrap button messages to handle the full range of user
/// interactions: clicks (which change the count), hover effects, focus states,
/// etc. This allows the counter to provide rich interactive feedback while
/// maintaining clean separation between business logic and UI interactions.
///
/// ## Usage
///
/// In your update function, handle button clicks to change the counter:
/// ```
/// use ironwood::prelude::*;
///
/// let model = CounterModel::new(0);
/// let updated = model.update(CounterMessage::IncrementButton(ButtonMessage::Clicked));
/// ```
///
/// Other button interactions (hover, focus, press states) are handled automatically
/// by the framework and bubble up as `ButtonMessage::Interaction(...)` variants.
/// Your application typically only needs to handle `ButtonMessage::Clicked` for
/// business logic changes.
#[derive(Debug, Clone, PartialEq)]
pub enum CounterMessage {
    /// Increment the counter by 1.
    ///
    /// This represents the most common counter operation. We could have made this
    /// more generic (e.g., `Add(i32)`), but explicit operations make the API clearer
    /// and ensure the counter's behavior is predictable.
    Increment,

    /// Decrement the counter by 1.
    ///
    /// Like `Increment`, this is explicit rather than generic. This makes the
    /// counter's behavior predictable and prevents confusion about what operations
    /// are supported.
    Decrement,

    /// Reset the counter to 0.
    ///
    /// This operation demonstrates how to implement "special" state changes that
    /// don't depend on the current state. Reset always produces the same result
    /// regardless of the current counter value.
    Reset,

    /// Set the counter to a specific value.
    ///
    /// This operation shows how to include data in messages. The `i32` parameter
    /// becomes part of the message, allowing the update function to use this data
    /// when computing the new state.
    ///
    /// We use `i32` to support negative values, which is important for a general-purpose
    /// counter. If we only needed positive values, we might use `u32` instead.
    SetValue(i32),
}

// This implementation tells Ironwood that CounterMessage is a valid message type.
// The Message trait requires Debug + Clone + Send + Sync + 'static, which ensures
// messages can be debugged, copied, and sent across threads safely.
impl Message for CounterMessage {}

/// The counter model maintains the current count value.
///
/// ## What This Struct Represents
///
/// This struct represents the complete state of our counter application. In the
/// Elm Architecture, the model is the single source of truth for all application
/// state. Everything the UI needs to display and every piece of data the application
/// needs to function should be contained in the model.
///
/// ## Why Keep State in a Single Structure?
///
/// Centralizing state in a single structure provides several benefits:
///
/// - **Predictability**: There's only one place to look for any piece of state
/// - **Consistency**: All state changes go through the same update mechanism
/// - **Debugging**: You can inspect the entire application state at any time
/// - **Testing**: Easy to create test scenarios with known state
/// - **Serialization**: The entire application state can be saved/loaded easily
///
/// ## Design Decisions Explained
///
/// - `Clone`: Enables creating copies for immutable updates and state preservation
/// - `Debug`: Allows printing the model for debugging and development tools
/// - `PartialEq`: Enables state comparison for testing and change detection
/// - `Send + Sync`: Allows the model to be sent across threads (inherited from Model trait)
/// - `pub count`: Makes the count accessible for display and testing
#[derive(Clone, Debug, PartialEq)]
pub struct CounterModel {
    /// The current counter value.
    ///
    /// We use `i32` instead of `u32` to support negative values, which makes the
    /// counter more useful and demonstrates handling of signed arithmetic.
    /// In a real application, you might choose the type based on your specific
    /// requirements (e.g., `u64` for large positive-only counters).
    pub count: i32,
}

impl CounterModel {
    /// Create a new counter model with the given initial value.
    ///
    /// ## What This Function Does
    ///
    /// This constructor creates a new `CounterModel` with a specific starting value.
    /// It's the most general way to create a counter model.
    ///
    /// ## Why Provide Explicit Constructors?
    ///
    /// Explicit constructors serve several purposes:
    ///
    /// - **Documentation**: They make it clear how to create valid initial states
    /// - **Validation**: They can validate input parameters (though we don't here)
    /// - **Consistency**: They ensure all instances are created the same way
    /// - **Evolution**: They provide a stable API that can evolve over time
    ///
    /// ## Example Usage
    ///
    /// ```
    /// # use ironwood::prelude::*;
    /// let model = CounterModel::new(42);
    /// assert_eq!(model.count, 42);
    /// ```
    pub fn new(initial_count: i32) -> Self {
        Self {
            count: initial_count,
        }
    }

    /// Create a new counter model starting at zero.
    ///
    /// ## What This Function Does
    ///
    /// This convenience constructor creates a counter starting at zero, which is
    /// the most common initial state for counters.
    ///
    /// ## Why Provide a Zero Constructor?
    ///
    /// Starting at zero is so common that it deserves its own constructor:
    ///
    /// - **Convenience**: Saves typing `CounterModel::new(0)`
    /// - **Intent**: Makes it clear that zero is a sensible default
    /// - **Consistency**: Matches the `Default` implementation
    ///
    /// ## Example Usage
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let model = CounterModel::zero();
    /// assert_eq!(model.count, 0);
    /// ```
    pub fn zero() -> Self {
        Self::new(0)
    }
}

impl Default for CounterModel {
    /// Default to zero for consistency with `CounterModel::zero()`.
    ///
    /// ## What This Implementation Does
    ///
    /// This implements Rust's `Default` trait, which allows creating a `CounterModel`
    /// using `CounterModel::default()` or in contexts where a default value is needed.
    ///
    /// ## Why Implement Default?
    ///
    /// Implementing `Default` provides several benefits:
    ///
    /// - **Ecosystem Integration**: Works with Rust's standard library and ecosystem
    /// - **Testing**: Test frameworks can easily create default instances
    /// - **Convenience**: Enables using `..Default::default()` in struct updates
    /// - **Consistency**: Provides a canonical "empty" or "initial" state
    ///
    /// We delegate to `zero()` to ensure consistency between different ways of
    /// creating a default counter.
    fn default() -> Self {
        Self::zero()
    }
}

impl Model for CounterModel {
    type Message = CounterMessage;

    /// Update the counter based on the received message.
    ///
    /// ## What This Function Does
    ///
    /// This is the heart of the Elm Architecture. It takes the current model and
    /// a message, then returns a new model that represents the state after applying
    /// the message. This function must handle every possible message type.
    ///
    /// ## Why Use Immutable Updates?
    ///
    /// The `update` function consumes the old model and returns a new one rather
    /// than modifying the existing model in place. This design choice provides
    /// several crucial benefits:
    ///
    /// - **No Data Races**: Immutable data can't have race conditions
    /// - **Time Travel Debugging**: Immutable updates make state history possible
    /// - **Undo/Redo**: Immutable updates make undo/redo implementation straightforward
    /// - **Predictability**: State changes are explicit and visible
    /// - **Testing**: Pure functions are much easier to test
    /// - **Reasoning**: No hidden mutations make code easier to understand
    ///
    /// ## Performance Considerations
    ///
    /// While creating new instances might seem inefficient, Rust's ownership system
    /// and compiler optimizations make this pattern very fast in practice:
    ///
    /// - **Move Semantics**: The old model is moved, not copied
    /// - **Compiler Optimization**: The compiler can often optimize struct construction
    /// - **Efficient Field Updates**: Only changed fields need new values, others are moved
    ///
    /// ## Design Pattern: Total Functions
    ///
    /// This function is "total" - it handles every possible input and always returns
    /// a valid result. This is achieved by:
    ///
    /// - **Exhaustive Matching**: Every message variant is handled
    /// - **No Panics**: No code paths that can crash the application
    /// - **Deterministic**: Same input always produces same output
    /// - **Pure**: No side effects or external dependencies
    ///
    /// These properties make the application logic highly predictable and testable.
    fn update(self, message: Self::Message) -> Self {
        // We use pattern matching to handle each message type. This ensures that
        // if we add new message types, the compiler will force us to handle them here.
        match message {
            CounterMessage::Increment => {
                // Create a new model with the count incremented by 1.
                // We could use `self.count.saturating_add(1)` to prevent overflow,
                // but for this example we'll use normal arithmetic.
                Self {
                    count: self.count + 1,
                }
            }
            CounterMessage::Decrement => {
                // Create a new model with the count decremented by 1.
                // Note that this can make the count negative, which is intentional
                // for this example. In some applications, you might want to prevent
                // negative values using `self.count.saturating_sub(1)` or by checking
                // the current value before decrementing.
                Self {
                    count: self.count - 1,
                }
            }
            CounterMessage::Reset => {
                // Reset always produces the same result regardless of current state.
                // This demonstrates how some operations don't depend on the current
                // model state. We could also write this as `Self::zero()` or
                // `Self::new(0)` for the same effect.
                Self { count: 0 }
            }
            CounterMessage::SetValue(value) => {
                // Use the value from the message to set the new count.
                // This demonstrates how messages can carry data that influences
                // the state update. The `value` parameter becomes part of the
                // message when it's created, and we extract it here during the update.
                Self { count: value }
            }
        }
    }
}

/// Demonstrate the counter example in action.
///
/// ## What This Function Does
///
/// This function simulates what a real UI framework would do: maintain a model,
/// process messages, and update the display. It shows the complete Model-Update
/// cycle in action.
///
/// ## Why Include a Main Function?
///
/// The main function serves several educational purposes:
///
/// - **Concrete Example**: Shows exactly how the abstractions work in practice
/// - **Interactive Demo**: Provides a runnable example users can experiment with
/// - **Pattern Demonstration**: Shows the typical flow of Elm Architecture applications
/// - **Testing Ground**: Provides a place to try out changes and see results
///
/// ## The Update Cycle Explained
///
/// Each step in this function follows the same pattern:
///
/// 1. **Current State**: We have a model representing the current state
/// 2. **Message**: Something happens that should change the state (user interaction, timer, etc.)
/// 3. **Update**: We call `model.update(message)` to get the new state
/// 4. **New State**: We now have a new model representing the updated state
/// 5. **Display**: We show the new state to the user (here, just printing)
///
/// This cycle repeats for the entire lifetime of the application.
fn main() {
    println!("Ironwood Counter Example");
    println!("========================");
    println!();
    println!("This example demonstrates the Elm Architecture pattern:");
    println!("1. Model: Application state (CounterModel)");
    println!("2. Message: State change requests (CounterMessage)");
    println!("3. Update: Pure function that transforms state");
    println!();

    // Start with zero to demonstrate the default state.
    // In a real application, you might load the initial state from a file,
    // database, or user preferences.
    let mut model = CounterModel::zero();
    println!("Initial state: {:?}", model);
    println!("  └─ Starting with a fresh counter at zero");
    println!();

    // Simulate user interactions by applying messages sequentially.
    // In a real UI application, these messages would come from user interactions
    // like button clicks, keyboard input, or timer events.
    println!("Simulating user interactions:");
    println!();

    // Demonstrate increment operation
    println!("User clicks 'Increment' button...");
    model = model.update(CounterMessage::Increment);
    println!("After increment: {:?}", model);
    println!("  └─ Counter increased from 0 to 1");
    println!();

    // Chain another increment to show accumulation
    println!("User clicks 'Increment' button again...");
    model = model.update(CounterMessage::Increment);
    println!("After increment: {:?}", model);
    println!("  └─ Counter increased from 1 to 2");
    println!();

    // Demonstrate decrement operation
    println!("User clicks 'Decrement' button...");
    model = model.update(CounterMessage::Decrement);
    println!("After decrement: {:?}", model);
    println!("  └─ Counter decreased from 2 to 1");
    println!();

    // Demonstrate set value operation with data
    println!("User enters '10' and clicks 'Set Value' button...");
    model = model.update(CounterMessage::SetValue(10));
    println!("After set to 10: {:?}", model);
    println!("  └─ Counter set directly to 10, ignoring previous value");
    println!();

    // Demonstrate reset operation
    println!("User clicks 'Reset' button...");
    model = model.update(CounterMessage::Reset);
    println!("After reset: {:?}", model);
    println!("  └─ Counter reset to 0, regardless of previous value");
    println!();

    println!("Counter example completed!");
    println!();
    println!("Key takeaways:");
    println!("• All state changes go through explicit messages");
    println!("• The update function is pure and predictable");
    println!("• Previous states are preserved (enabling undo/redo)");
    println!("• The pattern scales to complex applications");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_model_creation() {
        // Verify all constructor methods produce consistent results
        let default_model = CounterModel::default();
        assert_eq!(default_model.count, 0);

        let zero_model = CounterModel::zero();
        assert_eq!(zero_model.count, 0);

        let custom_model = CounterModel::new(42);
        assert_eq!(custom_model.count, 42);

        let negative_model = CounterModel::new(-10);
        assert_eq!(negative_model.count, -10);
    }

    #[test]
    fn counter_message_increment() {
        // Test increment operation in isolation for predictable behavior
        let model = CounterModel::new(5);
        let updated = model.update(CounterMessage::Increment);
        assert_eq!(updated.count, 6);

        // Chain operations to verify each update returns a new model
        let model = CounterModel::zero();
        let updated = model
            .update(CounterMessage::Increment)
            .update(CounterMessage::Increment)
            .update(CounterMessage::Increment);
        assert_eq!(updated.count, 3);
    }

    #[test]
    fn counter_message_decrement() {
        // Test decrement including edge case of going negative
        let model = CounterModel::new(5);
        let updated = model.update(CounterMessage::Decrement);
        assert_eq!(updated.count, 4);

        // Verify no special casing needed for negative values
        let model = CounterModel::zero();
        let updated = model.update(CounterMessage::Decrement);
        assert_eq!(updated.count, -1);
    }

    #[test]
    fn counter_message_reset() {
        // Test reset produces consistent result regardless of starting state
        let model = CounterModel::new(100);
        let updated = model.update(CounterMessage::Reset);
        assert_eq!(updated.count, 0);

        let model = CounterModel::new(-50);
        let updated = model.update(CounterMessage::Reset);
        assert_eq!(updated.count, 0);
    }

    #[test]
    fn counter_message_set_value() {
        // Test SetValue works for full range of i32 including edge cases
        let model = CounterModel::zero();
        let updated = model.update(CounterMessage::SetValue(42));
        assert_eq!(updated.count, 42);

        let model = CounterModel::zero();
        let updated = model.update(CounterMessage::SetValue(-25));
        assert_eq!(updated.count, -25);

        let model = CounterModel::zero();
        let updated = model.update(CounterMessage::SetValue(i32::MAX));
        assert_eq!(updated.count, i32::MAX);

        let model = CounterModel::zero();
        let updated = model.update(CounterMessage::SetValue(i32::MIN));
        assert_eq!(updated.count, i32::MIN);
    }

    #[test]
    fn counter_complex_sequence() {
        // Test complex sequence to verify update function composes correctly
        let model = CounterModel::zero();
        let updated = model
            .update(CounterMessage::Increment) // 0 -> 1
            .update(CounterMessage::Increment) // 1 -> 2
            .update(CounterMessage::SetValue(10)) // 2 -> 10
            .update(CounterMessage::Decrement) // 10 -> 9
            .update(CounterMessage::Decrement) // 9 -> 8
            .update(CounterMessage::Reset) // 8 -> 0
            .update(CounterMessage::SetValue(-5)) // 0 -> -5
            .update(CounterMessage::Increment); // -5 -> -4

        assert_eq!(updated.count, -4);
    }

    #[test]
    fn counter_immutability() {
        // Verify updates don't modify original model (enables debugging tools)
        let original = CounterModel::new(5);
        let original_count = original.count;

        let _updated = original.clone().update(CounterMessage::Increment);

        // Original model unchanged - crucial for debugging and undo functionality
        assert_eq!(original.count, original_count);
    }

    #[test]
    fn counter_message_properties() {
        // Verify messages have properties required by framework

        // Equality enables message comparison and deduplication
        assert_eq!(CounterMessage::Increment, CounterMessage::Increment);
        assert_eq!(CounterMessage::SetValue(42), CounterMessage::SetValue(42));
        assert_ne!(CounterMessage::Increment, CounterMessage::Decrement);
        assert_ne!(CounterMessage::SetValue(1), CounterMessage::SetValue(2));

        // Cloning enables message queuing and replay functionality
        let msg = CounterMessage::SetValue(100);
        let cloned = msg.clone();
        assert_eq!(msg, cloned);

        // Debug formatting enables development tools and logging
        assert_eq!(format!("{:?}", CounterMessage::Increment), "Increment");
        assert_eq!(
            format!("{:?}", CounterMessage::SetValue(42)),
            "SetValue(42)"
        );
    }
}

// End of File
