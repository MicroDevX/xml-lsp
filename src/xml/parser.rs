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
