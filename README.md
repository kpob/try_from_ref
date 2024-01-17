[![Crates.io](https://img.shields.io/crates/v/try_from_ref)](https://crates.io/crates/try_from_ref)
![License: MIT](https://img.shields.io/crates/l/try_from_ref)

### Description
This crate provides a proc macro to implement `TryFrom<&Struct>` for a target struct. It helps to convert a bigger struct to a few smaller, more specialized structs. The macro expands to a `TryFrom` implementation that tries to convert each field of the target struct from the whole source struct. If the conversion fails, the macro returns an error.

### Usage
```rust
use try_from_ref::TryFromRef;

struct Source {
    x: u32,
    y: u32,
    name: String,
    description: String,
}

#[derive(derive_try_from::TryFromRef)]
#[source(Source)]
#[err(&'static str)]
struct Target {
    sum: Sum,
    meta: Metadata,
    #[default]
    is_dirty: bool,
}
```
expands to
```rust
#[automatically_derived]
impl ::core::convert::TryFrom<&'_ Source> for Target {
    type Error = &'static str;
    fn try_from(input: &'_ Source) -> ::core::result::Result<Self, Self::Error> {
        Ok(Self {
            sum: input.try_into()?,
            meta: input.try_into()?,
            is_dirty: Default::default(),
        })
    }
}
```

At the struct level, two attributes are required:
- `#[source(SourceType)]` - the name of the source struct
- `#[err(ErrorType)]` - the error type to return if the conversion fails

At the field level, another two attributes are supported:
- `#[default]` - skips conversion, sets the default value of the type instead
- `#[expr(expression)]` - the expression to evaluate to get the value of the field

### MSRV (Minimum Supported Rust Version)
The minimum supported Rust version is 1.60.0.