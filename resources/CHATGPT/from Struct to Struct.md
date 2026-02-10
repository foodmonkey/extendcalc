In Rust, you implement the
From trait to define how one struct should be transformed into another. By implementing From, the Rust standard library automatically provides you with the reciprocal Into implementation for free. 
Basic Implementation Example
If you have a source struct (Source) and a target struct (Target), you implement the trait for the target type:
```Rust
struct Source {
    id: i32,
    email: String,
}

struct Target {
    user_id: i32,
    contact: String,
}

// Implement From<Source> for Target
impl From<Source> for Target {
    fn from(source: Source) -> Self {
        Target {
            user_id: source.id,
            contact: source.email,
        }
    }
}
```
How to Use It
Once implemented, you can convert between the types using two styles:

    Explicit Style (From): Use Target::from(instance) when you want it to be clear exactly what type you are creating.
    Method Style (Into): Use .into() for cleaner, chained code. You must often provide a type annotation so the compiler knows the destination type. 

```Rust
fn main() {
    let s = Source { id: 1, email: String::from("test@example.com") };

    // Option 1: Using From
    let t1 = Target::from(s);

    // Option 2: Using Into (requires type annotation)
    let s_new = Source { id: 2, email: String::from("rust@lang.org") };
    let t2: Target = s_new.into();
}
```
Key Rules

    Ownership: The from method takes ownership of the source struct (self). If you need to convert from a reference without consuming the original, implement From<&Source> instead.
    Infallibility: From is intended for conversions that cannot fail. If your conversion might result in an error (e.g., parsing a string that might be invalid), use the TryFrom trait instead, which returns a Result.
    Blanket Implementation: Always prefer implementing From over Into. The standard library has a blanket implementation that makes Into work automatically for any type that has a From implementation.
