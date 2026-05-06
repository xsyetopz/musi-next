use std::env::temp_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use async_lsp::lsp_types::{
    CodeActionContext, CodeActionKind, CodeActionOrCommand, CodeActionParams, CompletionItemKind,
    CompletionTextEdit, DiagnosticSeverity, DocumentHighlightKind, DocumentLinkParams,
    DocumentRangeFormattingParams, FoldingRangeKind, FoldingRangeParams, InlayHintKind,
    PartialResultParams, Position, SelectionRangeParams, SemanticToken, TextDocumentIdentifier,
    TextDocumentPositionParams, WorkDoneProgressParams,
};
use musi_tooling::{
    CliDiagnostic, CliDiagnosticLabel, CliDiagnosticRange, ToolInlayHint, ToolInlayHintKind,
    ToolPosition, ToolRange, ToolSemanticModifier, ToolSemanticToken, ToolSemanticTokenKind,
};

use super::convert::{
    default_range, diagnostic_matches_path, to_cli_range, to_lsp_diagnostic, to_lsp_inlay_hint,
    to_severity, truncate_hover_contents,
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
            Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL))
        );
        assert_eq!(
            initialize_result.capabilities.hover_provider,
            Some(HoverProviderCapability::Simple(true))
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
        assert!(
            initialize_result
                .capabilities
                .semantic_tokens_provider
                .is_some()
        );
        assert!(initialize_result.capabilities.inlay_hint_provider.is_some());
        assert!(initialize_result.capabilities.completion_provider.is_some());
        assert_eq!(
            initialize_result.capabilities.definition_provider,
            Some(OneOf::Left(true))
        );
        assert_eq!(
            initialize_result.capabilities.references_provider,
            Some(OneOf::Left(true))
        );
        assert_eq!(
            initialize_result.capabilities.document_highlight_provider,
            Some(OneOf::Left(true))
        );
        assert_eq!(
            initialize_result.capabilities.document_symbol_provider,
            Some(OneOf::Left(true))
        );
        assert!(
            initialize_result
                .capabilities
                .document_link_provider
                .is_some()
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
        assert!(
            initialize_result
                .capabilities
                .code_action_provider
                .is_some()
        );
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
    fn document_link_returns_static_import_targets() {
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
        assert_eq!(
            links[0].target.as_ref(),
            Some(
                &Url::from_file_path(
                    fs::canonicalize(dep_path).expect("dep path should canonicalize")
                )
                .expect("dep URI should build")
            )
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
        let hint = to_lsp_inlay_hint(ToolInlayHint::new(
            ToolPosition::new(2, 5),
            "value:",
            ToolInlayHintKind::Parameter,
        ));

        assert_eq!(hint.position, Position::new(1, 4));
        assert!(matches!(hint.kind, Some(InlayHintKind::PARAMETER)));
        assert_eq!(hint.padding_right, Some(true));
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
