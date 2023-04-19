use bevy_reflect::{reflect_trait, Reflect, TypeRegistry};

#[reflect_trait]
trait DoA {
    fn do_a(&self);
}

#[reflect_trait]
trait DoB {
    fn do_b(&self);
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
