use crate::entry_writer::EntryWriter;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use std::fs;
use std::io;
use std::str;

pub fn render_entry(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let name = helper
        .param(0)
        .and_then(|param| param.value().as_str())
        .ok_or_else(|| RenderError::new("name invalid"))?;
    let content = fs::read_to_string(format!("content/{}.md", name))?;
    EntryWriter::new(OutputWriter(out), name, &content).run()?;
    Ok(())
}

struct OutputWriter<'a, T: ?Sized>(&'a mut T);

impl<'a, T: ?Sized> io::Write for OutputWriter<'a, T>
where
    T: Output,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(str::from_utf8(buf).unwrap())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
