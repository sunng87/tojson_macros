# tojson_macros

This is a Rust syntax extension generates default `::rustc_serialize::json::ToJson`
implementation for you. Note that this library relies on libsyntax
which is only available in nightly channel.

[![](http://meritbadge.herokuapp.com/tojson_macros)](https://crates.io/crates/tojson_macros)
[![Build Status](https://travis-ci.org/sunng87/tojson_macros.svg?branch=master)](https://travis-ci.org/sunng87/tojson_macros)

## Crate

```toml
tojson_macros = "^0.2"
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

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
