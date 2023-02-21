#[test]
fn export_calls_camo() {
    use camo::{export, Camo, Item, Struct};

    struct Foo {}

    struct Bar {}

    impl Camo for Foo {
        fn camo() -> Item {
            Item::Struct(Struct {
                name: "Foo",
                fields: Vec::new(),
            })
        }
    }

    impl Camo for Bar {
        fn camo() -> Item {
            Item::Struct(Struct {
                name: "Bar",
                fields: Vec::new(),
            })
        }
    }

    let exports: Vec<Item> = export! { Foo, Bar };

    assert_eq!(exports, vec![Foo::camo(), Bar::camo()]);
}
