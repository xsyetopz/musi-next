use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::ops::ControlFlow;
use std::path::{Component, Path, PathBuf};
use std::pin::Pin;

use async_lsp::lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    CallHierarchyServerCapability, CodeAction, CodeActionKind, CodeActionOptions,
    CodeActionOrCommand, CodeActionParams, CodeActionProviderCapability, CodeActionResponse,
    CodeLens, CodeLensOptions, CodeLensParams, Command, CompletionItem, CompletionList,
    CompletionOptions, CompletionParams, CompletionResponse, DeclarationCapability, Diagnostic,
    DiagnosticOptions, DiagnosticServerCapabilities, DidChangeConfigurationParams,
    DidChangeTextDocumentParams, DidChangeWorkspaceFoldersParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentDiagnosticParams,
    DocumentDiagnosticReport, DocumentDiagnosticReportResult, DocumentFormattingParams,
    DocumentHighlight, DocumentHighlightParams, DocumentLink, DocumentLinkOptions,
    DocumentLinkParams, DocumentOnTypeFormattingOptions, DocumentOnTypeFormattingParams,
    DocumentRangeFormattingParams, DocumentSymbolParams, DocumentSymbolResponse,
    ExecuteCommandOptions, ExecuteCommandParams, FileOperationFilter, FileOperationPattern,
    FileOperationPatternKind, FileOperationRegistrationOptions, FoldingRange, FoldingRangeParams,
    FoldingRangeProviderCapability, FormattingOptions, FullDocumentDiagnosticReport,
    GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
    HoverProviderCapability, InitializeParams, InitializeResult, InitializedParams, InlayHint,
    InlayHintOptions, InlayHintParams, InlayHintServerCapabilities, LinkedEditingRangeParams,
    LinkedEditingRangeServerCapabilities, LinkedEditingRanges, Location, MarkupContent, MarkupKind,
    Moniker, MonikerParams, OneOf, Position, PrepareRenameResponse, PublishDiagnosticsParams,
    Range, ReferenceParams, RelatedFullDocumentDiagnosticReport, RenameFilesParams, RenameOptions,
    RenameParams, SelectionRange, SelectionRangeParams, SelectionRangeProviderCapability,
    SemanticToken, SemanticTokens, SemanticTokensDelta, SemanticTokensDeltaParams,
    SemanticTokensEdit, SemanticTokensFullDeltaResult, SemanticTokensFullOptions,
    SemanticTokensOptions, SemanticTokensParams, SemanticTokensRangeParams,
    SemanticTokensRangeResult, SemanticTokensResult, SemanticTokensServerCapabilities,
    ServerCapabilities, ServerInfo, SignatureHelp, SignatureHelpOptions, SignatureHelpParams,
    TextDocumentContentChangeEvent, TextDocumentItem, TextDocumentPositionParams,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions, TextEdit, TypeDefinitionProviderCapability, UniquenessLevel, Url,
    WillSaveTextDocumentParams, WorkDoneProgressOptions, WorkspaceDiagnosticParams,
    WorkspaceDiagnosticReport, WorkspaceDiagnosticReportResult, WorkspaceDocumentDiagnosticReport,
    WorkspaceEdit, WorkspaceFileOperationsServerCapabilities, WorkspaceFoldersServerCapabilities,
    WorkspaceFullDocumentDiagnosticReport, WorkspaceServerCapabilities, WorkspaceSymbol,
    WorkspaceSymbolOptions, WorkspaceSymbolParams, WorkspaceSymbolResponse,
    notification::PublishDiagnostics,
};
use async_lsp::{ClientSocket, LanguageServer, ResponseError};
use musi_fmt::{FormatOptions, format_text_for_path};
use musi_project::{PackageSource, ProjectOptions, load_project, load_project_ancestor};
use musi_tooling::{
    ToolDocumentSymbol, collect_project_diagnostics_with_overlay,
    completions_for_project_file_with_overlay, definition_for_project_file_with_overlay,
    document_links_for_project_file_with_overlay, document_symbols_for_project_file_with_overlay,
    folding_ranges_for_project_file_with_overlay, hover_for_project_file_with_overlay,
    inlay_hints_for_project_file_with_overlay, module_docs_for_project_file_with_overlay,
    prepare_rename_for_project_file_with_overlay, references_for_project_file_with_overlay,
    rename_for_project_file_with_overlay, selection_ranges_for_project_file_with_overlay,
    semantic_tokens_for_project_file_with_overlay, signature_help_for_project_file_with_overlay,
    type_definition_for_project_file_with_overlay, workspace_symbols_for_project_file_with_overlay,
    workspace_symbols_for_project_root,
};
use serde_json::{Value, json};

mod config;
mod convert;

use config::LspConfig;
use convert::{
    diagnostic_matches_path, encode_semantic_tokens, full_document_range, position_in_range,
    resolve_lsp_completion, resolve_lsp_document_link, resolve_lsp_inlay_hint,
    resolve_lsp_workspace_symbol, semantic_tokens_legend, to_lsp_call_hierarchy_item,
    to_lsp_completion, to_lsp_diagnostic, to_lsp_document_highlight, to_lsp_document_link,
    to_lsp_document_symbol, to_lsp_folding_range, to_lsp_inlay_hint, to_lsp_location,
    to_lsp_selection_range, to_lsp_signature_help, to_lsp_workspace_edit, to_lsp_workspace_symbol,
    to_tool_range, tool_location_matches_path, truncate_hover_contents,
};

type ServerFuture<T> = Pin<Box<dyn Future<Output = Result<T, ResponseError>> + Send + 'static>>;
type NotifyResult = ControlFlow<async_lsp::Result<()>>;
const REFERENCES_COMMAND: &str = "musi.references";

#[derive(Debug)]
pub struct MusiLanguageServer {
    client: ClientSocket,
    open_documents: HashMap<Url, String>,
    semantic_token_cache: HashMap<Url, SemanticTokenSnapshot>,
    workspace_roots: Vec<PathBuf>,
    config: LspConfig,
}

