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
//!
//! ## Component Architecture
//!
//! This counter demonstrates component composition in Ironwood:
//!
//! - `CounterModel` contains button components as fields
//! - Button interactions bubble up as `CounterMessage` variants
//! - The `update` method routes messages to appropriate components
//! - UI state is derived from model state, not stored separately
//!
//! This pattern allows you to build complex UIs from simple, reusable components
//! while maintaining the predictability of the Elm Architecture.

use ironwood::{backends::mock::MockBackend, prelude::*};

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
    IncrementButton(ButtonMessage),

    /// Decrement the counter by 1.
    ///
    /// Like `Increment`, this is explicit rather than generic. This makes the
    /// counter's behavior predictable and prevents confusion about what operations
    /// are supported.
    DecrementButton(ButtonMessage),

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

    /// The increment button component
    pub increment_button: Button,

    /// The decrement button component
    pub decrement_button: Button,
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
            increment_button: Button::new("+")
                .background_color(Color::GREEN)
                .with_text(|text| text.color(Color::WHITE).font_size(20.0)),
            decrement_button: Button::new("-")
                .background_color(Color::RED)
                .with_text(|text| text.color(Color::WHITE).font_size(20.0)),
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

    /// Get the text display for the current count value.
    ///
    /// ## What This Function Does
    ///
    /// This method creates a Text view that displays the current count value.
    /// Rather than storing the display text as state, we derive it from the
    /// count value whenever it's needed. This follows the Elm Architecture
    /// principle that views should be derived from state, not stored as state.
    ///
    /// ## Why Derive Views Instead of Storing Them?
    ///
    /// Deriving views from state provides several benefits:
    ///
    /// - **Single Source of Truth**: Only the count value needs to be maintained
    /// - **Consistency**: The display is always in sync with the actual count
    /// - **Simplicity**: No need to remember to update display text separately
    /// - **Immutability**: No risk of inconsistent state between count and display
    ///
    /// ## Example Usage
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let model = CounterModel::new(42);
    /// let display = model.count_display();
    /// // display.content will be "42"
    /// ```
    pub fn count_display(&self) -> Text {
        Text::new(format!("{}", self.count))
            .font_size(24.0)
            .color(Color::BLACK)
    }

    /// Create a complete UI layout for the counter.
    ///
    /// ## What This Method Demonstrates
    ///
    /// This method shows how to compose a complete user interface using layout containers.
    /// It demonstrates several important UI patterns:
    ///
    /// - **Vertical Layout**: Using `VStack` to arrange elements top-to-bottom
    /// - **Horizontal Layout**: Using `HStack` to arrange buttons side-by-side
    /// - **Spacing**: Adding visual breathing room between elements
    /// - **Alignment**: Centering content for visual balance
    /// - **Component Composition**: Combining text and buttons into a cohesive interface
    ///
    /// ## Layout Structure
    ///
    /// ```text
    /// VStack (centered, 16px spacing)
    /// ├── Text: "Counter Example"     (title)
    /// ├── Text: "42"                  (current count, large)
    /// └── HStack (centered, 12px spacing)
    ///     ├── Button: "−"            (decrement)
    ///     └── Button: "+"            (increment)
    /// ```
    ///
    /// This creates a classic counter interface that's both functional and visually appealing.
    ///
    /// ## Why Use Layout Containers?
    ///
    /// Layout containers provide several benefits over manual positioning:
    ///
    /// - **Responsive**: Automatically adapts to different screen sizes
    /// - **Consistent**: Uniform spacing and alignment across the interface
    /// - **Maintainable**: Easy to modify layout without affecting individual components
    /// - **Accessible**: Proper semantic structure for screen readers
    /// - **Cross-platform**: Same layout works across different backends
    ///
    /// ## Example Usage
    ///
    /// ```
    /// use ironwood::prelude::*;
    ///
    /// let model = CounterModel::new(42);
    /// let ui = model.view();
    /// // The ui can now be rendered by any backend
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn view(&self) -> VStack<(Text, Text, HStack<(Button, Button)>)> {
        VStack::new((
            // Title text
            Text::new("Counter Example")
                .font_size(20.0)
                .color(Color::rgb(0.3, 0.3, 0.3)),
            // Current count display (large and prominent)
            self.count_display(),
            // Button row with decrement and increment
            HStack::new((self.decrement_button.clone(), self.increment_button.clone()))
                .spacing(12.0)
                .alignment(Alignment::Center),
        ))
        .spacing(16.0)
        .alignment(Alignment::Center)
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
            CounterMessage::IncrementButton(button_msg) => {
                // Handle button interaction messages
                match button_msg {
                    ButtonMessage::Clicked => {
                        // Create a new model with the count incremented by 1.
                        // Using saturating arithmetic to handle overflow gracefully.
                        Self {
                            count: self.count.saturating_add(1),
                            increment_button: self.increment_button.update(button_msg),
                            ..self
                        }
                    }
                    ButtonMessage::Interaction(_) => {
                        // Handle other button interactions (hover, focus, etc.)
                        Self {
                            increment_button: self.increment_button.update(button_msg),
                            ..self
                        }
                    }
                }
            }
            CounterMessage::DecrementButton(button_msg) => {
                // Handle button interaction messages
                match button_msg {
                    ButtonMessage::Clicked => {
                        // Create a new model with the count decremented by 1.
                        // Using saturating arithmetic to handle underflow gracefully.
                        // This allows negative values, which is intentional for this example.
                        Self {
                            count: self.count.saturating_sub(1),
                            decrement_button: self.decrement_button.update(button_msg),
                            ..self
                        }
                    }
                    ButtonMessage::Interaction(_) => {
                        // Handle other button interactions (hover, focus, etc.)
                        Self {
                            decrement_button: self.decrement_button.update(button_msg),
                            ..self
                        }
                    }
                }
            }
            CounterMessage::Reset => {
                // Reset the count to 0 while preserving button interaction states.
                // This demonstrates selective state updates - only the count changes,
                // button states (hover, focus, etc.) are preserved.
                Self { count: 0, ..self }
            }
            CounterMessage::SetValue(value) => {
                // Use the value from the message to set the new count.
                // This demonstrates how messages can carry data that influences
                // the state update. The `value` parameter becomes part of the
                // message when it's created, and we extract it here during the update.
                Self {
                    count: value,
                    ..self
                }
            }
        }
    }
}

