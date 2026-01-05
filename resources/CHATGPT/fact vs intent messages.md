
ok so thinking about it i'm going with
Message::
    ::Init
    ::KeypadList
        ::Load
        ::Loaded
        ::SwitchToKeypad
    ::Keypad
        ::Assemble
        ::Assembled
        ::KeyPressed

my question is this the ::Loaded and ::Assembled messages feel more like a state 
than a DoSomething message

In 2025 Rust UI development, you have identified one of the most profound "aha!" moments in the MVU
(Model-View-Update) pattern:

Messages are not just commands; they are "Announcements of Fact."

You are correct that Loaded and Assembled feel like states,
but in a message-driven system, they act as the Events that trigger a state transition.

1. The "Fact vs. Intent" Pattern

To satisfy your tidy code requirements, think of your messages like this:

    Intent Messages (Load, Assemble):
    These are "commands" usually sent by the UI or the system to start a process.

    Fact Messages (Loaded, Assembled):
    These are "events" sent by the background tasks to say, "I am finished, here is the data."

2. Why they must be Messages (and not just States)

In an asynchronous system, the background task (the one reading the RON file)
cannot reach into your AppModel and change self.state = Loaded.
Rust's ownership rules strictly forbid this.

Instead, the task produces a message containing the result.
The update function is the only place allowed to hear that message and officially change the Structure::State.

3. Tidy Code: Grouping by "Domain"

Your proposed structure is excellent.
Here is how that looks in the code to keep the "Fact vs. Intent" clear:

```rust
 pub enum KeypadListAction {
     Load(AssembledKeypadSet, PathBuf), // Intent: "Please start loading"
     Loaded(CalcKeypadsResult),         // Fact: "The list is now ready"
     SwitchTo(String),                  // Intent: "Change the view"
 }

 pub enum KeypadAction {
     Assemble(CalcKeypadRef),           // Intent: "Build this specific keypad"
     Assembled(AssembledKeypadResult),  // Fact: "This keypad is ready"
     ButtonPressed(String),             // Fact: "User interacted with this"
 }
```

4. The "State Machine" Flow

Here is the sequence of how these "Fact" messages drive your Structure::State:

    UI issues KeypadList(Load).
        Update sets state = LoadingList.

        Background Task sends KeypadList(Loaded(result)).
        Update receives this "Fact," checks the result, and if Ok, sets state = Assembling.

        Background Task sends Keypad(Assembled(result)).
        Update receives this "Fact," pushes it to the Vec, and if it's the last one, sets state = Loaded.

5. Why this feels "Right" for 2025

    The View is the State:
    Your view function looks at Structure::State.
    It doesn't care how the data got there.

    The Update is the Transition:
    Your update function only cares about the Messages.
    It translates "Facts" from the background into "States" for the View.

Summary:

Do not feel bad about Loaded and Assembled being messages.
They are the "handshakes" that allow your background logic to safely talk to your UI.
Without them, your update function wouldn't know when to move the state machine from Assembling to Loaded.

Rust Design Patterns: Type State Iced: Architecture Overview Elm: How to structure Messages

ok but how does the update know that this is the last
Keypad::Assembled message?

given that they're all running async and no order of completion can be guaranteed

In 2025 Rust UI development, the most robust way to handle this is a "Count-Based Completion" check.

Since you already loaded the "manifest" (CalcKeypads),
your AppModel knows exactly how many keypads should exist.

As each async Keypad::Assembled message arrives, you simply compare your current progress against that total.

1. The Strategy

    Stage 1 (KeypadList::Loaded):
    You save the total count of keypads expected from the RON manifest into your state.

    Stage 2 (Keypad::Assembled):
    Every time a single keypad finishes (in any order), you push it into your Vec and check the length.

2. The Implementation Pattern

Here is how that looks in your update.rs logic:

