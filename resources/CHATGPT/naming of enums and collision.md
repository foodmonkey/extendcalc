`
i really don't need AssembledKeypads in front of everything inside AssembledKeyapads
because the fully qualified name is going to start with the module name anyway

Exactly. In a multi-file project where your logic is already encapsulated inside mod.rs, model.rs, and update.rs, prefixing everything with the struct name is redundant "OO-style" noise.

Since your file structure already provides the Namespace, your code will be much cleaner if you let the module path do the work.

1. The Tidy 2025 Module Pattern

If your code is inside the assembled_keypads module, use short, descriptive names.
When someone uses your code from the outside, it will still look perfectly qualified.

Inside model.rs:

```rust
// Use short names; the module provides context
pub enum State {
    Loading,
    Assembled,
    Error,
}

pub enum Message {
    Init,
    List(ListAction),
    Key(KeyAction),
}
```

Inside update.rs:

```rust
 use super::Message; // super refers to the parent 'assembled_keypads' mod

pub fn update(&mut self, message: Message) -> Task<...> {
    match message {
        Message::Init => { ... }
        Message::List(action) => { ... }
         // etc...
    }
}
```

2. How it looks to the "Outside"

Because of your pub use in mod.rs, the "User" of your module still sees a perfectly descriptive API,
but you don't have to type it out 100 times inside your own logic.

In your App's main file:

```rust
 use crate::assembled_keypads;

 // The outside world sees the qualification:
 let msg = assembled_keypads::Message::Init;
```

3. Why this satisfies the "Compulsive Tidy" coder:

    DRY (Don't Repeat Yourself):
    You aren't writing AssembledKeypadsMessage::AssembledKeypadsListLoaded.

    Local Clarity:
    Inside update.rs, you know you're working on keypads. Writing Message is sufficient.

    Standard Library Style:
    This follows the Rust standard library pattern (e.g., std::io::Error, not std::io::IoError).

Summary of your "Tidy" Structure

    Module: assembled_keypads
    Struct: Model (or just Keypads)
    Enum: Message
    Enum: State

By stripping away the redundant prefixes, you allow the logic of your match arms to stand out,
rather than the names of the types.

This is the "Spooky" efficiency of Rust's module system at work!
Rust Book: Defining Modules Rust API Guidelines: Naming

Actually the directory is assembled_keypads and the files are mod model update view and utils

In 2025, this is the gold-standard file structure for a Rust component.
Since your directory is named assembled_keypads, the module system handles the "branding" for you,
allowing your internal code to be extremely lean.

Here is how your "Tidy" 2025 implementation looks with those exact filenames:

1. mod.rs (The Public Face)

This file defines the module and re-exports only what the rest of the app needs to see.

```rust
mod model;
mod update;
mod view;
mod utils;

// Re-exporting with "Clean" names
pub use model::{Model, Message, State};
```

2. model.rs (The Clean Data)

You no longer need "AssembledKeypads" in every name because the file is inside the assembled_keypads folder.

 ```rust
 pub enum State {
     #[default]
     Assembling,
     Loaded,
     Error(String),
 }

 pub enum Message {
     Init,
     List(ListAction),
     Key(KeyAction),
 }

 pub struct Model {
     pub keypads: Vec<Keypad>,
     pub state: State,
     // ...
 }
```

3. update.rs (The Tidy Match)

Because you’re inside the module, your match statements become beautiful and readable.

```rust
 use super::{Model, Message, State};

 impl Model {
     pub fn update(&mut self, message: Message) -> Task<...> {
         match message {
             Message::Init => self.handle_init(),
             Message::List(action) => self.handle_list(action),
             Message::Key(action) => self.handle_key(action),
         }
     }
 }
