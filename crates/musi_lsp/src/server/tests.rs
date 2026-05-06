use std::env::temp_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use async_lsp::lsp_types::{
    ClientCapabilities, CodeActionContext, CodeActionKind, CodeActionOrCommand, CodeActionParams,
    CodeLensParams, CompletionItemKind, CompletionTextEdit, DeclarationCapability,
    DiagnosticOptions, DiagnosticServerCapabilities, DiagnosticSeverity,
    DidChangeConfigurationParams, DidChangeWorkspaceFoldersParams, DocumentDiagnosticParams,
    DocumentDiagnosticReport, DocumentDiagnosticReportResult, DocumentHighlightKind,
    DocumentLinkParams, DocumentOnTypeFormattingParams, DocumentRangeFormattingParams,
    ExecuteCommandParams, FoldingRangeKind, FoldingRangeParams, GotoDefinitionParams,
    GotoDefinitionResponse, InitializeParams, InlayHintKind, InlayHintServerCapabilities,
    InlayHintTooltip, LinkedEditingRangeParams, PartialResultParams, Position,
    SelectionRangeParams, SemanticToken, SignatureHelpParams, TextDocumentIdentifier,
    TextDocumentPositionParams, TextDocumentSaveReason, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncOptions, TextDocumentSyncSaveOptions,
    TypeDefinitionProviderCapability, WillSaveTextDocumentParams, WorkDoneProgressParams,
    WorkspaceDiagnosticParams, WorkspaceDiagnosticReportResult, WorkspaceDocumentDiagnosticReport,
    WorkspaceFolder, WorkspaceFoldersChangeEvent, WorkspaceSymbolParams, WorkspaceSymbolResponse,
};
use musi_tooling::{
    CliDiagnostic, CliDiagnosticLabel, CliDiagnosticRange, ToolInlayHint, ToolInlayHintKind,
    ToolPosition, ToolRange, ToolSemanticModifier, ToolSemanticToken, ToolSemanticTokenKind,
};

use super::convert::{
    default_range, diagnostic_matches_path, resolve_lsp_inlay_hint, to_cli_range,
    to_lsp_diagnostic, to_lsp_inlay_hint, to_severity, truncate_hover_contents,
};
use super::*;

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

fn temp_project() -> PathBuf {
    let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
    let path = temp_dir().join(format!("musi-lsp-test-{id}"));
    if path.exists() {
        fs::remove_dir_all(&path).expect("stale temp project should be removed");
    }
    fs::create_dir_all(&path).expect("temp project should be created");
    path
}

mod success {
    use super::*;

