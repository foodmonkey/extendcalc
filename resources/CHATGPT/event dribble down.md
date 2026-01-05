
In the libcosmic (MVU) pattern, the top-level application loop does not "filter down" 
automatically; instead, you explicitly delegate the responsibility.

1. The Message Flow
The top-level app loop only sees exactly what the view function tells it to see.
Because you have used .map(Message::Keypad) in your view, every internal
KeypadMessage from a button press arrives at the top-level update function wrapped
as a Message::Keypad(inner_msg).

2. How AssembledKeypad Intercepts
To make AssembledKeypad "intercept" the event, you pass the message down in your main
update function. The main app acts as a dispatcher:

```rust
// main.rs
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::Keypad(inner_msg) => {
            // 1. Pass the message DOWN for the keypad to handle its internal state
            // (e.g., updating which key is currently highlighted)
            let task = self.assembled_keypad.update(inner_msg.clone());

            // 2. "Intercept" or React to specific keypad events here
            match inner_msg {
                KeypadMessage::ButtonPressed(pos) => {
                    // Look up the actual Key data from your HashMap using the GridPosition
                    if let Some(key) = self.assembled_keypad.grid.get(&pos) {
                        println!("Key handled: {}", key.name);
                        // Trigger side effects in the main app (e.g., updating a display)
                    }
                }
            }

            task.map(Message::Keypad)
        }
    }
}
```

3. The view Connection
In your AssembledKeypad::view method, when you iterate over your HashMap to
build the grid of buttons, each button is assigned its specific GridPosition:

```rust
// assembled_keypad.rs

pub fn view(&self) -> Element<KeypadMessage> {
    let mut grid = Column::new();
    // Assuming you iterate rows and columns
    for row in 0..MAX_ROWS {
        let mut row_layout = Row::new();
        for col in 0..MAX_COLS {
            let pos = GridPosition { row, col };
            row_layout = row_layout.push(
                button("Key").on_press(KeypadMessage::ButtonPressed(pos)), // Attach specific pos
            );
        }
        grid = grid.push(row_layout);
    }
    grid.into()
}
```

Key Takeaways

The Main Loop sees Message::Keypad(...): It doesn't know it was a button; it only knows the
Keypad produced a message.

Explicit Delegation: You "filter it down" by manually calling
self.assembled_keypad.update(inner_msg) inside your main update branch.

Handling vs. Bubbling:
    Handling: 
    Happens inside assembled_keypad.update()
    (e.g., logic specific to the keypad's internal data structures).

    Bubbling: 
    Happens in the main update function right after the child update call,
    where you use the keypad's message to change other parts of your application state.

Yes, you absolutely can (and should) delegate that logic.
This is a core strength of the MVU architecture: composability.

Instead of having a massive match in your main app that knows about every tiny
detail of the keypad, you simply pass the inner_msg directly to the Keypad's own
update method.

The Implementation
You define an update method on your Keypad (or AssembledKeypad) struct that matches
on its own KeypadMessage enum.

```rust
// keypad.rs
impl Keypad {
    pub fn update(&mut self, message: KeypadMessage) -> Task<KeypadMessage> {
        match message {
            KeypadMessage::ButtonPressed(pos) => {
                // HANDLE IT HERE: The sub-module logic stays in the sub-module.
                if let Some(key) = self.grid.get_mut(&pos) {
                    key.is_active = true; // Example internal state change
                    println!("Key {} pressed in Keypad module", key.name);
                }
                Task::none()
            }
        }
    }
}
```

The Main Dispatcher
In your main App loop, you just act as a "courier."
You receive the wrapped message and hand the contents to the correct component.

```rust
// main.rs
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::Keypad(inner_msg) => {
            // DELEGATION: Pass the inner message down to the child
            let keypad_task = self.assembled_keypad.update(inner_msg);

            // OPTIONAL: If you want the app to react to *certain* keypad events
            // after the keypad is done handling them, you can still check them here.

            // Map the keypad's Task back up to the App's Message type
            keypad_task.map(Message::Keypad)
        }
    }
}
```

Why this works well:

Encapsulation: 
Your main.rs doesn't need to know how the grid works or what a GridPosition is.

Clean Code: 
The "AssembledKeypad" handles its own business (reading RON files, updating internal key states).

Single Point of Truth: 
If you add a new type of button to the keypad, you only change the KeypadMessage enum and the 
Keypad::update method—you don't have to touch the main app loop's logic at all.

This pattern is often called Simple Delegation and is the standard way to scale iced and
libcosmic applications.

These articles explain how to delegate message handling to child components and structure
your "iced" application:

Element in iced - Rust - Docs.rs

Finally, our update logic is pretty straightforward: simple delegation.
impl ManyCounters { pub fn update(&mut self, message: Mess...
Docs.rs

Iced project structure best practices for multiple views : r/rust
Apr 23, 2024 — It all comes down to creating structs for each view, nesting message enums,
and calling the map function on elements r...
