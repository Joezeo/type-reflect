#![allow(dead_code)]
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
struct ReflectDoA {
    get_func: fn(&dyn Any) -> &dyn DoA,
    get_mut_func: fn(&mut dyn Any) -> &mut dyn DoA,
    get_boxed_func: fn(Box<dyn Any>) -> Box<dyn DoA>,
}

struct ReflectDoB {
    get_func: fn(&dyn Any) -> &dyn DoB,
    get_mut_func: fn(&mut dyn Any) -> &mut dyn DoB,
    get_boxed_func: fn(Box<dyn Any>) -> Box<dyn DoB>,
}

trait FromType<T> {
    fn from_type() -> Self;
}

impl<T: Any + DoA> FromType<T> for ReflectDoA {
    fn from_type() -> Self {
        Self {
            get_func: |obj| obj.downcast_ref::<T>().unwrap(),
            get_mut_func: |obj| obj.downcast_mut::<T>().unwrap(),
            get_boxed_func: |obj| obj.downcast::<T>().unwrap(),
        }
    }
}

impl<T: Any + DoB> FromType<T> for ReflectDoB {
    fn from_type() -> Self {
        Self {
            get_func: |obj| obj.downcast_ref::<T>().unwrap(),
            get_mut_func: |obj| obj.downcast_mut::<T>().unwrap(),
            get_boxed_func: |obj| obj.downcast::<T>().unwrap(),
        }
    }
}

struct TypeRegistration {
    data: HashMap<TypeId, Box<dyn Any>>,
}

#[derive(Default)]
struct TypeRegistry {
    registers: HashMap<TypeId, TypeRegistration>,
}
impl TypeRegistry {
    fn register<Type: Any, ReflectTrait: FromType<Type> + Any>(&mut self) {
        self.registers
            .entry(TypeId::of::<Type>())
            .or_insert(TypeRegistration {
                data: Default::default(),
            })
            .data
            .insert(
                TypeId::of::<ReflectTrait>(),
                Box::new(ReflectTrait::from_type()),
            );
    }

    fn get_type_data<T: Any>(&self, type_id: TypeId) -> Option<&T> {
        self.registers.get(&type_id).and_then(|registration| {
            registration
                .data
                .get(&TypeId::of::<T>())
                .and_then(|reflect| reflect.downcast_ref::<T>())
        })
    }
}

#[test]
fn main() {
    let mut registry = TypeRegistry::default();
    registry.register::<Foo, ReflectDoA>();
    registry.register::<Foo, ReflectDoB>();
    registry.register::<Bar, ReflectDoB>();

    let list: Vec<Box<dyn Any>> = vec![Box::new(Foo {}), Box::new(Bar {})];
    for item in list.iter() {
        let item_ref = item.as_ref();
        if let Some(reflect) = registry.get_type_data::<ReflectDoA>(item_ref.type_id()) {
            (reflect.get_func)(item_ref).do_a();
        }
        if let Some(reflect) = registry.get_type_data::<ReflectDoB>(item_ref.type_id()) {
            (reflect.get_func)(item_ref).do_b();
        }
    }
}
