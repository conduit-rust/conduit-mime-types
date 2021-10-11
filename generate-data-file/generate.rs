use std::collections::BTreeMap;
use std::io::Write;

static JSON: &str = include_str!("../data/mime.json");

fn main() -> anyhow::Result<()> {
    let deserialized: BTreeMap<String, Vec<String>> = serde_json::from_str(JSON)?;

    let mut file = std::fs::File::create("../src/data.rs")?;

    file.write_all(b"#[rustfmt::skip]\n")?;
    file.write_all(b"pub static MIME_TYPES: &[(&str, &[&str])] = &[\n")?;
    for (mime_type, exts) in deserialized.iter() {
        file.write_all(b"    (\"")?;
        file.write_all(mime_type.as_bytes())?;
        file.write_all(b"\", &[")?;
        for ext in exts {
            if ext != &exts[0] {
                file.write_all(b", ")?;
            }

            file.write_all(b"\"")?;
            file.write_all(ext.as_bytes())?;
            file.write_all(b"\"")?;
        }
        file.write_all(b"]),\n")?;
    }
    file.write_all(b"];\n")?;

    Ok(())
}