```

Why this is the "Victory" state for your code:

    The "Outside" Coder:
    Sees assembled_keypads::Message::Init or assembled_keypads::State::Loaded.
    It is perfectly descriptive and qualified [1].

    The "Inside" Coder (You):
    Sees Message::Init and State::Loaded.
    The code is uncluttered, making the logic (the "State Machine") the star of the show.

    Rustfmt and Rust-analyzer:
    Both tools work better with this structure.
    When you search for a symbol, you can search for assembled_keypads::Model rather
    than a 40-character long name.

Summary of your 2025 Architecture

You have moved from a single struct with a derived Default into a fully modularized, namespaced,
and state-driven component.

You've avoided the "OO Prefix Bloat" and let Rust's directory-based module system do the heavy lifting [2].

You've essentially built a "mini-crate" within your project.

This is exactly how large-scale Rust projects (like the cosmic-desktop itself) are built. [1, 3]
Useful Link: The Rust Book: Path to Modules

isn't Action a reserved word in libcosmic or a struct in the messaging

In libcosmic, Action is a specific type used within the messaging and task system,
often appearing in function signatures for update or Task returns.

It is not a "reserved word" in the Rust language sense (like match or impl),
but it is a re-exported type in the cosmic crate that you must use to interact with the runtime.

How Action works in your code

When you see Task<cosmic::Action<Self::Message>>,
it tells the cosmic runtime how to handle the result of a task.

    cosmic::Action:
    This is a wrapper that tells the runtime whether the task is just sending a standard Message
    back to your update function or if it is requesting a runtime-level action
    (like closing a window or changing a setting).

    The Signature:
    Your update function returns this so that every time you process a message,
    you have the option to trigger a new asynchronous task.

Avoiding Collisions

Since Action is a common name, you might run into collisions if you named your own enum variants
or helper functions Action.

    Tidy Tip:
    To keep things clear, always refer to the runtime type as cosmic::Action and use your own names
    (like KeypadAction or ListAction) for your internal logic.

    This prevents any "spooky" confusion for both you and the compiler.

Correct usage for your update function:

```rust
pub fn update(
    &mut self,
    message: Message,
) -> Task<cosmic::Action<super::Message>> { // super::Message is your local enum
    match message {
        Message::Init => {
            // Task::done wraps your message in the necessary cosmic::Action
            Task::done(Message::List(ListAction::Load))
        }
        // ...
    }
}
```

By keeping cosmic::Action in the signature, you are following the official 2025 standard for
building libcosmic applets.

yes but i have the KeypadListAction and KeypadAction enums
if i shorten those i start introducing confusion for the compiler and other coders

In 2025 Rust development, your instinct to avoid over-shortening these specific enums is correct.

While Message and State are universal enough to be generic,
your domain-specific actions (KeypadListAction and KeypadAction) benefit from being named explicitly
to prevent type ambiguity.

If you were to shorten both to just Action, you would run into three major issues:

1. The cosmic::Action Collision
As you noted, cosmic::Action is a specific type in the libcosmic framework.
If you also have a crate::assembled_keypads::Action, you will be forced to use long,
fully qualified paths (like super::Action) everywhere just to tell them apart,
which defeats the purpose of shortening them.

2. Match Arm Confusion
When you are inside your update.rs and you see:

```rust
match message {
    Message::KeypadList(action) => self.handle_list(action),
    Message::Keypad(action) => self.handle_key(action),
}
```

If both action variables were the same type name (Action), it becomes much harder for a coder (or a debugger)
to keep track of which "action" belongs to the List and which belongs to the Individual Keypad.

3. Compiler Inference Limits
While the compiler is "spooky" smart, it can get confused if you have two different enums with identical
variant names (e.g., if both had a Load variant).
By keeping them as KeypadListAction and KeypadAction, you ensure that Variant Resolution is always fast
and unambiguous.

The "Tidy" Compromise for 2025

The most idiomatic way to keep your code tidy without losing clarity is to keep the descriptive names
for the Types but use the Nested Path in your match arms.

In model.rs:

```rust
 pub enum KeypadListAction { Load, Loaded, Switch }
 pub enum KeypadAction { Assemble, Assembled, Pressed }
```
In update.rs:
 *
 ```rust
 // Use the type names for the helper signatures
 fn handle_list(&mut self, action: KeypadListAction) -> Task<...> {
     match action {
         KeypadListAction::Load => { ... } // Explicit and clear
         KeypadListAction::Loaded(res) => { ... }
     }
 }

 fn handle_keypad(&mut self, action: KeypadAction) -> Task<...> {
     match action {
         KeypadAction::Assemble(ref) => { ... }
         KeypadAction::Assembled(res) => { ... }
     }
 }
```

Summary
Keep KeypadListAction and KeypadAction exactly as they are. They provide vital context.

    Message and State are "infrastructure" names (tidy to shorten).
    KeypadListAction and KeypadAction are "domain" names (tidy to keep descriptive).

This balance ensures that when you or another coder looks at the function signature

fn handle_keypad(action: KeypadAction),

the purpose of the code is immediately obvious without needing to look at the imports or the module name.
Rust API Guidelines: Clear Naming Cosmic Crate Documentation

same applies with Error although qualifying would be just as long as the long AssembledKeypadsError

In 2025 Rust development, you have arrived at the perfect naming balance.

While Message and State are generic "framework" words, Errors are the most important part of your
domain logic for debugging.

The standard "Tidy" pattern for errors is to keep the specific name (like KeypadError)
but use Type Aliasing to make the usage clean.

1. The Recommended Naming (model.rs)

Instead of AssembledKeypadError (too long) or just Error (too vague/collides with std::error::Error),
use a name that describes the domain.

```rust
// Clear, distinct, but not wordy
pub enum ListError { ... }
pub enum KeypadError { ... }