```rust
fn handle_keypad_action(&mut self, action: KeypadAction) -> Task<Message> {
    match action {
        KeypadAction::Assembled(result) => {
            match result {
                Ok(keypad) => {
                    // 1. Add the successfully assembled keypad to your collection
                    self.keypads.push(keypad);
                }
                Err(e) => {
                    // 2. Even on error, you might want to track that one failed
                    self.errors.push(format!("Keypad failed: {}", e));
                }
            }

            // 3. THE COMPLETION CHECK
            // We compare the number of results (success + fail) against the manifest
            let total_processed = self.keypads.len() + self.errors.len();
            let total_expected = self.keypads_list.keypads.len();

            if total_processed == total_expected {
                // If there were no errors, we are fully Loaded
                if self.errors.is_empty() {
                    self.keypads_state = AssembledKeypadState::Loaded;
                } else {
                    self.keypads_state = AssembledKeypadState::Error;
                }
            }

            Task::none()
        }
        // ... other arms
    }
}
```

Why this is the "Best Practice" for 2025:

    Order Independence:
    It doesn't matter if Keypad #50 finishes before Keypad #1.
    The len() of the Vec is a reliable counter regardless of completion order.

    Parallelism-Safe:
    Because the update function is the only place where the Vec is modified (the "Single Consumer" rule),
    you don't have to worry about "race conditions" where two tasks try to increment a counter at once.
    Rust handles this naturally via the message queue.

    Progress Tracking:
    This also gives you the data needed for a progress bar in your View.
    You can show (self.keypads.len() / self.keypads_list.keypads.len()) to the user while they wait.

    Error Awareness:
    By checking self.errors.len(), you ensure the app doesn't stay stuck in an "Assembling" state
    forever if one file is missing or corrupted.

Coder Tip: Identifying the "Finished" State

If you have a very large library and want to be extremely "tidy,"
you can add a pending_count field to your struct.

    Set self.pending_count = list.len() when the list loads.

    Decrement self.pending_count -= 1 on every Assembled message.

    When pending_count == 0, you are done.

However, simply checking self.keypads.len() + self.errors.len() is usually cleaner as it relies on the
data you already have.

ok - i thought there might be some internal counter on the dispatcher (Task::Batch)
that could be interrogated to see if this was the last completed async task

In 2025 Rust UI development, the answer is a firm no:
there is no way to interrogate a Task::Batch for its remaining count.

This is a intentional design choice in the MVU (Model-View-Update) pattern for three key reasons:

1. The "Fire and Forget" Dispatcher
When you call Task::batch, the dispatcher doesn't hold onto those tasks as a single group.
It unrolls them and puts them onto the runtime's execution queue.
From the dispatcher's perspective, once the batch is issued, its job is over.
The runtime just sees a stream of individual, independent tasks.

2. State Belonging
In Rust, the Dispatcher is responsible for doing work, but your Model is responsible for tracking work.
If the dispatcher kept track of the "batch status,"
you would have two different places trying to manage the state of your application.
By forcing the completion logic into your update function, Rust ensures there is only one "Source of Truth."

3. Error Resilience
A Task::Batch doesn't know what "success" looks like for your specific app.

    If 10 tasks are in a batch and 2 fail, the dispatcher just sees 10 completed events.
    Your update function, however, needs to know which ones failed to build that
    "breadcrumb" error string you designed.

The Idiomatic 2025 Way
Since you want tidy code, you can make the counter feel "internal" by putting it in your
AssembledKeypads struct. This is the standard pattern:

```rust
pub struct AssembledKeypads {
    // ... other fields
    pub keypads: Vec<AssembledKeypad>,
    pub expected_count: usize, // Set this when the List loads
}

// In update.rs
KeypadAction::Assembled(result) => {
    // 1. Process result (push to Vec or Error list)
    // 2. Check against the "internal" counter
    if self.keypads.len() + self.errors.len() == self.expected_count {
        self.keypads_state = AssembledKeypadState::Loaded;
    }
    Task::none()
}
```

Summary of the "Mental Shift"

    OO Mindset:
    "I'll ask the TaskManager if it's finished with the batch."

    Rust/MVU Mindset:
    "The TaskManager will tell me every time a single piece of work is done.
    I will count those notifications until I'm satisfied."

This keeps your update function as the absolute authority on the state of your application,
which is what makes Rust UIs so much easier to debug than OO ones.
Iced: Task Batching Elm: Command Batches
