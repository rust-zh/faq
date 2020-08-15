use anyhow::{anyhow, Context as _, Result};
use glob::glob;
use handlebars::Handlebars;
use serde::Serialize;
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

mod entry_renderer;
mod entry_writer;

#[derive(Serialize)]
struct Data {
    entries: Vec<String>,
}

fn main() -> Result<()> {
    env::set_current_dir(env::var("CARGO_MANIFEST_DIR")?)?;

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("render_entry", Box::new(entry_renderer::render_entry));

    let entries = glob("content/*.md")
        .unwrap()
        .map(|entry| -> Result<String> {
            let path = entry?;
            let name = path
                .file_stem()
                .and_then(OsStr::to_str)
                .ok_or_else(|| anyhow!("invalid file name: {:?}", path.file_name()))?;
            Ok(name.to_owned())
        })
        .collect::<Result<_>>()?;
    let data = Data { entries };

    let out = Path::new("out");
    let _ = fs::remove_dir_all(out);
    fs::create_dir_all(out)?;

    for entry in fs::read_dir("res")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension() == Some(OsStr::new("hbs")) {
            let template = fs::read_to_string(&path)?;
            let output = File::create(out.join(path.file_stem().unwrap()))?;
            let mut output = BufWriter::new(output);
            handlebars.render_template_to_write(&template, &data, &mut output)?;
        } else {
            fs::hard_link(&entry.path(), out.join(entry.file_name()))
                .with_context(|| format!("failed to link {:?}", entry.file_name()))?;
        }
    }

    Ok(())
}
