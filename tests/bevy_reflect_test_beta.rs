#![feature(trivial_bounds)]
use std::{any::Any, ptr::NonNull};
use bevy_reflect::{reflect_trait, Reflect, TypeRegistry, FromReflect};

#[reflect_trait]
trait DoA {
    fn do_a(&self);
}

#[reflect_trait]
trait DoB {
    fn do_b(&self);
}

#[reflect_trait]
trait DoC {
    fn do_c(&self);
}

#[derive(Reflect, FromReflect)]
struct Bazz {
    do_c: Box<dyn DoC>,
    p: Option<NonNull<dyn DoC>>,
    non: NonRe,
}

#[derive(Reflect)]
struct NonRe {

}

#[derive(Reflect)]
struct Foo;
impl DoA for Foo {
    fn do_a(&self) {
        println!("Foo do a")
    }
}
impl DoB for Foo {
    fn do_b(&self) {
        println!("Foo do b")
    }
}

#[derive(Reflect)]
struct Bar;
impl DoB for Bar {
    fn do_b(&self) {
        println!("Bar do b")
    }
}
#[test]
fn main() {
    let mut type_registry = TypeRegistry::default();
    type_registry.register::<Foo>();
    type_registry.register_type_data::<Foo, ReflectDoA>();
    type_registry.register_type_data::<Foo, ReflectDoB>();
    type_registry.register::<Bar>();
    type_registry.register_type_data::<Bar, ReflectDoB>();
    type_registry.register::<NonRe>();

    let list: Vec<Box<dyn Reflect>> = vec![Box::new(Foo), Box::new(Bar)];

    for item in list.iter() {
        let item_ref = item.as_ref();
        if let Some(reflect) = type_registry.get_type_data::<ReflectDoA>(item_ref.type_id()) {
            (reflect.get_func)(item_ref).unwrap().do_a()
        }
        if let Some(reflect) = type_registry.get_type_data::<ReflectDoB>(item_ref.type_id()) {
            (reflect.get_func)(item_ref).unwrap().do_b()
        }
    }
}

fn _type_cast<T: DoA + Any + Reflect>(type_registry: &TypeRegistry, obj: &T) {
    if let Some(reflect) = type_registry.get_type_data::<ReflectDoB>(obj.type_id()) {
        (reflect.get_func)(obj).unwrap().do_b()
    }
}
