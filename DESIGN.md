# Ironwood UI Framework - Requirements Specification

## 1. Project Overview

**Ironwood** is a Rust-native UI framework that combines the architectural benefits of Elm's unidirectional data flow with the ergonomic appeal of SwiftUI's declarative syntax, while leveraging Rust's zero-cost abstractions for maximum performance.

### 1.1 Core Philosophy

- **Elm Architecture**: Explicit state management through `Model → View → Message → Update` cycles
- **SwiftUI Ergonomics**: Declarative, composable view syntax with fluent builder APIs
- **Pragmatic Performance**: Zero-cost where beneficial, minimal-cost where necessary
- **Platform Native**: True native rendering, not web-based solutions

### 1.2 Target Use Cases

- Desktop applications requiring high performance
- Cross-platform apps with native look-and-feel
- Games and graphics applications needing GPU acceleration
- Developer tools and IDEs
- Alternative to Electron-based applications

## 2. Design Goals

### 2.1 Performance Requirements

- **Minimal dynamic dispatch** for core view composition (strategic use of `dyn Trait`)
- **Static typing** - all view hierarchies known at compile time
- **GPU acceleration** support via wgpu backend
- **Efficient memory management** - minimize allocations where reasonable
- **Basic layout caching** - avoid unnecessary recomputation

### 2.2 Ergonomic Requirements

- **Minimal boilerplate** for common operations
- **Simple composition** - tuple-based view composition
- **Type inference** - minimal explicit type annotations required
- **Macro-assisted APIs** - `msg!(expression)` for simple updates, `msg!{}` blocks for complex state changes and async operations
- **Simple to understand** - straightforward patterns that work for basic UIs

### 2.3 Architectural Requirements

- **Explicit state transitions** - all changes go through messages
- **Unidirectional data flow** - predictable state updates
- **Async/effect system** - clean handling of side effects and async operations
- **Testable design** - views and updates can be tested in isolation
- **Message composability** - messages can be combined and transformed

## 3. Technical Architecture

### 3.1 Core Type System

#### 3.1.1 Message Trait

```rust
// Base trait for all messages
pub trait Message: Debug + Clone + Send + Sync + 'static {}

// Example message type - only needs base Message trait
#[derive(Message)]
pub enum CounterMessage {
    Increment,
    Decrement,
    Reset,
    SetValue(i32),
}
```

#### 3.1.2 Model Trait

```rust
pub trait Model: Clone + Debug + Send + Sync + 'static {
    // Models are pure data structures that can be cloned efficiently
    // Debug for development, Send + Sync for potential future threading
    type Message: Message; // type Message implements Message trait

    /// Update the model with a message, consuming the old model and returning a new one
    fn update(self, message: Self::Message) -> Self;

    /// Generate the view for this model
    fn view(&self) -> impl View;
}

// Simple implementation
#[derive(Clone, Debug)]
struct CounterModel {
    count: i32,
}

impl Model for CounterModel {
    type Message = CounterMessage;

    fn update(self, message: CounterMessage) -> Self {
        match message {
            CounterMessage::Increment => Self { count: self.count + 1 },
            CounterMessage::Decrement => Self { count: self.count - 1 },
            CounterMessage::Reset => Self { count: 0 },
            CounterMessage::SetValue(value) => Self { count: value },
        }
    }

    fn view(&self) -> impl View {
        VStack::new((
            Text::new("Counter App").font_size(24.0),
            Text::new(&format!("Current value: {}", self.count)),
            HStack::new((
                Button::new("+").on_press(CounterMessage::Increment),
                Button::new("-").on_press(CounterMessage::Decrement),
                Button::new("Reset").on_press(CounterMessage::Reset),
            )).spacing(8.0),
        )).spacing(16.0)
    }
}
```

#### 3.1.3 Universal Message System

