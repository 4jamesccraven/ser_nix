#[cfg(test)]
mod test {
    use crate::to_string;
    use indexmap::IndexMap;
    use serde::Serialize;

    #[test]
    fn foo() {
        let str = String::from("Hello World!");

        let ser = to_string(&str).unwrap();

        assert_eq!(ser, String::from("\"Hello World!\""));
    }

    #[test]
    fn simple_struct() {
        #[derive(Serialize)]
        struct Foo {
            a: i64,
            b: String,
            c: bool,
        }

        let foo = Foo {
            a: 42,
            b: String::from("Hi!"),
            c: true,
        };

        let foo_str = to_string(&foo).unwrap();

        #[rustfmt::skip]
        let expected = concat!(
            "{\n",
            "  a = 42;\n",
            "  b = \"Hi!\";\n",
            "  c = true;\n",
            "}"
        );

        assert_eq!(foo_str, expected);
    }

    #[test]
    fn map() {
        let map: IndexMap<String, bool> = [("enable", true), ("with-setting", true)]
            .iter()
            .map(|(a, b)| (a.to_string(), *b))
            .collect();

        let map_str = to_string(&map).unwrap();

        #[rustfmt::skip]
        let expected = concat!(
            "{\n",
            "  enable = true;\n",
            "  with-setting = true;\n",
            "}"
        );

        assert_eq!(map_str, expected);
    }

    #[test]
    fn nested() {
        #[derive(Serialize, Clone)]
        struct Book {
            name: String,
            author: String,
            read: bool,
        }

        let book_one = Book {
            name: "Eragon".into(),
            author: "Christopher Paolini".into(),
            read: true,
        };

        let book_two = Book {
            name: "Eldest".into(),
            author: "Christopher Paolini".into(),
            read: true,
        };

        let book_three = Book {
            name: "Brisngr".into(),
            author: "Christopher Paolini".into(),
            read: true,
        };

        let book_four = Book {
            name: "Inheritance".into(),
            author: "Christopher Paolini".into(),
            read: true,
        };

        let book_five = Book {
            name: "Murtagh".into(),
            author: "Christopher Paolini".into(),
            read: true,
        };

        let library: IndexMap<String, Book> =
            vec![book_one, book_two, book_three, book_four, book_five]
                .iter()
                .map(|b| (b.name.clone(), b.clone()))
                .collect();

        let libraries = vec![library];

        let libraries_str = to_string(&libraries).unwrap();

        println!("{libraries_str}");

        let expected = concat!(
            "[\n",
            "  {\n",
            "    Eragon = {\n",
            "      name = \"Eragon\";\n",
            "      author = \"Christopher Paolini\";\n",
            "      read = true;\n",
            "    };\n",
            "    Eldest = {\n",
            "      name = \"Eldest\";\n",
            "      author = \"Christopher Paolini\";\n",
            "      read = true;\n",
            "    };\n",
            "    Brisngr = {\n",
            "      name = \"Brisngr\";\n",
            "      author = \"Christopher Paolini\";\n",
            "      read = true;\n",
            "    };\n",
            "    Inheritance = {\n",
            "      name = \"Inheritance\";\n",
            "      author = \"Christopher Paolini\";\n",
            "      read = true;\n",
            "    };\n",
            "    Murtagh = {\n",
            "      name = \"Murtagh\";\n",
            "      author = \"Christopher Paolini\";\n",
            "      read = true;\n",
            "    };\n",
            "  }\n",
            "]"
        );

        assert_eq!(libraries_str, expected);
    }

    #[test]
    fn string() {
        let test = String::from("${} $ \" \t \n ' \\");

        let test_str = to_string(&test).unwrap();

        let expected = String::from("\"''${} $ \\\" \\t \\n ' \\\\\"");

        assert_eq!(test_str, expected);
    }

