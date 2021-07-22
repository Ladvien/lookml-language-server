use lspower::{jsonrpc::Result, lsp::*, Client, LanguageServer, LspService, Server};
use serde_json::Value;
use tokio::net::TcpListener;

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[lspower::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Incremental)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    ..Default::default()
                }),
                semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(

                    SemanticTokensRegistrationOptions {
                        text_document_registration_options: TextDocumentRegistrationOptions { document_selector: None },
                        semantic_tokens_options:  SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions {
                                work_done_progress: Some(true),
                            },
                            legend: SemanticTokensLegend {
                                token_types: vec![SemanticTokenType::new("property")],
                                token_modifiers: vec![SemanticTokenModifier::new("declaration")],
                            },
                            range: Some(true),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                        },
                        static_registration_options: StaticRegistrationOptions { id: Some("lklml".to_string())},
                    }

                    // SemanticTokensOptions {
                    //     work_done_progress_options: WorkDoneProgressOptions {
                    //         work_done_progress: Some(true),
                    //     },
                    //     legend: SemanticTokensLegend {
                    //         token_types: vec![SemanticTokenType::new("property")],
                    //         token_modifiers: vec![SemanticTokenModifier::new("declaration")],
                    //     },
                    //     range: Some(true),
                    //     full: Some(SemanticTokensFullOptions::Bool(true)),
                    // },
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    ..Default::default()
                }),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::Info, "initialized!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::Info, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::Info, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::Info, "watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client.log_message(MessageType::Info, "command executed!").await;

        match self
            .client
            .apply_edit(WorkspaceEdit::default(), Default::default())
            .await
        {
            Ok(res) if res.applied => self.client.log_message(MessageType::Info, "applied").await,
            Ok(_) => self.client.log_message(MessageType::Info, "rejected").await,
            Err(err) => self.client.log_message(MessageType::Error, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, doc: DidOpenTextDocumentParams) {
        eprintln!("Opened doc.");
        // if doc.text_document.language_id == "lkml" {
        //     println!("{:?}", doc.text_document);
        // }

        // // Example of diagnostic message.
        // let diagnostics = Diagnostic {
        //     range: Range {
        //         start: Position {
        //             line: 10,
        //             character: 10,
        //         },
        //         end: Position {
        //             line: 10,
        //             character: 15,
        //         },
        //     },
        //     message: "Check, check".to_string(),
        //     severity: Some(DiagnosticSeverity::Information),
        //     code: None,
        //     code_description: None,
        //     source: None,
        //     related_information: None,
        //     tags: None,
        //     data: None,
        // };

        // self.client
        //     .publish_diagnostics(doc.text_document.uri, vec![diagnostics], None)
        //     .await;

        self.client.log_message(MessageType::Info, "file opened!").await;
    }

    async fn did_change(&self, _: DidChangeTextDocumentParams) {
        self.client.log_message(MessageType::Info, "file changed!").await;
    }

    async fn will_save(&self, _params: lsp::WillSaveTextDocumentParams) {
        log::warn!("Got a textDocument/willSave notification, but it is not implemented");
    }

    async fn will_save_wait_until(
        &self,
        _params: lsp::WillSaveTextDocumentParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::TextEdit>>> {
        log::error!("Got a textDocument/willSaveWaitUntil request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client.log_message(MessageType::Info, "file saved!").await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        eprintln!("Closed.");
        self.client.log_message(MessageType::Info, "file closed!").await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }

    async fn completion_resolve(&self, _params: lsp::CompletionItem) -> lspower::jsonrpc::Result<lsp::CompletionItem> {
        log::error!("Got a completionItem/resolve request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn hover(&self, _params: lsp::HoverParams) -> lspower::jsonrpc::Result<Option<lsp::Hover>> {
        log::error!("Got a textDocument/hover request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn signature_help(
        &self,
        _params: lsp::SignatureHelpParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::SignatureHelp>> {
        log::error!("Got a textDocument/signatureHelp request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn goto_declaration(
        &self,
        _params: lsp::request::GotoDeclarationParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::request::GotoDeclarationResponse>> {
        log::error!("Got a textDocument/declaration request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn goto_definition(
        &self,
        _params: lsp::GotoDefinitionParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::GotoDefinitionResponse>> {
        log::error!("Got a textDocument/definition request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn goto_type_definition(
        &self,
        _params: lsp::request::GotoTypeDefinitionParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::request::GotoTypeDefinitionResponse>> {
        log::error!("Got a textDocument/typeDefinition request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn goto_implementation(
        &self,
        _params: lsp::request::GotoImplementationParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::request::GotoImplementationResponse>> {
        log::error!("Got a textDocument/implementation request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn references(&self, _params: lsp::ReferenceParams) -> lspower::jsonrpc::Result<Option<Vec<lsp::Location>>> {
        log::error!("Got a textDocument/references request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn document_highlight(
        &self,
        _params: lsp::DocumentHighlightParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::DocumentHighlight>>> {
        log::error!("Got a textDocument/documentHighlight request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn document_symbol(&self, _params: lsp::DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        self.client.log_message(MessageType::Info, "Got symbol request.").await;
        // Err(lspower::jsonrpc::Error::method_not_found())
        let test = DocumentSymbol {
            name: "lkml.view".to_string(),
            detail: None,
            kind: SymbolKind::String,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 10,
                    character: 10,
                },
                end: Position {
                    line: 10,
                    character: 15,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 10,
                    character: 10,
                },
                end: Position {
                    line: 10,
                    character: 15,
                },
            },
            children: None,
        };
        Ok(Some(DocumentSymbolResponse::Nested(vec![test])))
    }

    async fn code_action(
        &self,
        _params: lsp::CodeActionParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::CodeActionResponse>> {
        log::error!("Got a textDocument/codeAction request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn code_lens(&self, _params: lsp::CodeLensParams) -> lspower::jsonrpc::Result<Option<Vec<lsp::CodeLens>>> {
        log::error!("Got a textDocument/codeLens request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn code_lens_resolve(&self, _params: lsp::CodeLens) -> lspower::jsonrpc::Result<lsp::CodeLens> {
        log::error!("Got a codeLens/resolve request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn document_link(
        &self,
        _params: lsp::DocumentLinkParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::DocumentLink>>> {
        log::error!("Got a textDocument/documentLink request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn document_link_resolve(&self, _params: lsp::DocumentLink) -> lspower::jsonrpc::Result<lsp::DocumentLink> {
        log::error!("Got a documentLink/resolve request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn document_color(
        &self,
        _params: lsp::DocumentColorParams,
    ) -> lspower::jsonrpc::Result<Vec<lsp::ColorInformation>> {
        log::error!("Got a textDocument/documentColor request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn color_presentation(
        &self,
        _params: lsp::ColorPresentationParams,
    ) -> lspower::jsonrpc::Result<Vec<lsp::ColorPresentation>> {
        log::error!("Got a textDocument/colorPresentation request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn formatting(
        &self,
        _params: lsp::DocumentFormattingParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::TextEdit>>> {
        log::error!("Got a textDocument/formatting request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn range_formatting(
        &self,
        _params: lsp::DocumentRangeFormattingParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::TextEdit>>> {
        log::error!("Got a textDocument/rangeFormatting request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn on_type_formatting(
        &self,
        _params: lsp::DocumentOnTypeFormattingParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::TextEdit>>> {
        log::error!("Got a textDocument/onTypeFormatting request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn rename(&self, _params: lsp::RenameParams) -> lspower::jsonrpc::Result<Option<lsp::WorkspaceEdit>> {
        log::error!("Got a textDocument/rename request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn prepare_rename(
        &self,
        _params: lsp::TextDocumentPositionParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::PrepareRenameResponse>> {
        log::error!("Got a textDocument/prepareRename request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn folding_range(
        &self,
        _params: lsp::FoldingRangeParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::FoldingRange>>> {
        log::error!("Got a textDocument/foldingRange request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn selection_range(
        &self,
        _params: lsp::SelectionRangeParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::SelectionRange>>> {
        log::error!("Got a textDocument/selectionRange request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn incoming_calls(
        &self,
        _params: lsp::CallHierarchyIncomingCallsParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::CallHierarchyIncomingCall>>> {
        log::error!("Got a callHierarchy/incomingCalls request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn outgoing_calls(
        &self,
        _params: lsp::CallHierarchyOutgoingCallsParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::CallHierarchyOutgoingCall>>> {
        log::error!("Got a callHierarchy/outgoingCalls request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn prepare_call_hierarchy(
        &self,
        _params: lsp::CallHierarchyPrepareParams,
    ) -> lspower::jsonrpc::Result<Option<Vec<lsp::CallHierarchyItem>>> {
        log::error!("Got a textDocument/prepareCallHierarchy request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn semantic_tokens_full(
        &self,
        _params: lsp::SemanticTokensParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::SemanticTokensResult>> {
        eprint!("Semantic tokens full");
        log::error!("Got a textDocument/semanticTokens/full request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn semantic_tokens_full_delta(
        &self,
        _params: lsp::SemanticTokensDeltaParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::SemanticTokensFullDeltaResult>> {
        eprint!("Semantic tokens full");
        log::error!("Got a textDocument/semanticTokens/full/delta request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn semantic_tokens_range(
        &self,
        _params: lsp::SemanticTokensRangeParams,
    ) -> lspower::jsonrpc::Result<Option<lsp::SemanticTokensRangeResult>> {
        eprint!("Semantic tokens full");
        log::error!("Got a textDocument/semanticTokens/range request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn semantic_tokens_refresh(&self) -> lspower::jsonrpc::Result<()> {
        log::error!("Got a workspace/semanticTokens/refresh request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn code_action_resolve(&self, _params: lsp::CodeAction) -> lspower::jsonrpc::Result<lsp::CodeAction> {
        log::error!("Got a codeAction/resolve request, but it is not implemented");
        Err(lspower::jsonrpc::Error::method_not_found())
    }

    async fn request_else(
        &self,
        method: &str,
        _params: Option<serde_json::Value>,
    ) -> lspower::jsonrpc::Result<Option<serde_json::Value>> {
        log::error!(
            "Got a {} request, but LanguageServer::request_else is not implemented",
            method
        );
        Err(lspower::jsonrpc::Error::method_not_found())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:5007").await?;
    let (stream, _) = listener.accept().await?;
    let (read, write) = tokio::io::split(stream);

    let (service, messages) = LspService::new(|client| Backend { client });
    Server::new(read, write).interleave(messages).serve(service).await;

    Ok(())
}
