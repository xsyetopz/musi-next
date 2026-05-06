use std::collections::HashMap;
use std::future::Future;
use std::ops::ControlFlow;
use std::path::Path;
use std::pin::Pin;

use async_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOptions, CodeActionOrCommand, CodeActionParams,
    CodeActionProviderCapability, CodeActionResponse, CompletionList, CompletionOptions,
    CompletionParams, CompletionResponse, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentFormattingParams,
    DocumentHighlight, DocumentHighlightParams, DocumentLink, DocumentLinkOptions,
    DocumentLinkParams, DocumentSymbolParams, DocumentSymbolResponse, FoldingRange,
    FoldingRangeParams, FoldingRangeProviderCapability, FormattingOptions, GotoDefinitionParams,
    GotoDefinitionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability,
    InitializeParams, InitializeResult, InitializedParams, InlayHint, InlayHintOptions,
    InlayHintParams, InlayHintServerCapabilities, Location, MarkupContent, MarkupKind, OneOf,
    PrepareRenameResponse, PublishDiagnosticsParams, Range, ReferenceParams, RenameOptions,
    RenameParams, SelectionRange, SelectionRangeParams, SelectionRangeProviderCapability,
    SemanticTokens, SemanticTokensFullOptions, SemanticTokensOptions, SemanticTokensParams,
    SemanticTokensRangeParams, SemanticTokensRangeResult, SemanticTokensResult,
    SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo,
    TextDocumentContentChangeEvent, TextDocumentItem, TextDocumentPositionParams,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Url, WorkDoneProgressOptions,
    WorkspaceEdit, WorkspaceSymbolParams, WorkspaceSymbolResponse,
    notification::PublishDiagnostics,
};
use async_lsp::{ClientSocket, LanguageServer, ResponseError};
use musi_fmt::{FormatOptions, format_text_for_path};
use musi_project::{ProjectOptions, load_project_ancestor};
use musi_tooling::{
    collect_project_diagnostics_with_overlay, completions_for_project_file_with_overlay,
    definition_for_project_file_with_overlay, document_links_for_project_file_with_overlay,
    document_symbols_for_project_file_with_overlay, folding_ranges_for_project_file_with_overlay,
    hover_for_project_file_with_overlay, inlay_hints_for_project_file_with_overlay,
    prepare_rename_for_project_file_with_overlay, references_for_project_file_with_overlay,
    rename_for_project_file_with_overlay, selection_ranges_for_project_file_with_overlay,
    semantic_tokens_for_project_file_with_overlay, workspace_symbols_for_project_file_with_overlay,
};

mod config;
mod convert;

use config::LspConfig;
use convert::{
    diagnostic_matches_path, encode_semantic_tokens, full_document_range, position_in_range,
    semantic_tokens_legend, to_lsp_completion, to_lsp_diagnostic, to_lsp_document_highlight,
    to_lsp_document_link, to_lsp_document_symbol, to_lsp_folding_range, to_lsp_inlay_hint,
    to_lsp_location, to_lsp_selection_range, to_lsp_symbol_information, to_lsp_workspace_edit,
    to_tool_range, tool_location_matches_path, truncate_hover_contents,
};

type ServerFuture<T> = Pin<Box<dyn Future<Output = Result<T, ResponseError>> + Send + 'static>>;
type NotifyResult = ControlFlow<async_lsp::Result<()>>;

#[derive(Debug)]
pub struct MusiLanguageServer {
    client: ClientSocket,
    open_documents: HashMap<Url, String>,
    config: LspConfig,
}

impl MusiLanguageServer {
    #[must_use]
    pub fn new(client: ClientSocket) -> Self {
        Self {
            client,
            open_documents: HashMap::new(),
            config: LspConfig::default(),
        }
    }

