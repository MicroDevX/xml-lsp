use roxmltree::{Document, Node};
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position};

pub fn get_hover_info(text: &str, position: Position) -> Option<Hover> {
    let offset = position_to_byte_offset(text, position)?;

    let doc = Document::parse(text).ok()?;

    let node = find_node_at_offset(&doc, offset)?;

    let hover_text = build_markdown_info(node);

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: hover_text,
        }),
        range: None,
    })
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
    None
}

fn find_node_at_offset<'a, 'input>(
    doc: &'a Document<'input>,
    offset: usize,
) -> Option<Node<'a, 'input>> {
    let mut current = doc.root_element();

    if !current.range().contains(&offset) {
        return None;
    }

    loop {
        let mut found_child = false;
        for child in current.children() {
            if child.is_element() && child.range().contains(&offset) {
                current = child;
                found_child = true;
                break;
            }
        }
        if !found_child {
            break;
        }
    }

    Some(current)
}

fn build_markdown_info(node: Node) -> String {
    let tag_name = node.tag_name().name();
    let attrs_count = node.attributes().count();
    let children_count = node.children().filter(|c| c.is_element()).count();

    let mut path = Vec::new();
    let mut parent = node.parent_element();
    while let Some(p) = parent {
        path.push(p.tag_name().name());
        parent = p.parent_element();
    }
    path.reverse();

    let path_str = if path.is_empty() {
        format!("/{}", tag_name)
    } else {
        format!("/{}/{}", path.join("/"), tag_name)
    };

    format!(
        "**Element:** `<{}>`\n\n**Path:** `{}`\n\n---\n* **Attributes:** {}\n* **Child Elements:** {}",
        tag_name, path_str, attrs_count, children_count
    )
}
