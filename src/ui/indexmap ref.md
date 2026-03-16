Since your
GridPosition uses usize (which is Copy) and your KeypadId contains a String, the Equivalent trait is the 2026 standard for high-performance lookups in an IndexMap.
By default, IndexMap::get(&K) requires you to provide a reference to the exact key type. If your key is (KeypadId, GridPosition), you would normally be forced to clone the KeypadId just to build the tuple for the lookup.
Here is how to implement the allocation-free lookup for your 2026 Rust project:
1. Define a "Lookup" Key
Create a companion struct that holds references instead of owned data. This struct lives only on the stack during the loop.

```rust
use indexmap::Equivalent;
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct GridKey {
    pub keypad_id: KeypadId,
    pub pos: GridPosition, // GridPosition(usize, usize)
}

/// This is our stack-allocated "search" key
#[derive(Hash, Eq, PartialEq)]
pub struct GridKeyRef<'a> {
    pub keypad_id: &'a KeypadId,
    pub pos: GridPosition,
}

// This trait tells IndexMap: "You can use GridKeyRef to find a GridKey"
impl<'a> Equivalent<GridKey> for GridKeyRef<'a> {
    fn equivalent(&self, key: &GridKey) -> bool {
        self.keypad_id == &key.keypad_id && self.pos == key.pos
    }
}
```
2. The Optimized Render Loop
Now you can iterate through your rows and columns without a single heap allocation.

```rust
// Assume key_grids: IndexMap<GridKey, KeyId>
for r in 0..rows {
    for c in 0..cols {
        let pos = GridPosition(r, c);
        
        // Create the reference key on the stack (zero allocation)
        let query = GridKeyRef {
            keypad_id: &current_keypad_id,
            pos,
        };

        // .get() now accepts &GridKeyRef thanks to the Equivalent trait
        if let Some(key_id) = key_grids.get(&query) {
            // lookup the SVG using the key_id
            let svg = svgs.get(key_id);
            // render(svg, r, c);
        }
    }
}
```

Why this is the best approach for 2026:

    Zero Heap Allocation: You aren't calling String::clone() 10,000 times a second inside your render loop [1].
    Pointer Equality: Because KeypadId derives Eq, the comparison inside equivalent is highly optimized.
    Clean 3NF: You maintain your "Flattened" structure while still having the convenience of a complex key [2].

Note on IndexMap: Ensure you are using the latest version of the indexmap crate (typically 2.x in 2026), as the Equivalent trait is the core mechanism for this "Heterogeneous Lookup" pattern [1].

Normally, Rust's IndexMap (and HashMap) is very strict: if your key is a (KeypadId, GridPosition), it wants a &(KeypadId, GridPosition). This is frustrating because to create that tuple, you have to own a KeypadId, which means cloning a String.
The Equivalent trait acts as a "bypass" by telling the map: "I don't have the exact key type you want, but I have this other thing that hashes the same way and compares the same way."
Why this is so powerful for your 3NF structure:

    Hashing Harmony: The GridKeyRef you create on the stack generates the exact same hash as the GridKey stored in the map.
    No Allocation: You are essentially pointing at the KeypadId that already exists in your Panels or Keypads collection. You're just "borrowing" it for a split second to check the map.
    Speed: In your rendering loop, the only thing happening is a bit of math (hashing) and a pointer comparison. No heap memory is touched.

It’s like a "Ghost Key"
Imagine the IndexMap is a locked door.

    The standard way: You have to forge a brand new physical key (Clone) every time you want to check the lock.
    The Equivalent way: You just show the door a photo of the key (GridKeyRef). The door recognizes it’s the same shape and lets you in without you ever needing to create a new physical object.

In 2026 Rust, this is the "pro" way to handle any map lookup where the key is a String or a struct containing a String.

1. Define the Lookup Struct and Trait
Add this to your module. It allows you to search your IndexMap using a &str instead of a full KeypadRef object.

```rust
use indexmap::Equivalent;

#[derive(Hash, Eq, PartialEq)]
pub struct KeypadRefLookup<'a> {
    pub id: &'a str,
}

impl<'a> Equivalent<KeypadRef> for KeypadRefLookup<'a> {
    fn equivalent(&self, key: &KeypadRef) -> bool {
        // Assuming KeypadRef has a field .id (String) or is a Newtype
        self.id == key.id.as_str() 
    }
}
```
. Update the Keypads Get Method
Change your get to be generic. This is the secret sauce: it will now accept either a &KeypadRef OR a KeypadRefLookup.

```rust
impl Keypads {
    // Q is the "Query" type. It must be "Equivalent" to our KeypadRef.
    pub fn get<Q>(&self, key: &Q) -> Option<&KeypadView> 
    where 
        Q: ?Sized + Hash + Eq + Equivalent<KeypadRef> 
    {
        self.keypad_views.get(key)
    }

    // Pro-tip: Avoid .unwrap() in production! 
    // Use .get().expect("Keypad not found") for better debugging.
}
```

3. Usage in the Render Loop
Now, if you have a raw string ID from a config file or a temporary calculation, you don't need to turn it into a KeypadRef to find your view:

```rust
let id_from_somewhere = "main_panel_keypad";

// No allocation! Just a reference-based lookup.
let lookup = KeypadRefLookup { id: id_from_somewhere };
if let Some(view) = keypads.get(&lookup) {
    // Render the view...
}
```
Why this is the "2026 Pro" version:

    The Equivalent Trait: This is the official way IndexMap handles "Heterogeneous Lookups."
    Flexibility: Your get method is now polymorphic. It works with the owned type and the reference type equally well.
    Safety: You still can't pass a random u32 or a KeyId by mistake; the types must satisfy the Equivalent contract.

Actually, you can have your cake and eat it too. In Rust,
type safety doesn't have to be a trade-off for speed if you use the Newtype Pattern for your lookup struct.
Instead of using a generic &str in your lookup (which is where the "any old string" risk comes in), you can wrap that reference in a specific "Ref" type. This ensures that even your "back door" only accepts things that are explicitly marked as a keypad reference.
The "Safe & Fast" 2026 Pattern

```rust
// 1. Your Owned Type (Stores the String)
#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct KeypadRef {
    pub id: String,
}

// 2. Your Reference Type (Stores the &str)
// This is STILL type-safe. You can't pass a raw &str where a KeypadRefPtr is expected.
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct KeypadRefPtr<'a>(pub &'a str);

// 3. The "Glue" (The Equivalent Trait)
impl<'a> indexmap::Equivalent<KeypadRef> for KeypadRefPtr<'a> {
    fn equivalent(&self, key: &KeypadRef) -> bool {
        self.0 == key.id.as_str()
    }
}
```

Why this preserves your Type Safety:

    Compiler Enforcement: If you have a function render_keypad(id: KeypadRefPtr), the compiler will reject a raw &str. You must explicitly wrap it: KeypadRefPtr("my_id").
    No "String Soup": You won't accidentally pass a KeyId string into a KeypadRef slot because they are different types, even if they both eventually point to a string.
    Zero Cost: The KeypadRefPtr is a "Transparent" wrapper. At runtime, the CPU sees it as a simple pointer. The safety check happens entirely at compile time.

Implementation in your Keypads struct:

```rust
impl Keypads {
    // We restrict the "back door" to only allow our specific Ref type
    pub fn get_by_ptr(&self, ptr: KeypadRefPtr) -> Option<&KeypadView> {
        self.keypad_views.get(&ptr)
    }
}
```
