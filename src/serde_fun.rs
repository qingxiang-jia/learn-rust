use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let k1 = String::from("AppName");
    let k2 = String::from("UUID");
    let k3 = String::from("Type");
    map.insert(k1, 1);
    map.insert(k2, 2);
    map.insert(k3, 3);
    let serialized = serde_json::to_string(&map).unwrap();
    println!("JSON of map: {}", serialized);
    let deserialized: HashMap<String, i32> = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);
}