//! Step 5: struct/union/enum/typedef/function parsing, plus the top-level
//! driver that assembles a whole file's `Item` list.
//!
//! The driver works in two passes over step 1's brace `Chunk`s:
//! 1. Split into "declaration units" - a run of chunks ending at the first
//!    top-level `;` (so a struct's `[header][Group][trailing ;]` stays
//!    together as one unit instead of being cut at the header or the
//!    group).
//! 2. Lower each unit into an `Item`. A unit containing a `Group` is
//!    classified as a struct/union/enum (by a `struct`/`union`/`enum`
//!    keyword in its header), a function (header ends in `NAME(...)`, body
//!    kept fully opaque), or - falling back - a braced constant
//!    initializer. A unit without a `Group` is tried as a constant
//!    declaration and otherwise kept as `Item::Raw`.
//!
//! `Item.raw` is always the exact original text for the unit (see ast.rs),
//! so a wrong or incomplete classification never loses bytes - it just
//! produces a less useful `ItemKind`.

use super::ast::{
    Chunk, Comment, EnumDecl, Field, FnSig, Item, ItemKind, RawToken, RecordDecl, RecordKind,
    Trivia, render_tokens,
};
use super::decl::{
    parse_declarator, try_parse_const_braced, try_parse_const_flat, try_parse_typedef_flat,
};
use super::preproc::parse_directive;

pub fn build_items(tokens: Vec<RawToken>) -> Vec<(Item, Trivia)> {
    let chunks = super::brace::group_braces(tokens);
    let mut items = Vec::new();
    let mut unit: Vec<Chunk> = Vec::new();

    for chunk in chunks {
        match chunk {
            Chunk::Group { .. } => unit.push(chunk),
            Chunk::Flat(toks) => {
                let (pieces, leftover) = split_into_pieces(toks);
                for piece in pieces {
                    unit.push(Chunk::Flat(piece));
                    let finished = std::mem::take(&mut unit);
                    items.push(lower_unit(finished));
                }
                if !leftover.is_empty() {
                    unit.push(Chunk::Flat(leftover));
                }
            }
        }
    }
    if !unit.is_empty() {
        items.push(lower_unit(unit));
    }
    items
}

fn is_comment(t: &RawToken) -> bool {
    matches!(t, RawToken::LineComment(_) | RawToken::BlockComment(_))
}

/// True for tokens that don't represent "real" pending declaration content:
/// comments, and whitespace-only code runs (blank lines between items).
fn is_trivial(t: &RawToken) -> bool {
    is_comment(t) || matches!(t, RawToken::Code(s) if s.text.trim().is_empty())
}

/// Splits a `Flat` chunk's tokens into complete pieces (each ending at a
/// top-level `;`, or being a lone preprocessor directive preceded only by
/// comments/blank lines) plus a trailing leftover with no terminator yet -
/// the pending header for whatever comes next (typically a `Group`).
fn split_into_pieces(toks: Vec<RawToken>) -> (Vec<Vec<RawToken>>, Vec<RawToken>) {
    let mut pieces = Vec::new();
    let mut cur: Vec<RawToken> = Vec::new();

    for tok in toks {
        match &tok {
            RawToken::PreprocLine(_) if cur.iter().all(is_trivial) => {
                cur.push(tok);
                pieces.push(std::mem::take(&mut cur));
            }
            RawToken::Code(span) => {
                let mut rest = span.text.as_str();
                let mut pos = span.start;
                loop {
                    if let Some(i) = rest.find(';') {
                        let piece_text = &rest[..=i];
                        let piece_start = pos;
                        pos = pos.advance(piece_text);
                        cur.push(RawToken::Code(super::ast::Span {
                            start: piece_start,
                            end: pos,
                            text: piece_text.to_string(),
                        }));
                        pieces.push(std::mem::take(&mut cur));
                        rest = &rest[i + 1..];
                    } else {
                        if !rest.is_empty() {
                            let piece_start = pos;
                            pos = pos.advance(rest);
                            cur.push(RawToken::Code(super::ast::Span {
                                start: piece_start,
                                end: pos,
                                text: rest.to_string(),
                            }));
                        }
                        break;
                    }
                }
            }
            _ => cur.push(tok),
        }
    }
    (pieces, cur)
}

fn drain_leading_comments(unit: &mut [Chunk]) -> Vec<Comment> {
    let mut leading = Vec::new();
    if let Some(Chunk::Flat(toks)) = unit.first_mut() {
        let mut i = 0;
        while i < toks.len() {
            match &toks[i] {
                RawToken::LineComment(s) => {
                    leading.push(Comment::Line(s.text.clone()));
                    i += 1;
                }
                RawToken::BlockComment(s) => {
                    leading.push(Comment::Block(s.text.clone()));
                    i += 1;
                }
                _ => break,
            }
        }
        toks.drain(0..i);
    }
    leading
}

