trait Foo {
    fn f(&self)-> i32;
}

trait Bar {
    fn f(&self)-> i32;
}

struct Baz;

impl Foo for Baz {
    fn f(&self)-> i32{
        println!("Baz’s impl of Foo");
        20
    }
}

impl Bar for Baz {
    fn f(&self) -> i32{
        println!("Baz’s impl of Bar");
        10
    }
}
fn main() {
    let b = <dyn Baz>::Foo;
    assert_eq!(10, b.f());
}
