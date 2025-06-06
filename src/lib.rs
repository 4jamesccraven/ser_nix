//! # ser_nix
//!
//! Nix is a declarative, atomic, and reproducible package manager
//! that is configured with the nix programming language
//!
//! ```nix
//! {
//!   a = 1;
//!   b = "Hello World";
//!   submap.foo = "bar";
//! }
//! ````
//!
//! ser_nix can be used to serialise arbitrary rust types into
//! corresponding nix data types. As the name implies, ser_nix
//! does *not* provide deserialisation capabilities, as the
//! process for doing so is non-trivial, and requires evaluating
//! nix code.
//!
//! ser_nix tries to follow the idioms of other serde libraries,
//! like [serde_json](https://docs.rs/serde_json/latest/serde_json/index.html).
//!
//! ```rust
//! use serde::Serialize;
//! use ser_nix::to_string;
//!
//! #[derive(Serialize)]
//! struct Person {
//!     name: String,
//!     age: u8,
//! }
//!
//! let cm = Person {
//!     name: "John Doe".into(),
//!     age: 65,
//! };
//!
//! let serialized = to_string(&cm).unwrap();
//!
//! let expected = "{\n  name = \"John Doe\";\n  age = 65;\n}".to_string();
//!
//! assert_eq!(serialized, expected);
//! ````
//!
//! ## Disclaimer
//!
//! This library was created mostly to be used as a subcomponent of my main
//! project, [mkdev](https://github.com/4jamesccraven/mkdev). Because of that
//! it is not as a full featured as other serde implemenatations, but I intend
//! to change that over time
mod error;
mod map;
mod seq;
mod ser;
mod r#struct;
mod test;
mod tuple;

pub use error::Error;
use ser::Serializer;

use serde::Serialize;

/// Serialise the given data structure as a String of Nix data
///
/// # Errors
///
/// Serialization can fail if the implemenatation of `Serialize` for `T`
/// fails.
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        pending_key: None,
        indent_depth: 0,
    };
    value.serialize(&mut serializer)?;

    Ok(post_processor(&serializer.output))
}

/// Removes extra whitespace that gets left behind due to indentation
fn post_processor(serialized: &String) -> String {
    serialized
        .lines()
        .map(|l| if l.chars().any(|c| c != ' ') { l } else { "" })
        .collect::<Vec<_>>()
        .join("\n")
}
