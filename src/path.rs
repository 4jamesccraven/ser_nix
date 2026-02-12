use crate::error::Error;
use serde::{Serialize, Serializer, ser};
use std::borrow::Cow;
use std::ops::Deref;
use std::path::{Component, Path, PathBuf};

pub(crate) const TOKEN: &str = "$ser_nix::private::Path";

/// Internal serializer that emits raw strings without quoting.
pub(crate) struct RawEmitter<'a> {
    pub output: &'a mut String,
}

impl ser::Serializer for RawEmitter<'_> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(v);
        Ok(())
    }

    fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("expected string"))
    }
}

/// An owned path that serializes as a Nix path literal.
///
/// Use this wrapper type when you want a path to be serialized as a Nix path
/// literal (e.g., `./foo.nix` or `/etc/nixos/configuration.nix`).
///
/// For borrowed paths, use [`NixPath`] or `#[serde(serialize_with = "ser_nix::as_nix_path")]`.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::{to_string, NixPathBuf};
///
/// #[derive(Serialize)]
/// struct Config {
///     source: NixPathBuf,
/// }
///
/// let config = Config {
///     source: NixPathBuf::new("./hardware-configuration.nix"),
/// };
///
/// let result = to_string(&config).unwrap();
/// assert!(result.contains("source = ./hardware-configuration.nix;"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NixPathBuf(PathBuf);

impl NixPathBuf {
    /// Creates a new `NixPathBuf`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        NixPathBuf(path.into())
    }

    /// Returns a reference to the underlying `Path`.
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Converts into the underlying `PathBuf`.
    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }
}

impl From<PathBuf> for NixPathBuf {
    fn from(path: PathBuf) -> Self {
        NixPathBuf(path)
    }
}

impl From<&Path> for NixPathBuf {
    fn from(path: &Path) -> Self {
        NixPathBuf(path.to_path_buf())
    }
}

impl From<&str> for NixPathBuf {
    fn from(s: &str) -> Self {
        NixPathBuf(PathBuf::from(s))
    }
}

impl AsRef<Path> for NixPathBuf {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Deref for NixPathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for NixPathBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_path(&self.0, serializer)
    }
}

/// A borrowed path that serializes as a Nix path literal.
///
/// Use this wrapper type for borrowed paths. For owned paths, use [`NixPathBuf`].
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::{to_string, NixPath};
/// use std::path::Path;
///
/// let path = Path::new("./hardware-configuration.nix");
/// let nix_path = NixPath::new(path);
///
/// let result = to_string(&nix_path).unwrap();
/// assert_eq!(result, "./hardware-configuration.nix");
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NixPath<'a>(&'a Path);

impl<'a> NixPath<'a> {
    /// Creates a new `NixPath` from a borrowed `Path`.
    pub fn new(path: &'a Path) -> Self {
        NixPath(path)
    }

    /// Returns a reference to the underlying `Path`.
    pub fn as_path(&self) -> &Path {
        self.0
    }
}

impl<'a> From<&'a Path> for NixPath<'a> {
    fn from(path: &'a Path) -> Self {
        NixPath(path)
    }
}

impl AsRef<Path> for NixPath<'_> {
    fn as_ref(&self) -> &Path {
        self.0
    }
}

impl Serialize for NixPath<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_path(self.0, serializer)
    }
}

/// Check if a path string contains characters that require quoting in Nix.
///
/// Nix path literals only allow `PATH_CHAR` (`[a-zA-Z0-9._\-+]`) and `/`.
/// See: https://github.com/NixOS/nix/blob/master/src/libexpr/lexer.l
///
/// Any other character (e.g. spaces, `%`, `$`, `@`) requires the path to be
/// expressed as a string concatenation like `/. + "/path"` instead of a bare
/// path literal.
fn needs_quoting(s: &str) -> bool {
    s.bytes().any(|b| {
        !matches!(b,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9'
            | b'.' | b'_' | b'-' | b'+' | b'/'
        )
    })
}

