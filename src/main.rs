use crate::entry_writer::EntryWriter;
use anyhow::{anyhow, Context, Result};
use glob::glob;
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

mod entry_writer;

const HEADER: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Rust 常见疑问汇总</title>
<link rel="stylesheet" href="style.css">
<h1>Rust 常见疑问汇总</h1>
"#;

const FOOTER: &str = r#"</html>"#;

fn main() -> Result<()> {
    env::set_current_dir(env::var("CARGO_MANIFEST_DIR")?)?;
    let out = Path::new("out");
    let _ = fs::remove_dir_all(out);
    fs::create_dir_all(out)?;
    for entry in fs::read_dir("res")? {
        let entry = entry?;
        fs::hard_link(&entry.path(), out.join(entry.file_name()))
            .with_context(|| format!("failed to link {:?}", entry.file_name()))?;
    }

    let output = File::create("out/index.html")?;
    let mut output = BufWriter::new(output);
    write!(&mut output, "{}", HEADER)?;

    for entry in glob("content/*.md").unwrap() {
        let path = entry?;
        let name = path
            .file_stem()
            .and_then(OsStr::to_str)
            .ok_or_else(|| anyhow!("invalid file name"))?;
        let content =
            fs::read_to_string(&path).with_context(|| format!("read content of {}", name))?;

        writeln!(&mut output, "<article>")?;
        EntryWriter::new(&mut output, name, &content).run()?;
        writeln!(&mut output, "</article>")?;
    }

    write!(&mut output, "{}", FOOTER)?;
    Ok(())
}
