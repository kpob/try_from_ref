#![no_main]

#[derive(derive_try_from::TryFromRef)]
#[err(syn::Error)]
struct MyStruct {
    field: u32
}