    #[test]
    fn multiline_string() {
        #[rustfmt::skip]
        let longer = concat!(
            "Lorem ipsum 'dolor ''sit amet,\n",
            "consectetur adipiscing elit,\n",
            "seddo eiusmod \ttempor incididunt\n",
            "ut labore et dolore magnam\n",
            "aliquam ${} \" quaerat voluptatem. Ut\n",
            "enim aeque doleamus animo, cum\n",
            "corpore $ dolemus, \\ fieri tamen\n",
            "permagna accessio potest, si\n",
            "aliquod aeternum et infinitum\n",
            "impendere malum nobis opinemur.\n",
            "Quod idem licet transferre in\n",
            "voluptatem, ut.",
        );

        let longer_test = to_string(&longer).unwrap();

        let expected = concat!(
            "''\n",
            "  Lorem ipsum 'dolor '''sit amet,\n",
            "  consectetur adipiscing elit,\n",
            "  seddo eiusmod \ttempor incididunt\n",
            "  ut labore et dolore magnam\n",
            "  aliquam ''${} \" quaerat voluptatem. Ut\n",
            "  enim aeque doleamus animo, cum\n",
            "  corpore $ dolemus, \\ fieri tamen\n",
            "  permagna accessio potest, si\n",
            "  aliquod aeternum et infinitum\n",
            "  impendere malum nobis opinemur.\n",
            "  Quod idem licet transferre in\n",
            "  voluptatem, ut.\n",
            "''",
        );

        println!("{longer_test}");

        assert_eq!(longer_test, expected);
    }

    #[test]
    fn none() {
        #[derive(Serialize)]
        struct OptionalVals {
            a: Option<i32>,
            b: Option<i32>,
        }

        let none = OptionalVals {
            a: Some(32),
            b: None,
        };

        let none_test = to_string(&none).unwrap();

        #[rustfmt::skip]
        let expected = concat!(
            "{\n",
            "  a = 32;\n",
            "  b = null;\n",
            "}",
        );

        assert_eq!(none_test, expected);
    }

    #[test]
    fn none_map() {
        let mut none_map: IndexMap<i32, Option<i32>> = IndexMap::new();

        none_map.insert(1, Some(1));
        none_map.insert(2, None);
        none_map.insert(3, Some(3));

        let none_map_test = to_string(&none_map).unwrap();

        #[rustfmt::skip]
        let expected = concat!(
            "{\n",
            "  1 = 1;\n",
            "  2 = null;\n",
            "  3 = 3;\n",
            "}",
        );

        assert_eq!(none_map_test, expected);
    }

    #[test]
    fn skip_serializing_if_none() {
        #[derive(Serialize)]
        struct OptionalVals {
            a: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            b: Option<i32>,
            c: Option<i32>,
        }

        let none = OptionalVals {
            a: Some(32),
            b: None,
            c: Some(64),
        };

        let none_test = to_string(&none).unwrap();

        #[rustfmt::skip]
        let expected = concat!(
            "{\n",
            "  a = 32;\n",
            "  c = 64;\n",
            "}",
        );

        assert_eq!(none_test, expected);
    }

    #[test]
    fn path_with_serialize_with() {
        use crate::as_nix_path;
        use std::path::PathBuf;

        #[derive(Serialize)]
        struct Config {
            #[serde(serialize_with = "as_nix_path")]
            source: PathBuf,
            #[serde(serialize_with = "as_nix_path")]
            config: PathBuf,
        }

        let config = Config {
            source: PathBuf::from("./hardware-configuration.nix"),
            config: PathBuf::from("/etc/nixos/configuration.nix"),
        };

        let config_str = to_string(&config).unwrap();

        #[rustfmt::skip]
        let expected = concat!(
            "{\n",
            "  source = ./hardware-configuration.nix;\n",
            "  config = /etc/nixos/configuration.nix;\n",
            "}",
        );

        assert_eq!(config_str, expected);
    }

    #[test]
    fn path_with_nix_path_buf_wrapper() {
        use crate::NixPathBuf;

        #[derive(Serialize)]
        struct Config {
            source: NixPathBuf,
            config: NixPathBuf,
        }

        let config = Config {
            source: NixPathBuf::new("./hardware-configuration.nix"),
            config: NixPathBuf::new("/etc/nixos/configuration.nix"),
        };

        let config_str = to_string(&config).unwrap();

        #[rustfmt::skip]
        let expected = concat!(
            "{\n",
            "  source = ./hardware-configuration.nix;\n",
            "  config = /etc/nixos/configuration.nix;\n",
            "}",
        );

        assert_eq!(config_str, expected);
    }