```rust
// Messages that can be applied to arbitrary models (universal messages only)
pub trait ApplicableMessage<T: Model>: Message {
    /// Apply this message to the model
    fn apply(self, model: &mut T);
}

// Universal message type that handles 90% of GUI interactions
#[derive(Message)]
pub enum Msg<T: Model> {
    // Direct value setting
    Set(T),

    // Transform with pure function
    Map(fn(T) -> T),

    // Transform with closure (for captures)
    MapWith(Box<dyn Fn(T) -> T>),

    // Conditional update
    If(fn(&T) -> bool, Box<Msg<T>>),

    // Batch multiple operations
    Batch(Vec<Msg<T>>),

    // Async operations
    MapAsync(Box<dyn Fn(T) -> Pin<Box<dyn Future<Output = T>>>>),

    // Reset to default
    Reset,

    // No-op (useful for conditional flows)
    None,
}

impl<T: Model + Default> ApplicableMessage<T> for Msg<T> {
    fn apply(self, model: &mut T) {
        *model = match self {
            Msg::Set(new_model) => new_model,
            Msg::Map(func) => func(std::mem::take(model)),
            Msg::MapWith(func) => func(std::mem::take(model)),
            Msg::If(condition, msg) => {
                if condition(model) {
                    msg.apply(model);
                    return;
                }
                std::mem::take(model)
            },
            Msg::Batch(messages) => {
                let mut current = std::mem::take(model);
                for msg in messages {
                    current = match msg {
                        Msg::Set(new_model) => new_model,
                        Msg::Map(func) => func(current),
                        Msg::MapWith(func) => func(current),
                        _ => {
                            *model = current;
                            msg.apply(model);
                            return;
                        }
                    };
                }
                current
            },
            Msg::MapAsync(_func) => {
                // Async operations require runtime support
                // Implementation will spawn future and send result back
                todo!("Async support requires runtime integration")
            },
            Msg::Reset => T::default(),
            Msg::None => std::mem::take(model),
        };
    }
}

// Usage examples:
// msg!(model.counter += 1) expands to: Msg::Map(|m| Self { counter: m.counter + 1, ..m })
// msg!{ batch { model.a = 1; model.b = 2; } } expands to: Msg::Batch(vec![...])
```

### 3.2 View System

#### 3.2.1 View Trait and Extraction System

```rust
// Views are composable UI elements that describe their structure
pub trait View {
    // Marker trait for view types
}

// Views are extracted by backends into renderable descriptions
pub trait ViewExtractor<V: View> {
    type Output;
    fn extract(view: &V, ctx: &RenderContext) -> Self::Output;
}

// Context provided during extraction (fonts, themes, etc.)
pub struct RenderContext {
    // Implementation managed by framework
}

// Example backend-specific extraction
pub struct WgpuBackend;
pub struct TestBackend;

// Each backend decides what it needs from each view type
impl ViewExtractor<Text> for WgpuBackend {
    type Output = WgpuTextRenderView;
    fn extract(text: &Text, ctx: &RenderContext) -> Self::Output {
        // Backend-specific logic for GPU rendering
        WgpuTextRenderView {
            content: text.content.clone(),
            font_size: text.font_size,
            color: text.color,
            // GPU-specific fields...
        }
    }
}

impl ViewExtractor<Text> for TestBackend {
    type Output = TestTextRenderView;
    fn extract(text: &Text, ctx: &RenderContext) -> Self::Output {
        // Simple structure for testing
        TestTextRenderView {
            content: text.content.clone(),
            style: format!("{}px {:?}", text.font_size, text.color),
        }
    }
}

// Backend output types (examples)
pub struct WgpuTextRenderView {
    content: String,
    font_size: f32,
    color: Color,
    // GPU buffers, shader handles, etc.
}

pub struct TestTextRenderView {
    content: String,
    style: String,
}
```

#### 3.2.2 Tuple Composition