#[derive(Debug, Clone)]
struct SemanticTokenSnapshot {
    result_id: String,
    data: Vec<SemanticToken>,
}

impl MusiLanguageServer {
    #[must_use]
    pub fn new(client: ClientSocket) -> Self {
        Self {
            client,
            open_documents: HashMap::new(),
            semantic_token_cache: HashMap::new(),
            workspace_roots: Vec::new(),
            config: LspConfig::default(),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn initialize_result() -> InitializeResult {
        InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: Some(true),
                        will_save_wait_until: Some(true),
                        save: Some(TextDocumentSyncSaveOptions::Supported(true)),
                    },
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_owned(), ",".to_owned()]),
                    retrigger_characters: Some(vec![",".to_owned()]),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
                moniker_provider: Some(OneOf::Left(true)),
                call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
                linked_editing_range_provider: Some(LinkedEditingRangeServerCapabilities::Simple(
                    true,
                )),
                document_highlight_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                document_link_provider: Some(DocumentLinkOptions {
                    resolve_provider: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![REFERENCES_COMMAND.to_owned()],
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                        will_rename: Some(FileOperationRegistrationOptions {
                            filters: vec![
                                FileOperationFilter {
                                    scheme: Some("file".to_owned()),
                                    pattern: FileOperationPattern {
                                        glob: "**/*.ms".to_owned(),
                                        matches: Some(FileOperationPatternKind::File),
                                        options: None,
                                    },
                                },
                                FileOperationFilter {
                                    scheme: Some("file".to_owned()),
                                    pattern: FileOperationPattern {
                                        glob: "**".to_owned(),
                                        matches: Some(FileOperationPatternKind::Folder),
                                        options: None,
                                    },
                                },
                            ],
                        }),
                        ..WorkspaceFileOperationsServerCapabilities::default()
                    }),
                }),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("musi".to_owned()),
                        inter_file_dependencies: true,
                        workspace_diagnostics: true,
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                    },
                )),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
                workspace_symbol_provider: Some(OneOf::Right(WorkspaceSymbolOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                    resolve_provider: Some(true),
                })),
                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![CodeActionKind::SOURCE_ORGANIZE_IMPORTS]),
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                        resolve_provider: Some(true),
                    },
                )),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                })),
                document_formatting_provider: Some(OneOf::Left(true)),
                document_range_formatting_provider: Some(OneOf::Left(true)),
                document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
                    first_trigger_character: ";".to_owned(),
                    more_trigger_character: Some(vec![
                        ")".to_owned(),
                        "]".to_owned(),
                        "}".to_owned(),
                    ]),
                }),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_owned(), "\"".to_owned(), "/".to_owned()]),
                    ..CompletionOptions::default()
                }),
                inlay_hint_provider: Some(OneOf::Right(InlayHintServerCapabilities::Options(
                    InlayHintOptions {
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                        resolve_provider: Some(true),
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
                            full: Some(SemanticTokensFullOptions::Delta { delta: Some(true) }),
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
        self.workspace_roots = workspace_roots(params);
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
        let _ = self.semantic_token_cache.remove(uri);
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

    fn update_configuration(&mut self, params: &DidChangeConfigurationParams) {
        self.config = LspConfig::from_settings(&params.settings);
    }

    fn update_workspace_folders(&mut self, params: DidChangeWorkspaceFoldersParams) {
        for folder in params.event.removed {
            if let Ok(path) = folder.uri.to_file_path() {
                self.workspace_roots
                    .retain(|root| !paths_match(root, &path));
            }
        }
        for folder in params.event.added {
            if let Ok(path) = folder.uri.to_file_path()
                && !self
                    .workspace_roots
                    .iter()
                    .any(|root| paths_match(root, &path))
            {
                self.workspace_roots.push(path);
            }
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

    fn resolve_completion(completion: CompletionItem) -> CompletionItem {
        resolve_lsp_completion(completion)
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
        if let Some(hover) = self.import_hover_at(&path, overlay, position) {
            return Some(hover);
        }
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

    fn import_hover_at(
        &self,
        path: &Path,
        overlay: Option<&str>,
        position: Position,
    ) -> Option<Hover> {
        let link = document_links_for_project_file_with_overlay(path, overlay)
            .into_iter()
            .find(|link| position_in_lsp_range(position, to_tool_range(&link.range)))?;
        let mut contents = format!(
            "```musi\n(module) {}\n```\n\nResolves to `{}`.",
            link.specifier, link.resolved
        );
        if let Some(docs) = module_docs_for_project_file_with_overlay(&link.target, None) {
            contents.push_str("\n\n");
            contents.push_str(&docs);
        }
        let contents = truncate_hover_contents(&contents, self.config.hover_maximum_length);
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: contents,
            }),
            range: Some(to_tool_range(&link.range)),
        })
    }

    fn signature_help_at(&self, params: SignatureHelpParams) -> Option<SignatureHelp> {
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
        signature_help_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
        )
        .map(to_lsp_signature_help)
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

    fn type_definition_at(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
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
        let location = type_definition_for_project_file_with_overlay(
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

    fn monikers_at(&self, params: MonikerParams) -> Option<Vec<Moniker>> {
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
        )?;
        let uri = Url::from_file_path(location.path).ok()?;
        Some(vec![Moniker {
            scheme: "musi".to_owned(),
            identifier: format!(
                "{}#{}:{}",
                uri.as_str(),
                location.range.start_line,
                location.range.start_col
            ),
            unique: UniquenessLevel::Project,
            kind: None,
        }])
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
        .map(|location| to_lsp_document_highlight(&location))
        .collect();
        Some(highlights)
    }

    fn linked_editing_ranges(
        &self,
        params: LinkedEditingRangeParams,
    ) -> Option<LinkedEditingRanges> {
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
        let ranges = references_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
            true,
        )
        .into_iter()
        .filter(|location| tool_location_matches_path(&path, location))
        .map(|location| to_tool_range(&location.range))
        .collect::<Vec<_>>();
        (ranges.len() > 1).then_some(LinkedEditingRanges {
            ranges,
            word_pattern: Some("[A-Za-z_][A-Za-z0-9_]*".to_owned()),
        })
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

    fn prepare_call_hierarchy_at(
        &self,
        params: CallHierarchyPrepareParams,
    ) -> Option<Vec<CallHierarchyItem>> {
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
        let symbols = document_symbols_for_project_file_with_overlay(&path, overlay);
        let symbol = symbol_at_position(&symbols, position)?;
        Some(vec![to_lsp_call_hierarchy_item(&text_document.uri, symbol)])
    }

    fn call_hierarchy_incoming_calls(
        &self,
        params: &CallHierarchyIncomingCallsParams,
    ) -> Option<Vec<CallHierarchyIncomingCall>> {
        let (uri, line, character) = call_hierarchy_item_data_parts(&params.item)?;
        let path = uri.to_file_path().ok()?;
        let overlay = self.open_documents.get(&uri).map(String::as_str);
        let mut calls = Vec::<CallHierarchyIncomingCall>::new();
        for location in references_for_project_file_with_overlay(
            &path,
            overlay,
            line.saturating_add(1),
            character.saturating_add(1),
            false,
        ) {
            let Some(reference_uri) = Url::from_file_path(&location.path).ok() else {
                continue;
            };
            let reference_overlay = self.open_documents.get(&reference_uri).map(String::as_str);
            let symbols =
                document_symbols_for_project_file_with_overlay(&location.path, reference_overlay);
            let Some(symbol) = caller_symbol_for_reference(&symbols, &location.range) else {
                continue;
            };
            let from = to_lsp_call_hierarchy_item(&reference_uri, symbol);
            let from_range = to_tool_range(&location.range);
            if let Some(call) = calls
                .iter_mut()
                .find(|call| call_hierarchy_items_match(&call.from, &from))
            {
                call.from_ranges.push(from_range);
            } else {
                calls.push(CallHierarchyIncomingCall {
                    from,
                    from_ranges: vec![from_range],
                });
            }
        }
        Some(calls)
    }

    const fn call_hierarchy_outgoing_calls(
        _: &CallHierarchyOutgoingCallsParams,
    ) -> Vec<CallHierarchyOutgoingCall> {
        Vec::new()
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

    fn resolve_document_link(link: DocumentLink) -> DocumentLink {
        resolve_lsp_document_link(link)
    }

    fn code_lenses(&self, params: CodeLensParams) -> Option<Vec<CodeLens>> {
        let uri = params.text_document.uri;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self.open_documents.get(&uri).map(String::as_str);
        let mut lenses = Vec::new();
        for symbol in document_symbols_for_project_file_with_overlay(&path, overlay) {
            push_reference_lenses(&path, &symbol, &mut lenses);
        }
        Some(lenses)
    }

    fn resolve_code_lens(&self, mut lens: CodeLens) -> CodeLens {
        if lens.command.is_some() {
            return lens;
        }
        let Some(data) = lens.data.as_ref() else {
            return lens;
        };
        let Some((uri, line, character)) = reference_lens_data_parts(data) else {
            return lens;
        };
        let Ok(path) = uri.to_file_path() else {
            return lens;
        };
        let overlay = self.open_documents.get(&uri).map(String::as_str);
        let references = references_for_project_file_with_overlay(
            &path,
            overlay,
            line.saturating_add(1),
            character.saturating_add(1),
            false,
        );
        if references.is_empty() {
            return lens;
        }
        lens.command = Some(Command::new(
            reference_lens_title(references.len()),
            REFERENCES_COMMAND.to_owned(),
            Some(vec![data.clone()]),
        ));
        lens
    }

    fn code_actions(&self, params: CodeActionParams) -> Option<CodeActionResponse> {
        if !code_action_kind_requested(
            params.context.only.as_deref(),
            &CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
        ) {
            return Some(Vec::new());
        }
        let uri = params.text_document.uri;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        if self.organize_imports_edit(&uri).is_none() {
            return Some(Vec::new());
        }
        Some(vec![CodeActionOrCommand::CodeAction(CodeAction {
            title: "Organize imports".to_owned(),
            kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: Some(true),
            disabled: None,
            data: Some(json!({
                "uri": uri.as_str(),
            })),
        })])
    }

    fn resolve_code_action(&self, mut action: CodeAction) -> CodeAction {
        if action.edit.is_some() {
            return action;
        }
        if action.kind.as_ref() != Some(&CodeActionKind::SOURCE_ORGANIZE_IMPORTS) {
            return action;
        }
        let Some(uri) = action
            .data
            .as_ref()
            .and_then(|data| data.get("uri"))
            .and_then(Value::as_str)
            .and_then(|uri| Url::parse(uri).ok())
        else {
            return action;
        };
        action.edit = self.organize_imports_edit(&uri);
        action
    }

    fn organize_imports_edit(&self, uri: &Url) -> Option<WorkspaceEdit> {
        let text = self.open_documents.get(uri)?;
        let path = uri.to_file_path().ok()?;
        let options = load_project_ancestor(&path, ProjectOptions::default())
            .ok()
            .map_or_else(FormatOptions::default, |project| {
                FormatOptions::from_manifest(project.manifest().fmt.as_ref())
            });
        let formatted = format_text_for_path(&path, text, &options).ok()?;
        formatted.changed.then(|| WorkspaceEdit {
            changes: Some(HashMap::from([(
                uri.clone(),
                vec![TextEdit::new(full_document_range(text), formatted.text)],
            )])),
            document_changes: None,
            change_annotations: None,
        })
    }

    fn execute_command_request(&self, params: &ExecuteCommandParams) -> Option<Value> {
        if params.command != REFERENCES_COMMAND {
            return None;
        }
        let argument = params.arguments.first()?;
        let uri = argument.get("uri")?.as_str()?;
        let line = usize::try_from(argument.get("line")?.as_u64()?).ok()?;
        let character = usize::try_from(argument.get("character")?.as_u64()?).ok()?;
        let uri = Url::parse(uri).ok()?;
        let path = uri.to_file_path().ok()?;
        let overlay = self.open_documents.get(&uri).map(String::as_str);
        let locations = references_for_project_file_with_overlay(
            &path,
            overlay,
            line.saturating_add(1),
            character.saturating_add(1),
            false,
        )
        .into_iter()
        .filter_map(to_lsp_location)
        .collect::<Vec<_>>();
        serde_json::to_value(locations).ok()
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
            .map(|range| to_lsp_folding_range(&range))
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

    fn workspace_symbols(&self, params: &WorkspaceSymbolParams) -> WorkspaceSymbolResponse {
        let open_paths = self
            .open_documents
            .keys()
            .filter_map(|uri| uri.to_file_path().ok())
            .collect::<Vec<_>>();
        let mut symbols = self
            .workspace_roots
            .iter()
            .flat_map(|root| workspace_symbols_for_project_root(root, &params.query))
            .filter(|symbol| {
                !open_paths
                    .iter()
                    .any(|path| paths_match(path, &symbol.location.path))
            })
            .collect::<Vec<_>>();
        symbols.extend(
            self.open_documents
                .iter()
                .filter_map(|(uri, text)| {
                    let path = uri.to_file_path().ok()?;
                    Some(workspace_symbols_for_project_file_with_overlay(
                        &path,
                        Some(text),
                        &params.query,
                    ))
                })
                .flatten(),
        );
        symbols.sort_by_key(|symbol| {
            (
                symbol.name.clone(),
                symbol.location.path.clone(),
                symbol.location.range.start_line,
                symbol.location.range.start_col,
            )
        });
        symbols.dedup_by_key(|symbol| {
            (
                symbol.name.clone(),
                symbol.location.path.clone(),
                symbol.location.range.start_line,
                symbol.location.range.start_col,
            )
        });
        let symbols = symbols
            .into_iter()
            .filter_map(to_lsp_workspace_symbol)
            .collect();
        WorkspaceSymbolResponse::Nested(symbols)
    }

    fn resolve_workspace_symbol(symbol: WorkspaceSymbol) -> WorkspaceSymbol {
        resolve_lsp_workspace_symbol(symbol)
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

    fn will_rename_files_edit(&self, params: &RenameFilesParams) -> Option<WorkspaceEdit> {
        let renames = params
            .files
            .iter()
            .filter_map(|file| {
                let old_uri = Url::parse(&file.old_uri).ok()?;
                let new_uri = Url::parse(&file.new_uri).ok()?;
                Some((old_uri.to_file_path().ok()?, new_uri.to_file_path().ok()?))
            })
            .collect::<Vec<_>>();
        if renames.is_empty() {
            return None;
        }
        let mut changes = HashMap::<Url, Vec<TextEdit>>::new();
        for document_path in self.workspace_source_paths() {
            if document_path
                .file_name()
                .is_some_and(|name| name == "musi.json")
            {
                continue;
            }
            let open_document = self.open_document_for_path(&document_path);
            let uri = open_document.map_or_else(
                || Url::from_file_path(&document_path).ok(),
                |(uri, _)| Some(uri.clone()),
            )?;
            let overlay = open_document.map(|(_, text)| text);
            for link in document_links_for_project_file_with_overlay(&document_path, overlay) {
                let Some(new_target) = renamed_target_path(&renames, &link.target) else {
                    continue;
                };
                let Some(specifier) = import_specifier_for_target(&document_path, &new_target)
                else {
                    continue;
                };
                changes.entry(uri.clone()).or_default().push(TextEdit::new(
                    to_tool_range(&link.range),
                    format!("\"{specifier}\""),
                ));
            }
        }
        (!changes.is_empty()).then_some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        })
    }

    fn semantic_tokens(&self, params: &SemanticTokensParams) -> Option<SemanticTokens> {
        self.semantic_tokens_for_uri(&params.text_document.uri, None)
    }

    fn semantic_range_tokens(&self, params: &SemanticTokensRangeParams) -> Option<SemanticTokens> {
        self.semantic_tokens_for_uri(&params.text_document.uri, Some(params.range))
    }

    fn semantic_tokens_full_response(
        &mut self,
        params: &SemanticTokensParams,
    ) -> Option<SemanticTokens> {
        let uri = &params.text_document.uri;
        let tokens = self.semantic_tokens(params)?;
        let result_id = tokens.result_id.clone()?;
        let snapshot = SemanticTokenSnapshot {
            result_id,
            data: tokens.data.clone(),
        };
        let _ = self.semantic_token_cache.insert(uri.clone(), snapshot);
        Some(tokens)
    }

    fn semantic_token_delta(
        &mut self,
        params: &SemanticTokensDeltaParams,
    ) -> Option<SemanticTokensFullDeltaResult> {
        let uri = &params.text_document.uri;
        let tokens = self.semantic_tokens_for_uri(uri, None)?;
        let result_id = tokens.result_id.clone()?;
        let next = SemanticTokenSnapshot {
            result_id,
            data: tokens.data.clone(),
        };
        let response = self
            .semantic_token_cache
            .get(uri)
            .filter(|previous| previous.result_id == params.previous_result_id)
            .map_or_else(
                || SemanticTokensFullDeltaResult::Tokens(tokens),
                |previous| {
                    SemanticTokensFullDeltaResult::TokensDelta(semantic_tokens_delta(
                        previous, &next,
                    ))
                },
            );
        let _ = self.semantic_token_cache.insert(uri.clone(), next);
        Some(response)
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

    fn resolve_inlay_hint(hint: InlayHint) -> InlayHint {
        resolve_lsp_inlay_hint(hint)
    }

    fn document_diagnostics(
        &self,
        params: &DocumentDiagnosticParams,
    ) -> DocumentDiagnosticReportResult {
        let Some(path) = params.text_document.uri.to_file_path().ok() else {
            return full_document_diagnostic_report(Vec::new());
        };
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return full_document_diagnostic_report(Vec::new());
        }
        let overlay = self
            .open_documents
            .get(&params.text_document.uri)
            .map(String::as_str);
        let diagnostics = collect_project_diagnostics_with_overlay(&path, overlay)
            .into_iter()
            .filter(|diag| diagnostic_matches_path(&path, diag))
            .map(to_lsp_diagnostic)
            .collect();
        full_document_diagnostic_report(diagnostics)
    }

    fn workspace_diagnostics(
        &self,
        _params: WorkspaceDiagnosticParams,
    ) -> WorkspaceDiagnosticReportResult {
        let paths = self.workspace_diagnostic_paths();
        let items = paths
            .into_iter()
            .filter_map(|path| {
                if path.file_name().is_some_and(|name| name == "musi.json") {
                    return None;
                }
                let open_document = self.open_document_for_path(&path);
                let uri = open_document.map_or_else(
                    || Url::from_file_path(&path).ok(),
                    |(uri, _)| Some(uri.clone()),
                )?;
                let overlay = open_document.map(|(_, text)| text);
                let diagnostics = collect_project_diagnostics_with_overlay(&path, overlay)
                    .into_iter()
                    .filter(|diag| diagnostic_matches_path(&path, diag))
                    .map(to_lsp_diagnostic)
                    .collect();
                Some(WorkspaceDocumentDiagnosticReport::Full(
                    WorkspaceFullDocumentDiagnosticReport {
                        uri,
                        version: None,
                        full_document_diagnostic_report: FullDocumentDiagnosticReport {
                            result_id: None,
                            items: diagnostics,
                        },
                    },
                ))
            })
            .collect();
        WorkspaceDiagnosticReportResult::Report(WorkspaceDiagnosticReport { items })
    }

    fn workspace_diagnostic_paths(&self) -> Vec<PathBuf> {
        let mut paths = self
            .workspace_roots
            .iter()
            .flat_map(|root| workspace_module_paths(root))
            .collect::<Vec<_>>();
        paths.extend(
            self.open_documents
                .keys()
                .filter_map(|uri| uri.to_file_path().ok()),
        );
        sort_dedup_paths(paths)
    }

    fn workspace_source_paths(&self) -> Vec<PathBuf> {
        let mut paths = self.workspace_diagnostic_paths();
        for root in &self.workspace_roots {
            collect_workspace_source_paths(root, &mut paths);
        }
        sort_dedup_paths(paths)
    }

    fn open_document_for_path(&self, path: &Path) -> Option<(&Url, &str)> {
        self.open_documents.iter().find_map(|(uri, text)| {
            let open_path = uri.to_file_path().ok()?;
            paths_match(&open_path, path).then_some((uri, text.as_str()))
        })
    }

    fn semantic_tokens_for_uri(&self, uri: &Url, range: Option<Range>) -> Option<SemanticTokens> {
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let overlay = self.open_documents.get(uri).map(String::as_str);
        let tokens = semantic_tokens_for_project_file_with_overlay(&path, overlay);
        let data = encode_semantic_tokens(&tokens, range.as_ref());
        Some(SemanticTokens {
            result_id: range.is_none().then(|| semantic_tokens_result_id(&data)),
            data,
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

    fn will_save_formatting(&self, params: WillSaveTextDocumentParams) -> Option<Vec<TextEdit>> {
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
        Some(vec![TextEdit::new(
            full_document_range(text),
            formatted.text,
        )])
    }

    fn document_on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> Option<Vec<TextEdit>> {
        if !on_type_formatting_trigger(&params.ch) {
            return Some(Vec::new());
        }
        let uri = params.text_document_position.text_document.uri;
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

    fn document_range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> Option<Vec<TextEdit>> {
        let uri = params.text_document.uri;
        let text = self.open_documents.get(&uri)?;
        let path = uri.to_file_path().ok()?;
        if path.file_name().is_some_and(|name| name == "musi.json") {
            return None;
        }
        let (start, end) = lsp_range_offsets(text, params.range)?;
        let selected = text.get(start..end)?;
        let mut options = load_project_ancestor(&path, ProjectOptions::default())
            .ok()
            .map_or_else(FormatOptions::default, |project| {
                FormatOptions::from_manifest(project.manifest().fmt.as_ref())
            });
        apply_document_formatting_options(&mut options, &params.options);
        let formatted = format_text_for_path(&path, selected, &options).ok()?;
        if !formatted.changed {
            return Some(Vec::new());
        }
        Some(vec![TextEdit::new(params.range, formatted.text)])
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

fn on_type_formatting_trigger(ch: &str) -> bool {
    matches!(ch, ";" | ")" | "]" | "}")
}

fn reference_lens_title(count: usize) -> String {
    if count == 1 {
        "1 reference".to_owned()
    } else {
        format!("{count} references")
    }
}

fn push_reference_lenses(path: &Path, symbol: &ToolDocumentSymbol, lenses: &mut Vec<CodeLens>) {
    if let Some(data) = reference_lens_data(path, symbol) {
        lenses.push(CodeLens {
            range: to_tool_range(&symbol.selection_range),
            command: None,
            data: Some(data),
        });
    }
    for child in &symbol.children {
        push_reference_lenses(path, child, lenses);
    }
}

fn symbol_at_position(
    symbols: &[ToolDocumentSymbol],
    position: Position,
) -> Option<&ToolDocumentSymbol> {
    symbols.iter().find_map(|symbol| {
        let selection_range = to_tool_range(&symbol.selection_range);
        if position_in_lsp_range(position, selection_range) {
            return Some(symbol);
        }
        symbol_at_position(&symbol.children, position)
    })
}

fn caller_symbol_for_reference<'a>(
    symbols: &'a [ToolDocumentSymbol],
    range: &musi_tooling::ToolRange,
) -> Option<&'a ToolDocumentSymbol> {
    symbols
        .iter()
        .flat_map(flatten_symbols)
        .filter(|symbol| {
            symbol.selection_range.start_line == range.start_line
                && symbol.selection_range.start_col < range.start_col
        })
        .max_by_key(|symbol| symbol.selection_range.start_col)
}

fn flatten_symbols(symbol: &ToolDocumentSymbol) -> Vec<&ToolDocumentSymbol> {
    let mut symbols = vec![symbol];
    for child in &symbol.children {
        symbols.extend(flatten_symbols(child));
    }
    symbols
}

fn call_hierarchy_item_data_parts(item: &CallHierarchyItem) -> Option<(Url, usize, usize)> {
    let data = item.data.as_ref()?;
    let uri = data.get("uri")?.as_str()?;
    let line = usize::try_from(data.get("line")?.as_u64()?).ok()?;
    let character = usize::try_from(data.get("character")?.as_u64()?).ok()?;
    Some((Url::parse(uri).ok()?, line, character))
}

fn call_hierarchy_items_match(left: &CallHierarchyItem, right: &CallHierarchyItem) -> bool {
    left.uri == right.uri && left.selection_range == right.selection_range
}

const fn position_in_lsp_range(position: Position, range: Range) -> bool {
    !position_lt(position, range.start) && position_lt(position, range.end)
}

const fn position_lt(left: Position, right: Position) -> bool {
    left.line < right.line || (left.line == right.line && left.character < right.character)
}

fn reference_lens_data(path: &Path, symbol: &ToolDocumentSymbol) -> Option<Value> {
    Some(json!({
        "uri": Url::from_file_path(path).ok()?.as_str(),
        "line": symbol.selection_range.start_line.saturating_sub(1),
        "character": symbol.selection_range.start_col.saturating_sub(1),
    }))
}

fn reference_lens_data_parts(data: &Value) -> Option<(Url, usize, usize)> {
    let uri = data.get("uri")?.as_str()?;
    let line = usize::try_from(data.get("line")?.as_u64()?).ok()?;
    let character = usize::try_from(data.get("character")?.as_u64()?).ok()?;
    Some((Url::parse(uri).ok()?, line, character))
}

fn semantic_tokens_result_id(tokens: &[SemanticToken]) -> String {
    let mut hasher = DefaultHasher::new();
    for token in tokens {
        token.delta_line.hash(&mut hasher);
        token.delta_start.hash(&mut hasher);
        token.length.hash(&mut hasher);
        token.token_type.hash(&mut hasher);
        token.token_modifiers_bitset.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}

fn semantic_tokens_delta(
    previous: &SemanticTokenSnapshot,
    next: &SemanticTokenSnapshot,
) -> SemanticTokensDelta {
    let mut prefix_len = 0usize;
    while prefix_len < previous.data.len()
        && prefix_len < next.data.len()
        && previous.data.get(prefix_len) == next.data.get(prefix_len)
    {
        prefix_len = prefix_len.saturating_add(1);
    }
    let mut suffix_len = 0usize;
    while suffix_len < previous.data.len().saturating_sub(prefix_len)
        && suffix_len < next.data.len().saturating_sub(prefix_len)
        && previous.data[previous.data.len() - suffix_len - 1]
            == next.data[next.data.len() - suffix_len - 1]
    {
        suffix_len = suffix_len.saturating_add(1);
    }
    let inserted = next.data[prefix_len..next.data.len() - suffix_len].to_vec();
    SemanticTokensDelta {
        result_id: Some(next.result_id.clone()),
        edits: vec![SemanticTokensEdit {
            start: len_to_u32(prefix_len),
            delete_count: len_to_u32(previous.data.len() - prefix_len - suffix_len),
            data: (!inserted.is_empty()).then_some(inserted),
        }],
    }
}

fn len_to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("semantic token vector length should fit u32")
}

#[allow(deprecated)]
fn workspace_roots(params: &InitializeParams) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(folders) = &params.workspace_folders {
        roots.extend(
            folders
                .iter()
                .filter_map(|folder| folder.uri.to_file_path().ok()),
        );
    }
    if roots.is_empty()
        && let Some(root_uri) = &params.root_uri
        && let Ok(path) = root_uri.to_file_path()
    {
        roots.push(path);
    }
    roots
}

fn workspace_module_paths(root: &Path) -> Vec<PathBuf> {
    let Ok(project) = load_project(root, ProjectOptions::default()) else {
        return Vec::new();
    };
    sort_dedup_paths(
        project
            .workspace()
            .packages
            .values()
            .filter(|package| matches!(package.source, PackageSource::Workspace))
            .flat_map(|package| package.module_keys.values().cloned())
            .collect(),
    )
}

fn collect_workspace_source_paths(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.components().any(is_ignored_workspace_component) {
            continue;
        }
        if path.is_dir() {
            collect_workspace_source_paths(&path, out);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("ms") {
            out.push(path);
        }
    }
}

fn is_ignored_workspace_component(component: Component<'_>) -> bool {
    let text = component.as_os_str().to_string_lossy();
    matches!(
        text.as_ref(),
        ".git" | ".cache" | ".musi" | "musi_modules" | "node_modules" | "target"
    )
}

fn sort_dedup_paths(mut paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths.sort_by_key(|path| canonical_path(path));
    paths.dedup_by(|left, right| paths_match(left, right));
    paths
}

fn paths_match(left: &Path, right: &Path) -> bool {
    canonical_path(left) == canonical_path(right)
}

fn canonical_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn renamed_target_path(renames: &[(PathBuf, PathBuf)], target: &Path) -> Option<PathBuf> {
    renames.iter().find_map(|(old_path, new_path)| {
        let target_key = canonical_path(target);
        let old_key = canonical_path(old_path);
        if target_key == old_key {
            return Some(new_path.clone());
        }
        target_key
            .strip_prefix(old_key)
            .ok()
            .map(|relative| new_path.join(relative))
    })
}

fn import_specifier_for_target(importer_path: &Path, target_path: &Path) -> Option<String> {
    let importer_dir = canonical_path(importer_path.parent()?);
    let target_path = canonical_target_path(target_path);
    let relative = relative_path(&importer_dir, &target_path)?;
    let relative = strip_musi_extension(relative);
    let mut specifier = relative.to_string_lossy().replace('\\', "/");
    if !specifier.starts_with('.') {
        specifier = format!("./{specifier}");
    }
    Some(specifier)
}

fn canonical_target_path(path: &Path) -> PathBuf {
    if let Ok(path) = path.canonicalize() {
        return path;
    }
    let mut missing = Vec::new();
    let mut current = path;
    while let Some(parent) = current.parent() {
        if let Some(file_name) = current.file_name() {
            missing.push(file_name.to_owned());
        }
        if let Ok(mut base) = parent.canonicalize() {
            for component in missing.iter().rev() {
                base.push(component);
            }
            return base;
        }
        current = parent;
    }
    path.to_path_buf()
}

fn relative_path(from_dir: &Path, target_path: &Path) -> Option<PathBuf> {
    let from_components = normal_components(from_dir);
    let target_components = normal_components(target_path);
    if from_components.first() != target_components.first() {
        return None;
    }
    let mut common = 0usize;
    while from_components.get(common) == target_components.get(common)
        && common < from_components.len()
        && common < target_components.len()
    {
        common = common.saturating_add(1);
    }
    let mut relative = PathBuf::new();
    for _ in common..from_components.len() {
        relative.push("..");
    }
    for component in &target_components[common..] {
        relative.push(component);
    }
    Some(relative)
}

fn normal_components(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| match component {
            Component::CurDir => None,
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                Some(component.as_os_str().to_string_lossy().into_owned())
            }
        })
        .collect()
}

fn strip_musi_extension(mut path: PathBuf) -> PathBuf {
    if path.extension().and_then(|extension| extension.to_str()) == Some("ms") {
        let _ = path.set_extension("");
    }
    path
}

const fn full_document_diagnostic_report(
    diagnostics: Vec<Diagnostic>,
) -> DocumentDiagnosticReportResult {
    DocumentDiagnosticReportResult::Report(DocumentDiagnosticReport::Full(
        RelatedFullDocumentDiagnosticReport {
            related_documents: None,
            full_document_diagnostic_report: FullDocumentDiagnosticReport {
                result_id: None,
                items: diagnostics,
            },
        },
    ))
}

fn lsp_range_offsets(text: &str, range: Range) -> Option<(usize, usize)> {
    let start = lsp_position_offset(text, range.start)?;
    let end = lsp_position_offset(text, range.end)?;
    (start <= end).then_some((start, end))
}

fn lsp_position_offset(text: &str, position: Position) -> Option<usize> {
    let target_line = usize::try_from(position.line).ok()?;
    let target_character = usize::try_from(position.character).ok()?;
    let mut line = 0usize;
    let mut character = 0usize;
    for (offset, ch) in text.char_indices() {
        if line == target_line && character == target_character {
            return Some(offset);
        }
        if ch == '\n' {
            line = line.saturating_add(1);
            character = 0;
        } else {
            character = character.saturating_add(1);
        }
    }
    (line == target_line && character == target_character).then_some(text.len())
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

    fn did_change_configuration(&mut self, params: DidChangeConfigurationParams) -> NotifyResult {
        self.update_configuration(&params);
        ControlFlow::Continue(())
    }

    fn did_change_workspace_folders(
        &mut self,
        params: DidChangeWorkspaceFoldersParams,
    ) -> NotifyResult {
        self.update_workspace_folders(params);
        ControlFlow::Continue(())
    }

    fn will_rename_files(
        &mut self,
        params: RenameFilesParams,
    ) -> ServerFuture<Option<WorkspaceEdit>> {
        let edit = self.will_rename_files_edit(&params);
        Box::pin(async move { Ok(edit) })
    }

    fn will_save_wait_until(
        &mut self,
        params: WillSaveTextDocumentParams,
    ) -> ServerFuture<Option<Vec<TextEdit>>> {
        let formatting_response = self.will_save_formatting(params);
        Box::pin(async move { Ok(formatting_response) })
    }

    fn document_diagnostic(
        &mut self,
        params: DocumentDiagnosticParams,
    ) -> ServerFuture<DocumentDiagnosticReportResult> {
        let report = self.document_diagnostics(&params);
        Box::pin(async move { Ok(report) })
    }

    fn workspace_diagnostic(
        &mut self,
        params: WorkspaceDiagnosticParams,
    ) -> ServerFuture<WorkspaceDiagnosticReportResult> {
        let report = self.workspace_diagnostics(params);
        Box::pin(async move { Ok(report) })
    }

    fn completion(&mut self, params: CompletionParams) -> ServerFuture<Option<CompletionResponse>> {
        let completion_response = self.completions(params);
        Box::pin(async move { Ok(completion_response) })
    }

    fn completion_item_resolve(&mut self, params: CompletionItem) -> ServerFuture<CompletionItem> {
        let completion = Self::resolve_completion(params);
        Box::pin(async move { Ok(completion) })
    }

    fn hover(&mut self, params: HoverParams) -> ServerFuture<Option<Hover>> {
        let hover_response = self.hover_at(params);
        Box::pin(async move { Ok(hover_response) })
    }

    fn signature_help(
        &mut self,
        params: SignatureHelpParams,
    ) -> ServerFuture<Option<SignatureHelp>> {
        let signature_help_response = self.signature_help_at(params);
        Box::pin(async move { Ok(signature_help_response) })
    }

    fn definition(
        &mut self,
        params: GotoDefinitionParams,
    ) -> ServerFuture<Option<GotoDefinitionResponse>> {
        let definition_response = self.definition_at(params);
        Box::pin(async move { Ok(definition_response) })
    }

    fn declaration(
        &mut self,
        params: GotoDefinitionParams,
    ) -> ServerFuture<Option<GotoDefinitionResponse>> {
        let declaration_response = self.definition_at(params);
        Box::pin(async move { Ok(declaration_response) })
    }

    fn type_definition(
        &mut self,
        params: GotoDefinitionParams,
    ) -> ServerFuture<Option<GotoDefinitionResponse>> {
        let type_definition_response = self.type_definition_at(params);
        Box::pin(async move { Ok(type_definition_response) })
    }

    fn references(&mut self, params: ReferenceParams) -> ServerFuture<Option<Vec<Location>>> {
        let reference_locations = self.references_at(params);
        Box::pin(async move { Ok(reference_locations) })
    }

    fn moniker(&mut self, params: MonikerParams) -> ServerFuture<Option<Vec<Moniker>>> {
        let monikers = self.monikers_at(params);
        Box::pin(async move { Ok(monikers) })
    }

    fn document_highlight(
        &mut self,
        params: DocumentHighlightParams,
    ) -> ServerFuture<Option<Vec<DocumentHighlight>>> {
        let document_highlights = self.document_highlights(params);
        Box::pin(async move { Ok(document_highlights) })
    }

    fn linked_editing_range(
        &mut self,
        params: LinkedEditingRangeParams,
    ) -> ServerFuture<Option<LinkedEditingRanges>> {
        let linked_ranges = self.linked_editing_ranges(params);
        Box::pin(async move { Ok(linked_ranges) })
    }

    fn document_symbol(
        &mut self,
        params: DocumentSymbolParams,
    ) -> ServerFuture<Option<DocumentSymbolResponse>> {
        let document_symbols = self.document_symbols(params);
        Box::pin(async move { Ok(document_symbols) })
    }

    fn prepare_call_hierarchy(
        &mut self,
        params: CallHierarchyPrepareParams,
    ) -> ServerFuture<Option<Vec<CallHierarchyItem>>> {
        let call_hierarchy_items = self.prepare_call_hierarchy_at(params);
        Box::pin(async move { Ok(call_hierarchy_items) })
    }

    fn incoming_calls(
        &mut self,
        params: CallHierarchyIncomingCallsParams,
    ) -> ServerFuture<Option<Vec<CallHierarchyIncomingCall>>> {
        let incoming_calls = self.call_hierarchy_incoming_calls(&params);
        Box::pin(async move { Ok(incoming_calls) })
    }

    fn outgoing_calls(
        &mut self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> ServerFuture<Option<Vec<CallHierarchyOutgoingCall>>> {
        let outgoing_calls = Self::call_hierarchy_outgoing_calls(&params);
        Box::pin(async move { Ok(Some(outgoing_calls)) })
    }

    fn document_link(
        &mut self,
        params: DocumentLinkParams,
    ) -> ServerFuture<Option<Vec<DocumentLink>>> {
        let document_links = self.document_links(params);
        Box::pin(async move { Ok(document_links) })
    }

    fn document_link_resolve(&mut self, params: DocumentLink) -> ServerFuture<DocumentLink> {
        let document_link = Self::resolve_document_link(params);
        Box::pin(async move { Ok(document_link) })
    }

    fn code_lens(&mut self, params: CodeLensParams) -> ServerFuture<Option<Vec<CodeLens>>> {
        let code_lenses = self.code_lenses(params);
        Box::pin(async move { Ok(code_lenses) })
    }

    fn code_lens_resolve(&mut self, params: CodeLens) -> ServerFuture<CodeLens> {
        let code_lens = self.resolve_code_lens(params);
        Box::pin(async move { Ok(code_lens) })
    }

    fn execute_command(&mut self, params: ExecuteCommandParams) -> ServerFuture<Option<Value>> {
        let command_response = self.execute_command_request(&params);
        Box::pin(async move { Ok(command_response) })
    }

    fn code_action(
        &mut self,
        params: CodeActionParams,
    ) -> ServerFuture<Option<CodeActionResponse>> {
        let code_actions = self.code_actions(params);
        Box::pin(async move { Ok(code_actions) })
    }

    fn code_action_resolve(&mut self, params: CodeAction) -> ServerFuture<CodeAction> {
        let code_action = self.resolve_code_action(params);
        Box::pin(async move { Ok(code_action) })
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
        Box::pin(async move { Ok(Some(workspace_symbols)) })
    }

    fn workspace_symbol_resolve(
        &mut self,
        params: WorkspaceSymbol,
    ) -> ServerFuture<WorkspaceSymbol> {
        let workspace_symbol = Self::resolve_workspace_symbol(params);
        Box::pin(async move { Ok(workspace_symbol) })
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

    fn range_formatting(
        &mut self,
        params: DocumentRangeFormattingParams,
    ) -> ServerFuture<Option<Vec<TextEdit>>> {
        let formatting_response = self.document_range_formatting(params);
        Box::pin(async move { Ok(formatting_response) })
    }

    fn on_type_formatting(
        &mut self,
        params: DocumentOnTypeFormattingParams,
    ) -> ServerFuture<Option<Vec<TextEdit>>> {
        let formatting_response = self.document_on_type_formatting(params);
        Box::pin(async move { Ok(formatting_response) })
    }

    fn semantic_tokens_full(
        &mut self,
        params: SemanticTokensParams,
    ) -> ServerFuture<Option<SemanticTokensResult>> {
        let semantic_tokens_response = self
            .semantic_tokens_full_response(&params)
            .map(SemanticTokensResult::Tokens);
        Box::pin(async move { Ok(semantic_tokens_response) })
    }

    fn semantic_tokens_full_delta(
        &mut self,
        params: SemanticTokensDeltaParams,
    ) -> ServerFuture<Option<SemanticTokensFullDeltaResult>> {
        let semantic_tokens_response = self.semantic_token_delta(&params);
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

    fn inlay_hint_resolve(&mut self, params: InlayHint) -> ServerFuture<InlayHint> {
        let hint = Self::resolve_inlay_hint(params);
        Box::pin(async move { Ok(hint) })
    }
}

#[cfg(test)]
mod tests;