    #[test]
    fn nix_path_borrowed() {
        use crate::NixPath;
        use std::path::Path;

        let path = Path::new("./test.nix");
        let nix_path = NixPath::new(path);

        let result = to_string(&nix_path).unwrap();
        assert_eq!(result, "./test.nix");
    }

    #[test]
    fn optional_path() {
        use crate::as_optional_nix_path;
        use std::path::{Path, PathBuf};

        #[derive(Serialize)]
        struct Config<'a> {
            #[serde(serialize_with = "as_optional_nix_path")]
            owned: Option<PathBuf>,
            #[serde(serialize_with = "as_optional_nix_path")]
            borrowed: Option<&'a Path>,
            #[serde(serialize_with = "as_optional_nix_path")]
            none: Option<PathBuf>,
        }

        let config = Config {
            owned: Some(PathBuf::from("./owned.nix")),
            borrowed: Some(Path::new("./borrowed.nix")),
            none: None,
        };

        let config_str = to_string(&config).unwrap();

        #[rustfmt::skip]
        let expected = concat!(
            "{\n",
            "  owned = ./owned.nix;\n",
            "  borrowed = ./borrowed.nix;\n",
            "  none = null;\n",
            "}",
        );

        assert_eq!(config_str, expected);
    }

    #[test]
    fn relative_path_gets_prefix() {
        use crate::NixPathBuf;

        // Paths without ./ prefix get it automatically
        let path = NixPathBuf::new("foo.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, "./foo.nix");

        // Paths with ./ prefix stay unchanged
        let path = NixPathBuf::new("./foo.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, "./foo.nix");

        // Paths with ../ prefix stay unchanged
        let path = NixPathBuf::new("../foo.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, "../foo.nix");

        // Absolute paths stay unchanged
        let path = NixPathBuf::new("/etc/nixos/foo.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, "/etc/nixos/foo.nix");
    }

    #[test]
    fn path_with_spaces() {
        use crate::NixPathBuf;

        // Relative path without prefix
        let path = NixPathBuf::new("path with spaces.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, r#"./. + "path with spaces.nix""#);

        // Relative path with ./ prefix
        let path = NixPathBuf::new("./path with spaces.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, r#"./. + "path with spaces.nix""#);

        // Relative path with ../ prefix
        let path = NixPathBuf::new("../path with spaces.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, r#"../. + "path with spaces.nix""#);

        // Absolute path
        let path = NixPathBuf::new("/etc/nixos/path with spaces.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, r#"/. + "/etc/nixos/path with spaces.nix""#);
    }

    #[test]
    fn path_with_special_chars() {
        use crate::NixPathBuf;

        // Path with double quotes - must be escaped
        let path = NixPathBuf::new(r#"path"with"quotes.nix"#);
        let result = to_string(&path).unwrap();
        assert_eq!(result, r#"./. + "path\"with\"quotes.nix""#);

        // Path with single quotes - doesn't require quoting
        let path = NixPathBuf::new("path'with'quotes.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, "./path'with'quotes.nix");

        // Path with dollar sign not followed by { - doesn't require quoting
        let path = NixPathBuf::new("path$var.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, "./path$var.nix");

        // Path with ${ - must be escaped
        let path = NixPathBuf::new("path${var}.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, r#"./. + "path\${var}.nix""#);

        // Path with backslash - must be escaped
        let path = NixPathBuf::new(r"path\with\backslash.nix");
        let result = to_string(&path).unwrap();
        assert_eq!(result, r#"./. + "path\\with\\backslash.nix""#);
    }

    #[test]
    fn newtype_var() {
        #[derive(Serialize)]
        enum Test {
            Inches(u8),
        }

        let newtype_var = Test::Inches(8);

        let newtype_var_test = to_string(&newtype_var).unwrap();

        let expected = String::from("{ inches = 8; }");

        assert_eq!(newtype_var_test, expected);
    }
}