/// Escape a string for use inside a Nix double-quoted string, writing to an existing buffer.
fn escape_nix_string_into(s: &str, out: &mut String) {
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '$' if chars.peek() == Some(&'{') => out.push_str("\\$"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
}

fn serialize_path<S>(path: &Path, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let path_str = path
        .to_str()
        .ok_or_else(|| serde::ser::Error::custom("path contains invalid UTF-8 characters"))?;

    let has_special_chars = needs_quoting(path_str);

    let result: Cow<str> = if path.is_absolute() {
        if has_special_chars {
            // /. + "/path with spaces"
            let mut buf = String::with_capacity(6 + path_str.len());
            buf.push_str("/. + \"");
            escape_nix_string_into(path_str, &mut buf);
            buf.push('"');
            Cow::Owned(buf)
        } else {
            Cow::Borrowed(path_str)
        }
    } else {
        match path.components().next() {
            Some(Component::CurDir) => {
                if has_special_chars {
                    // ./. + "rest" (strip the ./ prefix, quote the rest)
                    let rest = path.strip_prefix(Component::CurDir).unwrap();
                    let rest_str = rest.to_str().unwrap();
                    let mut buf = String::with_capacity(7 + rest_str.len());
                    buf.push_str("./. + \"");
                    escape_nix_string_into(rest_str, &mut buf);
                    buf.push('"');
                    Cow::Owned(buf)
                } else {
                    Cow::Borrowed(path_str)
                }
            }
            Some(Component::ParentDir) => {
                if has_special_chars {
                    // ../. + "rest" (strip the ../ prefix, quote the rest)
                    let rest = path.strip_prefix(Component::ParentDir).unwrap();
                    let rest_str = rest.to_str().unwrap();
                    let mut buf = String::with_capacity(8 + rest_str.len());
                    buf.push_str("../. + \"");
                    escape_nix_string_into(rest_str, &mut buf);
                    buf.push('"');
                    Cow::Owned(buf)
                } else {
                    Cow::Borrowed(path_str)
                }
            }
            _ => {
                // Bare relative path - needs ./ prefix to avoid Nix search path interpretation
                if has_special_chars {
                    // ./. + "path"
                    let mut buf = String::with_capacity(7 + path_str.len());
                    buf.push_str("./. + \"");
                    escape_nix_string_into(path_str, &mut buf);
                    buf.push('"');
                    Cow::Owned(buf)
                } else {
                    // ./path
                    let mut buf = String::with_capacity(2 + path_str.len());
                    buf.push_str("./");
                    buf.push_str(path_str);
                    Cow::Owned(buf)
                }
            }
        }
    };

    serializer.serialize_newtype_struct(TOKEN, result.as_ref())
}

/// Serialize a `Path` or `PathBuf` as a Nix path literal.
///
/// Use this function with `#[serde(serialize_with = "...")]` to serialize
/// `Path` or `PathBuf` fields as path literals.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::to_string;
/// use std::path::PathBuf;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "ser_nix::as_nix_path")]
///     source: PathBuf,
/// }
///
/// let config = Config {
///     source: PathBuf::from("./hardware-configuration.nix"),
/// };
///
/// let result = to_string(&config).unwrap();
/// assert!(result.contains("source = ./hardware-configuration.nix;"));
/// ```
pub fn as_nix_path<S>(value: &Path, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serialize_path(value, serializer)
}

/// Serialize an `Option<PathBuf>` or `Option<&Path>` as a Nix path literal, or null if None.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use ser_nix::to_string;
/// use std::path::PathBuf;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "ser_nix::as_optional_nix_path")]
///     source: Option<PathBuf>,
/// }
///
/// let config = Config {
///     source: Some(PathBuf::from("./path.nix")),
/// };
///
/// let result = to_string(&config).unwrap();
/// assert!(result.contains("source = ./path.nix;"));
/// ```
pub fn as_optional_nix_path<P, S>(value: &Option<P>, serializer: S) -> Result<S::Ok, S::Error>
where
    P: AsRef<Path>,
    S: Serializer,
{
    match value {
        Some(v) => serialize_path(v.as_ref(), serializer),
        None => serializer.serialize_none(),
    }
}
