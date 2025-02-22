/// Parse a block literal, but only if it has the given tag.
///
/// Trailing whitespaces are removed from the content. Newlines
/// are normalized to `\n`.
pub fn parse_block_literal_with_tag(block_literal: &str, tag: &str) -> Option<String> {
    let (tag, content) = parse_block_literal_interal(block_literal, tag);
    if tag.is_empty() {
        return None;
    }
    let content = content.trim_end().lines().collect::<Vec<_>>().join("\n");
    Some(content)
}

/// Parse a block literal content to the tag and the content.
///
/// Trailing whitespaces are removed from the content. Newlines
/// are normalized to `\n`.
pub fn parse_block_literal(block_literal: &str) -> (&str, String) {
    let (tag, content) = parse_block_literal_interal(block_literal, "");
    let content = content.trim_end().lines().collect::<Vec<_>>().join("\n");
    (tag, content)
}

/// Parse a block literal content to the tag and the content.
///
/// Return tag will be empty string for invalid input
///
/// Trailing whitespaces are removed from the content. Newlines
/// are normalized to `\n`.
fn parse_block_literal_interal<'a>(block_literal: &'a str, expect_tag: &str) -> (&'a str, &'a str) {
    let block_literal = block_literal.strip_prefix("'''").unwrap_or(block_literal);
    let block_literal = block_literal.strip_suffix("'''").unwrap_or(block_literal);
    let first_newline = match block_literal.find('\n') {
        Some(i) => i,
        None => {
            // this shouldn't be possible because the block literal
            // requires one newline to be parsed
            if expect_tag.is_empty() {
                return (block_literal, "");
            } else {
                return ("", "");
            }
        }
    };
    let tag = &block_literal[..first_newline];
    if !expect_tag.is_empty() && tag != expect_tag {
        return ("", "");
    }
    let content = &block_literal[first_newline + 1..];

    (tag, content)
}