```rust
// Tuple composition for combining views - the core composition mechanism
impl<V1: View, V2: View> View for (V1, V2) {}
impl<V1: View, V2: View, V3: View> View for (V1, V2, V3) {}
// ... extend to reasonable tuple sizes

// Backends handle extraction of tuple compositions
impl<V1: View, V2: View, B> ViewExtractor<(V1, V2)> for B
where
    B: ViewExtractor<V1> + ViewExtractor<V2>,
{
    type Output = (B::Output, B::Output);

    fn extract(views: &(V1, V2), ctx: &RenderContext) -> Self::Output {
        let child1 = B::extract(&views.0, ctx);
        let child2 = B::extract(&views.1, ctx);
        (child1, child2)
    }
}

// Usage: (text_view, button_view, another_view)
// Backend extracts each view independently
```

#### 3.2.3 Container Views

```rust
// Vertical stack container
#[derive(View)]
pub struct VStack<V> {
    pub children: V,
    pub spacing: f32,
}

impl<V> VStack<V> {
    pub fn new(children: V) -> Self {
        Self {
            children,
            spacing: 8.0,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

// Backends handle container extraction
impl<V: View, B> ViewExtractor<VStack<V>> for B
where
    B: ViewExtractor<V>,
{
    type Output = B::Output; // Backend decides how to represent containers

    fn extract(vstack: &VStack<V>, ctx: &RenderContext) -> Self::Output {
        // Backend extracts the children and handles layout as needed
        B::extract(&vstack.children, ctx)
    }
}

// Horizontal stack container
#[derive(View)]
pub struct HStack<V> {
    pub children: V,
    pub spacing: f32,
}

impl<V> HStack<V> {
    pub fn new(children: V) -> Self {
        Self {
            children,
            spacing: 8.0,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

// Backends handle container extraction
impl<V: View, B> ViewExtractor<HStack<V>> for B
where
    B: ViewExtractor<V>,
{
    type Output = B::Output; // Backend decides how to represent containers

    fn extract(hstack: &HStack<V>, ctx: &RenderContext) -> Self::Output {
        // Backend extracts the children and handles layout as needed
        B::extract(&hstack.children, ctx)
    }
}
```

#### 3.2.4 Basic Views

```rust
// Simple text view
#[derive(View)]
pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub color: Color,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            font_size: 16.0,
            color: Color::BLACK,
        }
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

// Simple image view
#[derive(View)]
pub struct Image {
    pub source: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl Image {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            width: None,
            height: None,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

// Simple button view
#[derive(View)]
pub struct Button {
    pub text: String,
    pub style: ButtonStyle,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: ButtonStyle::default(),
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }
}

// Basic style types
#[derive(Debug, Clone, Default)]
pub struct ButtonStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub border_radius: f32,
}

// Basic color type (implementation details omitted)
#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
```

### 3.3 Event System

#### 3.3.1 Event Trait and Broadcasting

```rust
// Events are typed data that can be broadcast
pub trait Event: Debug + Clone + Send + Sync + 'static {}

// Common events
#[derive(Event)]
pub struct ClickEvent {
    pub component_id: String,
    pub position: (f32, f32),
}

#[derive(Event)]
pub struct FocusEvent {
    pub component_id: String,
    pub gained: bool,
}
```

#### 3.3.2 Event Emission and Handling

```rust
// Components can emit events
pub trait EventEmitter {
    fn emit<E: Event>(&self, event: E);
}

// Views can handle events
pub trait EventHandler<T: Model> {
    fn on_event<E: Event>(
        self,
        handler: impl Fn(E) -> Msg<T>
    ) -> impl View;
}

// Usage
Button::new("Save")
    .on_click(msg!(model.save_attempted = true))     // Primary message
    .emits(ClickEvent {
        component_id: "save_button".to_string(),
        position: (0.0, 0.0),
    })

// Multiple event handlers
app_view()
    .on_event(|ClickEvent { component_id, .. }| {
        msg!(analytics.track_click(component_id))
    })
    .on_event(|ClickEvent { .. }| {
        msg!(accessibility.announce("Button activated"))
    })
    .on_event(|FocusEvent { gained, .. }| {
        msg!(model.focus_state = gained)
    })
```

