#[macro_use]
extern crate tojson_macros;

extern crate rustc_serialize;

use rustc_serialize::json::{Json, ToJson};

#[derive(ToJson)]
struct Person {
    name: String,
    age: u8,
    nationality: Option<String>,
}

#[derive(ToJson)]
struct Person2<'a, T>
    where T: ToJson
{
    name: &'a str,
    age: T,
}

#[test]
fn test_gen() {
    let p = Person {
        name: "Vidar Kjartansson".to_string(),
        age: 25,
        nationality: Some("Iceland".to_string()),
    };
    let pj = p.to_json();

    println!("{}", pj);
    assert_eq!(Json::String("Vidar Kjartansson".to_string()),
               *pj.find("name").unwrap());
    assert_eq!(Json::U64(25), *pj.find("age").unwrap());
    assert_eq!(Json::String("Iceland".to_string()),
               *pj.find("nationality").unwrap());

    let p2_name = "Solvi Ottesen";
    let p2 = Person2::<u8> {
        name: p2_name,
        age: 31,
    };
    let pj2 = p2.to_json();
    println!("{}", p2.to_json());

    assert_eq!(Json::String("Solvi Ottesen".to_string()),
               *pj2.find("name").unwrap());
    assert_eq!(Json::U64(31), *pj2.find("age").unwrap());
}
