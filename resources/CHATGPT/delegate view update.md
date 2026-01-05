
Yes, you can modularize a custom struct (like Keypad) by placing its specific view, 
update, and message logic within its own Rust module. 

This is a common pattern in the Elm Architecture (which libcosmic and its underlying iced library follow) 
to manage complexity as applications grow.

Modularization Pattern

To implement this for a Keypad struct, follow these steps:

    State: 
    Define the Keypad struct with its internal data.

    Messages: 
    Create a KeypadMessage enum within the module to represent actions internal to the keypad.

    Update: 
    Implement a method on the struct 
    (e.g., pub fn update(&mut self, message: KeypadMessage))
    that modifies its own state.

    View: 
    Implement a method 
    (e.g., pub fn view(&self) -> Element<KeypadMessage>) 
    that describes the keypad's UI layout.

Integration with the Main App

To connect the modular Keypad to your main AppModel, you "wrap" or "map" the nested logic:

    Main Message Enum: 
    Add a variant to your main Message enum that holds a KeypadMessage:

```rust
enum Message {
    Keypad(keypad::KeypadMessage),
    // other main messages
}
```

Main Update Loop: In the main update function, delegate the message to the keypad's update method:

```rust
match message {
    Message::Keypad(inner_msg) => {
        self.keypad.update(inner_msg);
    }
}
```

Main View Function: 
In your main view, call the keypad's view and use .map(Message::Keypad) to convert its 
internal messages into the main app's message type.

```rust
self.keypad.view().map(Message::Keypad)
```

Why Use This Pattern?

    Decoupling: 
    Keeps the main update and view functions from becoming too large and "spaghetti-like".
    
    Reusability: 
    Allows you to use the same Keypad component across different parts of the app or in other projects.
    
    Testing: 
    Makes it easier to write unit tests for specific component logic in isolation.

In a modularized libcosmic application, your Keypad functions should mirror the logic of the 
main app but operate strictly on the Keypad's own types.

1. View Function Signature
The view function converts your internal Keypad state into a UI element.
It should return a cosmic::Element (or iced::Element) specialized to your local message type.

```rust
    // Inside keypad.rs
    pub fn view(&self) -> Element<KeypadMessage> {
        // Return a layout (e.g., Column, Grid) containing your buttons
        // Each button will produce a `KeypadMessage`
    }
```
Returns: 
Element<KeypadMessage>.

Integration: 
In your main AppModel::view, you will call 
self.keypad.view().map(Message::Keypad) 
to convert the local KeypadMessage into the global Message.

2. Update Function Signature
The update function handles logic internal to the keypad.
To stay consistent with the main app's signature, it returns a Task containing a cosmic::Action.

```rust
// Inside keypad.rs
pub fn update(&mut self, message: KeypadMessage) -> Task<cosmic::Action<KeypadMessage>> {
    match message {
        KeypadMessage::DigitPressed(n) => {
            self.current_value.push(n);
            Task::none() // No side effects needed
        }
        KeypadMessage::Submit => {
            // Example of an async task or action
            Task::none()
        }
    }
}
```
Returns: 
Task<cosmic::Action<KeypadMessage>>.

Integration: In your main AppModel::update, you map the returned task to the global message type:

```rust
match message {
    Message::Keypad(inner) => {
        // Call the local update and map its resulting task
        self.keypad.update(inner).map(Message::Keypad)
    }
}
```
Why use cosmic::Action?

In libcosmic, Action is often used instead of just a raw Message because it provides built-in support 
for common desktop operations (like closing windows or changing themes) alongside your custom messages.

By returning Task<cosmic::Action<KeypadMessage>>, your component remains fully compatible with the main application's runtime.
