use crate::CODE_HIGHLIGHT_CLASS_STYLE;
use once_cell::sync::Lazy;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use std::io::{self, Write};
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use v_htmlescape::escape;

pub struct EntryWriter<'n, 'c, W> {
    output: W,
    name: &'n str,
    content: &'c str,
    current_syntax: Option<&'static SyntaxReference>,
}

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

impl<'n, 'c, W> EntryWriter<'n, 'c, W>
where
    W: Write,
{
    pub fn new(output: W, name: &'n str, content: &'c str) -> Self {
        EntryWriter {
            output,
            name,
            content,
            current_syntax: None,
        }
    }

    pub fn run(mut self) -> io::Result<()> {
        Parser::new(self.content)
            .map(|event| self.handle_event(event))
            .collect()
    }

    fn handle_event(&mut self, event: Event<'c>) -> io::Result<()> {
        let output = &mut self.output;
        match event {
            Event::Start(tag) => self.start_tag(tag),
            Event::End(tag) => self.end_tag(tag),
            Event::Text(text) => {
                if let Some(syntax) = self.current_syntax {
                    let mut generator = ClassedHTMLGenerator::new_with_class_style(
                        syntax,
                        &SYNTAX_SET,
                        CODE_HIGHLIGHT_CLASS_STYLE,
                    );
                    for line in text.lines() {
                        generator.parse_html_for_line(line);
                    }
                    write!(output, "{}", generator.finalize())
                } else {
                    write!(output, "{}", escape(&text))
                }
            }
            Event::Code(code) => write!(output, "<code>{}</code>", escape(&code)),
            Event::Html(html) => write!(output, "{}", html),
            Event::SoftBreak => todo!(),
            Event::HardBreak => write!(output, "<br>"),
            Event::Rule => unimplemented!("rule not supported"),
            Event::FootnoteReference(_) => unimplemented!("footnote not supported"),
            Event::TaskListMarker(_) => unimplemented!("task list marker not supported"),
        }
    }

    fn start_tag(&mut self, tag: Tag<'c>) -> io::Result<()> {
        let output = &mut self.output;
        match tag {
            Tag::Paragraph => write!(output, "<p>"),
            Tag::Heading(level) if level == 1 => {
                write!(output, r##"<h2 id="{0}"><a href="#{0}">"##, self.name)
            }
            Tag::Heading(level) => write!(output, "<h{}>", level + 1),
            Tag::Link(_, dest, _) => write!(output, r#"<a href="{}">"#, dest),
            Tag::CodeBlock(CodeBlockKind::Fenced(lang)) => {
                assert!(self.current_syntax.is_none());
                self.current_syntax = match SYNTAX_SET.find_syntax_by_token(&lang) {
                    None => panic!("unknown language `{}`", lang),
                    Some(syntax) => Some(syntax),
                };
                write!(output, r#"<pre class="code">"#)
            }
            tag => unimplemented!("tag {:?} not supported", tag),
        }
    }

    fn end_tag(&mut self, tag: Tag<'c>) -> io::Result<()> {
        let output = &mut self.output;
        match tag {
            Tag::Paragraph => write!(output, "</p>"),
            Tag::Heading(level) if level == 1 => write!(output, "</a></h2>"),
            Tag::Heading(level) => write!(output, "</h{}>", level + 1),
            Tag::Link(_, _, _) => write!(output, "</a>"),
            Tag::CodeBlock(CodeBlockKind::Fenced(_)) => {
                self.current_syntax = None;
                write!(output, "</pre>")
            }
            tag => unimplemented!("tag {:?} not supported", tag),
        }
    }
}