#### 3.3.3 Event vs Message Guidelines

**Use Messages for:**

- Primary business logic
- State changes that require exhaustive handling
- Component-to-parent communication
- Critical application flow

**Use Events for:**

- Analytics and logging
- Accessibility announcements
- Cross-cutting concerns
- Optional reactive behaviors
- Multiple subscribers to same occurrence

### 3.4 Application Structure

```rust
// Simple app runner
pub struct App<T: Model> {
    model: T,
}

impl<T: Model> App<T> {
    pub fn new(model: T) -> Self {
        Self { model }
    }

    pub fn run(self) {
        // Framework handles the render loop
        // - Gets view by calling model.view()
        // - Handles layout and rendering
        // - Processes input events into messages
        // - Applies messages to model via model.update()
        // - Repeats
    }
}
```

## 4. API Examples

### 4.1 Message Creation

```rust
// msg!() macro for simple updates
Button::new("Increment").on_press(msg!(model.counter += 1))
Button::new("Reset").on_press(msg!(model.counter = 0))

// msg!{} blocks for complex updates and async operations
Button::new("Load Data").on_press(msg! {
    batch {
        model.loading = true;
        model.error = None;
    }
    async {
        let data = fetch_user_data(model.user_id).await;
        model.data = data;
        model.loading = false;
    }
})

// Custom messages for complex logic
#[derive(Debug, Clone)]
enum AppMessage {
    Increment,
    Decrement,
    Reset,
    SetValue(i32),
    LoadUserData(i32), // user_id
    DataLoaded(UserData),
}

Button::new("Inc").on_press(AppMessage::Increment)
Button::new("Load").on_press(AppMessage::LoadUserData(42))
```

### 4.2 View Composition

```rust
// Model provides its own view - access to model data is automatic
impl Model for CounterModel {
    type Message = CounterMessage;

    fn update(self, message: CounterMessage) -> Self {
        match message {
            CounterMessage::Increment => Self { count: self.count + 1 },
            CounterMessage::Decrement => Self { count: self.count - 1 },
            CounterMessage::Reset => Self { count: 0 },
        }
    }

    fn view(&self) -> impl View {
        VStack::new((
            Text::new("Counter App").font_size(24.0),
            Text::new(&format!("Current value: {}", self.count)),
            HStack::new((
                Button::new("+").on_press(CounterMessage::Increment),
                Button::new("-").on_press(CounterMessage::Decrement),
                Button::new("Reset").on_press(CounterMessage::Reset),
            )).spacing(8.0),
        )).spacing(16.0)
    }
}

// Usage in app
#[derive(Clone, Debug, Default)]
struct CounterModel {
    count: i32,
}

fn main() {
    let app = App::new(CounterModel::default());
    app.run();  // App gets the view by calling model.view()
}
```

### 4.3 Event Handling Example

```rust
// Complex dashboard with event handling
fn dashboard_view() -> impl View {
    VStack::new((
        // Header with navigation
        header_component()
            .map_message(|HeaderMsg::NavigateTo(page)|
                AppMsg::PageChanged(page)
            ),

        // Main content area
        HStack::new((
            // Sidebar with filters
            sidebar_component()
                .map_message(|SidebarMsg::FilterChanged(filter)|
                    AppMsg::FilterApplied(filter)
                )
                .on_message(SidebarMsg::ResetFilters,
                    msg!(model.filters = FilterSet::default())
                ),

            // Data table
            data_table_component()
                .map_message(|TableMsg::RowSelected(id)|
                    AppMsg::RowFocused(id)
                )
                .map_message(|TableMsg::SortChanged(col)|
                    AppMsg::SortApplied(col)
                ),
        )),

        // Status bar
        status_bar_component(),
    ))

    // Global event handlers for cross-cutting concerns
    .on_event(|ClickEvent { component_id, position }| {
        msg!(analytics.track_click(component_id, position))
    })
    .on_event(|ClickEvent { .. }| {
        msg!(model.last_interaction = SystemTime::now())
    })
    .on_event(|FocusEvent { component_id, gained }| {
        msg!(accessibility.announce_focus_change(component_id, gained))
    })
    .on_event(|ErrorEvent { message, component_id }| {
        msg!(error_log.record(message, component_id))
    })
    .on_event(|PerformanceEvent { component_id, render_time }| {
        msg!(performance_monitor.record_render_time(component_id, render_time))
    })
}
```

