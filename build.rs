use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

static JSON: &str = include_str!("data/mime.json");

fn main() {
    let json: BTreeMap<String, Vec<String>> = serde_json::from_str(JSON).unwrap();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut used_exts = HashSet::new();
    let mut ext_by_type = phf_codegen::Map::new();
    let mut type_by_ext = phf_codegen::Map::new();

    for (mime_type, exts) in json.iter() {
        let mime_type_value = format!("\"{}\"", mime_type);

        for ext in exts {
            if used_exts.insert(ext) {
                type_by_ext.entry(ext, &mime_type_value);
            }
        }

        let exts: Vec<_> = exts.iter().map(|ext| format!("\"{}\"", ext)).collect();
        let exts = format!("&[{}]", exts.join(", "));
        ext_by_type.entry(mime_type, &exts);
    }

    writeln!(
        &mut file,
        "static EXT_BY_TYPE: phf::Map<&'static str, &[&'static str]> = \n{};\n",
        ext_by_type.build()
    )
    .unwrap();

    writeln!(
        &mut file,
        "static TYPE_BY_EXT: phf::Map<&'static str, &'static str> = \n{};\n",
        type_by_ext.build()
    )
    .unwrap();
}
