use roxmltree::{Document, Error as XmlError, TextPos};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

pub struct ParseResult<'a> {
    pub document: Option<Document<'a>>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn parse_xml(text: &str) -> ParseResult {
    match Document::parse(text) {
        Ok(doc) => ParseResult {
            document: Some(doc),
            diagnostics: vec![],
        },
        Err(err) => {
            let diagnostic = build_diagnostic(err);
            ParseResult {
                document: None,
                diagnostics: vec![diagnostic],
            }
        }
    }
}

fn build_diagnostic(err: XmlError) -> Diagnostic {
    let pos = err.pos();
    let range = create_lsp_range(pos);

    let message = match err {
        XmlError::UnexpectedCloseTag(expected, actual, _) => {
            format!(
                "Unexpected close tag: expected '{}', found '{}'",
                expected, actual
            )
        }
        XmlError::UnknownEntityReference(entity, _) => {
            format!("Unknown entity reference: '&{};'", entity)
        }
        XmlError::DuplicatedAttribute(attr, _) => {
            format!("Duplicated attribute: '{}'", attr)
        }
        _ => err.to_string(),
    };

    Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("xml-lsp".to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn create_lsp_range(pos: TextPos) -> Range {
    let line = if pos.row > 0 { pos.row - 1 } else { 0 };
    let character = if pos.col > 0 { pos.col - 1 } else { 0 };

    let start = Position::new(line, character);

    let end = Position::new(line, character + 1);

    Range::new(start, end)
}
