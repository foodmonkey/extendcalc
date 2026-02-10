Thinking
Searching
Thinking a little longer
Exactly.
Task::perform is the standard way to bridge asynchronous code with your application's message loop. 
Here is how it works under the hood:

    Execution: You pass it a Future (your async function call) and a "mapping" closure.
    Concurrency: The iced runtime spawns that future onto its async executor so it runs without blocking the UI thread.
    Completion: Once the future resolves, the runtime takes the result, applies your mapping closure to turn it into a Message, and pushes that message into your update function. 

Example Implementation
If you have an async function fetch_data, your helper would look like this:
```Rust
// The helper returns the "recipe" for the task
fn load_data_task() -> Task<Message> {
    Task::perform(
        fetch_data(),           // 1. The async function call
        Message::DataReceived   // 2. The message raised on completion
    )
}

// In your update function
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::ClickLoad => {
            load_data_task() // The runtime starts the task here
        }
        Message::DataReceived(data) => {
            self.data = data;
            Task::none()
        }
    }
}
```
Pro-Tips

    Chaining: If you need to do something after a task finishes without a message, use methods like .then() or .map() directly on the Task.
    Batching: If your helper needs to fire off multiple things at once, you can use Task::batch(vec![task1, task2])

so a Task::perform executes an async function outside of the MVU loop - i could potentially then spawn another Task::perform from the async task that is running

Thinking
Searching
Thinking a little longer
In libcosmic (and the underlying Iced runtime), it's important to clarify
who manages the execution:

    Runtime-Led Execution: Task::perform does not "execute" the function immediately when you call it. It creates a recipe (a Task object) that you return to the libcosmic runtime. The runtime then executes that task outside the main MVU loop.
    Chaining vs. Spawning: While an async function can technically spawn other background futures (e.g., using tokio::spawn if the runtime allows), the "proper" way to trigger a second task based on the first is through monadic chaining. 

How to "Spawn" from another Task
Instead of trying to manually spawn a new Task::perform from inside your async code, you should chain them so the runtime handles the sequence:

    Chaining with .then(): If you want Task B to run automatically after Task A finishes, use the .then() method. This allows you to take the output of the first task and return a new task.
```Rust
fn chained_task() -> Task<Message> {
    Task::future(first_async_op())
        .then(|result| {
            // Return a new Task based on the result
            Task::future(second_async_op(result))
        })
        .map(Message::FinalResult)
}
```
Sequential Execution: If you just need one async function to run after another, simply await both inside a single async block. This is often simpler than managing multiple Task objects.
```Rust
Task::perform(async move {
    let data = first_op().await;
    let final_data = second_op(data).await;
    final_data
}, Message::Finished)
```
Branching via Messages: If you want the first task to finish, update the UI, and then decide if another task should start, return the second task from your update function when it receives the first task's completion message. 

Summary
You don't "spawn" a libcosmic Task from inside an async function because a Task is a runtime instruction. You either await the secondary work inside the first async function or chain the Task objects using .then() so the runtime knows what to do next.

I hear you—the "Boilerplate Tax" of nested enums in Rust can be brutal, especially when you have to wrap every single view-level event like
Message::Keypad(KeypadMessage::Digit(5)). It kills the "succinct" vibe real fast.
If you want to keep the flat structure but stay organized, use the "Group and Delegate" pattern. This gives you the logic separation of nesting without the wrapping nightmare.
The "Clean Flat" Pattern
Use the | (OR) pattern in your update function to catch the prefix, then pass the message into a dedicated logic handler.
```Rust
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        // Use a wildcard match if your keypad messages share a common prefix 
        // or just list them if the list is manageable.
        m @ Message::Key0 | m @ Message::Key1 | m @ Message::KeyEnter | m @ Message::KeyClear => {
            self.handle_keypad(m)
        }

        Message::OtherAppEvent => Task::none(),
    }
}

impl Sandbox {
    fn handle_keypad(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Key0 => self.input.push('0'),
            Message::Key1 => self.input.push('1'),
            Message::KeyClear => self.input.clear(),
            Message::KeyEnter => return Task::perform(submit(self.input.clone()), Message::Done),
            _ => unreachable!("Only keypad messages are passed here"),
        }
        Task::none()
    }
}
```
Why this is better for your sanity:

    Direct UI Wiring: Your buttons just stay Message::Key0. No nested constructors.
    Exhaustive Logic: The helper function only cares about the keypad.
    Task Handling: The helper can still return a Task (like for KeyEnter) or Task::none(), keeping the MVU flow intact.

Pro-Tip: Macro Shortcut
If your keypad has 10+ buttons and the | list in the main update is getting ugly, you can use a simple macro to define the "group":
```Rust
macro_rules! is_keypad {
    () => {
        Message::Key0 | Message::Key1 | Message::KeyEnter | Message::KeyClear
    };
}

// Then in update:
match message {
    is_keypad!() => self.handle_keypad(message),
    _ => ...
}
```
This keeps the main update loop incredibly readable while allowing you to keep everything flat.
