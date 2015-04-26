# tojson_macros

This is a Rust syntax extension generates default `ToJson`
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
```

## License

MIT
