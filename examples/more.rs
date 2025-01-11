use derive_more::{derive::Into, Add, Display, From};

#[derive(PartialEq, From, Add, Into, Display, Clone, Copy)]
struct MyInt(i32);

#[derive(PartialEq, From, Add, Display, Debug)]
enum MyEnum {
    #[display("int: {_0}")]
    Int(i32),
    Uint(u32),
    #[display("nothing")]
    Nothing,
}

fn main() {
    let my_int: MyInt = 10.into();
    let v = my_int + 20.into();
    let v1: i32 = v.into();

    println!("v1: {}", v1);

    let e: MyEnum = 10_i32.into();
    let e1: MyEnum = 20_u32.into();
    let e2: MyEnum = MyEnum::Nothing;

    println!("e: {e}, e1: {e1}, e2: {e2}");
    println!("e: {e:?}, e1: {e1:?}, e2: {e2:?}");
}