## 5. Implementation Plan

### 5.1 Crate Structure

```text
ironwood/                   # Workspace root
├── src/                    # Main "ironwood" crate (re-exports + App type)
├── crates/
│   ├── core/              # ironwood-core - Core traits (Model, Message, View)
│   ├── macros/            # ironwood-macros - msg!() macro implementation
│   ├── winit/             # ironwood-winit - Window management and event handling
│   ├── wgpu-backend/      # ironwood-wgpu-backend - GPU rendering backend
│   ├── web-backend/       # ironwood-web-backend - Web/WASM backend
│   ├── native-backend/    # ironwood-native-backend - Native widget integration
│   └── test-backend/      # ironwood-test-backend - Mock backend for testing
├── Cargo.toml             # Workspace configuration
└── examples/              # Example applications
```

**Crate Dependencies:**
- `ironwood` (main) - Re-exports core APIs, includes default backends
- `ironwood-core` - Foundation traits, no dependencies
- `ironwood-macros` - Procedural macros, depends on `core`
- `ironwood-winit` - Window/event handling, depends on `core` + `winit`
- `ironwood-wgpu-backend` - GPU rendering, depends on `core` + `wgpu` + `winit`
- `ironwood-web-backend` - Web platform, depends on `core` + `web-sys`
- `ironwood-native-backend` - Native widgets, depends on `core` + platform APIs
- `ironwood-test-backend` - Testing utilities, depends on `core`

### 5.2 Development Phases

#### Phase 1: Core Framework (MVP)

- [ ] Core traits: `Model`, `Message`, `View`, `ViewExtractor`
- [ ] Universal message system: `Msg<T>` with basic variants
- [ ] `msg!()` macro for simple updates
- [ ] Basic views: `Text`, `Button`, `VStack`, `HStack`
- [ ] Tuple composition system
- [ ] Simple test backend for validation
- [ ] Basic app runner and event loop

#### Phase 2: Enhanced Composition

- [ ] Event system for cross-cutting concerns
- [ ] Additional views: `Image`, `TextInput`, `Spacer`
- [ ] Layout modifiers and styling
- [ ] `msg!{}` block syntax for complex updates
- [ ] Async effects system (`MapAsync`)

#### Phase 3: Production Features

- [ ] GPU backend with `wgpu`
- [ ] Advanced layout algorithms
- [ ] Animation and transitions
- [ ] Accessibility features
- [ ] Theme system
- [ ] Performance optimization and caching

#### Phase 4: Ecosystem

- [ ] Developer tools and debugging
- [ ] Additional backends (native platforms)
- [ ] Widget library expansion
- [ ] Documentation and examples
- [ ] Community and package ecosystem

### 5.3 Success Metrics

#### Developer Experience

- **Learning curve**: New developers can build basic UIs within 30 minutes
- **Familiarity**: Coming from React/SwiftUI feels natural and familiar
- **Error messages**: Compile errors are helpful and actionable
- **Development workflow**: Hot reload works for rapid iteration
- **Productivity**: Productive within first day for Rust developers
- **IDE support**: Works well with rust-analyzer and other tools

#### Performance Goals

- **Startup time**: Competitive with native applications
- **Memory efficiency**: Competitive with other native UI frameworks (Qt, GTK, native platform APIs)
- **Rendering**: Smooth 120 FPS animations on modest hardware
- **Incremental updates**: Efficient updates for large data sets
- **Frame budget**: Sub-second incremental builds for typical applications
- **Bundle size**: Comparable to native applications

