//! Step 3: comments as trivia. The `Comment`/`Trivia` types themselves live
//! in ast.rs (every stage needs them); this module adds the one piece of
//! comment-specific structure worth recognizing in v1: the repeated
//! `//` / `// Name` / `//` "banner" convention that precedes almost every
//! function and struct in this codebase (see m_misc.c's `M_DrawText`).
//!
//! Same-line trailing comments (e.g. `// S_NULL` after a `states[]` entry)
//! are not attached separately in v1 - they live inside a brace group's
//! opaque `inner` tokens already, so they're preserved as part of the
//! enclosing item's raw text without needing dedicated trailing-trivia
//! logic.

use super::ast::Comment;

/// If `comments` looks like a `//` / `// Name` / `//` banner, returns the
/// middle line trimmed. Otherwise `None`.
pub fn banner_doc(comments: &[Comment]) -> Option<String> {
    if comments.len() < 3 {
        return None;
    }
    let last3 = &comments[comments.len() - 3..];
    let mut lines = Vec::with_capacity(3);
    for c in last3 {
        match c {
            Comment::Line(s) => lines.push(s.trim_end_matches(['\n', '\r']).trim_start_matches("//").trim()),
            Comment::Block(_) => return None,
        }
    }
    if !lines[0].is_empty() || lines[1].is_empty() || !lines[2].is_empty() {
        return None;
    }
    Some(lines[1].to_string())
}

#[cfg(test)]
mod tests;