    fn initialize_result() -> InitializeResult {
        InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_highlight_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                document_link_provider: Some(DocumentLinkOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![CodeActionKind::SOURCE_ORGANIZE_IMPORTS]),
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                        resolve_provider: Some(false),
                    },
                )),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                })),
                document_formatting_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_owned()]),
                    ..CompletionOptions::default()
                }),
                inlay_hint_provider: Some(OneOf::Right(InlayHintServerCapabilities::Options(
                    InlayHintOptions {
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                        resolve_provider: Some(false),
                    },
                ))),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions {
                                work_done_progress: None,
                            },
                            legend: semantic_tokens_legend(),
                            range: Some(true),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                        },
                    ),
                ),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "musi_lsp".to_owned(),
                version: None,
            }),
        }
    }

    fn configure(&mut self, params: &InitializeParams) {
        self.config = LspConfig::from_initialize_params(params);
    }

    fn did_open_document(&mut self, item: TextDocumentItem) {
        let path = item.uri.to_file_path().ok();
        let uri = item.uri;
        let text = item.text;
        let _ = self.open_documents.insert(uri.clone(), text);
        if let Some(path) = path {
            self.publish_document_diagnostics(&uri, &path);
        }
    }

    fn did_change_document(&mut self, uri: &Url, changes: &[TextDocumentContentChangeEvent]) {
        let Some(change) = changes.last() else {
            return;
        };
        let _ = self.open_documents.insert(uri.clone(), change.text.clone());
        if let Ok(path) = uri.to_file_path() {
            self.publish_document_diagnostics(uri, &path);
        }
    }

    fn did_close_document(&mut self, uri: &Url) {
        let _ = self.open_documents.remove(uri);
        drop(
            self.client
                .notify::<PublishDiagnostics>(PublishDiagnosticsParams {
                    uri: uri.clone(),
                    diagnostics: Vec::new(),
                    version: None,
                }),
        );
    }

    fn did_save_document(&self, uri: &Url) {
        if let Ok(path) = uri.to_file_path() {
            self.publish_document_diagnostics(uri, &path);
        }
    }
}

