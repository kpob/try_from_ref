#![no_main]

#[derive(derive_try_from::TryFromRef)]
#[source(SourceStruct)]
struct MyStruct {
    field: u32
}