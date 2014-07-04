extern crate serialize;

use std::io::BufReader;
use std::collections::HashMap;
use serialize::{Decodable, json};

static json: &'static str = include_str!("../data/mime.json");

pub struct Types {
    ext_by_type: HashMap<String, Vec<String>>,
    type_by_ext: HashMap<String, String>

}

impl Types {
    pub fn new() -> Result<Types, ()> {
        let parsed = try!(json::from_str(json).map_err(|_| ()));
        let mut decoder = json::Decoder::new(parsed);
        let decoded: HashMap<String, Vec<String>> =
            try!(Decodable::decode(&mut decoder).map_err(|_| ()));

        let mut by_ext = HashMap::new();

        for (mime_type, exts) in decoded.iter() {
            for ext in exts.iter() {
                by_ext.insert(ext.clone(), mime_type.clone());
            }
        }

        Ok(Types { ext_by_type: decoded, type_by_ext: by_ext })
    }

    pub fn get_extension<'a>(&'a self, name: &str) -> Option<&'a [String]> {
        self.ext_by_type.find_equiv(&name).map(|v| v.as_slice())
    }

    pub fn get_mime_type<'a>(&'a self, ext: &str) -> Option<&'a str> {
        self.type_by_ext.find_equiv(&ext).map(|v| v.as_slice())
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use Types;

    #[bench]
    fn bench_load_types(b: &mut test::Bencher) {
        b.iter(|| {
            Types::new()
        });
    }

    #[test]
    fn test_by_ext() {
        let t = Types::new().ok().expect("Types didn't load");
        assert_eq!(t.get_extension("text/css"), Some(["css".to_str()].as_slice()));
    }

    #[test]
    fn test_by_type() {
        let t = Types::new().ok().expect("Types didn't load");
        assert_eq!(t.get_mime_type("css"), Some("text/css"));
    }
}
