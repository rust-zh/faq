use pulldown_cmark::{Event, Parser, Tag};
use std::io::{self, Write};
use v_htmlescape::escape;

pub struct EntryWriter<'n, 'c, W> {
    output: W,
    name: &'n str,
    content: &'c str,
}

impl<'n, 'c, W> EntryWriter<'n, 'c, W>
where
    W: Write,
{
    pub fn new(output: W, name: &'n str, content: &'c str) -> Self {
        EntryWriter {
            output,
            name,
            content,
        }
    }

    pub fn run(mut self) -> io::Result<()> {
        Parser::new(self.content)
            .map(|event| self.handle_event(event))
            .collect()
    }

    fn handle_event(&mut self, event: Event) -> io::Result<()> {
        let output = &mut self.output;
        match event {
            Event::Start(tag) => self.start_tag(tag),
            Event::End(tag) => self.end_tag(tag),
            Event::Text(text) => write!(output, "{}", escape(&text)),
            Event::Code(code) => write!(output, "<code>{}</code>", escape(&code)),
            Event::Html(html) => write!(output, "{}", html),
            Event::SoftBreak => todo!(),
            Event::HardBreak => write!(output, "<br>"),
            Event::Rule => unimplemented!("rule not supported"),
            Event::FootnoteReference(_) => unimplemented!("footnote not supported"),
            Event::TaskListMarker(_) => unimplemented!("task list marker not supported"),
        }
    }

    fn start_tag(&mut self, tag: Tag) -> io::Result<()> {
        let output = &mut self.output;
        match tag {
            Tag::Paragraph => write!(output, "<p>"),
            Tag::Heading(level) if level == 1 => {
                write!(output, r##"<h2 id="{0}"><a href="#{0}">"##, self.name)
            }
            Tag::Heading(level) => write!(output, "<h{}>", level + 1),
            Tag::Link(_, dest, _) => write!(output, r#"<a href="{}">"#, dest),
            tag => unimplemented!("tag {:?} not supported", tag),
        }
    }

    fn end_tag(&mut self, tag: Tag) -> io::Result<()> {
        let output = &mut self.output;
        match tag {
            Tag::Paragraph => write!(output, "</p>"),
            Tag::Heading(level) if level == 1 => write!(output, "</a></h2>"),
            Tag::Heading(level) => write!(output, "</h{}>", level + 1),
            Tag::Link(_, _, _) => write!(output, "</a>"),
            tag => unimplemented!("tag {:?} not supported", tag),
        }
    }
}
