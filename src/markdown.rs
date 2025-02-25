use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use regex::Regex;

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-")
        .trim_matches('-')
        .to_string()
}

fn generate_toc(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);
    let mut headings = Vec::new();
    let mut current_text = String::new();
    let mut in_heading = false;
    let mut current_level = 0u8;
    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                current_level = level as u8;
                current_text.clear();
            }
            Event::Text(text) => {
                if in_heading {
                    current_text.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if in_heading {
                    let id = slugify(&current_text);
                    headings.push((current_level, current_text.clone(), id));
                    in_heading = false;
                }
            }
            _ => {}
        }
    }
    let mut toc = String::from(r##"<nav class="table-of-contents"><ul>"##);
    for (level, text, id) in headings {
        toc.push_str(&format!(
            r##"<li class="toc-item level-{}"><a href="#{}">{}</a></li>"##,
            level, id, text
        ));
    }
    toc.push_str("</ul></nav>");
    toc
}

pub fn parse_markdown(content: &str) -> String {
    // if content contains [toc] marker, generate toc html
    let toc_html = if content.contains("[toc]") {
        generate_toc(content)
    } else {
        String::new()
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(content, options);
    let mut heading_counter = 1;
    let mut in_code_block = false;
    let mut html = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let class = match level {
                    HeadingLevel::H1 => "heading-1",
                    HeadingLevel::H2 => "heading-2",
                    HeadingLevel::H3 => "heading-3",
                    _ => "heading-4",
                };
                let level_num = level as u8 + 1;
                let id = format!("heading-{}", heading_counter);
                heading_counter += 1;
                html.push_str(&format!(
                    r#"<h{} id="{}" class="markdown-{}">"#,
                    level_num, id, class
                ));
            }
            Event::End(TagEnd::Heading(level)) => {
                let level_num = level as u8 + 1;
                html.push_str(&format!("</h{}>", level_num));
            }
            Event::Start(Tag::List(ordered)) => {
                html.push_str(if ordered.is_some() {
                    r#"<ol class="ordered-list">"#
                } else {
                    r#"<ul class="unordered-list">"#
                });
            }
            Event::End(TagEnd::List(_)) => {
                html.push_str(if html.ends_with("</li>") {
                    "</ol>"
                } else {
                    "</ul>"
                });
            }
            Event::Start(Tag::Item) => {
                html.push_str(r#"<li class="list-item">"#);
            }
            Event::End(TagEnd::Item) => {
                html.push_str("</li>");
            }
            Event::Start(Tag::CodeBlock { .. }) => {
                in_code_block = true;
                html.push_str(
                    r#"<div class="command-window">
    <div class="command-header">
        <div class="window-controls">
            <span class="window-control close"></span>
            <span class="window-control minimize"></span>
            <span class="window-control maximize"></span>
        </div>
    </div>
    <div class="command-body">
        <div class="command-line">
            <span class="terminal-prompt">$</span>
            <span class="terminal-command">"#,
                );
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                html.push_str("</span></div></div></div>");
            }
            Event::Start(Tag::Paragraph) => {
                html.push_str(r#"<p class="markdown-paragraph">"#);
            }
            Event::End(TagEnd::Paragraph) => {
                html.push_str("</p>");
            }
            Event::Start(Tag::Emphasis) => {
                html.push_str(r#"<em class="markdown-emphasis">"#);
            }
            Event::End(TagEnd::Emphasis) => {
                html.push_str("</em>");
            }
            Event::Start(Tag::Strong) => {
                html.push_str(r#"<strong class="markdown-strong">"#);
            }
            Event::End(TagEnd::Strong) => {
                html.push_str("</strong>");
            }
            Event::Start(Tag::Link {
                dest_url, title, ..
            }) => {
                html.push_str(&format!(
                    r#"<a href="{}" class="markdown-link" title="{}">"#,
                    dest_url, title
                ));
            }
            Event::End(TagEnd::Link) => {
                html.push_str("</a>");
            }
            Event::Text(text) => {
                if in_code_block {
                    html.push_str(&text);
                } else {
                    html.push_str(&process_text(&text));
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                html.push_str("<br>");
            }
            _ => {}
        }
    }

    // replace [toc] marker with generated toc if present
    html.replace("[toc]", &toc_html)
}

fn process_text(text: &str) -> String {
    // use regex to find isolated numbers and wrap them
    let re = Regex::new(r"(\b\d+\b)").unwrap();
    let replaced = re.replace_all(
        text,
        r#"<span class="count-up" data-final-value="$1">0</span>"#,
    );
    // preserve newlines with <br>
    replaced.split('\n').collect::<Vec<&str>>().join("<br>")
}

pub fn preprocess_markdown(content: String) -> String {
    content
        .replace("\r\n", "\n")
        .replace("\n#", "\n\n#")
        .replace("\n```", "\n\n```")
}
