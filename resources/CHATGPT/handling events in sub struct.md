
In libcosmic, the way to handle events from a child component (like your Keypad struct) and 
"bubble them up" to the main application's MVU loop is by defining the child component's view 
function to accept and utilize a message wrapper/mapper function provided by the parent.

This allows the parent to wrap the child's specific messages into its own main application messages.

Implementation Steps

Here is how you can implement this pattern:

    Define KeypadMessage in keypad.rs
    Create an enum for events specific to the keypad.

```rust
// keypad.rs

pub enum KeypadMessage {
    ButtonPressed(char),
    // other keypad-specific messages
}
```

2.Define a Wrapper Message in the Main App's messages.rs (or equivalent file)

The main application's Message enum will include an entry that contains the child
component's message type.

```rust
// messages.rs (or main app file)

pub enum Message {
    Keypad(keypad::KeypadMessage), // The wrapper
                                   // other main app messages
}
```

3.Update the Main App's update Function

In the main app's update function, you will handle the Keypad message by delegating
the logic to the keypad's own update function.

```rust
// main app file

fn update(
    &mut self,
    message: Message,
    _state: &mut cosmic::executor::AsyncState,
) -> cosmic::iced_core::Command<Message> {
    match message {
        Message::Keypad(keypad_message) => {
            // Delegate to the keypad's update logic
            self.keypad.update(keypad_message);
            // The main app can perform actions based on the keypad state after the update
            // For example, update a display field in the main model
        } // other main app message handlers
    }
    // ... returns Command<Message>
}
```

4.Update the Child Component's view Function

The crucial part is in the Keypad.rs view function. It should accept a function or closure
(on_message) that maps its internal KeypadMessage into the main application's Message type.

```rust
// keypad.rs

use cosmic::Element;
// ...

impl Keypad {
    // ... (model and update function remain similar)

    // The view function now takes a mapper closure
    pub fn view<F>(&self, on_message: F) -> Element<Message>
    where
        F: Fn(KeypadMessage) -> Message + 'static + Clone,
    {
        // Use the mapper when setting up event handlers for buttons
        widget::button::text("1")
            .on_click(on_message(KeypadMessage::ButtonPressed('1')))
            .into()
        // ... other buttons
    }
}
```

5.Use the Child's view in the Main App's view Function

In your main application's view function, you call the keypad.view and provide the mapping logic (a simple constructor call).

```rust
// main app file

fn view(&self, _state: &mut cosmic::executor::AsyncState) -> Element<Message> {
    // ...
    self.keypad.view(Message::Keypad) // Pass the Message::Keypad constructor as the mapper
    // ...
}
```

This architecture ensures that the child component manages its own internal state and UI, but all generated events are wrapped and processed within the single, top-level MVU loop of the main application.

This provides a clear, predictable flow of data and events.
