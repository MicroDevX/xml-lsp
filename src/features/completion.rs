use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position};

pub fn get_completion_items(text: &str, position: Position) -> Option<Vec<CompletionItem>> {
    let offset = position_to_byte_offset(text, position)?;
    let text_before_cursor = &text[..offset];

    if text_before_cursor.ends_with("</") {
        if let Some(last_open_tag) = find_last_unclosed_tag(text_before_cursor) {
            let item = CompletionItem {
                label: format!("{}>", last_open_tag),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(format!("Close tag </{}>", last_open_tag)),
                insert_text: Some(format!("{}>", last_open_tag)),
                ..Default::default()
            };
            return Some(vec![item]);
        }
    }

    if text_before_cursor.ends_with('<') {
        let snippet_item = CompletionItem {
            label: "element".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("Insert XML element with closing tag".to_string()),
            insert_text: Some("$1>$2</$1>".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        };
        return Some(vec![snippet_item]);
    }

    None
}

fn position_to_byte_offset(text: &str, pos: Position) -> Option<usize> {
    let mut current_line = 0;
    let mut current_col = 0;

    for (i, c) in text.char_indices() {
        if current_line == pos.line as usize && current_col == pos.character as usize {
            return Some(i);
        }
        if c == '\n' {
            current_line += 1;
            current_col = 0;
        } else {
            current_col += c.len_utf16() as usize;
        }
    }

    if current_line == pos.line as usize && current_col == pos.character as usize {
        return Some(text.len());
    }

    None
}

fn find_last_unclosed_tag(text_before_cursor: &str) -> Option<String> {
    let mut stack: Vec<String> = Vec::new();
    let mut chars = text_before_cursor.char_indices().peekable();

    while let Some((_, ch)) = chars.next() {
        if ch == '<' {
            if chars.peek().map_or(false, |&(_, c)| c == '!' || c == '?') {
                continue;
            }

            let is_closing = chars.peek().map_or(false, |&(_, c)| c == '/');
            if is_closing {
                chars.next();
            }

            let mut tag_name = String::new();
            while let Some(&(_, c)) = chars.peek() {
                if c.is_alphanumeric() || c == '_' || c == '-' || c == ':' {
                    tag_name.push(c);
                    chars.next();
                } else {
                    break;
                }
            }

            if tag_name.is_empty() {
                continue;
            }

            let mut is_self_closing = false;
            while let Some((_, c)) = chars.next() {
                if c == '/' && chars.peek().map_or(false, |&(_, next_c)| next_c == '>') {
                    is_self_closing = true;
                    chars.next();
                    break;
                }
                if c == '>' {
                    break;
                }
            }

            if !is_self_closing {
                if is_closing {
                    if let Some(pos) = stack.iter().rposition(|t| t == &tag_name) {
                        stack.truncate(pos);
                    }
                } else {
                    stack.push(tag_name);
                }
            }
        }
    }

    stack.pop()
}
