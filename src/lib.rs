use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn get_extension(name: &str) -> Option<&[&str]> {
    EXT_BY_TYPE.get(name).copied()
}

pub fn get_mime_type(ext: &str) -> Option<&str> {
    TYPE_BY_EXT.get(ext).copied()
}

pub fn mime_for_path(path: &Path) -> Option<&str> {
    path.extension()
        .and_then(|s| s.to_str())
        .and_then(|ext| get_mime_type(ext))
}

#[cfg(test)]
mod test {
    use crate::{get_extension, get_mime_type, mime_for_path};
    use std::path::Path;

    #[test]
    fn test_by_ext() {
        assert_eq!(get_extension("text/css").unwrap(), ["css"]);
    }

    #[test]
    fn test_by_type() {
        assert_eq!(get_mime_type("css"), Some("text/css"));
    }

    #[test]
    fn test_by_path() {
        test_path_none("foo");
        test_path_none("/path/to/foo");
        test_path("foo.css", "text/css");
        test_path("/path/to/foo.css", "text/css");
        test_path("foo.html.css", "text/css");
        test_path("/path/to/foo.html.css", "text/css");
        test_path("/path/to.html/foo.css", "text/css");
    }

    fn test_path(path: &str, expected: &str) {
        assert_eq!(mime_for_path(Path::new(path)), Some(expected));
    }

    fn test_path_none(path: &str) {
        assert_eq!(mime_for_path(Path::new(path)), None);
    }
}
