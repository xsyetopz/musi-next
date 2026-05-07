use std::collections::HashMap;
use std::future::Future;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use async_lsp::lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    CodeAction, CodeActionParams, CodeActionResponse, CodeLens, CodeLensParams, Command,
    CompletionItem, CompletionList, CompletionParams, CompletionResponse,
    DidChangeConfigurationParams, DidChangeTextDocumentParams, DidChangeWorkspaceFoldersParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    DocumentDiagnosticParams, DocumentDiagnosticReportResult, DocumentFormattingParams,
    DocumentHighlight, DocumentHighlightParams, DocumentLink, DocumentLinkParams,
    DocumentOnTypeFormattingParams, DocumentRangeFormattingParams, DocumentSymbolParams,
    DocumentSymbolResponse, ExecuteCommandParams, FoldingRange, FoldingRangeParams,
    GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
    InitializeParams, InitializeResult, InitializedParams, InlayHint, InlayHintParams,
    LinkedEditingRangeParams, LinkedEditingRanges, Location, MarkupContent, MarkupKind, Moniker,
    MonikerKind, MonikerParams, PartialResultParams, Position, PrepareRenameResponse,
    PublishDiagnosticsParams, ReferenceContext, ReferenceParams, RenameFilesParams, RenameParams,
    SelectionRange, SelectionRangeParams, SemanticTokensDeltaParams, SemanticTokensFullDeltaResult,
    SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensRangeResult,
    SemanticTokensResult, SignatureHelp, SignatureHelpParams, TextDocumentContentChangeEvent,
    TextDocumentIdentifier, TextDocumentItem, TextDocumentPositionParams, TextEdit,
    UniquenessLevel, Url, WillSaveTextDocumentParams, WorkDoneProgressParams,
    WorkspaceDiagnosticParams, WorkspaceDiagnosticReportResult, WorkspaceEdit, WorkspaceSymbol,
    WorkspaceSymbolParams, WorkspaceSymbolResponse, notification::PublishDiagnostics,
};
#[cfg(test)]
use async_lsp::lsp_types::{
    CallHierarchyServerCapability, CodeActionProviderCapability, FormattingOptions,
    HoverProviderCapability, OneOf, Range, WorkDoneProgressOptions,
};
use async_lsp::{ClientSocket, LanguageServer, ResponseError};
use musi_tooling::{
    ToolMonikerKind, completions_for_project_file_with_overlay,
    definition_for_project_file_with_overlay, document_links_for_project_file_with_overlay,
    document_symbols_for_project_file_with_overlay, folding_ranges_for_project_file_with_overlay,
    hover_for_project_file_with_overlay, module_docs_for_project_file_with_overlay,
    moniker_for_project_file_with_overlay, outgoing_calls_for_project_file_with_overlay,
    prepare_rename_for_project_file_with_overlay, references_for_project_file_with_overlay,
    rename_for_project_file_with_overlay, selection_ranges_for_project_file_with_overlay,
    signature_help_for_project_file_with_overlay, type_definition_for_project_file_with_overlay,
};
use serde_json::{Value, json};

mod capabilities;
mod code_actions;
mod config;
mod convert;
mod diagnostics;
mod formatting;
mod hints;
mod navigation;
mod semantic;
mod symbols;
mod workspace;