impl MusiLanguageServer {
    fn completions(&self, params: CompletionParams) -> Option<CompletionResponse> {
        let text_document = params.text_document_position.text_document;
        let position = params.text_document_position.position;
        let path = text_document.uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self
            .open_documents
            .get(&text_document.uri)
            .map(String::as_str);
        let items = completions_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
        )
        .into_iter()
        .map(to_lsp_completion)
        .collect();
        Some(CompletionResponse::List(CompletionList {
            is_incomplete: false,
            items,
        }))
    }

    fn hover_at(&self, params: HoverParams) -> Option<Hover> {
        let text_document = params.text_document_position_params.text_document;
        let position = params.text_document_position_params.position;
        let path = text_document.uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self
            .open_documents
            .get(&text_document.uri)
            .map(String::as_str);
        let hover = hover_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
        )?;
        let contents = truncate_hover_contents(&hover.contents, self.config.hover_maximum_length);
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: contents,
            }),
            range: Some(to_tool_range(&hover.range)),
        })
    }

    fn definition_at(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let text_document = params.text_document_position_params.text_document;
        let position = params.text_document_position_params.position;
        let path = text_document.uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self
            .open_documents
            .get(&text_document.uri)
            .map(String::as_str);
        let location = definition_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
        )
        .and_then(to_lsp_location)?;
        Some(GotoDefinitionResponse::Scalar(location))
    }

    fn references_at(&self, params: ReferenceParams) -> Option<Vec<Location>> {
        let text_document = params.text_document_position.text_document;
        let position = params.text_document_position.position;
        let path = text_document.uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self
            .open_documents
            .get(&text_document.uri)
            .map(String::as_str);
        let locations = references_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
            params.context.include_declaration,
        )
        .into_iter()
        .filter_map(to_lsp_location)
        .collect();
        Some(locations)
    }

    fn document_highlights(
        &self,
        params: DocumentHighlightParams,
    ) -> Option<Vec<DocumentHighlight>> {
        let text_document = params.text_document_position_params.text_document;
        let position = params.text_document_position_params.position;
        let path = text_document.uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self
            .open_documents
            .get(&text_document.uri)
            .map(String::as_str);
        let highlights = references_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
            true,
        )
        .into_iter()
        .filter(|location| tool_location_matches_path(&path, location))
        .map(to_lsp_document_highlight)
        .collect();
        Some(highlights)
    }

    fn document_symbols(&self, params: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
        let uri = params.text_document.uri;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self.open_documents.get(&uri).map(String::as_str);
        let symbols = document_symbols_for_project_file_with_overlay(&path, overlay)
            .into_iter()
            .map(to_lsp_document_symbol)
            .collect();
        Some(DocumentSymbolResponse::Nested(symbols))
    }

    fn document_links(&self, params: DocumentLinkParams) -> Option<Vec<DocumentLink>> {
        let uri = params.text_document.uri;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self.open_documents.get(&uri).map(String::as_str);
        let links = document_links_for_project_file_with_overlay(&path, overlay)
            .into_iter()
            .filter_map(to_lsp_document_link)
            .collect();
        Some(links)
    }

    fn code_actions(&self, params: CodeActionParams) -> Option<CodeActionResponse> {
        if !code_action_kind_requested(
            params.context.only.as_deref(),
            &CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
        ) {
            return Some(Vec::new());
        }
        let uri = params.text_document.uri;
        let text = self.open_documents.get(&uri)?;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let options = load_project_ancestor(&path, ProjectOptions::default())
            .ok()
            .map_or_else(FormatOptions::default, |project| {
                FormatOptions::from_manifest(project.manifest().fmt.as_ref())
            });
        let formatted = format_text_for_path(&path, text, &options).ok()?;
        if !formatted.changed {
            return Some(Vec::new());
        }
        let edit = WorkspaceEdit {
            changes: Some(HashMap::from([(
                uri,
                vec![TextEdit::new(full_document_range(text), formatted.text)],
            )])),
            document_changes: None,
            change_annotations: None,
        };
        Some(vec![CodeActionOrCommand::CodeAction(CodeAction {
            title: "Organize imports".to_owned(),
            kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
            diagnostics: None,
            edit: Some(edit),
            command: None,
            is_preferred: Some(true),
            disabled: None,
            data: None,
        })])
    }

    fn folding_ranges(&self, params: FoldingRangeParams) -> Option<Vec<FoldingRange>> {
        let uri = params.text_document.uri;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self.open_documents.get(&uri).map(String::as_str);
        let ranges = folding_ranges_for_project_file_with_overlay(&path, overlay)
            .into_iter()
            .map(to_lsp_folding_range)
            .collect();
        Some(ranges)
    }

    fn selection_ranges(&self, params: SelectionRangeParams) -> Option<Vec<SelectionRange>> {
        let uri = params.text_document.uri;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self.open_documents.get(&uri).map(String::as_str);
        let positions = params
            .positions
            .into_iter()
            .map(|position| {
                Some(musi_tooling::ToolPosition::new(
                    usize::try_from(position.line).ok()?.saturating_add(1),
                    usize::try_from(position.character).ok()?.saturating_add(1),
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        let ranges = selection_ranges_for_project_file_with_overlay(&path, overlay, &positions)
            .into_iter()
            .filter_map(|range| range.map(to_lsp_selection_range))
            .collect();
        Some(ranges)
    }

    fn workspace_symbols(&self, params: &WorkspaceSymbolParams) -> Option<WorkspaceSymbolResponse> {
        let (uri, text) = self.open_documents.iter().next()?;
        let path = uri.to_file_path().ok()?;
        let symbols =
            workspace_symbols_for_project_file_with_overlay(&path, Some(text), &params.query)
                .into_iter()
                .filter_map(to_lsp_symbol_information)
                .collect();
        Some(WorkspaceSymbolResponse::Flat(symbols))
    }

    fn prepare_rename_at(
        &self,
        params: TextDocumentPositionParams,
    ) -> Option<PrepareRenameResponse> {
        let text_document = params.text_document;
        let position = params.position;
        let path = text_document.uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self
            .open_documents
            .get(&text_document.uri)
            .map(String::as_str);
        let (range, placeholder) = prepare_rename_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
        )?;
        Some(PrepareRenameResponse::RangeWithPlaceholder {
            range: to_tool_range(&range),
            placeholder,
        })
    }

    fn rename_at(&self, params: RenameParams) -> Option<WorkspaceEdit> {
        let text_document = params.text_document_position.text_document;
        let position = params.text_document_position.position;
        let path = text_document.uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self
            .open_documents
            .get(&text_document.uri)
            .map(String::as_str);
        rename_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
            &params.new_name,
        )
        .map(to_lsp_workspace_edit)
    }

    fn semantic_tokens(&self, params: &SemanticTokensParams) -> Option<SemanticTokens> {
        self.semantic_tokens_for_uri(&params.text_document.uri, None)
    }

    fn semantic_range_tokens(&self, params: &SemanticTokensRangeParams) -> Option<SemanticTokens> {
        self.semantic_tokens_for_uri(&params.text_document.uri, Some(params.range))
    }

    fn inlay_hints(&self, params: &InlayHintParams) -> Option<Vec<InlayHint>> {
        if !self.config.inlay_hints.enabled {
            return Some(Vec::new());
        }
        let uri = &params.text_document.uri;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self.open_documents.get(uri).map(String::as_str);
        let hints = inlay_hints_for_project_file_with_overlay(&path, overlay)
            .into_iter()
            .filter(|hint| self.config.inlay_hints.allows(hint))
            .filter(|hint| position_in_range(hint.position, params.range))
            .map(to_lsp_inlay_hint)
            .collect();
        Some(hints)
    }

    fn semantic_tokens_for_uri(&self, uri: &Url, range: Option<Range>) -> Option<SemanticTokens> {
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self.open_documents.get(uri).map(String::as_str);
        let tokens = semantic_tokens_for_project_file_with_overlay(&path, overlay);
        Some(SemanticTokens {
            result_id: None,
            data: encode_semantic_tokens(&tokens, range.as_ref()),
        })
    }

    fn document_formatting(&self, params: DocumentFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = params.text_document.uri;
        let text = self.open_documents.get(&uri)?;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let mut options = load_project_ancestor(&path, ProjectOptions::default())
            .ok()
            .map_or_else(FormatOptions::default, |project| {
                FormatOptions::from_manifest(project.manifest().fmt.as_ref())
            });
        apply_document_formatting_options(&mut options, &params.options);
        let formatted = format_text_for_path(&path, text, &options).ok()?;
        if !formatted.changed {
            return Some(Vec::new());
        }
        Some(vec![TextEdit::new(
            full_document_range(text),
            formatted.text,
        )])
    }

    fn publish_document_diagnostics(&self, uri: &Url, path: &Path) {
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return;
        }
        let overlay = self.open_documents.get(uri).map(String::as_str);
        let diagnostics = collect_project_diagnostics_with_overlay(path, overlay)
            .into_iter()
            .filter(|diag| diagnostic_matches_path(path, diag))
            .map(to_lsp_diagnostic)
            .collect();
        drop(
            self.client
                .notify::<PublishDiagnostics>(PublishDiagnosticsParams {
                    uri: uri.clone(),
                    diagnostics,
                    version: None,
                }),
        );
    }
}

