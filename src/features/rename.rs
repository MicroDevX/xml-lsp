use roxmltree::Document;
use std::collections::HashMap;
use tower_lsp::lsp_types::{Position, Range, TextEdit, Url, WorkspaceEdit};

pub fn get_rename_edit(
    text: &str,
    uri: Url,
    position: Position,
    new_name: String,
) -> Option<WorkspaceEdit> {
    let offset = position_to_byte_offset(text, position)?;

    let doc = Document::parse(text).ok()?;

    let node = find_node_at_offset(&doc, offset)?;

    if !node.is_element() {
        return None;
    }

    let mut edits = Vec::new();
    let tag_name = node.tag_name().name();

    let open_start_offset = node.range().start + 1;
    let open_end_offset = open_start_offset + tag_name.len();

    if offset >= open_start_offset && offset <= open_end_offset
        || is_in_close_tag(&node, offset, text)
    {
        if let (Some(start), Some(end)) = (
            offset_to_position(text, open_start_offset),
            offset_to_position(text, open_end_offset),
        ) {
            edits.push(TextEdit {
                range: Range::new(start, end),
                new_text: new_name.clone(),
            });
        }

        if text[node.range()].ends_with(&format!("</{}>", tag_name)) {
            let close_end_offset = node.range().end - 1; // قبل الـ '>'
            let close_start_offset = close_end_offset - tag_name.len();

            if let (Some(start), Some(end)) = (
                offset_to_position(text, close_start_offset),
                offset_to_position(text, close_end_offset),
            ) {
                edits.push(TextEdit {
                    range: Range::new(start, end),
                    new_text: new_name,
                });
            }
        }
    } else {
        return None;
    }

    if edits.is_empty() {
        None
    } else {
        let mut changes = HashMap::new();
        changes.insert(uri, edits);

        Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        })
    }
}

fn is_in_close_tag(node: &roxmltree::Node, offset: usize, text: &str) -> bool {
    let tag_name = node.tag_name().name();
    let expected_close = format!("</{}>", tag_name);
    let node_text = &text[node.range()];

    if node_text.ends_with(&expected_close) {
        let close_start = node.range().end - expected_close.len();
        let close_end = node.range().end;
        offset >= close_start && offset <= close_end
    } else {
        false
    }
}

fn position_to_byte_offset(text: &str, pos: Position) -> Option<usize> {
    /* ... */
    None
}
fn offset_to_position(text: &str, offset: usize) -> Option<Position> {
    /* ... */
    None
}
fn find_node_at_offset<'a, 'input>(
    doc: &'a Document<'input>,
    offset: usize,
) -> Option<roxmltree::Node<'a, 'input>> {
    /* ... */
    None
}