use config::LspConfig;
#[cfg(test)]
use convert::encode_semantic_tokens;
#[cfg(test)]
use convert::full_document_range;
use convert::{
    resolve_lsp_completion, resolve_lsp_document_link, to_lsp_call_hierarchy_item,
    to_lsp_completion, to_lsp_document_highlight, to_lsp_document_link, to_lsp_folding_range,
    to_lsp_location, to_lsp_selection_range, to_lsp_signature_help, to_lsp_symbol_kind,
    to_lsp_workspace_edit, to_tool_range, tool_location_matches_path, truncate_hover_contents,
};
#[cfg(test)]
use formatting::apply_document_formatting_options;
#[cfg(test)]
use musi_fmt::FormatOptions;
use navigation::{
    call_hierarchy_item_data_parts, call_hierarchy_items_match, caller_symbol_for_reference,
    import_definition_at, import_document_highlights, import_linked_editing_ranges,
    position_in_lsp_range, push_reference_lenses, reference_lens_data_parts, reference_lens_title,
    symbol_at_position,
};
use semantic::SemanticTokenSnapshot;
use workspace::{
    collect_workspace_source_paths, import_specifier_for_target, paths_match, renamed_target_path,
    sort_dedup_paths, workspace_roots,
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

    fn initialize_result() -> InitializeResult {
        capabilities::initialize_result(REFERENCES_COMMAND)
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
        if let Some(location) = import_definition_at(&path, overlay, position) {
            return Some(GotoDefinitionResponse::Scalar(location));
        }
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
        if let Some(locations) = self.import_references_at(&path, overlay, position) {
            return Some(locations);
        }
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

    fn import_references_at(
        &self,
        path: &Path,
        overlay: Option<&str>,
        position: Position,
    ) -> Option<Vec<Location>> {
        let target = document_links_for_project_file_with_overlay(path, overlay)
            .into_iter()
            .find(|link| position_in_lsp_range(position, to_tool_range(&link.range)))?
            .target;
        let mut locations = Vec::new();
        for candidate_path in self.workspace_source_paths() {
            let open = self.open_document_for_path(&candidate_path);
            let candidate_overlay = open.map(|(_, text)| text);
            let Some(uri) = open
                .map(|(uri, _)| uri.clone())
                .or_else(|| Url::from_file_path(&candidate_path).ok())
            else {
                continue;
            };
            locations.extend(
                document_links_for_project_file_with_overlay(&candidate_path, candidate_overlay)
                    .into_iter()
                    .filter(|link| paths_match(&link.target, &target))
                    .map(|link| Location {
                        uri: uri.clone(),
                        range: to_tool_range(&link.range),
                    }),
            );
        }
        locations.sort_by_key(|location| {
            (
                location.uri.to_string(),
                location.range.start.line,
                location.range.start.character,
            )
        });
        locations.dedup_by_key(|location| {
            (
                location.uri.to_string(),
                location.range.start.line,
                location.range.start.character,
            )
        });
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
        let moniker = moniker_for_project_file_with_overlay(
            &path,
            overlay,
            usize::try_from(position.line).ok()?.saturating_add(1),
            usize::try_from(position.character).ok()?.saturating_add(1),
        )?;
        let uri = Url::from_file_path(moniker.location.path).ok()?;
        Some(vec![Moniker {
            scheme: "musi".to_owned(),
            identifier: format!(
                "{}#{}:{}",
                uri.as_str(),
                moniker.location.range.start_line,
                moniker.location.range.start_col
            ),
            unique: UniquenessLevel::Project,
            kind: Some(match moniker.kind {
                ToolMonikerKind::Import => MonikerKind::Import,
                ToolMonikerKind::Local => MonikerKind::Local,
            }),
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
        if let Some(highlights) = import_document_highlights(&path, overlay, position) {
            return Some(highlights);
        }
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
        if let Some(ranges) = import_linked_editing_ranges(&path, overlay, position) {
            return Some(ranges);
        }
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

    fn call_hierarchy_outgoing_calls(
        &self,
        params: &CallHierarchyOutgoingCallsParams,
    ) -> Option<Vec<CallHierarchyOutgoingCall>> {
        let (uri, line, character) = call_hierarchy_item_data_parts(&params.item)?;
        let path = uri.to_file_path().ok()?;
        let overlay = self.open_documents.get(&uri).map(String::as_str);
        let calls = outgoing_calls_for_project_file_with_overlay(
            &path,
            overlay,
            line.saturating_add(1),
            character.saturating_add(1),
        )
        .into_iter()
        .filter_map(|call| {
            let uri = Url::from_file_path(call.to.location.path).ok()?;
            Some(CallHierarchyOutgoingCall {
                to: CallHierarchyItem {
                    name: call.to.name,
                    kind: to_lsp_symbol_kind(call.to.kind),
                    tags: None,
                    detail: None,
                    uri: uri.clone(),
                    range: to_tool_range(&call.to.location.range),
                    selection_range: to_tool_range(&call.to.location.range),
                    data: Some(json!({
                        "uri": uri.as_str(),
                        "line": call.to.location.range.start_line.saturating_sub(1),
                        "character": call.to.location.range.start_col.saturating_sub(1),
                    })),
                },
                from_ranges: call
                    .from_ranges
                    .into_iter()
                    .map(|range| to_tool_range(&range))
                    .collect(),
            })
        })
        .collect();
        Some(calls)
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

    fn execute_command_request(&self, params: &ExecuteCommandParams) -> Option<Value> {
        if params.command != REFERENCES_COMMAND {
            return None;
        }
        let argument = params.arguments.first()?;
        let uri = argument.get("uri")?.as_str()?;
        let line = usize::try_from(argument.get("line")?.as_u64()?).ok()?;
        let character = usize::try_from(argument.get("character")?.as_u64()?).ok()?;
        let uri = Url::parse(uri).ok()?;
        let locations = self.references_at(ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position::new(u32::try_from(line).ok()?, u32::try_from(character).ok()?),
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
            context: ReferenceContext {
                include_declaration: false,
            },
        })?;
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

    fn workspace_source_paths(&self) -> Vec<PathBuf> {
        let mut paths = self.workspace_diagnostic_paths();
        for root in self.workspace_query_roots() {
            collect_workspace_source_paths(&root, &mut paths);
        }
        sort_dedup_paths(paths)
    }
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
        let outgoing_calls = self.call_hierarchy_outgoing_calls(&params);
        Box::pin(async move { Ok(outgoing_calls) })
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
