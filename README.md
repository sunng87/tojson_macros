# tojson_macros

This is a Rust syntax extension generates default `::rustc_serialize::json::ToJson`
implementation for you.

[![](http://meritbadge.herokuapp.com/tojson_macros)](https://crates.io/crates/tojson_macros)
[![Build Status](https://travis-ci.org/sunng87/tojson_macros.svg?branch=master)](https://travis-ci.org/sunng87/tojson_macros)

## Crate

```toml
tojson_macros = "*"
```

## Example

Simply add `#[derive(ToJson)]` to your struct.

```rust
#![feature(custom_derive, plugin)]
#![plugin(tojson_macros)]

extern crate rustc_serialize;

use rustc_serialize::json::ToJson;

#[derive(ToJson)]
struct Person {
    name: String,
    age: u8
}

let p = Person { name: "Ning".to_string(), age: 28u8 };
println!("{}", p.to_json());
```

## License

MIT
