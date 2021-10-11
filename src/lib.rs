mod data;

use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Types {
    pub(crate) ext_by_type: HashMap<String, Vec<String>>,
    type_by_ext: HashMap<String, String>,
}

impl Types {
    pub fn new() -> Result<Types, ()> {
        let mut by_type = HashMap::new();
        let mut by_ext = HashMap::new();
        for (mime_type, exts) in data::MIME_TYPES.iter() {
            by_type.insert(mime_type.to_string(), exts.iter().map(|ext| ext.to_string()).collect());

            for ext in exts.iter() {
                by_ext.insert(ext.to_string(), mime_type.to_string());
            }
        }

        Ok(Types {
            ext_by_type: by_type,
            type_by_ext: by_ext,
        })
    }

    pub fn get_extension<'a>(&'a self, name: &str) -> Option<&'a [String]> {
        self.ext_by_type.get(name).map(|v| &v[..])
    }

    pub fn get_mime_type<'a>(&'a self, ext: &str) -> Option<&'a str> {
        self.type_by_ext.get(ext).map(|v| &v[..])
    }

    pub fn mime_for_path<'a>(&'a self, path: &Path) -> &'a str {
        path.extension()
            .and_then(|s| s.to_str())
            .and_then(|ext| self.get_mime_type(ext))
            .unwrap_or_else(|| "text/plain")
    }
}

#[cfg(test)]
mod test {
    use crate::Types;
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::path::Path;

    static JSON: &str = include_str!("../data/mime.json");

    #[test]
    fn test_against_json_data() {
        let mut deserializer = serde_json::Deserializer::from_str(JSON);
        let deserialized: HashMap<String, Vec<String>> =
            Deserialize::deserialize(&mut deserializer).unwrap();

        let t = Types::new().ok().expect("Types didn't load");
        assert_eq!(t.ext_by_type, deserialized);
    }

    #[test]
    fn test_by_ext() {
        let t = Types::new().ok().expect("Types didn't load");
        assert_eq!(t.get_extension("text/css").unwrap(), ["css".to_string()]);
    }

    #[test]
    fn test_by_type() {
        let t = Types::new().ok().expect("Types didn't load");
        assert_eq!(t.get_mime_type("css"), Some("text/css"));
    }

    #[test]
    fn test_by_path() {
        let t = Types::new().ok().expect("Types didn't load");

        test_path(&t, "foo", "text/plain");
        test_path(&t, "/path/to/foo", "text/plain");
        test_path(&t, "foo.css", "text/css");
        test_path(&t, "/path/to/foo.css", "text/css");
        test_path(&t, "foo.html.css", "text/css");
        test_path(&t, "/path/to/foo.html.css", "text/css");
        test_path(&t, "/path/to.html/foo.css", "text/css");
    }

    fn test_path(types: &Types, path: &str, expected: &str) {
        assert_eq!(types.mime_for_path(Path::new(path)), expected);
    }
}