#### Ecosystem Integration

- **Library compatibility**: Works well with existing Rust libraries
- **Async integration**: Easy integration with async runtimes (tokio, async-std)
- **Community**: Active community development and contributions
- **Documentation**: Comprehensive guides and examples
- **Stability**: Semantic versioning with clear migration paths
- **Production readiness**: Used in production applications

### 5.4 Performance Benchmarks

**Target performance metrics:**

- **Rendering**: 120 FPS for complex UIs (1000+ elements)
- **Layout**: < 1ms layout computation for typical screens
- **Frame time**: < 8ms total frame time including rendering
- **Memory**: Usage competitive with native frameworks
- **Startup**: Time competitive with native applications
- **Incremental updates**: Efficient updates for large data sets

### 5.5 Platform Support

Ironwood supports dual rendering backends on all platforms, allowing applications to use native widgets and custom GPU-rendered widgets simultaneously:

**Desktop Platforms:**

- **Windows 10+**: Native Win32/WinUI widgets + wgpu GPU rendering
- **macOS 10.15+**: Native Cocoa widgets (via objc2) + wgpu GPU rendering
- **Linux**: Native GTK widgets + wgpu GPU rendering

**Mobile Platforms:**

- **iOS**: Native UIKit widgets + wgpu GPU rendering
- **Android**: Native Android Views + wgpu GPU rendering

**Web Platform:**

- **Web**: Native DOM elements + wgpu GPU rendering (WebGL/WebGPU)

**Backend Architecture:**

- **Native backend**: Platform-specific widgets embedded in winit windows
- **wgpu backend**: Custom cross-platform GPU rendering for themed widgets and specialized views
- **Mock backend**: Headless backend for testing and CI environments (no actual rendering)
- **Simultaneous use**: Applications can combine both backends as needed (e.g., native toolbars with custom GPU-rendered content areas)

## 6. Testing Strategy

### 6.1 Unit Tests

- **Message application correctness** - Verify all message types update models correctly
- **Layout algorithm validation** - Test layout calculations for various scenarios
- **View composition compilation** - Ensure view hierarchies compile and type-check
- **Macro expansion testing** - Validate `msg!()` macro generates correct code

### 6.2 Integration Tests

- **Full render pipeline testing** - End-to-end rendering from model to pixels
- **Event handling correctness** - Input events properly generate messages
- **Cross-platform rendering consistency** - Same visual output across platforms
- **Performance regression testing** - Automated benchmarks catch slowdowns

### 6.3 Performance Tests

- **Rendering benchmarks** - FPS measurements for various UI complexities
- **Layout performance tests** - Time measurements for layout algorithms
- **Memory usage profiling** - Track allocations and memory efficiency
- **Startup time measurements** - Application launch performance

## 7. Documentation Strategy

### 7.1 API Documentation

- **Comprehensive rustdoc** - All public APIs fully documented with examples
- **Code examples** - Every major feature demonstrated with working code
- **Migration guides** - Help developers transition from other frameworks

### 7.2 Tutorial Series

- **"Getting Started"** - Basic counter app walkthrough
- **"Complex UIs"** - Multi-component applications and state management
- **"Custom Views"** - Creating reusable view components
- **"Performance Optimization"** - Advanced techniques and best practices

### 7.3 Example Applications

- **TodoMVC implementation** - Standard comparison application
- **File manager application** - Complex state and navigation patterns
- **Simple game** - Showcasing GPU features and animations
- **IDE-like application** - Complex layout and performance requirements

## 8. Architecture Decisions Record

### 8.1 Immutable Message Transforms

**Decision**: Use immutable transforms (`T -> T`) for all message updates - both universal messages and custom messages
**Rationale**: Provides consistent, predictable update semantics across the framework. Rust's move semantics and compiler optimizations make this as efficient as manual mutation while being easier to test and reason about
**Trade-offs**: Slightly more verbose for simple updates, but gains consistency and functional programming benefits

