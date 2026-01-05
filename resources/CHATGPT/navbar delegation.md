
This is a common concern when moving from object-oriented programming to the
Model-View-Update (MVU) architecture. In Rust and libcosmic, your state changes are
actually more traceable because you are removing "hidden" side effects and replacing
them with an explicit message trail.

To keep your code readable and maintain your "mental map" of who is changing what,
use these two idiomatic strategies for 2025:

1. Delegation: The "Pass-Through" Pattern
Instead of the App changing the child's state directly, the App should send a message
to the child. This keeps the "logic of change" inside the child's code.

In your App's update:

```rust
// App's update function
fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
    match message {
        // App handles the Nav Bar click
        AppMessage::NavSelectionChanged(new_id) => {
            // INSTEAD of: self.aks.active_id = new_id;
            // DO THIS: Delegate the message to the child
            self.aks
                .update(AssembledKeypadsMessage::SwitchToKeypad(new_id))
                .map(AppMessage::AKSMessage)
        }
    }
}
```

In your AKS (AssembledKeypads) update:

```rust
// AKS's update function (The source of truth for its own changes)
pub fn update(&mut self, message: AssembledKeypadsMessage) -> Task<AssembledKeypadsMessage> {
    match message {
        AssembledKeypadsMessage::SwitchToKeypad(id) => {
            // YOU CAN SEE THE CHANGE HERE
            println!("Keypad switched to: {}", id);
            self.active_keypad_id = id;
            Task::none()
        }
    }
}
```

Why this helps: You can now search your project for SwitchToKeypad, and you will see exactly
one place where the data is mutated—inside the AKS module.

2. The "Single Source of Truth" in the View
In libcosmic, widgets don't "hold" state; they "reflect" state. You don't tell a nav bar to
activate(id). You build a nav bar and say, "Your current active item is whatever is in my model."

In your App's view:

```rust
fn view(&self) -> Element<AppMessage> {
    let navigation = nav_bar(
        &self.aks.keypads_list,
        // The nav bar just "watches" the child's state
        Some(self.aks.active_keypad_id),
    )
    .on_select(AppMessage::NavSelectionChanged);

    column![navigation, self.aks.view().map(AppMessage::AKSMessage)].into()
}
```

Traceability Checklist
If you're worried about losing track of state changes:

    Don't use public fields: Keep active_keypad_id private (or pub(crate)) and only change
    it inside the update function.

    Use Explicit Messages: Use descriptive message names like SwitchToKeypad rather than
    generic names like SetId.

    Searchability: In Rust, you can search for self.active_keypad_id =. In a well-structured MVU app,
    that assignment will only appear once in the entire codebase,
    right inside the child's update function.

Summary: The App owns the nav_bar (the trigger), but the AKS owns the active_id (the state).
By sending a message from the App to the AKS to perform the switch,
you keep the logic encapsulated and perfectly traceable.


Why the Delegation Approach is "Clean OO" for Rust
In a traditional OO language, you would make active_keypad_id private and provide a
switch_to(id) method. In the 2025 libcosmic/MVU pattern:

    The AssembledKeypadsMessage::SwitchToKeypad(id) is your "Method Call."
    The AKS::update function is your "Method Body."

This preserves the Encapsulation you're looking for. If you ever need to add logic—like
"don't switch if a calculation is running" or "log the switch to a file"—you only have
to change it in one place: the AKS::update function.

A Quick Implementation Pattern
To keep it perfectly clean, your App module doesn't even need to know how AKS handles
the switch. It just passes the message along:

```rust
// Inside App::update
match message {
    AppMessage::NavClicked(id) => {
        // You are "sending a command" to the child logic
        let aks_task = self.aks.update(AssembledKeypadsMessage::SwitchToKeypad(id));

        // Return the task so the child can perform its own async work if needed
        aks_task.map(AppMessage::AKSMessage)
    }
    // ...
}
```

The "Traceable" Result
By 2025 standards, this is the most robust way to build a COSMIC app. You get:

    Searchability: Searching for SwitchToKeypad takes you straight to the logic.
    Immutability: Your view remains a "dumb" reflection of the data.
    Decoupling: The App doesn't need to know that AKS has a field named active_keypad_id.
    It only needs to know that AKS accepts a SwitchToKeypad message.

You’ve successfully bridged the gap between OO design principles and Functional state management!
COSMIC's documentation encourages this exact separation to keep the desktop environment
responsive and bug-free.
