An opinionated Nix serializer, using the serde framework.

`ser_nix` exists to allow generation of human readable Nix code from Rust that
can be included in NixOS configurations, devShells, etc.

> [!NOTE]
> This is not a deserializer, for that look into
> [tvix_serde](https://docs.tvix.dev/rust/tvix_serde/index.html) (which only
> implements the deserializer half). There is no guarantee of compatibility
> between the two.

### Example
```rust
use serde::Serialize;
use ser_nix::to_string;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u8,
}

let cm = Person {
    name: "John Doe".into(),
    age: 65,
};

let serialized = to_string(&cm).unwrap();

let expected = "{\n  name = \"John Doe\";\n  age = 65;\n}".to_string();

assert_eq!(serialized, expected);
```
see the [docs](https://docs.rs/ser_nix/latest/ser_nix/) for more.

## Design philosophy
This crate is intentionally opinionated in the way it produces output. Unlike
other crates in the Serde ecosystem, there is no "pretty" serialization and
"normal" serialization; output is formatted/pretty by default.

## Contributing
Contributions, issues, etc. are all welcome and encouraged.
Please ensure code is formatted with `rustfmt` prior to submitting PRs.

### License
`ser_nix` is licensed under the MIT license.
