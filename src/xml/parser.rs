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

// testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_xml() {
        let xml = "<root><child id=\"1\"/></root>";
        let result = parse_xml(xml);
        assert!(result.document.is_some());
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn test_invalid_xml_missing_close_tag() {
        let xml = "<root><child id=\"1\"></root>";
        let result = parse_xml(xml);
        assert!(result.document.is_none());
        assert_eq!(result.diagnostics.len(), 1);

        let diag = &result.diagnostics[0];
        assert_eq!(diag.severity, Some(DiagnosticSeverity::ERROR));
        assert!(diag.message.contains("Unexpected close tag"));
        assert_eq!(diag.range.start.line, 0);
    }
}
