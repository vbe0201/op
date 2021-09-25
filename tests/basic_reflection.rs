use op::{PropertyAs, Type};

#[derive(Type)]
pub struct Foo {
    #[property(flags(A | B | C))] // Just for illustration, currently unhandled.
    bar: u32,
}

#[test]
fn test_reflect() {
    let mut foo = Foo { bar: 1337 };

    *foo.property_as_mut::<u32>("bar").unwrap() += 1;
    assert_eq!(foo.bar, 1338);
}
