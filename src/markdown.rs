use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

pub fn parse_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(content, options);
    let mut step_count = 0;
    let mut list_depth = 0;
    let mut in_code_block = false;
    let mut in_ordered_list = false;
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
                html.push_str(&format!(r#"<h{} class="markdown-{}">"#, level_num, class));
            }
            Event::End(TagEnd::Heading(level)) => {
                let level_num = level as u8 + 1;
                html.push_str(&format!("</h{}>", level_num));
            }
            Event::Start(Tag::List(ordered)) => {
                list_depth += 1;
                step_count = 0;
                if ordered.is_some() {
                    in_ordered_list = true;
                    html.push_str(r#"<ol class="ordered-list">"#);
                } else {
                    in_ordered_list = false;
                    html.push_str(r#"<ul class="unordered-list">"#);
                }
            }
            Event::End(TagEnd::List(_)) => {
                if in_ordered_list {
                    html.push_str("</ol>");
                } else {
                    html.push_str("</ul>");
                }
                list_depth -= 1;
                step_count = 0;
            }
            Event::Start(Tag::Item) => {
                step_count += 1;
                if list_depth == 1 && in_ordered_list {
                    html.push_str(&format!(
                        r#"<li class="ordered-step">
                            <div class="step-indicator">{}</div>
                            <div class="step-content">"#,
                        step_count
                    ));
                } else {
                    html.push_str(r#"<li class="nested-item">"#);
                }
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
                    html.push_str(&text.replace('<', "&lt;").replace('>', "&gt;"));
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                html.push_str("<br>");
            }
            _ => {}
        }
    }

    html
}

pub fn preprocess_markdown(content: String) -> String {
    content
        .replace("\r\n", "\n")
        .replace("\n#", "\n\n#")
        .replace("\n```", "\n\n```")
}
