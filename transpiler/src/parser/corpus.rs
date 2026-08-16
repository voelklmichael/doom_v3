//! Corpus-wide known-type-name collection, used to resolve the C
//! cast-vs-parenthesized-expression ambiguity (see `stmt::expr`). Kept
//! separate from `stmt/` (this module touches the filesystem, reusing the
//! existing `parse_file` pipeline) so `stmt::expr` itself stays easy to
//! unit test with a hand-built `KnownTypeNames`.
//!
//! Deliberately corpus-wide, not file-local: a `.c` file's function bodies
//! routinely use typedefs defined only in headers it `#include`s, and this
//! parser does no include-resolution - a file-local-only set would be
//! wrong for real code in this corpus (e.g. `p_local.h`'s `intercept_t`
//! used throughout `.c` files that don't define it themselves).

use super::ast::{CondGroup, Item, ItemKind, Trivia};
use super::stmt::expr::KnownTypeNames;
use std::path::PathBuf;

/// Parses every file in `paths` (via the existing `parse_file` pipeline)
/// and harvests every `typedef` name and tagged `struct`/`union`/`enum`
/// name into one `KnownTypeNames` set. A file that fails to parse is
/// skipped, not fatal - this is a best-effort disambiguation aid, not a
/// correctness-critical pass in its own right.
pub fn collect_known_type_names(paths: &[PathBuf]) -> KnownTypeNames {
    let mut known = KnownTypeNames::new();
    for path in paths {
        if let Ok(file) = crate::parse_file(path) {
            collect_from_items(&file.items, &mut known);
        }
    }
    known
}

fn collect_from_items(items: &[(Item, Trivia)], known: &mut KnownTypeNames) {
    for (item, _) in items {
        match &item.kind {
            ItemKind::Typedef(td) => known.insert(td.name.clone()),
            ItemKind::Record(rd) => {
                if let Some(tag) = &rd.tag {
                    known.insert(tag.clone());
                }
                if let Some(name) = &rd.typedef_name {
                    known.insert(name.clone());
                }
            }
            ItemKind::Enum(ed) => {
                if let Some(tag) = &ed.tag {
                    known.insert(tag.clone());
                }
                if let Some(name) = &ed.typedef_name {
                    known.insert(name.clone());
                }
            }
            ItemKind::Conditional(cg) => collect_from_conditional(cg, known),
            _ => {}
        }
    }
}

fn collect_from_conditional(cg: &CondGroup, known: &mut KnownTypeNames) {
    for branch in &cg.branches {
        collect_from_items(&branch.body, known);
    }
    if let Some(else_body) = &cg.else_body {
        collect_from_items(else_body, known);
    }
}

#[cfg(test)]
mod tests;
