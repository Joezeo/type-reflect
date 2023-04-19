use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

trait DoA {
    fn do_a(&self);
}

trait DoB {
    fn do_b(&self);
}

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

struct Bar;
impl DoB for Bar {
    fn do_b(&self) {
        println!("Bar do b")
    }
}

fn get_cast_do_a_fn<T: DoA + Any>() -> fn(&dyn Any) -> &dyn DoA {
    |obj: &dyn Any| obj.downcast_ref::<T>().unwrap()
}

fn get_cast_do_b_fn<T: DoB + Any>() -> fn(&dyn Any) -> &dyn DoB {
    |obj: &dyn Any| obj.downcast_ref::<T>().unwrap()
}

#[derive(Default)]
struct TypeRegistry {
    do_a_registry: HashMap<TypeId, fn(&dyn Any) -> &dyn DoA>,
    do_b_registry: HashMap<TypeId, fn(&dyn Any) -> &dyn DoB>,
}

impl TypeRegistry {
    fn register_do_a<T: DoA + Any>(&mut self) {
        self.do_a_registry
            .insert(TypeId::of::<T>(), get_cast_do_a_fn::<T>());
    }

    fn register_do_b<T: DoB + Any>(&mut self) {
        self.do_b_registry
            .insert(TypeId::of::<T>(), get_cast_do_b_fn::<T>());
    }
}

#[test]
fn main() {
    let mut type_registry = TypeRegistry::default();
    type_registry.register_do_a::<Foo>();
    type_registry.register_do_b::<Foo>();
    type_registry.register_do_b::<Bar>();

    let vec: Vec<Box<dyn Any>> = vec![Box::new(Foo {}), Box::new(Bar {})];
    let iter = vec.into_iter();
    for item in iter {
        if let Some(do_a_cast) = type_registry.do_a_registry.get(&item.as_ref().type_id()) {
            do_a_cast(item.as_ref()).do_a();
        }

        if let Some(do_b_cast) = type_registry.do_b_registry.get(&item.as_ref().type_id()) {
            do_b_cast(item.as_ref()).do_b();
        }
    }
}