### 8.2 Tuple Composition vs Trait Objects

**Decision**: Use tuple composition as primary mechanism with strategic `dyn Trait` where needed
**Rationale**: Maintains compile-time type information while allowing runtime flexibility
**Trade-offs**: Some ergonomic complexity, but better performance and type safety

### 8.3 Universal Messages vs Custom Messages

**Decision**: Provide both `Msg<T>` universal messages and custom message types
**Rationale**: `Msg<T>` handles 90% of GUI interactions, custom messages for complex domain logic
**Trade-offs**: Two message systems, but covers full spectrum of use cases

### 8.4 View Extraction vs Immediate Mode

**Decision**: Use extraction pattern separating frontend API from backend implementation
**Rationale**: Clean separation of concerns, testability, multiple backend support
**Trade-offs**: Additional abstraction layer, but enables better architecture

### 8.5 Events vs Messages for Cross-Cutting Concerns

**Decision**: Dual system - messages for primary logic, events for cross-cutting concerns
**Rationale**: Messages ensure exhaustive handling, events allow optional reactive behaviors
**Trade-offs**: Two communication systems, but cleaner separation of concerns

## 9. Comparison to Existing Solutions

### 9.1 vs Tauri/Electron

**Advantages**: True native performance, no web runtime overhead, type-safe state management
**Trade-offs**: Smaller ecosystem, Rust learning curve vs web technologies

### 9.2 vs Egui

**Advantages**: More structured architecture, better state management, GPU acceleration
**Trade-offs**: More complex setup vs immediate mode simplicity

### 9.3 vs Flutter

**Advantages**: Memory safety, zero-cost abstractions, better concurrency
**Trade-offs**: Smaller ecosystem, newer platform vs mature tooling

### 9.4 vs React/SwiftUI

**Advantages**: Compile-time guarantees, predictable performance, no runtime overhead
**Trade-offs**: Less dynamic, steeper learning curve vs familiar patterns

## 10. Conclusion

Ironwood represents a new approach to native UI development that combines the architectural benefits of functional programming with the performance characteristics of systems programming. By leveraging Rust's type system and zero-cost abstractions, it provides a foundation for building high-performance, maintainable user interfaces without sacrificing developer ergonomics.

The framework's design prioritizes:

1. **Architectural clarity** through explicit state management
2. **Performance** through compile-time optimization
3. **Ergonomics** through declarative syntax and helpful macros
4. **Flexibility** through composable components and dual message systems
5. **Reliability** through Rust's type safety and memory management

This foundation enables developers to build complex, interactive applications with confidence in both correctness and performance, establishing a new paradigm for native UI development in the Rust ecosystem.

## 10. Outstanding Design Issues & Implementation Decisions

This section identifies known design gaps and considerations that need to be resolved during MVP implementation. Some of these choices will become clear through actual implementation experience.

### 10.1 Critical Architecture Issues

#### User Interaction Handling (High Priority)
**Issue**: Examples show `Button::new("+").on_press(CounterMessage::Increment)` but no `on_press()` method is defined.

**Options**:
- **Option A**: Button returns different type: `Button::on_press() -> InteractiveButton<M>`
- **Option B**: Button stores message: `Button<M: Message> { on_press: Option<M> }`
- **Option C**: Framework-level event binding (views stay pure)
- **Option D**: Event handlers passed separately from view construction

**Decision needed**: How do user interactions turn into messages?

#### View Rendering Pipeline (High Priority)
**Issue**: `View` is a marker trait with no methods. How does rendering actually work?

**Missing pieces**:
- How does framework traverse view tree?
- When/how do views get converted to pixels?
- What's the interface between views and backends?

**Options**:
- Add `render()` method to View trait
- Use ViewExtractor pattern as designed
- Create intermediate render tree representation
- Direct backend-specific rendering

#### Error Handling Strategy (Medium Priority)
**Issue**: No error handling defined anywhere in the system.