    #[test]
    fn initialize_result_advertises_full_sync_and_hover() {
        let initialize_result = MusiLanguageServer::initialize_result();

        assert_eq!(
            initialize_result.server_info.expect("server info").name,
            "musi_lsp"
        );
        assert_eq!(
            initialize_result.capabilities.text_document_sync,
            Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::FULL),
                    will_save: Some(true),
                    will_save_wait_until: Some(true),
                    save: Some(TextDocumentSyncSaveOptions::Supported(true)),
                }
            ))
        );
        assert_eq!(
            initialize_result.capabilities.hover_provider,
            Some(HoverProviderCapability::Simple(true))
        );
        let signature_help = initialize_result
            .capabilities
            .signature_help_provider
            .expect("signature help provider");
        assert_eq!(
            signature_help.trigger_characters.as_deref(),
            Some(&["(".to_owned(), ",".to_owned()][..])
        );
        assert_eq!(
            signature_help.retrigger_characters.as_deref(),
            Some(&[",".to_owned()][..])
        );
        assert_eq!(
            initialize_result.capabilities.document_formatting_provider,
            Some(OneOf::Left(true))
        );
        assert_eq!(
            initialize_result
                .capabilities
                .document_range_formatting_provider,
            Some(OneOf::Left(true))
        );
        let on_type_formatting = initialize_result
            .capabilities
            .document_on_type_formatting_provider
            .expect("on type formatting provider");
        assert_eq!(on_type_formatting.first_trigger_character, ";");
        assert_eq!(
            on_type_formatting.more_trigger_character.as_deref(),
            Some(&[")".to_owned(), "]".to_owned(), "}".to_owned()][..])
        );
        assert!(
            initialize_result
                .capabilities
                .semantic_tokens_provider
                .is_some()
        );
        let inlay_hint = initialize_result
            .capabilities
            .inlay_hint_provider
            .expect("inlay hint provider");
        let OneOf::Right(InlayHintServerCapabilities::Options(inlay_hint)) = inlay_hint else {
            panic!("inlay hint options expected");
        };
        assert_eq!(inlay_hint.resolve_provider, Some(true));
        let completion = initialize_result
            .capabilities
            .completion_provider
            .expect("completion provider");
        assert_eq!(completion.resolve_provider, Some(true));
        assert_eq!(
            initialize_result.capabilities.declaration_provider,
            Some(DeclarationCapability::Simple(true))
        );
        assert_eq!(
            initialize_result.capabilities.definition_provider,
            Some(OneOf::Left(true))
        );
        assert_eq!(
            initialize_result.capabilities.type_definition_provider,
            Some(TypeDefinitionProviderCapability::Simple(true))
        );
        assert_eq!(
            initialize_result.capabilities.references_provider,
            Some(OneOf::Left(true))
        );
        assert!(
            initialize_result
                .capabilities
                .linked_editing_range_provider
                .is_some()
        );
        let execute_command = initialize_result
            .capabilities
            .execute_command_provider
            .expect("execute command provider");
        assert_eq!(execute_command.commands, ["musi.references"]);
        let workspace = initialize_result
            .capabilities
            .workspace
            .expect("workspace capabilities");
        let folders = workspace
            .workspace_folders
            .expect("workspace folder capabilities");
        assert_eq!(folders.supported, Some(true));
        assert_eq!(folders.change_notifications, Some(OneOf::Left(true)));
        assert_eq!(
            initialize_result.capabilities.document_highlight_provider,
            Some(OneOf::Left(true))
        );
        assert_eq!(
            initialize_result.capabilities.document_symbol_provider,
            Some(OneOf::Left(true))
        );
        let document_link = initialize_result
            .capabilities
            .document_link_provider
            .expect("document link provider");
        assert_eq!(document_link.resolve_provider, Some(true));
        let code_lens = initialize_result
            .capabilities
            .code_lens_provider
            .expect("code lens provider");
        assert_eq!(code_lens.resolve_provider, Some(true));
        assert_eq!(
            initialize_result.capabilities.diagnostic_provider,
            Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
                identifier: Some("musi".to_owned()),
                inter_file_dependencies: true,
                workspace_diagnostics: true,
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: None,
                },
            }))
        );
        assert!(
            initialize_result
                .capabilities
                .folding_range_provider
                .is_some()
        );
        assert!(
            initialize_result
                .capabilities
                .selection_range_provider
                .is_some()
        );
        let code_action = initialize_result
            .capabilities
            .code_action_provider
            .expect("code action provider");
        let CodeActionProviderCapability::Options(code_action) = code_action else {
            panic!("code action options expected");
        };
        assert_eq!(code_action.resolve_provider, Some(true));
        assert_eq!(
            initialize_result.capabilities.workspace_symbol_provider,
            Some(OneOf::Left(true))
        );
        assert!(initialize_result.capabilities.rename_provider.is_some());
    }

    #[test]
    fn full_document_range_covers_complete_text() {
        assert_eq!(
            full_document_range("one\ntwo"),
            Range {
                start: Position::new(0, 0),
                end: Position::new(1, 3),
            }
        );
    }

    #[test]
    fn formatting_options_override_manifest_indentation() {
        let mut options = FormatOptions {
            use_tabs: true,
            indent_width: 8,
            ..FormatOptions::default()
        };

        apply_document_formatting_options(
            &mut options,
            &FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                ..FormattingOptions::default()
            },
        );

        assert!(!options.use_tabs);
        assert_eq!(options.indent_width, 4);
    }

    #[test]
    fn document_formatting_formats_multiline_match_like_cli_formatter() {
        let uri = Url::parse("file:///tmp/index.ms").expect("uri should parse");
        let source = r"export let isLess (target : Ordering) : Bool := match target(
    | .Less => 0 = 0
    | _ => 0 = 1);
";
        let expected = r"export let isLess (target : Ordering) : Bool :=
  match target (
  | .Less => 0 = 0
  | _ => 0 = 1
  );