fn apply_document_formatting_options(
    options: &mut FormatOptions,
    formatting_options: &FormattingOptions,
) {
    options.indent_width = usize::try_from(formatting_options.tab_size).unwrap_or(2);
    options.use_tabs = !formatting_options.insert_spaces;
}

fn code_action_kind_requested(only: Option<&[CodeActionKind]>, target: &CodeActionKind) -> bool {
    only.is_none_or(|kinds| {
        kinds.iter().any(|kind| {
            kind == target
                || target
                    .as_str()
                    .strip_prefix(kind.as_str())
                    .is_some_and(|suffix| suffix.starts_with('.'))
        })
    })
}

impl LanguageServer for MusiLanguageServer {
    type Error = ResponseError;
    type NotifyResult = NotifyResult;

    fn initialize(&mut self, params: InitializeParams) -> ServerFuture<InitializeResult> {
        self.configure(&params);
        Box::pin(async { Ok(Self::initialize_result()) })
    }

    fn initialized(&mut self, _: InitializedParams) -> NotifyResult {
        ControlFlow::Continue(())
    }

    fn shutdown(&mut self, (): ()) -> ServerFuture<()> {
        Box::pin(async { Ok(()) })
    }

    fn did_open(&mut self, params: DidOpenTextDocumentParams) -> NotifyResult {
        self.did_open_document(params.text_document);
        ControlFlow::Continue(())
    }

    fn did_change(&mut self, params: DidChangeTextDocumentParams) -> NotifyResult {
        self.did_change_document(&params.text_document.uri, &params.content_changes);
        ControlFlow::Continue(())
    }

    fn did_close(&mut self, params: DidCloseTextDocumentParams) -> NotifyResult {
        self.did_close_document(&params.text_document.uri);
        ControlFlow::Continue(())
    }

    fn did_save(&mut self, params: DidSaveTextDocumentParams) -> NotifyResult {
        self.did_save_document(&params.text_document.uri);
        ControlFlow::Continue(())
    }

    fn completion(&mut self, params: CompletionParams) -> ServerFuture<Option<CompletionResponse>> {
        let completion_response = self.completions(params);
        Box::pin(async move { Ok(completion_response) })
    }