**Considerations**:
- What if view rendering fails?
- What if message processing fails?
- What if backend initialization fails?
- Should we use `Result` types or panics?

### 10.2 API Design Questions

#### Universal Message System Complexity
**Issue**: `Msg<T>` enum has several questionable design choices.

**Specific concerns**:
- Why both `fn(T) -> T` and `Box<dyn Fn(T) -> T>`?
- Is `MapAsync` too complex for a message?
- Does `Batch` create infinite recursion risk?
- Should `If` take `&T` or `T` for consistency?

**Options**:
- Simplify to just `Update(Box<dyn FnOnce(T) -> T>)` and `Batch`
- Remove async handling from messages entirely
- Make all function signatures consistent

#### ViewExtractor Pattern Scalability
**Issue**: N × M implementations (N views × M backends) doesn't scale.

**Considerations**:
- Adding new view requires implementing for all backends
- No shared logic between similar views
- Complex trait bounds and associated types

**Options**:
- Use trait objects instead: `dyn View -> RenderNode`
- Create intermediate representation
- Macro-generate extractors
- Abandon extraction pattern for direct rendering

#### Tuple Composition Limitations
**Issue**: Tuples only support fixed arity and small numbers of children.

**Problems**:
- No dynamic child lists (Vec of views)
- Awkward for real UI layouts
- Limited to ~12 elements

**Options**:
- Add `Vec<Box<dyn View>>` support
- Create custom container types
- Use builder pattern for dynamic composition
- Macro-based composition

### 10.3 Implementation Complexity Issues

#### Message Application Mechanism
**Issue**: Current `std::mem::take` approach in `Msg<T>::apply` is complex and error-prone.

**Concerns**:
- Temporarily replaces model with `Default::default()`
- Complex control flow with early returns
- Error-prone batch processing logic

**Options**:
- Simplify to consume and return pattern
- Use `Option<T>` wrapper to avoid Default requirement
- Split complex variants into separate methods

#### Missing Derive Macro Definitions
**Issue**: Examples use `#[derive(View)]`, `#[derive(Message)]`, `#[derive(Model)]` but these don't exist.

**Considerations**:
- What should these macros actually generate?
- Are they necessary or just convenience?
- What trait bounds should they require?

### 10.4 Testing & Development Questions

#### Mock Backend Capabilities
**Issue**: Mock backend mentioned but not specified.

**Questions needed**:
- What should mock backend capture?
- How should it simulate user interactions?
- What API should it expose for testing?
- How does it integrate with the main architecture?

#### Performance Validation
**Issue**: Performance claims made without validation mechanism.

**Unknowns**:
- How to measure 120 FPS target?
- What constitutes "complex UIs (1000+ elements)"?
- How to benchmark against Qt/GTK fairly?
- What metrics matter most?

### 10.5 Platform Integration Decisions

#### Native Widget Integration Details
**Issue**: Dual backend approach outlined but implementation unclear.

**Questions**:
- How do native widgets integrate with winit windows?
- How does layout work across native/custom widgets?
- How do we handle platform differences in widget behavior?
- What's the API boundary between backends?

#### Event System Justification
**Issue**: Event system adds complexity but benefits unclear.

**Considerations**:
- Do we need events if messages handle component communication?
- What use cases require broadcasting vs direct messages?
- How do events integrate with the Model-View-Update cycle?
- Is this premature optimization?

### 10.6 Development Strategy

**Recommendation**: Start with minimal MVP that resolves core architectural questions:

1. **Phase 0.1**: Basic Model trait with view method, simple Text/Button views
2. **Phase 0.2**: Solve user interaction handling (button clicks → messages)
3. **Phase 0.3**: Implement basic rendering pipeline (mock backend first)
4. **Phase 0.4**: Add container views (VStack/HStack) and layout
5. **Phase 0.5**: Validate with working counter example

**Decision process**: Let implementation experience guide design choices rather than over-architecting upfront.

## 11. Conclusion
