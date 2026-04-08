use super::error::Error;
use super::ser::Serializer;

use serde::{Serialize, ser};

/// Returns `true` if `s` is a valid Nix identifier that can appear unquoted
/// as an attribute name (e.g. `foo`, `build-inputs`, `x86_64-linux`).
///
/// Keys that are not valid identifiers (e.g. `8080/tcp`, `/var/data`) must
/// remain quoted in Nix attribute sets.
fn is_nix_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '\'')
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut key_serializer = Serializer {
            output: String::new(),
            pending_key: None,
            indent_depth: self.indent_depth,
        };
        key.serialize(&mut key_serializer)?;
        let mut base_key = key_serializer.output;

        // Only strip quotes from keys that are valid Nix identifiers.
        // Keys like "8080/tcp" must remain quoted because they contain
        // characters not allowed in bare Nix attribute names.
        if let Some(stripped) = base_key
            .strip_prefix("\"")
            .and_then(|s| s.strip_suffix("\""))
        {
            if is_nix_identifier(stripped) {
                base_key = stripped.to_string();
            }
        }

        self.pending_key = Some(base_key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut value_serializer = Serializer {
            output: String::new(),
            pending_key: None,
            indent_depth: self.indent_depth,
        };

        value.serialize(&mut value_serializer)?;
        let val = value_serializer.output;

        self.output += "\n";

        self.indent();
        let key = self.pending_key.take().expect("Value without key.");
        self.output += &format!("{} = {};", key, val);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.indent_depth -= 1;
        self.output += "\n";
        self.indent();
        self.output += "}";
        Ok(())
    }
}
