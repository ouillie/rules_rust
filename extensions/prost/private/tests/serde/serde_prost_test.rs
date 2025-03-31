//! Tests serde serialization of generated messages.

use std::collections::HashMap;

use foo_proto::foo::bar::{A, B};

#[test]
fn test_package_importer() {
    let mut fields = HashMap::new();
    fields.insert(1, 2.5);
    fields.insert(10, 20.5);
    let a = A {
        name: String::from("a test!"),
        fields,
    };

    let json = serde_json::to_string(&a).unwrap();

    assert_eq!(json, "{\"name\":\"a test!\",\"fields\":{\"1\":2.5,\"10\":20.5}}");
}