impl View for CounterModel {}

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
    println!("Initial state: count = {}", model.count);
    println!("  └─ Starting with a fresh counter at zero");
    println!();

    // Extract the complete UI layout to show how containers work
    let ctx = RenderContext::new();
    let ui_layout = model.view();
    let layout_extracted = MockBackend::extract(&ui_layout, &ctx);

    println!("Complete UI Layout:");
    println!(
        "  Layout type: VStack with {} spacing, {:?} alignment",
        layout_extracted.spacing, layout_extracted.alignment
    );
    println!("  Title: '{}'", layout_extracted.content.0.content);
    println!("  Count display: '{}'", layout_extracted.content.1.content);
    println!(
        "  Button row: HStack with {} spacing, {:?} alignment",
        layout_extracted.content.2.spacing, layout_extracted.content.2.alignment
    );
    println!(
        "    Decrement: '{}'",
        layout_extracted.content.2.content.0.text
    );
    println!(
        "    Increment: '{}'",
        layout_extracted.content.2.content.1.text
    );
    println!();

    // Also extract individual components to show their state
    let increment_extracted = MockBackend::extract(&model.increment_button, &ctx);
    let decrement_extracted = MockBackend::extract(&model.decrement_button, &ctx);
    let display_extracted = MockBackend::extract(&model.count_display(), &ctx);

    println!("Individual Component States:");
    println!(
        "  Increment button: '{}' (enabled: {})",
        increment_extracted.text,
        increment_extracted.interaction_state.is_enabled()
    );
    println!(
        "  Decrement button: '{}' (enabled: {})",
        decrement_extracted.text,
        decrement_extracted.interaction_state.is_enabled()
    );
    println!("  Count display: '{}'", display_extracted.content);
    println!();

    // Simulate user interactions by applying messages sequentially.
    // In a real UI application, these messages would come from user interactions
    // like button clicks, keyboard input, or timer events.
    println!("Simulating user interactions:");
    println!();

    // Demonstrate increment operation
    println!("User clicks '+' button...");
    model = model.update(CounterMessage::IncrementButton(ButtonMessage::Clicked));
    println!("After increment: count = {}", model.count);
    println!("  └─ Counter increased from 0 to 1");

    // Show how the UI layout reflects the new state
    let updated_layout = model.view();
    let updated_extracted = MockBackend::extract(&updated_layout, &ctx);
    println!(
        "  └─ Updated count display: '{}'",
        updated_extracted.content.1.content
    );
    println!();

    // Chain another increment to show accumulation
    println!("User clicks '+' button again...");
    model = model.update(CounterMessage::IncrementButton(ButtonMessage::Clicked));
    println!("After increment: count = {}", model.count);
    println!("  └─ Counter increased from 1 to 2");

    // Show layout update again
    let updated_layout = model.view();
    let updated_extracted = MockBackend::extract(&updated_layout, &ctx);
    println!(
        "  └─ Updated count display: '{}'",
        updated_extracted.content.1.content
    );
    println!();

    // Demonstrate decrement operation
    println!("User clicks '-' button...");
    model = model.update(CounterMessage::DecrementButton(ButtonMessage::Clicked));
    println!("After decrement: count = {}", model.count);
    println!("  └─ Counter decreased from 2 to 1");
    println!();

    // Demonstrate programmatic value setting (not from UI)
    println!("Application programmatically sets counter to 10...");
    model = model.update(CounterMessage::SetValue(10));
    println!("After set to 10: count = {}", model.count);
    println!("  └─ Counter set directly to 10, ignoring previous value");
    println!();

    // Demonstrate programmatic reset (not from UI)
    println!("Application programmatically resets counter...");
    model = model.update(CounterMessage::Reset);
    println!("After reset: count = {}", model.count);
    println!("  └─ Counter reset to 0, regardless of previous value");
    println!();

    // Show final UI state
    let display_extracted = MockBackend::extract(&model.count_display(), &ctx);
    println!("Final count display: '{}'", display_extracted.content);
    println!();

    println!("Counter example completed!");
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
        let updated = model.update(CounterMessage::IncrementButton(ButtonMessage::Clicked));
        assert_eq!(updated.count, 6);

        // Chain operations to verify each update returns a new model
        let model = CounterModel::zero();
        let updated = model
            .update(CounterMessage::IncrementButton(ButtonMessage::Clicked))
            .update(CounterMessage::IncrementButton(ButtonMessage::Clicked))
            .update(CounterMessage::IncrementButton(ButtonMessage::Clicked));
        assert_eq!(updated.count, 3);
    }

    #[test]
    fn counter_message_decrement() {
        // Test decrement including edge case of going negative
        let model = CounterModel::new(5);
        let updated = model.update(CounterMessage::DecrementButton(ButtonMessage::Clicked));
        assert_eq!(updated.count, 4);

        // Verify no special casing needed for negative values
        let model = CounterModel::zero();
        let updated = model.update(CounterMessage::DecrementButton(ButtonMessage::Clicked));
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
            .update(CounterMessage::IncrementButton(ButtonMessage::Clicked)) // 0 -> 1
            .update(CounterMessage::IncrementButton(ButtonMessage::Clicked)) // 1 -> 2
            .update(CounterMessage::SetValue(10)) // 2 -> 10
            .update(CounterMessage::DecrementButton(ButtonMessage::Clicked)) // 10 -> 9
            .update(CounterMessage::DecrementButton(ButtonMessage::Clicked)) // 9 -> 8
            .update(CounterMessage::Reset) // 8 -> 0
            .update(CounterMessage::SetValue(-5)) // 0 -> -5
            .update(CounterMessage::IncrementButton(ButtonMessage::Clicked)); // -5 -> -4

        assert_eq!(updated.count, -4);
    }

    #[test]
    fn counter_immutability() {
        // Verify updates don't modify original model (enables debugging tools)
        let original = CounterModel::new(5);
        let original_count = original.count;

        let _updated = original
            .clone()
            .update(CounterMessage::IncrementButton(ButtonMessage::Clicked));

        // Original model unchanged - crucial for debugging and undo functionality
        assert_eq!(original.count, original_count);
    }

    #[test]
    fn counter_message_properties() {
        // Verify messages have properties required by framework

        // Equality enables message comparison and deduplication
        assert_eq!(
            CounterMessage::IncrementButton(ButtonMessage::Clicked),
            CounterMessage::IncrementButton(ButtonMessage::Clicked)
        );
        assert_eq!(CounterMessage::SetValue(42), CounterMessage::SetValue(42));
        assert_ne!(
            CounterMessage::IncrementButton(ButtonMessage::Clicked),
            CounterMessage::DecrementButton(ButtonMessage::Clicked)
        );
        assert_ne!(CounterMessage::SetValue(1), CounterMessage::SetValue(2));

        // Cloning enables message queuing and replay functionality
        let msg = CounterMessage::SetValue(100);
        let cloned = msg.clone();
        assert_eq!(msg, cloned);
    }

    #[test]
    fn counter_overflow_handling() {
        // Test saturating arithmetic prevents overflow panics
        let model = CounterModel::new(i32::MAX);
        let updated = model.update(CounterMessage::IncrementButton(ButtonMessage::Clicked));
        assert_eq!(updated.count, i32::MAX); // Should saturate, not overflow

        let model = CounterModel::new(i32::MIN);
        let updated = model.update(CounterMessage::DecrementButton(ButtonMessage::Clicked));
        assert_eq!(updated.count, i32::MIN); // Should saturate, not underflow
    }

    #[test]
    fn counter_button_interaction_messages() {
        // Test that non-click button messages preserve count but update button state
        let model = CounterModel::new(5);
        let original_count = model.count;

        // Test hover interaction doesn't change count
        let updated =
            model
                .clone()
                .update(CounterMessage::IncrementButton(ButtonMessage::Interaction(
                    InteractionMessage::HoverChanged(true),
                )));
        assert_eq!(updated.count, original_count);

        // Test focus interaction doesn't change count
        let updated =
            model
                .clone()
                .update(CounterMessage::DecrementButton(ButtonMessage::Interaction(
                    InteractionMessage::FocusChanged(true),
                )));
        assert_eq!(updated.count, original_count);
    }

    #[test]
    fn counter_ui_components() {
        // Test that UI components are properly initialized and updated
        let model = CounterModel::new(5);
        let ctx = RenderContext::new();

        // Test initial UI state
        let increment_extracted = MockBackend::extract(&model.increment_button, &ctx);
        let decrement_extracted = MockBackend::extract(&model.decrement_button, &ctx);
        let display_extracted = MockBackend::extract(&model.count_display(), &ctx);

        assert_eq!(increment_extracted.text, "+");
        assert_eq!(decrement_extracted.text, "-");
        assert_eq!(display_extracted.content, "5");

        // Test UI updates after count change
        let updated = model.update(CounterMessage::IncrementButton(ButtonMessage::Clicked));
        let display_extracted = MockBackend::extract(&updated.count_display(), &ctx);
        assert_eq!(display_extracted.content, "6");
    }

    #[test]
    fn counter_view() {
        // Test that the complete view is properly structured
        let model = CounterModel::new(123);
        let ctx = RenderContext::new();

        let ui_layout = model.view();
        let layout_extracted = MockBackend::extract(&ui_layout, &ctx);

        // Test VStack properties
        assert_eq!(layout_extracted.spacing, 16.0);
        assert_eq!(layout_extracted.alignment, Alignment::Center);

        // Test title text
        assert_eq!(layout_extracted.content.0.content, "Counter Example");
        assert_eq!(layout_extracted.content.0.font_size, 20.0);

        // Test count display
        assert_eq!(layout_extracted.content.1.content, "123");
        assert_eq!(layout_extracted.content.1.font_size, 24.0);

        // Test button row (HStack)
        let button_row = &layout_extracted.content.2;
        assert_eq!(button_row.spacing, 12.0);
        assert_eq!(button_row.alignment, Alignment::Center);

        // Test buttons in the row
        assert_eq!(button_row.content.0.text, "-");
        assert_eq!(button_row.content.1.text, "+");
    }
}

// End of File
