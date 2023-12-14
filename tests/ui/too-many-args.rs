#![no_main]

#[derive(derive_try_from::TryFromRef)]
#[source(SourceStruct)]
#[err(syn::Error)]
struct MyStruct {
    #[expr(1)]
    number: u32,
    #[default]
    #[expr("Hello".to_string())]
    text: String,
}