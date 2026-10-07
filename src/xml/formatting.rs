use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::Event;
use std::io::Cursor;
use tower_lsp::lsp_types::{Position, Range, TextEdit};

pub fn format_xml_document(text: &str) -> Option<Vec<TextEdit>> {
    let mut reader = Reader::from_str(text);
    reader.config_mut().trim_text(true);

    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);

    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(event) => {
                if writer.write_event(event).is_err() {
                    return None;
                }
            }
            Err(_) => return None,
        }
    }

    let result = writer.into_inner().into_inner();
    let formatted_text = String::from_utf8(result).ok()?;

    let line_count = text.lines().count() as u32;
    let last_line_len = text.lines().last().unwrap_or("").len() as u32;

    let full_range = Range::new(
        Position::new(0, 0),
        Position::new(line_count, last_line_len),
    );

    Some(vec![TextEdit {
        range: full_range,
        new_text: formatted_text,
    }])
}
