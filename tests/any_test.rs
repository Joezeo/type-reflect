use std::any::Any;

struct Foo {}

trait Bar: Any {
    fn do_something(&self) {
        println!("Do Something")
    }

    fn as_any(&self) -> &dyn Any;
}

impl Bar for Foo {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn main() {
    let foo = Foo {};
    let foo_any = foo.as_any();
    println!("{:?}", foo_any.type_id());
    assert!(foo_any.downcast_ref::<Foo>().is_some());
    upper_cast(&foo);

    let mut x = 0u32;
    let mut y = 0i32;

    modify_if_u32(&mut x);
    modify_if_u32(&mut y);
    assert_eq!(x, 40);
    assert_eq!(y, 0);
}

fn modify_if_u32(num: &mut dyn Any) {
    if let Some(num) = num.downcast_mut::<u32>() {
        *num = 40;
    }
}

fn upper_cast(bar: &dyn Bar) {
    let foo = bar.as_any().downcast_ref::<Foo>().unwrap();
    foo.do_something();
}