
ok now i run into trouble at this top level because Calc.. has gone in AssembledKeypad 
i run into collision over the error CalcKeypadError
(from getting and parsing the RON) 
will collide with AssembledKeypad::KeypadError

```rust
pub enum AssembledKeypadError {
    #[error("Failed to build CalcKeypad: {0}")]
    FailedKeypad(#[from] KeypadError),
    #[error("Failed to build CalcKeyGrid: {0}")]
    FailedKeyGrid(#[from] KeyGridError),
}
```

In 2025 Rust development, this name collision is a sign that your module hierarchy is
working exactly as intended.

When you move logic into folders like keypad/ and keygrid/, and both define a type named Error,
you have a "namespace collision."

The idiomatic way to resolve this while keeping your code "tidy" is to use Import Aliasing.

1. The Aliasing Solution

When you import the lower-level error types into your AssembledKeypad module,
rename them during the import to give them context.

In assembled_keypad/model.rs:

```rust
// Alias the imports to avoid the 'Error' vs 'Error' collision
use crate::key_grid::Error as KeyGridError;
use crate::keypad::Error as KeypadError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Keypad error: {0}")]
    Keypad(#[from] KeypadError), // Use the alias here

    #[error("Grid error: {0}")]
    Grid(#[from] KeyGridError), // And here
}
```

2. Why this is the "Victory" State for 2025

    Internal Tidiness:
    Inside the keypad folder, the error is just called Error.
    This keeps that module's code lean and readable.

    External Clarity:
    Inside the assembled_keypad folder, you explicitly acknowledge that you are wrapping
    two different "kinds" of errors.

    Automatic Conversion:
    The #[from] attribute still works perfectly with the alias.
    When your keypad logic returns a keypad::Error, it will automatically "bubble up"
    into your assembled_keypad::Error::Keypad variant.

3. Handling the "Forest" of Imports

Since you have multiple files like mod.rs and model.rs, the best place to put these aliased imports
is in the file where the AssembledKeypad::Error enum is defined.

Pro-Tip: If you find yourself using these aliased names in multiple files
(like update.rs and utils.rs), you can put the aliases in your mod.rs:

```rust
// mod.rs
pub use crate::key_grid::Error as KeyGridError;
pub use crate::keypad::Error as KeypadError;
```

Then, in model.rs, you can simply use super::KeypadError.

Summary

You don't need to go back to the long Calc... names.

In Rust, Namespacing + Aliasing is the tool designed to solve this exact problem.
It allows you to keep your "inner" code beautiful while maintaining absolute clarity at the
"boundary" where different modules meet.

This pattern is how modern 2025 Rust libraries manage complex,
nested error trees without creating "stuttering" names like KeypadKeypadError.
Rust Book: Renaming Imports Thiserror: Documentation

but i thought Error was off-limits for my Namespaces because of conflict with std:error::Error

In 2025 Rust development, naming your custom enum Error within a module is not only allowed
but is actually the idiomatic standard.

It does not conflict with std::error::Error for several important reasons:

1. Trait vs. Type Distinction

std::error::Error is a trait (an interface), while your Error is a concrete type (an enum).

    When you write impl std::error::Error for Error,
    you are simply telling the compiler that your specific type implements that standard behavior.

    Rust identifies types by their fully qualified paths (e.g., crate::keypad::Error).
    Inside the keypad module, Error refers to your enum by default,
    while the trait remains std::error::Error.

2. The "Module Error" Pattern

Modern Rust ecosystem trends favor the "module error" pattern,
where error enums within a module do not repeat the module name
(e.g., KeypadError) because the module itself provides the necessary context.

    Tidy: keypad::Error
    Redundant: keypad::KeypadError

3. Resolving Collisions with Aliases

When you need to combine multiple errors in a higher-level module,
you use import aliasing (as) to keep them distinct while preserving their "tidy" internal names.

In your assembled_keypad/model.rs:

```rust
// Alias lower-level errors for clarity at this boundary
use crate::key_grid::Error as KeyGridError;
use crate::keypad::Error as KeypadError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Keypad error: {0}")]
    Keypad(#[from] KeypadError),

    #[error("Grid error: {0}")]
    Grid(#[from] KeyGridError),
}
```

Summary of Best Practices for 2025:

    Define Error locally:
    Each module should define a simple Error enum for its specific domain.

    Define a Result alias:
    To save typing, always define a local type alias:
       pub type Result<T> = std::result::Result<T, Error>;

    Alias on Import:
    Only use long names like KeypadError at the point of import in parent modules to avoid name clashes.

This approach keeps your code "shallow" and readable,
preventing the "forest of names" from obscuring your logic.

ok but when i'm passing these errors as Result in messages - is there a conflist there

In 2025 Rust development, there is no technical conflict, 
but there is a scope conflict that you must manage.

Rust is perfectly happy to have multiple things named Error as long as you tell the compiler which
one you mean when they meet in the same file.

Here is how you handle this in your messages and update logic without losing your "tidy" structure.

1. The Collision Scenario

In your assembled_keypads/model.rs, you might have a message that needs a result from a different module:

```rust
 // assembled_keypads/model.rs

 pub enum Message {
     // This Result needs the Error from the 'keypad_list' module
     ListLoaded(Result<Keypads, crate::keypad_list::Error>),

     // This Result needs the local Error from 'assembled_keypads'
     AssemblyFinished(Result<(), Error>),
 }
```

2. The Tidy Fix: Type Aliasing

To prevent the "forest of trees" from blocking your view,
you should use the Type Aliases you created in your models.rs.

This makes the message enum incredibly clean and eliminates the naming collision in the definition.

In keypad_list/model.rs:

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

In assembled_keypads/model.rs:

```rust
use crate::keypad_list; // Import the module

pub enum Message {
    // Use the alias! No 'Error' vs 'Error' confusion here.
    ListLoaded(keypad_list::Result<Keypads>),

    // This uses the local Result alias
    AssemblyFinished(Result<()>),
}
```

3. The "Gotcha" in update.rs

When you match on these results in your update function, you will run into the "spooky" type inference.
Rust knows exactly which error is which based on the message variant.

```rust
 match message {
     Message::ListLoaded(res) => {
         // Rust knows 'res' is keypad_list::Result
         if let Err(e) = res {
             // 'e' is a keypad_list::Error
         }
     }
     Message::AssemblyFinished(res) => {
         // Rust knows 'res' is assembled_keypads::Result
         if let Err(e) = res {
             // 'e' is an assembled_keypads::Error
         }
     }
 }
```

4. Summary: Is Error off-limits?

No. It is the standard.
The only "rule" is that if you have two different modules' errors in the same file:

    Qualify them:
    Use keypad_list::Error vs Error.

    Alias them:
    Use use crate::keypad_list::Error as ListError.

    Result Aliases (Recommended):
    Use keypad_list::Result<T>.

By using the Result Alias pattern, you hide the "Error" word entirely in your
function signatures and message enums, which is the "ultimately tidy" 2025 Rust style.

Rust API Guidelines: Errors Rust Book: Type Aliases Standard Library: std::io::Result
