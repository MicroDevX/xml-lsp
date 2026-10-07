use roxmltree::{Document, Node};
use tower_lsp::lsp_types::{DocumentSymbol, DocumentSymbolResponse, Position, Range, SymbolKind};

#[allow(deprecated)]
pub fn get_document_symbols(text: &str) -> Option<DocumentSymbolResponse> {
    let doc = Document::parse(text).ok()?;
    let root = doc.root_element();

    let root_symbol = build_symbol_tree(&root, text)?;

    Some(DocumentSymbolResponse::Nested(vec![root_symbol]))
}

#[allow(deprecated)]
fn build_symbol_tree(node: &Node, text: &str) -> Option<DocumentSymbol> {
    let tag_name = node.tag_name().name().to_string();

    let full_range = get_node_range(node, text)?;

    let selection_range = full_range;

    let mut detail = String::new();
    for attr in node.attributes() {
        detail.push_str(&format!("{}=\"{}\" ", attr.name(), attr.value()));
    }
    let detail = if detail.is_empty() {
        None
    } else {
        Some(detail.trim().to_string())
    };

    let mut children = Vec::new();
    for child in node.children() {
        if child.is_element() {
            if let Some(child_symbol) = build_symbol_tree(&child, text) {
                children.push(child_symbol);
            }
        }
    }

    Some(DocumentSymbol {
        name: tag_name,
        detail,
        kind: SymbolKind::OBJECT,
        tags: None,
        deprecated: None,
        range: full_range,
        selection_range,
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
    })
}

fn get_node_range(node: &Node, text: &str) -> Option<Range> {
    let start_offset = node.range().start;
    let end_offset = node.range().end;

    let start_pos = offset_to_position(text, start_offset)?;
    let end_pos = offset_to_position(text, end_offset)?;

    Some(Range::new(start_pos, end_pos))
}

fn offset_to_position(text: &str, offset: usize) -> Option<Position> {
    if offset > text.len() {
        return None;
    }

    let slice = &text[..offset];
    let line = slice.chars().filter(|&c| c == '\n').count() as u32;

    let last_line = slice.lines().last().unwrap_or("");
    let character = last_line.encode_utf16().count() as u32;

    Some(Position::new(line, character))
}