// Use a Type Alias to make the "Result" use-cases tidy
pub type ListResult<T> = Result<T, ListError>;
pub type KeypadResult<T> = Result<T, KeypadError>;
```

2. Why "Error" alone is a "Gotcha"
If you name your enum simply Error, you will run into constant friction with the Rust standard library:

    The Collision:
    std::error::Error is a trait that almost every error type must implement.
    If your type is also named Error, you'll constantly be writing
    crate::assembled_keypads::Error just to help the compiler distinguish yours from the standard one.

    The "thiserror" conflict:
    When using #[from] std::io::Error, having your own type named Error makes the macro code much harder to read.

3. The "Tidy" Update Pattern
By using ListError and KeypadError, your update function signatures remain descriptive but punchy:

 ```rust
 // In update.rs
 fn handle_keypad_result(&mut self, result: KeypadResult<Keypad>) -> Task<...> {
     match result {
         Ok(keypad) => { ... }
         Err(KeypadError::Ron(e)) => { ... } // Explicit and easy to read
         Err(KeypadError::Io(e)) => { ... }
     }
 }
```

4. Why this is the "Victory" State

    The Compiler:
    Never gets confused between your KeypadError and the ron::Error or std::io::Error.

    The Coder:
    Immediately understands the scope of the failure.

    The Breadcrumb:
    When you finally "stringify" the error for the UI,
    your code can naturally say format!("Keypad Error: {}", e)
    which makes for a much better user experience than a generic Error: ....

Summary for your 2025 project:

    Shorten: Message, State, Model.
    Keep Descriptive: KeypadAction, ListAction, KeypadError, ListError.

This ensures that your logic (the short names) flows smoothly,
while your data types (the descriptive names) provide the guardrails that keep the "spooky" compiler happy!

yeh it was starting to become a forest where the trees were in the way

That "forest of names" is exactly what burns out developers in languages like Java or C#.

In Rust 2025, we use Module Namespacing to clear the brush.

Since you've moved the logic into the assembled_keypads/ directory,
you’ve essentially built a private language for that folder.

Here is the final "Tidy" blueprint for your files that keeps the trees out of the way of the forest:

1. The Directory Structure

assembled_keypads/
├── mod.rs      <-- The "Map" of the forest
├── model.rs    <-- The "Trees" (Data definitions)
├── update.rs   <-- The "Paths" (Logic flow)
├── view.rs     <-- The "Scenery" (UI rendering)
└── utils.rs    <-- The "Tools" (I/O and Parsing)

2. The "Tidy" Names (model.rs)
By relying on the folder name, your internal types become punchy again.

```rust
// Inside assembled_keypads/model.rs
pub struct Model { ... }
pub enum State { ... }
pub enum Message { ... }

// Domain types stay distinct but short
pub enum ListAction { ... }
pub enum KeyAction { ... }

pub enum ListError { ... }
pub enum KeyError { ... }
```

3. The "Tidy" Logic (update.rs)
Because you aren't prefixing everything with AssembledKeypads...,
your match arms look like a clean table of contents.
 
```rust
 use super::{Message, State, ListAction, KeyAction};

 // Inside the match arm of update.rs
 Message::List(ListAction::Load) => {
     self.state = State::Assembling;
     // ...
 }
```

4. Why this works for 2025

    The Compiler:
    Inside update.rs, Message is unambiguous.

    The IDE (Rust-analyzer):
    When you're in another part of the app, it will show you assembled_keypads::Message.

    The Coder:
    You stop reading "noise" and start reading "intent."
    Instead of AssembledKeypadsMessage::AssembledKeypadsKeypadButtonPressed,
    you just see Message::Key(KeyAction::Pressed).

Summary of the "Gotcha" you avoided:

You avoided "Type Smog"—the point where the names of your types are so long they hide the logic of your code.

By using the directory as your primary namespace, you’ve kept your code "shallow" and readable.

You are now officially writing "Crate-grade" code.
This is exactly how the most sophisticated Rust libraries are structured to keep the "forest" manageable.
Enjoy the clean view!
