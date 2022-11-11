mod json;
use json::JsonType;
use std::collections::HashMap;

fn main() {
    let mut j1 = HashMap::new();
    j1.insert(String::from("foo"), JsonType::Number(5.14));
    j1.insert(String::from("bar"), JsonType::Boolean(true));
    let mut j2 = HashMap::new();
    j2.insert(String::from("foo2"), JsonType::String(String::from("asdf")));
    j2.insert(
        String::from("bar2"),
        JsonType::Array(vec![
            JsonType::Number(9.8),
            JsonType::String(String::from("ohhh")),
        ]),
    );
    j1.insert(String::from("sub"), JsonType::Object(j2));
    println!("{:?}", j1);
}
