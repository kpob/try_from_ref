#![no_main]

struct Source;

#[derive(derive_try_from::TryFromRef)]
#[source(Source)]
#[err(syn::Error)]
struct MyStruct {
    #[source(OtherSource)]
    field: u32
}