fn lower_unit(mut unit: Vec<Chunk>) -> (Item, Trivia) {
    let leading = drain_leading_comments(&mut unit);
    let raw: String = unit.iter().map(Chunk::render).collect();
    let trivia = Trivia { leading };

    if let [Chunk::Flat(toks)] = unit.as_slice() {
        let mut directives = toks
            .iter()
            .filter(|t| matches!(t, RawToken::PreprocLine(_)));
        let only_directive = directives.next();
        if directives.next().is_none()
            && toks
                .iter()
                .all(|t| matches!(t, RawToken::PreprocLine(_)) || is_trivial(t))
        {
            if let Some(RawToken::PreprocLine(s)) = only_directive {
                let d = parse_directive(&s.text);
                return (
                    Item {
                        kind: ItemKind::Preproc(d),
                        raw,
                    },
                    trivia,
                );
            }
        }
    }

    if unit.iter().any(|c| matches!(c, Chunk::Group { .. })) {
        let kind = classify_group_unit(&unit, &raw);
        return (Item { kind, raw }, trivia);
    }

    let kind = if let Some(td) = try_parse_typedef_flat(&raw) {
        ItemKind::Typedef(td)
    } else if let Some(cd) = try_parse_const_flat(&raw) {
        ItemKind::Const(cd)
    } else if let Some(sig) = raw
        .trim()
        .strip_suffix(';')
        .and_then(|h| try_parse_fn_sig(h.trim()))
    {
        ItemKind::FunctionDecl(sig)
    } else {
        ItemKind::Raw
    };
    (Item { kind, raw }, trivia)
}

/// Renders `chunks` for classification purposes only: comments are dropped
/// so a doc comment that happens to contain a word like `struct` can never
/// be mistaken for part of the declarator. `Item.raw` (the round-trip
/// source of truth) is built separately via `Chunk::render` and is
/// unaffected by this - this only feeds keyword/signature detection.
fn declarator_text(chunks: &[Chunk]) -> String {
    chunks
        .iter()
        .map(|c| match c {
            Chunk::Flat(toks) => toks
                .iter()
                .filter(|t| !matches!(t, RawToken::LineComment(_) | RawToken::BlockComment(_)))
                .map(RawToken::text)
                .collect::<String>(),
            other => other.render(),
        })
        .collect()
}

fn classify_group_unit(unit: &[Chunk], raw: &str) -> ItemKind {
    let gi = match unit.iter().position(|c| matches!(c, Chunk::Group { .. })) {
        Some(i) => i,
        None => return ItemKind::Raw,
    };
    let header: String = declarator_text(&unit[..gi]);
    let (inner, open_text, close_text) = match &unit[gi] {
        Chunk::Group { inner, open, close } => {
            (inner.clone(), open.text.clone(), close.text.clone())
        }
        _ => unreachable!(),
    };
    let trailing: String = unit[gi + 1..].iter().map(Chunk::render).collect();
    let header_trim = header.trim();
    let trailing_trim = trailing.trim().trim_end_matches(';').trim();

    if let Some((kw, kw_pos)) = find_record_keyword(header_trim) {
        return classify_record_or_enum(header_trim, kw, kw_pos, inner, trailing_trim);
    }

    if let Some(sig) = try_parse_fn_sig(header_trim) {
        let body_raw = format!("{open_text}{}{close_text}", render_tokens(&inner));
        return ItemKind::FunctionDef(sig, body_raw);
    }

    if header_trim.ends_with('=') {
        let group_raw = format!("{open_text}{}{close_text}", render_tokens(&inner));
        if let Some(cd) = try_parse_const_braced(header_trim, &group_raw) {
            return ItemKind::Const(cd);
        }
    }

    let _ = raw;
    ItemKind::Raw
}

const RECORD_KEYWORDS: [&str; 3] = ["struct", "union", "enum"];

fn find_record_keyword(header: &str) -> Option<(&'static str, usize)> {
    for kw in RECORD_KEYWORDS {
        if let Some(idx) = find_word(header, kw) {
            return Some((kw, idx));
        }
    }
    None
}

