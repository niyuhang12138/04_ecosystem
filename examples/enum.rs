use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::{
    Display, EnumCount, EnumIs, EnumIter, EnumMessage, EnumProperty, EnumString, IntoEnumIterator,
    IntoStaticStr, VariantNames,
};

#[derive(
    Display,
    EnumIter,
    EnumMessage,
    EnumProperty,
    EnumString,
    IntoStaticStr,
    VariantNames,
    EnumIs,
    EnumCount,
)]
enum MyEnum {
    A,
    B(String),
    C,
}

#[allow(unused)]
#[derive(Display, Debug, Serialize, Deserialize)]
enum Color {
    #[strum(serialize = "redred", to_string = "red")]
    Red,
    Green {
        range: usize,
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    },
}

fn main() -> Result<()> {
    println!("{:?}", MyEnum::VARIANTS);
    MyEnum::iter().for_each(|v| println!("{v}"));
    println!("total: {}", MyEnum::COUNT);
    let my_enum = MyEnum::B("hello".to_string());
    println!("{}", my_enum.is_b());
    let s: &'static str = my_enum.into();
    println!("{s}");

    let red = Color::Red;
    let green = Color::Green { range: 10 };
    let blue = Color::Blue(10);
    let yellow = Color::Yellow;
    let purple = Color::Purple { sat: 10 };
    println!("{red}, {green}, {blue}, {yellow}, {purple}");

    let color_json = serde_json::to_string(&red)?;
    println!("{color_json}");
    Ok(())
}
