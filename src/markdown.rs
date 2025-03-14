use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
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
    // Generate the TOC HTML
    let mut toc = String::from(r##"<nav class="table-of-contents"><h4>Contents</h4><ul>"##);
    for (level, text, id) in headings.iter() {
        toc.push_str(&format!(
            r##"<li class="toc-item level-{}"><a href="#{}">{}</a></li>"##,
            level, id, text
        ));
    }
    toc.push_str("</ul></nav>");
    toc
}

fn process_text(text: &str) -> String {
    let re = Regex::new(r"(\b\d+\b)").unwrap();
    let replaced = re.replace_all(
        text,
        r#"<span class="count-up" data-final-value="$1">0</span>"#,
    );
    replaced.split('\n').collect::<Vec<_>>().join("<br>")
}

fn generate_heading_id(text: &str, counter: usize) -> String {
    let slug = slugify(text);
    if !slug.is_empty() {
        slug
    } else {
        format!("heading-{}", counter)
    }
}

pub fn parse_markdown(content: &str) -> String {
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
    let mut list_stack: Vec<bool> = Vec::new();
    let mut current_heading_text = String::new();
    let mut in_heading = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                current_heading_text.clear();

                let class = match level {
                    HeadingLevel::H1 => "heading-1",
                    HeadingLevel::H2 => "heading-2",
                    HeadingLevel::H3 => "heading-3",
                    _ => "heading-4",
                };

                let level_num = level as u8 + 1;
                html.push_str(&format!(
                    r#"<h{} class="markdown-{} markdown-heading" "#,
                    level_num, class
                ));
            }
            Event::End(TagEnd::Heading(level)) => {
                if in_heading {
                    let id = generate_heading_id(&current_heading_text, heading_counter);
                    heading_counter += 1;
                    let level_num = level as u8 + 1;
                    let class_name = match level {
                        HeadingLevel::H1 => "heading-1",
                        HeadingLevel::H2 => "heading-2",
                        HeadingLevel::H3 => "heading-3",
                        _ => "heading-4",
                    };

                    let old_tag = format!(
                        r#"<h{} class="markdown-{} markdown-heading" "#,
                        level_num, class_name
                    );

                    let new_tag = format!(
                        r#"<h{} id="{}" class="markdown-{} markdown-heading">"#,
                        level_num, id, class_name
                    );

                    html = html.replace(&old_tag, &new_tag);

                    // Use a raw string with two hashes to avoid conflicts with inner quotes
                    html.push_str(&format!(
                        r##"<a class="heading-anchor" href="#{}">&#xB6;</a></h{}>"##,
                        id, level_num
                    ));
                    in_heading = false;
                } else {
                    let level_num = level as u8 + 1;
                    html.push_str(&format!("</h{}>", level_num));
                }
            }
            Event::Start(Tag::List(ordered)) => {
                let is_ordered = ordered.is_some();
                list_stack.push(is_ordered);
                if is_ordered {
                    html.push_str(r#"<ol class="ordered-list">"#);
                } else {
                    html.push_str(r#"<ul class="unordered-list">"#);
                }
            }
            Event::End(TagEnd::List(_)) => {
                if let Some(is_ordered) = list_stack.pop() {
                    if is_ordered {
                        html.push_str("</ol>");
                    } else {
                        html.push_str("</ul>");
                    }
                }
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
                let title_attr = if title.is_empty() {
                    String::new()
                } else {
                    format!(" title=\"{}\"", title)
                };
                html.push_str(&format!(
                    r#"<a href="{}" class="markdown-link"{}>"#,
                    dest_url, title_attr
                ));
            }
            Event::End(TagEnd::Link) => {
                html.push_str("</a>");
            }
            Event::Text(text) => {
                if in_heading {
                    current_heading_text.push_str(&text);
                }
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

    html.replace("[toc]", &toc_html)
}

pub fn preprocess_markdown(content: String) -> String {
    content
        .replace("\r\n", "\n")
        .replace("\n#", "\n\n#")
        .replace("\n```", "\n\n```")
}