    fn hover(&mut self, params: HoverParams) -> ServerFuture<Option<Hover>> {
        let hover_response = self.hover_at(params);
        Box::pin(async move { Ok(hover_response) })
    }

    fn definition(
        &mut self,
        params: GotoDefinitionParams,
    ) -> ServerFuture<Option<GotoDefinitionResponse>> {
        let definition_response = self.definition_at(params);
        Box::pin(async move { Ok(definition_response) })
    }

    fn references(&mut self, params: ReferenceParams) -> ServerFuture<Option<Vec<Location>>> {
        let reference_locations = self.references_at(params);
        Box::pin(async move { Ok(reference_locations) })
    }

    fn document_highlight(
        &mut self,
        params: DocumentHighlightParams,
    ) -> ServerFuture<Option<Vec<DocumentHighlight>>> {
        let document_highlights = self.document_highlights(params);
        Box::pin(async move { Ok(document_highlights) })
    }

    fn document_symbol(
        &mut self,
        params: DocumentSymbolParams,
    ) -> ServerFuture<Option<DocumentSymbolResponse>> {
        let document_symbols = self.document_symbols(params);
        Box::pin(async move { Ok(document_symbols) })
    }

    fn document_link(
        &mut self,
        params: DocumentLinkParams,
    ) -> ServerFuture<Option<Vec<DocumentLink>>> {
        let document_links = self.document_links(params);
        Box::pin(async move { Ok(document_links) })
    }

    fn code_action(
        &mut self,
        params: CodeActionParams,
    ) -> ServerFuture<Option<CodeActionResponse>> {
        let code_actions = self.code_actions(params);
        Box::pin(async move { Ok(code_actions) })
    }

    fn folding_range(
        &mut self,
        params: FoldingRangeParams,
    ) -> ServerFuture<Option<Vec<FoldingRange>>> {
        let folding_ranges = self.folding_ranges(params);
        Box::pin(async move { Ok(folding_ranges) })
    }

    fn selection_range(
        &mut self,
        params: SelectionRangeParams,
    ) -> ServerFuture<Option<Vec<SelectionRange>>> {
        let selection_ranges = self.selection_ranges(params);
        Box::pin(async move { Ok(selection_ranges) })
    }

    fn symbol(
        &mut self,
        params: WorkspaceSymbolParams,
    ) -> ServerFuture<Option<WorkspaceSymbolResponse>> {
        let workspace_symbols = self.workspace_symbols(&params);
        Box::pin(async move { Ok(workspace_symbols) })
    }

    fn prepare_rename(
        &mut self,
        params: TextDocumentPositionParams,
    ) -> ServerFuture<Option<PrepareRenameResponse>> {
        let prepare_rename_response = self.prepare_rename_at(params);
        Box::pin(async move { Ok(prepare_rename_response) })
    }

    fn rename(&mut self, params: RenameParams) -> ServerFuture<Option<WorkspaceEdit>> {
        let rename_response = self.rename_at(params);
        Box::pin(async move { Ok(rename_response) })
    }

    fn formatting(
        &mut self,
        params: DocumentFormattingParams,
    ) -> ServerFuture<Option<Vec<TextEdit>>> {
        let formatting_response = self.document_formatting(params);
        Box::pin(async move { Ok(formatting_response) })
    }

    fn semantic_tokens_full(
        &mut self,
        params: SemanticTokensParams,
    ) -> ServerFuture<Option<SemanticTokensResult>> {
        let semantic_tokens_response = self
            .semantic_tokens(&params)
            .map(SemanticTokensResult::Tokens);
        Box::pin(async move { Ok(semantic_tokens_response) })
    }

    fn semantic_tokens_range(
        &mut self,
        params: SemanticTokensRangeParams,
    ) -> ServerFuture<Option<SemanticTokensRangeResult>> {
        let semantic_range_response = self
            .semantic_range_tokens(&params)
            .map(SemanticTokensRangeResult::Tokens);
        Box::pin(async move { Ok(semantic_range_response) })
    }

    fn inlay_hint(&mut self, params: InlayHintParams) -> ServerFuture<Option<Vec<InlayHint>>> {
        let inlay_hints_response = self.inlay_hints(&params);
        Box::pin(async move { Ok(inlay_hints_response) })
    }
}

#[cfg(test)]
mod tests;
