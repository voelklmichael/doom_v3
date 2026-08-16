//! Step 7d: local declaration statements - the `stmt`-level counterpart to
//! `parser::decl`'s top-level `VarDecl`, but supporting several
//! comma-separated declarators sharing one base type/storage
//! (`int *a, b[4];` - common inside function bodies, unlike top-level code,
//! which never repeats a declarator this way).

use super::super::ast::{Storage, Type};
use super::super::decl::{parse_bare_declarator_suffix, parse_declarator_with_base};
use super::expr::{BASE_TYPE_WORDS, Expr, KnownTypeNames, parse_expr};
use super::lex::{CTok, Punct, render_ctoks_no_comments};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DeclStmt {
    pub storage: Vec<Storage>,
    pub base_ty: Type,
    pub declarators: Vec<Declarator>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Declarator {
    pub ty: Type,
    pub name: String,
    pub initializer: Option<LocalInit>,
}

/// Parallels the top-level `ast::Init`, but with the real `Expr` grammar for
/// scalar elements instead of raw text - kept as its own type rather than
/// unifying with `Init` (the two serve different granularities: `Init` is a
/// module/global-scope initializer, `LocalInit` a function-local one, and
/// forcing them to share a shape isn't worth the coupling).
#[derive(Debug, Clone, Serialize)]
pub enum LocalInit {
    Braced(Vec<LocalInit>),
    Expr(Expr),
}

/// Tries to parse `toks` (one declaration statement's tokens, *not*
/// including the terminating `;`) as a `DeclStmt`. Splits on top-level `,`
/// (paren/bracket-aware - see `split_ctoks_top_level`): the first piece
/// supplies the shared storage/base-type via `parse_declarator_with_base`;
/// each later piece applies its own `*`/`[]` decoration on top of that same
/// base type via `parse_bare_declarator_suffix` (C forbids repeating the
/// type word on a later declarator). Returns `None` if even the first
/// declarator doesn't parse - callers degrade to `StmtKind::Raw` in that
/// case, same as every other best-effort parse in this codebase.
pub fn try_parse_decl_stmt(toks: &[CTok], known: &KnownTypeNames) -> Option<DeclStmt> {
    let pieces = split_ctoks_top_level(toks, Punct::Comma);
    let mut pieces = pieces.into_iter();
    let first = pieces.next()?;
    let (first_decl, first_init) = split_ctoks_top_level_first(&first, Punct::Assign);
    let first_text = render_ctoks_no_comments(&first_decl);
    let (storage, base_ty, ty, name) = parse_declarator_with_base(first_text.trim())?;
    let mut declarators = vec![Declarator {
        ty,
        name,
        initializer: first_init.map(|toks| parse_local_init(&toks, known)),
    }];

    for piece in pieces {
        let (decl_toks, init_toks) = split_ctoks_top_level_first(&piece, Punct::Assign);
        let text = render_ctoks_no_comments(&decl_toks);
        let (ty, name) = parse_bare_declarator_suffix(text.trim(), &base_ty)?;
        declarators.push(Declarator {
            ty,
            name,
            initializer: init_toks.map(|toks| parse_local_init(&toks, known)),
        });
    }

    Some(DeclStmt {
        storage,
        base_ty,
        declarators,
    })
}

/// A declarator's initializer is a real expression, unless it's exactly one
/// braced group (`= { ... }`) - then it recurses per top-level-comma
/// element, mirroring `parser::decl::parse_braced_init`'s shape one layer
/// up (with real `Expr` elements instead of raw text).
fn parse_local_init(toks: &[CTok], known: &KnownTypeNames) -> LocalInit {
    if let [CTok::Group { inner, .. }] = trim_trivia(toks) {
        let chunks = super::super::brace::group_braces(inner.clone());
        let ctoks = super::lex::tokenize_chunks(chunks);
        let pieces = split_ctoks_top_level(&ctoks, Punct::Comma);
        return LocalInit::Braced(
            pieces
                .iter()
                .map(|p| trim_trivia(p))
                .filter(|p| !p.is_empty())
                .map(|p| parse_local_init(p, known))
                .collect(),
        );
    }
    LocalInit::Expr(parse_expr(toks, known))
}

pub(super) fn trim_trivia(toks: &[CTok]) -> &[CTok] {
    let start = toks
        .iter()
        .position(|t| !t.is_trivial())
        .unwrap_or(toks.len());
    let end = toks
        .iter()
        .rposition(|t| !t.is_trivial())
        .map_or(start, |i| i + 1);
    &toks[start..end]
}

/// True if `toks` looks like it opens a local declaration: a storage
/// keyword, a `struct`/`union`/`enum` tag keyword, a C base-type keyword, or
/// a corpus typedef/tag name recognized via `known`. Used both by
/// `stmt::parse`'s top-level statement dispatch and by a `for`-loop's own
/// init clause, to decide whether to attempt `try_parse_decl_stmt` at all
/// (an expression-statement should never be misparsed as looking for a
/// declarator just because `try_parse_decl_stmt` happens to accept
/// something it shouldn't).
pub(super) fn looks_like_decl_start(toks: &[CTok], known: &KnownTypeNames) -> bool {
    match trim_trivia(toks).first() {
        Some(CTok::Ident(span)) => {
            let w = span.text.as_str();
            Storage::from_keyword(w).is_some()
                || w == "struct"
                || w == "union"
                || w == "enum"
                || BASE_TYPE_WORDS.contains(&w)
                || known.contains(w)
        }
        _ => false,
    }
}

/// Splits `toks` on top-level occurrences of `sep` - `(`/`[` increase
/// depth, `)`/`]` decrease it; a `CTok::Group` is already one complete
/// balanced unit from `group_braces` and doesn't affect depth itself.
/// Mirrors `record::split_tokens_top_level`, one layer up (`CTok` instead
/// of `RawToken`). Always includes a final trailing piece (possibly empty),
/// unlike `record::split_tokens_top_level`'s separate `leftover` - simpler
/// here since every caller wants that last piece too, just filtering it out
/// when empty.
pub(super) fn split_ctoks_top_level(toks: &[CTok], sep: Punct) -> Vec<Vec<CTok>> {
    let mut pieces = Vec::new();
    let mut cur = Vec::new();
    let mut depth: i32 = 0;
    for tok in toks {
        match tok {
            CTok::Punct(Punct::LParen, _) | CTok::Punct(Punct::LBracket, _) => {
                depth += 1;
                cur.push(tok.clone());
            }
            CTok::Punct(Punct::RParen, _) | CTok::Punct(Punct::RBracket, _) => {
                depth -= 1;
                cur.push(tok.clone());
            }
            CTok::Punct(p, _) if *p == sep && depth <= 0 => {
                pieces.push(std::mem::take(&mut cur));
            }
            _ => cur.push(tok.clone()),
        }
    }
    pieces.push(cur);
    pieces
}

/// Splits `toks` at the *first* top-level occurrence of `sep`, returning
/// `(before, Some(after))`, or `(toks, None)` if `sep` never appears at top
/// level. Used for a declarator's own `= initializer` split - since the
/// `CTok` lexer already tokenizes `==`/`+=`/etc. as their own distinct
/// `Punct` variants (never as `Assign` plus something else), matching
/// `Punct::Assign` here can never misfire on a compound-assignment operator
/// the way the string-based `decl::split_top_level_eq` has to guard against.
fn split_ctoks_top_level_first(toks: &[CTok], sep: Punct) -> (Vec<CTok>, Option<Vec<CTok>>) {
    let mut depth: i32 = 0;
    for (i, tok) in toks.iter().enumerate() {
        match tok {
            CTok::Punct(Punct::LParen, _) | CTok::Punct(Punct::LBracket, _) => depth += 1,
            CTok::Punct(Punct::RParen, _) | CTok::Punct(Punct::RBracket, _) => depth -= 1,
            CTok::Punct(p, _) if *p == sep && depth <= 0 => {
                return (toks[..i].to_vec(), Some(toks[i + 1..].to_vec()));
            }
            _ => {}
        }
    }
    (toks.to_vec(), None)
}

#[cfg(test)]
mod tests;