";
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let edits = server
            .document_formatting(DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                options: FormattingOptions {
                    tab_size: 2,
                    insert_spaces: true,
                    ..FormattingOptions::default()
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .expect("formatting should run");

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, expected);
    }

    #[test]
    fn will_save_wait_until_formats_open_document() {
        let uri = Url::parse("file:///tmp/index.ms").expect("uri should parse");
        let source = "let x:=1;";
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let edits = server
            .will_save_formatting(WillSaveTextDocumentParams {
                text_document: TextDocumentIdentifier { uri },
                reason: TextDocumentSaveReason::MANUAL,
            })
            .expect("will save formatting should run");

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].range, full_document_range(source));
        assert_eq!(edits[0].new_text, "let x := 1;\n");
    }

    #[test]
    fn document_formatting_uses_manifest_profile_and_overrides() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            "{\n  \"name\": \"app\",\n  \"version\": \"0.1.0\",\n  \"fmt\": { \"profile\": \"expanded\", \"matchArmIndent\": \"pipeAligned\" }\n}\n",
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        fs::write(&path, "export let result : Int := 1;").expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let source = "export let describe (target : Ordering) : String := match target(| .Less => \"less\" | .GreaterThanEverything => \"greater\" | _ => \"same\");";
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let edits = server
            .document_formatting(DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                options: FormattingOptions {
                    tab_size: 2,
                    insert_spaces: true,
                    ..FormattingOptions::default()
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .expect("formatting should run");

        assert_eq!(edits.len(), 1);
        assert_eq!(
            edits[0].new_text,
            "export let describe (\n  target : Ordering\n) : String :=\n  match target (\n  | .Less                  => \"less\"\n  | .GreaterThanEverything => \"greater\"\n  | _                      => \"same\"\n  );\n"
        );
    }

    #[test]
    fn document_formatting_formats_musi_fences_in_markdown_documents() {
        let root = temp_project();
        let path = root.join("README.md");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let source = "# Example\n\n```musi\nlet testing:=import \"@std/testing\";\nlet io:=import \"@std/io\";\n```\n";
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let edits = server
            .document_formatting(DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                options: FormattingOptions {
                    tab_size: 2,
                    insert_spaces: true,
                    ..FormattingOptions::default()
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .expect("formatting should run");

        assert_eq!(edits.len(), 1);
        assert_eq!(
            edits[0].new_text,
            "# Example\n\n```musi\nlet io := import \"@std/io\";\nlet testing := import \"@std/testing\";\n```\n"
        );
    }

    #[test]
    fn document_range_formatting_formats_selected_source_range() {
        let uri = Url::parse("file:///tmp/index.ms").expect("uri should parse");
        let source = r#"let testing:=import "@std/testing";
let io:=import "@std/io";
let value:=1;
"#;
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let edits = server
            .document_range_formatting(DocumentRangeFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                range: Range {
                    start: Position::new(0, 0),
                    end: Position::new(2, 0),
                },
                options: FormattingOptions {
                    tab_size: 2,
                    insert_spaces: true,
                    ..FormattingOptions::default()
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .expect("range formatting should run");

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].range.start, Position::new(0, 0));
        assert_eq!(edits[0].range.end, Position::new(2, 0));
        assert_eq!(
            edits[0].new_text,
            "let io := import \"@std/io\";\nlet testing := import \"@std/testing\";\n"
        );
    }

    #[test]
    fn document_on_type_formatting_formats_open_document_on_trigger() {
        let uri = Url::parse("file:///tmp/index.ms").expect("uri should parse");
        let source = "let x:=1;";
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let edits = server
            .document_on_type_formatting(DocumentOnTypeFormattingParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(0, 9),
                },
                ch: ";".to_owned(),
                options: FormattingOptions {
                    tab_size: 2,
                    insert_spaces: true,
                    ..FormattingOptions::default()
                },
            })
            .expect("on type formatting should run");

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].range, full_document_range(source));
        assert_eq!(edits[0].new_text, "let x := 1;\n");
    }

    #[test]
    fn document_on_type_formatting_ignores_non_trigger_characters() {
        let uri = Url::parse("file:///tmp/index.ms").expect("uri should parse");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server
            .open_documents
            .insert(uri.clone(), "let x:=1;".to_owned());

        let edits = server
            .document_on_type_formatting(DocumentOnTypeFormattingParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(0, 5),
                },
                ch: "x".to_owned(),
                options: FormattingOptions {
                    tab_size: 2,
                    insert_spaces: true,
                    ..FormattingOptions::default()
                },
            })
            .expect("on type formatting should run");

        assert!(edits.is_empty());
    }

    #[test]
    fn completion_returns_keywords_and_current_bindings() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = r"let before := 1;
let current := bef;
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let response = server
            .completions(CompletionParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(1, 18),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
                context: None,
            })
            .expect("completion response should exist");
        assert!(matches!(response, CompletionResponse::List(_)));
        let items = match response {
            CompletionResponse::List(list) => {
                assert!(!list.is_incomplete);
                list.items
            }
            CompletionResponse::Array(items) => items,
        };
        let before = items
            .iter()
            .find(|item| item.label == "before")
            .expect("before completion should exist");

        assert!(items.iter().any(|item| item.label == "let"));
        assert_eq!(before.kind, Some(CompletionItemKind::VARIABLE));
        assert_eq!(before.detail, None);
        assert_eq!(before.documentation, None);
        assert!(before.data.is_some());
        let before = server.resolve_completion(before.clone());
        assert_eq!(before.detail.as_deref(), Some("binding"));
        assert_eq!(before.documentation, None);
        let edit = before
            .text_edit
            .as_ref()
            .and_then(|edit| match edit {
                CompletionTextEdit::Edit(edit) => Some(edit),
                CompletionTextEdit::InsertAndReplace(_) => None,
            })
            .expect("completion should provide replacement edit");
        assert_eq!(edit.range.start, Position::new(1, 15));
        assert_eq!(edit.range.end, Position::new(1, 18));
        assert_eq!(edit.new_text, "before");
    }

    #[test]
    fn completion_after_dot_returns_member_items() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = r"let point := { x := 1, y := 2 };
