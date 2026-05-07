use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use async_lsp::lsp_types::{SemanticToken, SemanticTokensDelta, SemanticTokensEdit};

#[derive(Debug, Clone)]
pub(super) struct SemanticTokenSnapshot {
    pub(super) result_id: String,
    pub(super) data: Vec<SemanticToken>,
}

pub(super) fn semantic_tokens_result_id(tokens: &[SemanticToken]) -> String {
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

pub(super) fn semantic_tokens_delta(
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
