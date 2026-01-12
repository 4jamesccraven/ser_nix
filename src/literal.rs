use serde::{Serialize, Serializer};
use std::borrow::Cow;

pub(crate) const TOKEN: &str = "$ser_nix::private::Literal";

/// A raw Nix expression that serializes without quotes.
///
/// Use this wrapper type when you want to output a raw Nix expression
/// (e.g., `pkgs.hello`, `lib.mkForce true`, `builtins.currentSystem`).
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::{to_string, NixLiteral};
///
/// #[derive(Serialize)]
/// struct Config {
///     package: NixLiteral<'static>,
/// }
///
/// let config = Config {
///     package: NixLiteral::from("pkgs.hello"),
/// };
///
/// let result = to_string(&config).unwrap();
/// assert!(result.contains("package = pkgs.hello;"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NixLiteral<'a>(Cow<'a, str>);

impl<'a> NixLiteral<'a> {
    /// Creates a new `NixLiteral` from a string slice.
    pub fn new(expr: &'a str) -> Self {
        NixLiteral(Cow::Borrowed(expr))
    }

    /// Returns the underlying expression as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts into an owned `String`.
    pub fn into_string(self) -> String {
        self.0.into_owned()
    }
}

impl From<String> for NixLiteral<'static> {
    fn from(expr: String) -> Self {
        NixLiteral(Cow::Owned(expr))
    }
}

impl<'a> From<&'a str> for NixLiteral<'a> {
    fn from(expr: &'a str) -> Self {
        NixLiteral(Cow::Borrowed(expr))
    }
}

impl<'a> From<&'a String> for NixLiteral<'a> {
    fn from(expr: &'a String) -> Self {
        NixLiteral(Cow::Borrowed(expr.as_str()))
    }
}

impl AsRef<str> for NixLiteral<'_> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn serialize_literal<S>(expr: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_newtype_struct(TOKEN, expr)
}

impl Serialize for NixLiteral<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_literal(&self.0, serializer)
    }
}

/// Serialize a string as a raw Nix expression (without quotes).
///
/// Use this function with `#[serde(serialize_with = "...")]` to serialize
/// string fields as raw Nix expressions.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::to_string;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "ser_nix::as_literal")]
///     package: String,
/// }
///
/// let config = Config {
///     package: "pkgs.hello".to_string(),
/// };
///
/// let result = to_string(&config).unwrap();
/// assert!(result.contains("package = pkgs.hello;"));
/// ```
pub fn as_literal<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serialize_literal(value, serializer)
}

/// Serialize an `Option<String>` as a raw Nix expression, or null if None.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::to_string;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "ser_nix::as_optional_literal")]
///     package: Option<String>,
/// }
///
/// let config = Config {
///     package: Some("pkgs.hello".to_string()),
/// };
///
/// let result = to_string(&config).unwrap();
/// assert!(result.contains("package = pkgs.hello;"));
/// ```
pub fn as_optional_literal<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serialize_literal(v, serializer),
        None => serializer.serialize_none(),
    }
}
