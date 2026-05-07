//! Inlay hint helpers for the LSP server.

use async_lsp::lsp_types::{InlayHint, InlayHintParams};
use musi_tooling::inlay_hints_for_project_file_with_overlay;

use super::MusiLanguageServer;
use super::convert::{position_in_range, resolve_lsp_inlay_hint, to_lsp_inlay_hint};

impl MusiLanguageServer {
    pub(super) fn inlay_hints(&self, params: &InlayHintParams) -> Option<Vec<InlayHint>> {
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

    pub(super) fn resolve_inlay_hint(hint: InlayHint) -> InlayHint {
        resolve_lsp_inlay_hint(hint)
    }
}
