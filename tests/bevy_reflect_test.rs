use bevy_reflect::{Reflect, Struct, DynamicStruct, FromReflect};

#[derive(Reflect, FromReflect, Default, Debug, PartialEq, Eq)]
struct Foo {
    str: String,
    num: i32,
    b: bool,
}

#[test]
fn main() {
    let mut foo_1 = Foo::default();
    foo_1.field_mut("str").unwrap().apply(&"Hello World".to_string());
    foo_1.field_mut("num").unwrap().apply(&100);
    foo_1.field_mut("b").unwrap().apply(&true);
    println!("{:?}", foo_1);

    let mut dynamic_struct = DynamicStruct::default();
    dynamic_struct.insert("str", "Hello World".to_string());
    dynamic_struct.insert("num", 100);
    dynamic_struct.insert("b", true);
    let mut foo_2 = Foo::default();
    foo_2.apply(&dynamic_struct);
    assert_eq!(foo_1, foo_2);

    let foo = Foo::default();
    let value = foo.clone_value();
    let _ = Foo::from_reflect(value.as_ref()).unwrap();
}