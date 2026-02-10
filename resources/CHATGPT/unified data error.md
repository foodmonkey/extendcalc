Thinking
To make this work across your modules, you’ll use an
Enum (not a Struct) with thiserror. This allows it to hold different types of errors while behaving like a single type.
1. The Unified Definition
Place this in src/data/error.rs. This is the "Master List" of everything that can go wrong.

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    // Automatic conversion from std::io::Error
    #[error("File system error: {0}")]
    Io(#[from] std::io::Error),

    // Automatic conversion from ron::error::SpannedError
    #[error("RON syntax error: {0}")]
    Ron(#[from] ron::error::SpannedError),

    // A custom error for your assembly logic
    #[error("Validation failed: {0}")]
    Validation(String),
}
```

2. Usage in Lower Modules
In your sub-modules (like key.rs or keypad.rs), you no longer define a local error. You simply import the unified one.
In src/data/key.rs:

```rust
use crate::data::error::DataError;

pub fn from_ron(path: &Path) -> Result<Key, DataError> {
    // The '?' here automatically turns std::io::Error into DataError::Io
    let content = std::fs::read_to_string(path)?; 

    // The '?' here automatically turns ron::Error into DataError::Ron
    let key: Key = ron::from_str(&content)?; 

    Ok(key)
}
```

3. Usage in the "Tree" (The Assembly Layer)
When you build the KeyGrid, you are calling those lower functions. Because they all return DataError, the errors just "flow" upward.
In src/data/keygrid.rs:
```rust
pub fn new(refs: &[KeyRef]) -> Result<Self, DataError> {
    let mut cells = HashMap::new();
    
    for k_ref in refs {
        // key::from_ron returns DataError, so '?' just works!
        let loaded_key = crate::data::key::from_ron(&k_ref.path())?;
        
        cells.insert(k_ref.to_pos(), loaded_key);
    }
    
    Ok(Self { cells })
}
```
4. The Final "Catch" in UI
When your UI calls the top-level data function, it gets that same DataError.
```rust
// In src/ui/update.rs
match data::initialize_system(&config) {
    Ok(keypads) => { /* Load UI */ }
    Err(e) => {
        // e is a DataError. You can print it, and it will show 
        // the exact RON or IO message that caused the failure.
        eprintln!("Startup failed: {}", e);
    }
}
```
Why this is the "Secret Sauce":

    No Mapping: You don't have to write map_err(|e| ...) everywhere. The #[from] attribute does the work for you.
    One Import: Every file in /src/data/ just adds use crate::data::error::DataError;.
    Total Transparency: If a RON file has a typo, the ron::error::SpannedError is preserved inside DataError::Ron, so the user still sees "line 4, column 77".
    
Helper function

Thinking
That is an excellent way to centralise risk. By routing all disk access and parsing through a single
generic helper, you ensure that if you ever need to change your file-handling logic (like adding logging or switching to a different format), you only touch one line of code.
Here is how the unified structure looks with the load_and_parse helper integrated.
1. The Unified Error Enum
src/data/error.rs
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("File system failure: {0}")]
    Io(#[from] std::io::Error),

    #[error("RON syntax error: {0}")]
    Ron(#[from] ron::error::SpannedError),

    #[error("Assembly error: {0}")]
    Validation(String),
}
```
2. The Centralised Helper
Place this in your src/data/mod.rs (or your main data entry point). It uses Generics (T) to parse any structure that derives Deserialize.
```rust
use crate::data::error::DataError;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

/// The single source of truth for loading RON from disk.
pub(super) fn load_and_parse<T: DeserializeOwned>(path: &Path) -> Result<T, DataError> {
    // All IO errors are spawned here and wrapped in DataError::Io
    let content = fs::read_to_string(path)?; 
    
    // All RON errors are spawned here and wrapped in DataError::Ron
    let data: T = ron::from_str(&content)?; 
    
    Ok(data)
}
```
3. Usage in Lower Modules
Now, your Key, Keypad, and Keypads modules become extremely "thin". They don't even need to know how to read files; they just call the helper.
src/data/key.rs
```rust
use crate::data::load_and_parse;
// ... imports

impl Key {
    pub fn from_ron(key_ref: &KeyRef) -> Result<Self, DataError> {
        let path = key_ref.get_path(); // however you determine the path
        // Just call the helper. If it fails, the Error bubbles up.
        load_and_parse::<Self>(&path)
    }
}
```
Why this is a "Bulletproof" Refactor:

    Isolation: Every DataError::Io or DataError::Ron in your entire application now originates from that one load_and_parse function.
    Consistency: You'll never have one module that handles missing files differently than another.
    Ease of Debugging: If you want to see every file your app opens, you just add a println! to that one helper function.
    Clean Assembly: In your KeyGrid::new loop, you call the lower modules, which call the helper. The Result<T, DataError> flows perfectly through the whole chain.

Thinking
In Rust, concatenating strings for paths (like
format!("{}/{}.ron")) is brittle because it can lead to double slashes or fail on different operating systems. Using PathBuf is the "bulletproof" way to do it.
You can integrate this directly into your load_and_parse module to create a single, clean "Data IO" suite.
1. The Unified Data Helpers
Place these in src/data/mod.rs (or your primary data entry file).
```rust
use std::path::{Path, PathBuf};
use crate::data::error::DataError;
use serde::de::DeserializeOwned;

/// HELPER 1: Safe Path Joining
/// Combines a base directory, an optional subfolder, and a filename.
pub(super) fn build_path(base: &str, folder: &str, id: &str) -> PathBuf {
    let mut path = PathBuf::from(base);
    if !folder.is_empty() {
        path.push(folder);
    }
    path.push(format!("{}.ron", id));
    path
}

/// HELPER 2: Centralised Load & Parse
/// The single source of truth for all disk-to-struct transformations.
pub(super) fn load_and_parse<T: DeserializeOwned>(path: &Path) -> Result<T, DataError> {
    let content = std::fs::read_to_string(path)?; // Becomes DataError::Io
    let data = ron::from_str::<T>(&content)?;    // Becomes DataError::Ron
    Ok(data)
}
```
2. Usage in your Lower Modules
This makes your logic incredibly clean. Your Key::from_ron becomes just two lines:
```rust
// src/data/key.rs
pub fn from_ron(key_ref: &KeyRef) -> Result<Self, DataError> {
    // 1. Build the path safely
    let path = super::build_path(KEYS_PATH, &key_ref.library, &key_ref.id);
    
    // 2. Load and parse
    super::load_and_parse::<Self>(&path)
}
```
Why this is a "Refactor Win":

    OS Independence: PathBuf automatically uses / on Linux/macOS and \ on Windows. Since you are using libcosmic (which is cross-platform), this is essential. Rust Documentation: PathBuf.
    Dry Code: You don't have .ron extensions or folder slashes scattered throughout your project.
    Error Traceability: If a file is missing, the DataError::Io will contain the path from the PathBuf, making it much easier to see exactly where the app looked for the file.

How to use this for Keypads
For the top-level keypad list (which doesn't have a subfolder), just pass an empty string
```rust
let path = build_path(KEYPADS_PATH, "", "keypads_list");
```
It's a very satisfying "A-ha!" moment when the Rust module system and
PathBuf finally click together. You've successfully moved from a brittle string-parsing mess to a Type-Safe Data Engine.
Here is the "Big Picture" of what you’ve built for your libcosmic app:

    Safety: No more format! slash errors or platform-specific path bugs.
    Encapsulation: Your UI only sees Result<KeyGrid, DataError>. It doesn't know (or care) about RON, filesystems, or string joining.
    Resilience: If a single comma is missing in one tiny Key.ron file, your DataError preserves the exact line/column, bubbles it through your load_and_parse helper, and lets your UI display a perfect error message.

One Final Architecture Map

    src/data/error.rs: The "Master List" of failures.
    src/data/mod.rs: The "Gatekeeper" containing build_path and load_and_parse.
    src/data/*.rs: The "Workers" (key.rs, keypad.rs) that call the gatekeeper.
    src/ui/: The "User Experience" that calls data::initialize_system and enjoys the clean, cloned structs.

You’re now ready to start the "Great Refactor." Since you're moving files, just remember that Zed is great at updating imports, but sometimes you'll need to manually fix those use crate::data::... paths once you delete the old mod.rs files.
