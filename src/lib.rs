use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Types {}

impl Types {
    pub fn new() -> Result<Types, ()> {
        Ok(Types {})
    }

    pub fn get_extension(&self, name: &str) -> Option<&[&str]> {
        EXT_BY_TYPE.get(name).copied()
    }

    pub fn get_mime_type(&self, ext: &str) -> Option<&str> {
        TYPE_BY_EXT.get(ext).copied()
    }

    pub fn mime_for_path(&self, path: &Path) -> &str {
        path.extension()
            .and_then(|s| s.to_str())
            .and_then(|ext| self.get_mime_type(ext))
            .unwrap_or_else(|| "text/plain")
    }
}

#[cfg(test)]
mod test {
    use crate::Types;
    use std::path::Path;

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