fn find_word(haystack: &str, word: &str) -> Option<usize> {
    let bytes = haystack.as_bytes();
    let mut i = 0;
    while let Some(pos) = haystack.get(i..).and_then(|h| h.find(word)) {
        let start = i + pos;
        let end = start + word.len();
        let before_ok = start == 0 || !is_ident_byte(bytes[start - 1]);
        let after_ok = end >= bytes.len() || !is_ident_byte(bytes[end]);
        if before_ok && after_ok {
            return Some(start);
        }
        i = start + 1;
    }
    None
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn split_names(trailing: &str) -> Vec<String> {
    trailing
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn classify_record_or_enum(
    header: &str,
    kw: &str,
    kw_pos: usize,
    inner: Vec<RawToken>,
    trailing: &str,
) -> ItemKind {
    let before_kw = header[..kw_pos].trim();
    let is_typedef = find_word(before_kw, "typedef").is_some();
    let after_kw = header[kw_pos + kw.len()..].trim();
    let tag = if after_kw.is_empty() {
        None
    } else {
        Some(after_kw.to_string())
    };
    let names = split_names(trailing);
    let typedef_name = if is_typedef {
        names.first().cloned()
    } else {
        None
    };

    if kw == "enum" {
        let variants = parse_enum_variants(&inner);
        ItemKind::Enum(EnumDecl {
            tag,
            variants,
            names,
            typedef_name,
        })
    } else {
        let kind = if kw == "union" {
            RecordKind::Union
        } else {
            RecordKind::Struct
        };
        let fields = parse_fields(&inner);
        ItemKind::Record(RecordDecl {
            kind,
            tag,
            fields,
            names,
            typedef_name,
        })
    }
}

/// Splits `inner` tokens on top-level `,` (re-grouping any nested braces
/// first, since an enum value can itself be a parenthesized/braced
/// expression referencing an earlier constant, e.g. `INVULNTICS = (30*TICRATE)`).
fn parse_enum_variants(inner: &[RawToken]) -> Vec<(String, Option<String>)> {
    let text = render_tokens(inner);
    let mut variants = Vec::new();
    for part in split_top_level(&text, ',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some((name, val)) = part.split_once('=') {
            variants.push((name.trim().to_string(), Some(val.trim().to_string())));
        } else {
            variants.push((part.to_string(), None));
        }
    }
    variants
}

/// Splits `inner` tokens (a struct/union body) on top-level `;` into
/// fields. A nested anonymous struct/union (a literal `{` inside the body)
/// is not descended into further in v1 - it's kept as one field with its
/// raw text as the type.
fn parse_fields(inner: &[RawToken]) -> Vec<Field> {
    let chunks = super::brace::group_braces(inner.to_vec());
    let mut fields = Vec::new();

    for chunk in chunks {
        match chunk {
            Chunk::Flat(toks) => {
                let text = render_tokens(&toks);
                for part in split_top_level(&text, ';') {
                    let part = part.trim();
                    if part.is_empty() {
                        continue;
                    }
                    if let Some(f) = parse_field(part) {
                        fields.push(f);
                    }
                }
            }
            Chunk::Group { open, inner, close } => {
                // Nested anonymous struct/union: not descended into in v1,
                // kept as one field with its raw text as the "type".
                let raw = format!("{}{}{}", open.text, render_tokens(&inner), close.text);
                fields.push(Field {
                    ty: raw,
                    name: String::new(),
                    array_dims: Vec::new(),
                    bitfield: None,
                });
            }
        }
    }
    fields
}

fn parse_field(decl_text: &str) -> Option<Field> {
    let (decl_text, bitfield) = match decl_text.rsplit_once(':') {
        Some((d, b)) if !b.trim().is_empty() && b.trim().chars().all(|c| c.is_ascii_digit()) => {
            (d.trim(), Some(b.trim().to_string()))
        }
        _ => (decl_text, None),
    };
    let (storage, ty, name, array_dims) = parse_declarator(decl_text)?;
    let mut ty = ty;
    if !storage.is_empty() {
        ty = format!("{} {}", storage.join(" "), ty);
    }
    Some(Field {
        ty,
        name,
        array_dims,
        bitfield,
    })
}

/// Splits `s` on top-level occurrences of `sep`, treating `(...)`, `[...]`
/// and `{...}` as opaque (never splitting inside them). Used for enum
/// variant lists and struct field lists once they're back down to plain
/// text.
fn split_top_level(s: &str, sep: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                depth += 1;
                cur.push(c);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                cur.push(c);
            }
            c if c == sep && depth <= 0 => {
                out.push(std::mem::take(&mut cur));
            }
            c => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur);
    }
    out
}

/// Recognizes a function signature: header text ending in `NAME ( params )`,
/// possibly spread across multiple lines (this codebase's usual style, see
/// `M_DrawText` in m_misc.c). `params` is kept as raw text.
fn try_parse_fn_sig(header: &str) -> Option<FnSig> {
    let h = header.trim();
    if !h.ends_with(')') {
        return None;
    }
    let open_paren = matching_open_paren(h)?;
    let params_raw = h[open_paren + 1..h.len() - 1].trim().to_string();
    let before_paren = h[..open_paren].trim();
    let name_start = before_paren
        .rfind(|c: char| !(c.is_alphanumeric() || c == '_'))
        .map(|i| i + 1)
        .unwrap_or(0);
    let name = before_paren[name_start..].trim().to_string();
    if name.is_empty() || !name.chars().next().unwrap().is_alphabetic() && !name.starts_with('_') {
        return None;
    }
    let ret_raw = before_paren[..name_start].trim();
    let tokens: Vec<&str> = ret_raw.split_whitespace().collect();
    const STORAGE_KW: &[&str] = &["static", "extern", "inline"];
    let mut storage = Vec::new();
    let mut ty_parts = Vec::new();
    for t in tokens {
        if STORAGE_KW.contains(&t) && ty_parts.is_empty() {
            storage.push(t.to_string());
        } else {
            ty_parts.push(t);
        }
    }
    let ret_ty = ty_parts.join(" ");
    Some(FnSig {
        storage,
        ret_ty,
        name,
        params_raw,
    })
}

fn matching_open_paren(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    if bytes.last() != Some(&b')') {
        return None;
    }
    let mut depth = 0i32;
    for (i, &b) in bytes.iter().enumerate().rev() {
        match b {
            b')' => depth += 1,
            b'(' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests;
