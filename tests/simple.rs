#![feature(custom_derive, plugin)]
#![plugin(tojson_macros)]

extern crate rustc_serialize;

use rustc_serialize::json::{Json, ToJson};

#[derive(ToJson)]
struct Person {
    name: String,
    age: u8,
    nationality: Option<String>
}

#[test]
fn test_gen () {
    let p = Person { name: "Vidar Kjartansson".to_string(), age: 25,
                     nationality: Some("Iceland".to_string())};
    let pj = p.to_json();

    println!("{}", pj);
    assert_eq!(Json::String("Vidar Kjartansson".to_string()),
               *pj.find("name").unwrap());
}
