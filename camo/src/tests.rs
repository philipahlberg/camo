use std::convert::TryFrom;

use crate::{BuiltinType, PathSegment, TypePath};

#[test]
fn type_path_from_segments() {
    let path = TypePath::from([PathSegment {
        name: "foo",
        arguments: Vec::new(),
    }]);

    assert_eq!(
        path,
        TypePath {
            segments: vec![PathSegment {
                name: "foo",
                arguments: Vec::new(),
            }]
        }
    );
}

#[test]
fn builtin_from_type_path() {
    let path = TypePath::from([PathSegment {
        name: "i32",
        arguments: Vec::new(),
    }]);

    assert_eq!(BuiltinType::try_from(path), Ok(BuiltinType::I32));
}