point.
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let response = server
            .completions(CompletionParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(1, 6),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
                context: None,
            })
            .expect("completion response should exist");
        assert!(matches!(response, CompletionResponse::List(_)));
        let items = match response {
            CompletionResponse::List(list) => {
                assert!(!list.is_incomplete);
                list.items
            }
            CompletionResponse::Array(items) => items,
        };

        assert!(items.iter().any(|item| item.label == "x"));
        assert!(items.iter().any(|item| item.label == "y"));
        assert!(!items.iter().any(|item| item.label == "let"));
        assert!(
            items
                .iter()
                .all(|item| item.kind == Some(CompletionItemKind::PROPERTY))
        );
    }

    #[test]
    fn signature_help_returns_active_callable_parameter() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = "\
let render (port : Int, secure : Bool) : Int := port;
render(8080, 1 = 1);
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let help = server
            .signature_help_at(SignatureHelpParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(1, 13),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                context: None,
            })
            .expect("signature help should exist");

        assert_eq!(help.active_signature, Some(0));
        assert_eq!(help.active_parameter, Some(1));
        assert_eq!(help.signatures[0].label, "render(Int, Bool) -> Int");
    }

    #[test]
    fn did_change_configuration_updates_hover_settings() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = "let message : String := \"Hello\";\nmessage;\n";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        server.update_configuration(DidChangeConfigurationParams {
            settings: serde_json::json!({
                "hover": {
                    "maximumLength": 10,
                },
            }),
        });
        let hover = server
            .hover_at(HoverParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(1, 2),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .expect("hover should resolve");

        let HoverContents::Markup(contents) = hover.contents else {
            panic!("markup hover expected");
        };
        assert_eq!(contents.value, "```musi\n(v…");
    }

    #[test]
    fn workspace_symbols_use_initialize_workspace_roots_without_open_documents() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "src/index.ms"
}
"#,
        )
        .expect("manifest should be written");
        fs::create_dir_all(root.join("src")).expect("src dir should be created");
        fs::write(
            root.join("src/index.ms"),
            "let extra := import \"./extra\";\nlet entryValue := extra.extraValue;\n",
        )
        .expect("entry should be written");
        fs::write(root.join("src/extra.ms"), "export let extraValue := 2;\n")
            .expect("extra should be written");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        #[allow(deprecated)]
        server.configure(&InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: None,
            initialization_options: None,
            capabilities: ClientCapabilities::default(),
            trace: None,
            workspace_folders: Some(vec![WorkspaceFolder {
                uri: Url::from_file_path(&root).expect("workspace URI should build"),
                name: "app".to_owned(),
            }]),
            client_info: None,
            locale: None,
            work_done_progress_params: WorkDoneProgressParams::default(),
        });

        let response = server
            .workspace_symbols(&WorkspaceSymbolParams {
                query: "Value".to_owned(),
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("workspace symbols should run");
        let WorkspaceSymbolResponse::Flat(symbols) = response else {
            panic!("flat workspace symbols expected");
        };
        let names = symbols
            .iter()
            .map(|symbol| symbol.name.as_str())
            .collect::<Vec<_>>();

        assert!(names.contains(&"entryValue"));
        assert!(names.contains(&"extraValue"));
    }

    #[test]
    fn workspace_symbols_use_open_document_overlay_for_open_files() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "src/index.ms"
}
"#,
        )
        .expect("manifest should be written");
        fs::create_dir_all(root.join("src")).expect("src dir should be created");
        let path = root.join("src/index.ms");
        fs::write(&path, "let entryValue := 1;\n").expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("entry URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        #[allow(deprecated)]
        server.configure(&InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: None,
            initialization_options: None,
            capabilities: ClientCapabilities::default(),
            trace: None,
            workspace_folders: Some(vec![WorkspaceFolder {
                uri: Url::from_file_path(&root).expect("workspace URI should build"),
                name: "app".to_owned(),
            }]),
            client_info: None,
            locale: None,
            work_done_progress_params: WorkDoneProgressParams::default(),
        });
        let _ = server
            .open_documents
            .insert(uri, "let unsavedValue := 1;\n".to_owned());

        let response = server
            .workspace_symbols(&WorkspaceSymbolParams {
                query: "Value".to_owned(),
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("workspace symbols should run");
        let WorkspaceSymbolResponse::Flat(symbols) = response else {
            panic!("flat workspace symbols expected");
        };
        let names = symbols
            .iter()
            .map(|symbol| symbol.name.as_str())
            .collect::<Vec<_>>();

        assert!(names.contains(&"unsavedValue"), "{names:?}");
        assert!(!names.contains(&"entryValue"), "{names:?}");
    }

    #[test]
    fn workspace_folder_changes_update_workspace_symbol_roots() {
        let old_root = temp_project();
        fs::write(
            old_root.join("musi.json"),
            r#"{
  "name": "old",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("old manifest should be written");
        fs::write(old_root.join("index.ms"), "let oldValue := 1;\n")
            .expect("old entry should be written");
        let new_root = temp_project();
        fs::write(
            new_root.join("musi.json"),
            r#"{
  "name": "new",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("new manifest should be written");
        fs::write(new_root.join("index.ms"), "let newValue := 1;\n")
            .expect("new entry should be written");
        let old_folder = WorkspaceFolder {
            uri: Url::from_file_path(&old_root).expect("old workspace URI should build"),
            name: "old".to_owned(),
        };
        let new_folder = WorkspaceFolder {
            uri: Url::from_file_path(&new_root).expect("new workspace URI should build"),
            name: "new".to_owned(),
        };
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        #[allow(deprecated)]
        server.configure(&InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: None,
            initialization_options: None,
            capabilities: ClientCapabilities::default(),
            trace: None,
            workspace_folders: Some(vec![old_folder.clone()]),
            client_info: None,
            locale: None,
            work_done_progress_params: WorkDoneProgressParams::default(),
        });

        server.update_workspace_folders(DidChangeWorkspaceFoldersParams {
            event: WorkspaceFoldersChangeEvent {
                added: vec![new_folder],
                removed: vec![old_folder],
            },
        });

        let response = server
            .workspace_symbols(&WorkspaceSymbolParams {
                query: "Value".to_owned(),
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("workspace symbols should run");
        let WorkspaceSymbolResponse::Flat(symbols) = response else {
            panic!("flat workspace symbols expected");
        };
        let names = symbols
            .iter()
            .map(|symbol| symbol.name.as_str())
            .collect::<Vec<_>>();

        assert!(names.contains(&"newValue"), "{names:?}");
        assert!(!names.contains(&"oldValue"), "{names:?}");
    }

    #[test]
    fn declaration_reuses_symbol_definition_location() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = r"let value := 1;
let other := value;
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let declaration = server
            .definition_at(GotoDefinitionParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(1, 14),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("declaration should exist");

        let GotoDefinitionResponse::Scalar(location) = declaration else {
            panic!("declaration should return scalar location");
        };
        assert_eq!(location.range.start, Position::new(0, 4));
        assert_eq!(location.range.end, Position::new(0, 9));
    }

    #[test]
    fn type_definition_resolves_named_value_type() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = "\
let Box[T] := data {
  value : T;
};
let boxedName : Box[String] := {
  value := \"Nora\"
};
boxedName.value;
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let response = server
            .type_definition_at(GotoDefinitionParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(6, 2),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("type definition should resolve");
        let GotoDefinitionResponse::Scalar(location) = response else {
            panic!("scalar location expected");
        };

        assert_eq!(location.range.start, Position::new(0, 4));
        assert_eq!(location.range.end, Position::new(0, 7));
    }

    #[test]
    fn document_highlight_returns_declaration_and_same_file_references() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = r"let value := 1;
let other := value + value;
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let highlights = server
            .document_highlights(DocumentHighlightParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(1, 15),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("document highlight response should exist");

        assert_eq!(highlights.len(), 3);
        assert!(
            highlights
                .iter()
                .all(|highlight| highlight.kind == Some(DocumentHighlightKind::TEXT))
        );
        assert!(
            highlights
                .iter()
                .any(|highlight| highlight.range.start == Position::new(0, 4))
        );
        assert!(
            highlights
                .iter()
                .filter(|highlight| highlight.range.start.line == 1)
                .count()
                == 2
        );
    }

    #[test]
    fn linked_editing_range_returns_same_file_symbol_ranges() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = r"let value := 1;
let other := value + value;
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let linked = server
            .linked_editing_ranges(LinkedEditingRangeParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(1, 14),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .expect("linked editing ranges should exist");

        assert_eq!(linked.ranges.len(), 3);
        assert!(linked.ranges.contains(&Range {
            start: Position::new(0, 4),
            end: Position::new(0, 9),
        }));
        assert!(linked.ranges.contains(&Range {
            start: Position::new(1, 13),
            end: Position::new(1, 18),
        }));
        assert!(linked.ranges.contains(&Range {
            start: Position::new(1, 21),
            end: Position::new(1, 26),
        }));
        assert_eq!(
            linked.word_pattern.as_deref(),
            Some("[A-Za-z_][A-Za-z0-9_]*")
        );
    }

    #[test]
    fn code_action_returns_source_organize_imports_edit() {
        let root = temp_project();
        let path = root.join("index.ms");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let source = "let testing:=import \"@std/testing\";\nlet io:=import \"@std/io\";\n";
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let actions = server
            .code_actions(CodeActionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                range: full_document_range(source),
                context: CodeActionContext {
                    diagnostics: Vec::new(),
                    only: Some(vec![CodeActionKind::SOURCE_ORGANIZE_IMPORTS]),
                    trigger_kind: None,
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("code action response should exist");

        assert_eq!(actions.len(), 1);
        let CodeActionOrCommand::CodeAction(action) = &actions[0] else {
            panic!("organize imports should be returned as code action");
        };
        assert_eq!(action.kind, Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS));
        assert_eq!(action.is_preferred, Some(true));
        assert!(action.edit.is_none());
        assert!(action.data.is_some());
        let action = server.resolve_code_action(action.clone());
        let edit = action.edit.as_ref().expect("action should provide edit");
        let changes = edit.changes.as_ref().expect("edit should include changes");
        let edits = changes.get(&uri).expect("edit should target document URI");
        assert_eq!(edits.len(), 1);
        assert_eq!(
            edits[0].new_text,
            "let io := import \"@std/io\";\nlet testing := import \"@std/testing\";\n"
        );
    }

    #[test]
    fn document_link_resolves_static_import_targets() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let dep_path = root.join("dep.ms");
        let source = "let dep := import \"./dep\";\n";
        fs::write(&path, source).expect("entry should be written");
        fs::write(&dep_path, "export let value := 1;\n").expect("dep should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let links = server
            .document_links(DocumentLinkParams {
                text_document: TextDocumentIdentifier { uri },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("document links should run");

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].range.start, Position::new(0, 18));
        assert_eq!(links[0].range.end, Position::new(0, 25));
        assert_eq!(links[0].target, None);
        assert_eq!(links[0].tooltip, None);
        assert!(links[0].data.is_some());

        let link = server.resolve_document_link(links[0].clone());

        assert_eq!(
            link.target.as_ref(),
            Some(
                &Url::from_file_path(
                    fs::canonicalize(dep_path).expect("dep path should canonicalize")
                )
                .expect("dep URI should build")
            )
        );
        assert!(
            link.tooltip
                .as_deref()
                .is_some_and(|tooltip| tooltip.starts_with("Open `"))
        );
    }

    #[test]
    fn hover_returns_module_docs_on_module_doc_comments() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = "--! module docs\n--! more module docs\nlet message : String := \"Hello\";\n";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let hover = server
            .hover_at(HoverParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position::new(0, 3),
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .expect("module doc hover should resolve");

        let HoverContents::Markup(contents) = hover.contents else {
            panic!("markup hover expected");
        };
        assert_eq!(hover.range.expect("hover range").start, Position::new(0, 0));
        assert!(contents.value.contains("module docs"));
        assert!(contents.value.contains("more module docs"));
    }

    #[test]
    fn code_lens_returns_reference_counts_for_document_symbols() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = r"let value := 1;
let other := value + value;
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let lenses = server
            .code_lenses(CodeLensParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("code lenses should run");

        let value_lens = lenses
            .iter()
            .find(|lens| lens.range.start == Position::new(0, 4))
            .expect("value reference lens should exist");
        assert_eq!(value_lens.command, None);
        assert!(value_lens.data.is_some());

        let value_lens = server.resolve_code_lens(value_lens.clone());

        let command = value_lens.command.as_ref().expect("lens command");
        assert_eq!(command.title, "2 references");
        assert_eq!(command.command, "musi.references");
        let arguments = command.arguments.as_ref().expect("lens arguments");
        assert_eq!(arguments.len(), 1);
        assert_eq!(
            arguments[0].get("uri").and_then(|value| value.as_str()),
            Some(uri.as_str())
        );
        assert_eq!(
            arguments[0].get("line").and_then(|value| value.as_u64()),
            Some(0)
        );
        assert_eq!(
            arguments[0]
                .get("character")
                .and_then(|value| value.as_u64()),
            Some(4)
        );
    }

    #[test]
    fn execute_references_command_returns_reference_locations() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = r"let value := 1;
let other := value + value;
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let result = server
            .execute_command_request(ExecuteCommandParams {
                command: "musi.references".to_owned(),
                arguments: vec![serde_json::json!({
                    "uri": uri.as_str(),
                    "line": 0,
                    "character": 4,
                })],
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .expect("references command should return locations");
        let locations: Vec<Location> =
            serde_json::from_value(result).expect("locations should deserialize");

        assert_eq!(locations.len(), 2);
        assert!(locations.iter().all(|location| location.uri == uri));
        assert!(
            locations
                .iter()
                .all(|location| location.range.start.line == 1)
        );
    }

    #[test]
    fn folding_range_returns_multiline_node_and_comment_ranges() {
        let root = temp_project();
        let path = root.join("index.ms");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let source = "\
/-- docs
    more docs -/
let Pair := data {
  left : Int;
  right : Int;
};
";
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let ranges = server
            .folding_ranges(FoldingRangeParams {
                text_document: TextDocumentIdentifier { uri },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("folding range response should exist");

        assert!(ranges.iter().any(|range| {
            range.kind == Some(FoldingRangeKind::Comment)
                && range.start_line == 0
                && range.end_line == 1
        }));
        assert!(
            ranges
                .iter()
                .any(|range| range.start_line == 2 && range.end_line == 5)
        );
    }

    #[test]
    fn selection_range_expands_identifier_selection_to_parent_ranges() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = r"let value := 1;
let other := value + 2;
";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let ranges = server
            .selection_ranges(SelectionRangeParams {
                text_document: TextDocumentIdentifier { uri },
                positions: vec![Position::new(1, 13)],
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
            })
            .expect("selection ranges should run");

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].range.start, Position::new(1, 13));
        assert_eq!(ranges[0].range.end, Position::new(1, 18));
        assert!(
            ranges[0]
                .parent
                .as_ref()
                .is_some_and(|parent| parent.range.start.line == 1
                    && parent.range.end.character >= ranges[0].range.end.character)
        );
    }

    #[test]
    fn did_save_document_is_handled_without_routing_fallback() {
        let uri = Url::parse("file:///tmp/index.ms").expect("uri should parse");
        let params = DidSaveTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
            text: None,
        };
        let handler: fn(&mut MusiLanguageServer, DidSaveTextDocumentParams) -> NotifyResult =
            <MusiLanguageServer as LanguageServer>::did_save;

        let _ = params;
        let _ = handler;
    }

    #[test]
    fn document_diagnostic_returns_full_report_for_open_document() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = "let value := 1;\n";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let report = server.document_diagnostics(DocumentDiagnosticParams {
            text_document: TextDocumentIdentifier { uri },
            identifier: Some("musi".to_owned()),
            previous_result_id: None,
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        });

        let DocumentDiagnosticReportResult::Report(DocumentDiagnosticReport::Full(report)) = report
        else {
            panic!("document diagnostics should return a full report");
        };
        assert_eq!(report.full_document_diagnostic_report.result_id, None);
    }

    #[test]
    fn workspace_diagnostic_returns_open_document_reports() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        let source = "let value := 1;\n";
        fs::write(&path, source).expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        let _ = server.open_documents.insert(uri.clone(), source.to_owned());

        let report = server.workspace_diagnostics(WorkspaceDiagnosticParams {
            identifier: Some("musi".to_owned()),
            previous_result_ids: Vec::new(),
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        });

        let WorkspaceDiagnosticReportResult::Report(report) = report else {
            panic!("workspace diagnostics should return a report");
        };
        assert_eq!(report.items.len(), 1);
        let WorkspaceDocumentDiagnosticReport::Full(item) = &report.items[0] else {
            panic!("workspace diagnostics should use full reports");
        };
        let report_path = item
            .uri
            .to_file_path()
            .expect("report URI should be file path");
        assert!(paths_match(&report_path, &path));
        assert_eq!(item.version, None);
    }

    #[test]
    fn workspace_diagnostic_uses_initialize_workspace_roots_without_open_documents() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        fs::write(&path, "let value : Int := \"bad\";\n").expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        #[allow(deprecated)]
        server.configure(&InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: None,
            initialization_options: None,
            capabilities: ClientCapabilities::default(),
            trace: None,
            workspace_folders: Some(vec![WorkspaceFolder {
                uri: Url::from_file_path(&root).expect("workspace URI should build"),
                name: "app".to_owned(),
            }]),
            client_info: None,
            locale: None,
            work_done_progress_params: WorkDoneProgressParams::default(),
        });

        let report = server.workspace_diagnostics(WorkspaceDiagnosticParams {
            identifier: Some("musi".to_owned()),
            previous_result_ids: Vec::new(),
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        });

        let WorkspaceDiagnosticReportResult::Report(report) = report else {
            panic!("workspace diagnostics should return a report");
        };
        assert_eq!(report.items.len(), 1);
        let WorkspaceDocumentDiagnosticReport::Full(item) = &report.items[0] else {
            panic!("workspace diagnostics should use full reports");
        };
        let report_path = item
            .uri
            .to_file_path()
            .expect("report URI should be file path");
        assert!(paths_match(&report_path, &path));
        assert!(!item.full_document_diagnostic_report.items.is_empty());
    }

    #[test]
    fn workspace_diagnostic_uses_open_document_overlay_for_open_files() {
        let root = temp_project();
        fs::write(
            root.join("musi.json"),
            r#"{
  "name": "app",
  "version": "0.1.0",
  "entry": "index.ms"
}
"#,
        )
        .expect("manifest should be written");
        let path = root.join("index.ms");
        fs::write(&path, "let value : Int := 1;\n").expect("entry should be written");
        let uri = Url::from_file_path(&path).expect("file URI should build");
        let mut server = MusiLanguageServer::new(ClientSocket::new_closed());
        #[allow(deprecated)]
        server.configure(&InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: None,
            initialization_options: None,
            capabilities: ClientCapabilities::default(),
            trace: None,
            workspace_folders: Some(vec![WorkspaceFolder {
                uri: Url::from_file_path(&root).expect("workspace URI should build"),
                name: "app".to_owned(),
            }]),
            client_info: None,
            locale: None,
            work_done_progress_params: WorkDoneProgressParams::default(),
        });
        let _ = server
            .open_documents
            .insert(uri.clone(), "let value : Int := \"bad\";\n".to_owned());

        let report = server.workspace_diagnostics(WorkspaceDiagnosticParams {
            identifier: Some("musi".to_owned()),
            previous_result_ids: Vec::new(),
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        });

        let WorkspaceDiagnosticReportResult::Report(report) = report else {
            panic!("workspace diagnostics should return a report");
        };
        assert_eq!(report.items.len(), 1);
        let WorkspaceDocumentDiagnosticReport::Full(item) = &report.items[0] else {
            panic!("workspace diagnostics should use full reports");
        };
        assert_eq!(item.uri, uri);
        assert!(!item.full_document_diagnostic_report.items.is_empty());
    }

    #[test]
    fn cli_range_is_zero_based_lsp_range() {
        let range = to_cli_range(&CliDiagnosticRange {
            start_line: 3,
            start_col: 5,
            end_line: 3,
            end_col: 8,
        });

        assert_eq!(range.start, Position::new(2, 4));
        assert_eq!(range.end, Position::new(2, 7));
    }

    #[test]
    fn tool_range_is_zero_based_lsp_range() {
        let range = to_tool_range(&ToolRange::new(2, 3, 2, 8));

        assert_eq!(range.start, Position::new(1, 2));
        assert_eq!(range.end, Position::new(1, 7));
    }

    #[test]
    fn semantic_token_encoding_uses_relative_positions() {
        let tokens = vec![
            ToolSemanticToken::new(
                ToolRange::new(1, 1, 1, 4),
                ToolSemanticTokenKind::Keyword,
                Vec::new(),
            ),
            ToolSemanticToken::new(
                ToolRange::new(2, 3, 2, 10),
                ToolSemanticTokenKind::Variable,
                vec![
                    ToolSemanticModifier::Declaration,
                    ToolSemanticModifier::Definition,
                ],
            ),
        ];

        assert_eq!(
            encode_semantic_tokens(&tokens, None),
            vec![
                SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 3,
                    token_type: 10,
                    token_modifiers_bitset: 0,
                },
                SemanticToken {
                    delta_line: 1,
                    delta_start: 2,
                    length: 7,
                    token_type: 4,
                    token_modifiers_bitset: 0b11,
                },
            ]
        );
    }

    #[test]
    fn diagnostic_matching_normalizes_file_paths() {
        let path = temp_dir().join("project").join("index.ms");
        let dotted = temp_dir().join("project").join(".").join("index.ms");
        let diagnostic = CliDiagnostic::new("sema", "error", "type mismatch")
            .with_file(Some(dotted.display().to_string()));

        assert!(diagnostic_matches_path(Path::new(&path), &diagnostic));
    }

    #[test]
    fn lsp_diagnostic_uses_primary_range_and_related_file_uri() {
        let path = temp_dir().join("project").join("index.ms");
        let path_text = path.display().to_string();
        let diagnostic = CliDiagnostic::new("resolve", "error", "unbound name `missing`")
            .with_file(Some(path_text.clone()))
            .with_range(Some(CliDiagnosticRange {
                start_line: 2,
                start_col: 3,
                end_line: 2,
                end_col: 10,
            }))
            .with_labels(vec![CliDiagnosticLabel::new(
                Some(path_text),
                Some(CliDiagnosticRange {
                    start_line: 2,
                    start_col: 3,
                    end_line: 2,
                    end_col: 10,
                }),
                "unbound name `missing`".to_owned(),
            )]);

        let converted = to_lsp_diagnostic(diagnostic);

        assert_eq!(converted.range.start, Position::new(1, 2));
        assert_eq!(converted.range.end, Position::new(1, 9));
        let related = converted
            .related_information
            .expect("related information should exist");
        assert_eq!(related[0].location.uri.scheme(), "file");
    }

    #[test]
    fn inlay_hint_conversion_uses_lsp_kind_and_padding() {
        let mut tool_hint = ToolInlayHint::new(
            ToolPosition::new(2, 5),
            "value:",
            ToolInlayHintKind::Parameter,
        );
        tool_hint.tooltip = Some("parameter `value`".to_owned());
        let hint = to_lsp_inlay_hint(tool_hint);

        assert_eq!(hint.position, Position::new(1, 4));
        assert!(matches!(hint.kind, Some(InlayHintKind::PARAMETER)));
        assert_eq!(hint.padding_right, Some(true));
        assert!(hint.tooltip.is_none());
        assert!(hint.data.is_some());

        let hint = resolve_lsp_inlay_hint(hint);

        assert!(matches!(
            hint.tooltip,
            Some(InlayHintTooltip::String(ref tooltip)) if tooltip == "parameter `value`"
        ));
    }

    #[test]
    fn hover_contents_truncate_to_configured_limit() {
        assert_eq!(truncate_hover_contents("abcdef", 3), "abc…");
        assert_eq!(truncate_hover_contents("abc", 3), "abc");
    }
}

mod failure {
    use super::*;

    #[test]
    fn unknown_severity_defaults_to_error() {
        assert_eq!(to_severity("fatal"), DiagnosticSeverity::ERROR);
    }

    #[test]
    fn missing_cli_range_uses_strict_default_range() {
        assert_eq!(
            default_range(),
            Range {
                start: Position::new(0, 0),
                end: Position::new(0, 1),
            }
        );
    }
}
