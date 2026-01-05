
this requires some thinking about 

- you have to plan your error states rather than just reacting to them, 
is there any way to do logic operations on Messages (and or not etc) 

- i can see how if you planned your error states with a number of error types in the Model 
- you could perform the logic compare at that point in the Update part of the cycle 
- does that make sense

In libcosmic and the MVU architecture, you do not perform logical operations (AND, OR, NOT)
on the Message objects themselves.

Instead, you perform those operations on your Model’s state inside the update function.

1. Planning States Instead of "Reacting"
You are exactly right: you must plan your states. In MVU, a message like ButtonPressed is just a signal.
The logic of "Can I press this button right now?" belongs in the update function,
where you compare the incoming message against the current state of your Model.

2. Logical Composition in the update Loop
Since Messages in Rust are Enums, you handle them using pattern matching combined with guards.
This allows you to implement complex logic like
"Only process this key if we are NOT in an error state AND the keypad is active."

```rust
// Inside AssembledKeypad::update
pub fn update(&mut self, message: KeypadMessage) -> Task<KeypadMessage> {
    match message {
        // Logic: Only allow clearing the error if an error exists
        KeypadMessage::ClearError if self.error_state.is_some() => {
            self.error_state = None;
            Task::none()
        }

        // Logic: Ignore button presses IF in error mode OR IF keypad is locked
        KeypadMessage::ButtonPressed(_) if self.error_state.is_some() || self.is_locked => {
            // Effectively a "NOT" or "AND" logic gate
            Task::none()
        }

        // Normal path
        KeypadMessage::ButtonPressed(pos) => {
            self.process_key(pos);
            Task::none()
        }

        _ => Task::none(),
    }
}
```

3. State-Based Logic (The "Curly" Part)

You can use the Model to track complex combinations of error types.
For example, if you have multiple independent error flags:

```rust
struct KeypadState {
    has_ron_error: bool,
    has_math_error: bool,
}

// Logic check: "Is the UI blocked?"
// (has_ron_error OR has_math_error)
if self.state.has_ron_error || self.state.has_math_error {
    // block interaction
}
```

4. Chaining Messages with Task

If you need a "Logical Sequence" of events (e.g., "Validate, then Save, then Clear"),
you use Task::chain or Task::batch.

    Task::batch: 
    Runs multiple tasks (logical "AND" - do all these things at once).

    Task::chain: 
    Runs tasks one after another (logical "THEN").

Summary of the Paradigm Shift

    Messages are "Simple Events" (They carry no logic).

    Update is "Logic & Validation" (Where AND/OR/NOT happens using Rust code).

    Model is "Memory" (Where the result of that logic is stored).

    View is "Reflection" (Where the state is checked to enable/disable widgets).

By performing the logic at the Update stage, you ensure that your Model remains the single
source of truth, and your UI (the View) simply reacts to the finalized result of
those logical comparisons.
