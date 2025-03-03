use mdbook::{preprocess::CmdPreprocessor, BookItem};

/// Read a mdbook CmdPreprocessor input from stdin
/// and write the output to stdout
pub fn run_highlight() -> anyhow::Result<()> {
    eprintln!("Running skybook highlighter");
    let (_ctx, mut book) = CmdPreprocessor::parse_input(std::io::stdin())?;

    let mut errors = Vec::new();

    book.for_each_mut(|item| {
        if let Err(e) = process_book_item(item) {
            errors.push(e);
        }
    });

    if !errors.is_empty() {
        for error in errors {
            eprintln!("Error: {}", error);
        }
        std::process::exit(1);
    }

    serde_json::to_writer(std::io::stdout(), &book)?;

    Ok(())
}

fn process_book_item(item: &mut BookItem) -> anyhow::Result<()> {
    if let BookItem::Chapter(chapter) = item {
        process_chapter_content(&mut chapter.content)?;
    }

    Ok(())
}

fn process_chapter_content(content: &mut String) -> anyhow::Result<()> {
    handle_skybook_script_highlighting(content)?;

    Ok(())
}

/// Handle code blocks with the `skybook` language, using the skybook parser
fn handle_skybook_script_highlighting(content: &mut String) -> anyhow::Result<()> {
    let old_content = std::mem::take(content);
    let mut is_in_skybook_block = false;
    let mut skybook_block_content = String::new();
    for line in old_content.lines() {
        if is_in_skybook_block {
            if line.trim() == "```" {
                is_in_skybook_block = false;
                let script_block = parse_skybook_script(&skybook_block_content, true)?;
                content.push_str(&script_block);
                content.push('\n');
                continue;
            }
            skybook_block_content.push_str(line);
            skybook_block_content.push('\n');
            continue;
        }
        if !line.trim_start().starts_with("```skybook") {
            // handle inline <skyb></skyb> tags
            if !line.contains("<skyb>") {
                content.push_str(line);
                content.push('\n');
                continue;
            }
            let mut rest_idx = 0;
            while let Some(start_idx) = line[rest_idx..].find("<skyb>") {
                let script_start_idx = rest_idx + start_idx + 6;
                let length = line[script_start_idx..]
                    .find("</skyb>")
                    .unwrap_or(line.len());
                let script = &line[script_start_idx..script_start_idx + length];
                let script_block = parse_skybook_script(script, false)?;
                if start_idx != 0 {
                    content.push_str(&line[rest_idx..rest_idx + start_idx]);
                }
                content.push_str(&script_block);
                rest_idx = script_start_idx + length + 7;
            }
            if rest_idx < line.len() {
                content.push_str(&line[rest_idx..]);
            }
            content.push('\n');
            continue;
        }
        is_in_skybook_block = true;
    }

    Ok(())
}

fn parse_skybook_script(script: &str, pre: bool) -> anyhow::Result<String> {
    let mut output = if pre {
        String::from("<pre><code>")
    } else {
        String::from("<code>")
    };
    let tokens = skybook_parser::parse_tokens(script);
    let mut idx = 0;
    for (span, token) in tokens {
        // text before the token
        if span.lo > idx {
            output.push_str(&escape_html(&script[idx..span.lo]));
        }
        let token_class = get_skybook_token_css_class(token);
        // the token
        output.push_str(&format!(
            "<span class=\"{}\">{}</span>",
            token_class,
            escape_html(&script[span.lo..span.hi])
        ));
        idx = span.hi;
    }
    // text after the last token
    if idx < script.len() {
        output.push_str(&escape_html(&script[idx..]));
    }

    output.push_str("</code>");
    if pre {
        output.push_str("</pre>");
    }

    Ok(output)
}

fn escape_html(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

fn get_skybook_token_css_class(ty: skybook_parser::syn::TT) -> String {
    let token_name = serde_json::to_string(&ty)
        .unwrap_or_default()
        .replace("\"", "");

    format!("skybook-tt-{}", token_name)
}
