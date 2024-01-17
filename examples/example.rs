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

#[derive(derive_try_from::TryFromRef)]
#[source(Source)]
#[err(&'static str)]
struct Sum {
    #[expr(input.x + input.y)]
    pub value: u32,
}

struct Metadata {
    pub text: String,
}

// Metadata is more complex and requires custom implementation of TryFrom
impl TryFrom<&'_ Source> for Metadata {
    type Error = &'static str;

    fn try_from(value: &'_ Source) -> Result<Self, Self::Error> {
        let text = format!("{}\n {}", &value.name, &value.description);

        if text.len() > 20 {
            return Err("The metadata is too long. It should have at most 20 characters");
        }

        Ok(Self { text })
    }
}

fn main() {
    let mut source = Source {
        x: 10,
        y: 5,
        name: "My name".to_string(),
        description: "Some text".to_string(),
    };
    let target = Target::try_from(&source).unwrap();

    println!("{:?}", target.sum.value);
    println!("{:?}", target.meta.text);
    println!("{:?}", target.is_dirty);

    source.description = "Some text making the metadata too long".to_string();
    println!("{}", Target::try_from(&source).err().unwrap());
}
