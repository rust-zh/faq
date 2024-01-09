use crate::CODE_HIGHLIGHT_CLASS_STYLE;
use once_cell::sync::Lazy;
use pulldown_cmark::{BrokenLink, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};
use std::collections::HashMap;
use std::io::{self, Write};
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use v_htmlescape::escape;

pub struct EntryWriter<'a, W> {
    output: W,
    name: &'a str,
    content: &'a str,
    glossary: &'a HashMap<String, String>,
    current_syntax: Option<&'static SyntaxReference>,
}

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

impl<'a, W> EntryWriter<'a, W>
where
    W: Write,
{
    pub fn new(
        output: W,
        name: &'a str,
        content: &'a str,
        glossary: &'a HashMap<String, String>,
    ) -> Self {
        EntryWriter {
            output,
            name,
            content,
            glossary,
            current_syntax: None,
        }
    }

    pub fn run(mut self) -> io::Result<()> {
        let glossary = self.glossary;
        let mut broken_link_callback = move |link: BrokenLink<'_>| {
            glossary
                .get(&*link.reference)
                .map(|title| ("".into(), title.as_str().into()))
        };
        Parser::new_with_broken_link_callback(
            self.content,
            Options::empty(),
            Some(&mut broken_link_callback),
        )
        .try_for_each(|event| self.handle_event(event))?;
        Ok(())
    }

    fn handle_event(&mut self, event: Event<'_>) -> io::Result<()> {
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
                    for line in text.split_inclusive('\n') {
                        generator
                            .parse_html_for_line_which_includes_newline(line)
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
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

    fn start_tag(&mut self, tag: Tag<'_>) -> io::Result<()> {
        let output = &mut self.output;
        match tag {
            Tag::Paragraph => write!(output, "<p>"),
            Tag::Heading(HeadingLevel::H1, _, _) => {
                write!(output, r##"<h2 id="{0}"><a href="#{0}">"##, self.name)
            }
            Tag::Heading(level, _, _) => write!(output, "<h{}>", level as i32 + 1),
            Tag::Link(_, dest, title) => {
                write!(output, "<a")?;
                if !dest.is_empty() {
                    write!(output, r#" href="{}""#, dest)?;
                }
                if !title.is_empty() {
                    write!(output, r#" title="{}""#, title)?;
                }
                write!(output, ">")
            }
            Tag::CodeBlock(CodeBlockKind::Fenced(lang)) => {
                assert!(self.current_syntax.is_none());
                self.current_syntax = match SYNTAX_SET.find_syntax_by_token(&lang) {
                    None => panic!("unknown language `{}`", lang),
                    Some(syntax) => Some(syntax),
                };
                write!(output, r#"<pre class="code">"#)
            }
            Tag::List(None) => write!(output, "<ul>"),
            Tag::Item => write!(output, "<li>"),
            tag => unimplemented!("tag {:?} not supported", tag),
        }
    }

    fn end_tag(&mut self, tag: Tag<'_>) -> io::Result<()> {
        let output = &mut self.output;
        match tag {
            Tag::Paragraph => write!(output, "</p>"),
            Tag::Heading(HeadingLevel::H1, _, _) => write!(output, "</a></h2>"),
            Tag::Heading(level, _, _) => write!(output, "</h{}>", level as i32 + 1),
            Tag::Link(_, _, _) => write!(output, "</a>"),
            Tag::CodeBlock(CodeBlockKind::Fenced(_)) => {
                self.current_syntax = None;
                write!(output, "</pre>")
            }
            Tag::List(None) => write!(output, "</ul>"),
            Tag::Item => write!(output, "</li>"),
            tag => unimplemented!("tag {:?} not supported", tag),
        }
    }
}
