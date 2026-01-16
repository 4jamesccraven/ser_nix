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
//! ## Option and None values
//!
//! Following serde_json conventions, `Option::None` values are serialized
//! as `null` by default. To omit fields when they are `None`, use the
//! `#[serde(skip_serializing_if = "Option::is_none")]` attribute:
//!
//! ```rust
//! use serde::Serialize;
//! use ser_nix::to_string;
//!
//! #[derive(Serialize)]
//! struct Config {
//!     enabled: Option<bool>,
//!     #[serde(skip_serializing_if = "Option::is_none")]
//!     optional: Option<String>,
//! }
//!
//! let config = Config {
//!     enabled: None,
//!     optional: None,
//! };
//!
//! let serialized = to_string(&config).unwrap();
//! // Output: { enabled = null; }
//! ```
//!
//! ## Nix paths
//!
//! In Nix, paths like `./foo.nix` or `/etc/nixos/configuration.nix` are written
//! without quotes. There are two ways to serialize paths as unquoted Nix paths:
//!
//! ### Using `NixPathBuf` wrapper type
//!
//! ```rust
//! use serde::Serialize;
//! use ser_nix::{to_string, NixPathBuf};
//!
//! #[derive(Serialize)]
//! struct NixConfig {
//!     source: NixPathBuf,
//! }
//!
//! let config = NixConfig {
//!     source: NixPathBuf::new("./hardware-configuration.nix"),
//! };
//!
//! let serialized = to_string(&config).unwrap();
//! // Output: { source = ./hardware-configuration.nix; }
//! ```
//!
//! For borrowed paths, use `NixPath<'a>`:
//!
//! ```rust
//! use ser_nix::{to_string, NixPath};
//! use std::path::Path;
//!
//! let path = Path::new("./config.nix");
//! let result = to_string(&NixPath::new(path)).unwrap();
//! assert_eq!(result, "./config.nix");
//! ```
//!
//! ### Using `#[serde(serialize_with = "...")]`
//!
//! ```rust
//! use serde::Serialize;
//! use ser_nix::to_string;
//! use std::path::PathBuf;
//!
//! #[derive(Serialize)]
//! struct NixConfig {
//!     #[serde(serialize_with = "ser_nix::as_nix_path")]
//!     source: PathBuf,
//!     description: String,
//! }
//!
//! let config = NixConfig {
//!     source: PathBuf::from("./hardware-configuration.nix"),
//!     description: "Hardware config".to_string(),
//! };
//!
//! let serialized = to_string(&config).unwrap();
//! // source is unquoted: ./hardware-configuration.nix
//! // description is quoted: "Hardware config"
//! ```
//!
//! ## Disclaimer
//!
//! This library was created mostly to be used as a subcomponent of my main
//! project, [mkdev](https://github.com/4jamesccraven/mkdev). Because of that
//! it is not as a full featured as other serde implemenatations, but I intend
//! to change that over time
mod error;
mod map;
mod path;
mod seq;
mod ser;
mod r#struct;
mod test;
mod tuple;

pub use error::Error;
pub use path::{NixPath, NixPathBuf, as_nix_path, as_optional_nix_path};
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
fn post_processor(serialized: &str) -> String {
    serialized
        .lines()
        .map(|l| if l.chars().any(|c| c != ' ') { l } else { "" })
        .collect::<Vec<_>>()
        .join("\n")
}
