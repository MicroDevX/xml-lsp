use crate::features::completion::get_completion_items;
use crate::features::hover::get_hover_info;
use crate::features::rename::get_rename_edit;
use crate::features::symbols::get_document_symbols;
use crate::xml::formatting::format_xml_document;
use crate::xml::parser::parse_xml;
use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub document_map: DashMap<Url, String>,
}

impl Backend {
    async fn on_change(&self, uri: Url, text: String, version: i32) {
        self.document_map.insert(uri.clone(), text.clone());

        let parse_result = parse_xml(&text);

        self.client
            .publish_diagnostics(uri.clone(), parse_result.diagnostics, Some(version))
            .await;

        self.client
            .log_message(MessageType::LOG, format!("Parsed {}", uri.as_str()))
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                document_symbol_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["<".to_string(), "/".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "xml-lsp server initialized and ready!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(
            params.text_document.uri,
            params.text_document.text,
            params.text_document.version,
        )
        .await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.pop() {
            self.on_change(
                params.text_document.uri,
                change.text,
                params.text_document.version,
            )
            .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        self.document_map.remove(&uri);

        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;

        let text = match self.document_map.get(&uri) {
            Some(doc) => doc.value().clone(),
            None => return Ok(None),
        };

        let edits = format_xml_document(&text);

        Ok(edits)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let text = match self.document_map.get(&uri) {
            Some(doc) => doc.value().clone(),
            None => return Ok(None),
        };

        let hover_result = get_hover_info(&text, position);

        Ok(hover_result)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let text = match self.document_map.get(&uri) {
            Some(doc) => doc.value().clone(),
            None => return Ok(None),
        };

        if let Some(items) = get_completion_items(&text, position) {
            Ok(Some(CompletionResponse::Array(items)))
        } else {
            Ok(None)
        }
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;

        let text = match self.document_map.get(&uri) {
            Some(doc) => doc.value().clone(),
            None => return Ok(None),
        };

        let symbols = get_document_symbols(&text);

        Ok(symbols)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;

        let text = match self.document_map.get(&uri) {
            Some(doc) => doc.value().clone(),
            None => return Ok(None),
        };

        let edit = get_rename_edit(&text, uri, position, new_name);

        Ok(edit)
    }
}
