use crate::entry_writer::EntryWriter;
use anyhow::{anyhow, Context as _, Result};
use glob::glob;
use handlebars::Handlebars;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::Path;
use std::str;
use syntect::highlighting::ThemeSet;
use syntect::html::{css_for_theme_with_class_style, ClassStyle};

mod entry_writer;

#[derive(Serialize)]
struct Data {
    entries: Vec<String>,
}

const CODE_HIGHLIGHT_CLASS_STYLE: ClassStyle = ClassStyle::SpacedPrefixed { prefix: "s-" };

fn main() -> Result<()> {
    env::set_current_dir(env::var("CARGO_MANIFEST_DIR")?)?;

    let glossary = fs::read_to_string("content/glossary.toml").context("failed to read glossary file")?;
    let glossary: HashMap<String, String> =
        toml::from_str(&glossary).context("failed to parse glossary file")?;

    let mut handlebars = Handlebars::new();
    handlebars.register_helper(
        "render_entry",
        Box::new(
            |helper: &handlebars::Helper,
             _: &Handlebars,
             _: &handlebars::Context,
             _: &mut handlebars::RenderContext,
             out: &mut dyn handlebars::Output|
             -> handlebars::HelperResult {
                let name = helper
                    .param(0)
                    .and_then(|param| param.value().as_str())
                    .ok_or_else(|| handlebars::RenderError::new("name invalid"))?;
                let content = fs::read_to_string(format!("content/{}.md", name))?;
                EntryWriter::new(OutputWriter(out), name, &content, &glossary).run()?;
                Ok(())
            },
        ),
    );

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
            fs::hard_link(entry.path(), out.join(entry.file_name()))
                .with_context(|| format!("failed to link {:?}", entry.file_name()))?;
        }
    }

    // Write style sheet for code highlight.
    let theme_set = ThemeSet::load_defaults();
    let theme = theme_set.themes.get("Solarized (light)").unwrap();
    fs::write(
        out.join("highlight.css"),
        css_for_theme_with_class_style(theme, CODE_HIGHLIGHT_CLASS_STYLE)?,
    )?;

    Ok(())
}

struct OutputWriter<'a, T: ?Sized>(&'a mut T);

impl<'a, T: ?Sized> io::Write for OutputWriter<'a, T>
where
    T: handlebars::Output,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(str::from_utf8(buf).unwrap())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
