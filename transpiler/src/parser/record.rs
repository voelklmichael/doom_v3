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
//!    kept fully opaque), or - falling back - a braced variable
//!    initializer. A unit without a `Group` is tried as a variable
//!    declaration (with or without an initializer) and otherwise kept as
//!    `Item::Raw`.
//!
//! `Item.raw` is always the exact original text for the unit (see ast.rs),
//! so a wrong or incomplete classification never loses bytes - it just
//! produces a less useful `ItemKind`.

use super::ast::{
    Chunk, Comment, EnumDecl, EnumVariant, Field, FnSig, Item, ItemKind, Param, RawToken,
    RecordDecl, RecordKind, Span, Storage, Trivia, Type, render_tokens, render_tokens_no_comments,
    split_top_level,
};
use super::decl::{
    parse_declarator, parse_type_text, try_parse_typedef_flat, try_parse_var_braced,
    try_parse_var_flat, wrap_array_dims,
};
use super::preproc::parse_directive;
use super::stmt::expr::KnownTypeNames;

/// Parses a whole file's tokens with no corpus type-name context - function
/// bodies still get fully parsed (see `ItemKind::FunctionDef`), just with
/// less precise cast-vs-parenthesized-expression disambiguation (only C
/// base keywords and `struct`/`union`/`enum` tags are recognized as types,
/// no corpus typedefs). Used by callers that don't need body-parsing
/// accuracy (this module's own tests, `parser::corpus`'s own graph-building
/// pass, which only looks at top-level `Typedef`/`Record`/`Enum` items and
/// never at `FunctionDef` bodies at all). Real per-file output should use
/// `build_items_with_types` with a `KnownTypeNames` from
/// `parser::corpus::compute_known_type_names` instead.
pub fn build_items(tokens: Vec<RawToken>) -> Vec<(Item, Trivia)> {
    build_items_with_types(tokens, &KnownTypeNames::new())
}

pub fn build_items_with_types(
    tokens: Vec<RawToken>,
    known: &KnownTypeNames,
) -> Vec<(Item, Trivia)> {
    let chunks = super::brace::group_braces(tokens);
    let mut items = Vec::new();
    let mut unit: Vec<Chunk> = Vec::new();

    for chunk in chunks {
        match chunk {
            Chunk::Group { .. } => {
                // A function body never has trailing content (no `;` ever
                // follows a function's closing `}`), unlike a struct/enum
                // body or a braced const initializer, which need to keep
                // `unit` open to pick up their trailing `;`/names. Detecting
                // that here (by re-using the same header check
                // `classify_group_unit` will apply later) and flushing
                // immediately stops whatever follows - typically a comment
                // and the next declaration - from being silently absorbed
                // into this function's `raw`.
                let header: String = unit.iter().map(Chunk::render).collect();
                unit.push(chunk);
                if try_parse_fn_sig(header.trim()).is_some() {
                    let finished = std::mem::take(&mut unit);
                    items.push(lower_unit(finished, known));
                }
            }
            Chunk::Flat(toks) => {
                let (pieces, leftover) = split_into_pieces(toks);
                for piece in pieces {
                    unit.push(Chunk::Flat(piece));
                    let finished = std::mem::take(&mut unit);
                    items.push(lower_unit(finished, known));
                }
                if !leftover.is_empty() {
                    unit.push(Chunk::Flat(leftover));
                }
            }
        }
    }
    if !unit.is_empty() {
        items.push(lower_unit(unit, known));
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
    reattach_trailing_comments(&mut pieces, &mut cur);
    (pieces, cur)
}

/// A comment on the same line as the `;` that just closed a piece describes
/// that piece, not whatever declaration follows - e.g. in `int x; // meaning
/// \nint y;`, "// meaning" is about `x`, not `y`. But each piece above is
/// found purely by scanning forward for the next `;`, so such a comment
/// always ends up at the *start* of the following piece (or `leftover`)
/// instead of the end of the one it belongs to. This moves it back: for
/// each piece (and `leftover`) that begins with an optional same-line
/// (no-newline) whitespace run followed immediately by a comment, that
/// prefix is spliced onto the end of the previous piece. A comment preceded
/// by a newline is left alone - it's a leading comment for what follows,
/// the case `drain_leading_comments` already handles.
fn reattach_trailing_comments(pieces: &mut [Vec<RawToken>], leftover: &mut Vec<RawToken>) {
    for i in 0..pieces.len() {
        if let Some(taken) = take_leading_same_line_comment(&mut pieces[i]) {
            if i == 0 {
                // No earlier piece *in this call* to attach to (e.g. right
                // after a function body's `}`, flushed by the caller before
                // this chunk was even scanned) - leave the bytes where they
                // were; still correct, just not reattached across that
                // boundary.
                pieces[i].splice(0..0, taken);
            } else {
                pieces[i - 1].extend(taken);
            }
        }
    }
    if let Some(taken) = take_leading_same_line_comment(leftover) {
        match pieces.last_mut() {
            Some(last) => last.extend(taken),
            None => {
                leftover.splice(0..0, taken);
            }
        }
    }
}

/// If `toks` starts with an optional whitespace-only `Code` token containing
/// no newline, immediately followed by a `LineComment`/`BlockComment`,
/// drains and returns that prefix (comment included). Returns `None`
/// (leaving `toks` untouched) otherwise.
fn take_leading_same_line_comment(toks: &mut Vec<RawToken>) -> Option<Vec<RawToken>> {
    let mut end = 0;
    if let Some(RawToken::Code(span)) = toks.first() {
        if span.text.contains('\n') {
            return None;
        }
        end = 1;
    }
    match toks.get(end) {
        Some(RawToken::LineComment(_)) | Some(RawToken::BlockComment(_)) => {
            Some(toks.drain(0..=end).collect())
        }
        _ => None,
    }
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

fn lower_unit(mut unit: Vec<Chunk>, known: &KnownTypeNames) -> (Item, Trivia) {
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
            && let Some(RawToken::PreprocLine(s)) = only_directive
        {
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

    if unit.iter().any(|c| matches!(c, Chunk::Group { .. })) {
        let kind = classify_group_unit(&unit, &raw, known);
        return (Item { kind, raw }, trivia);
    }

    // Classification (unlike `raw` itself) must ignore any leading comment
    // `drain_leading_comments` left behind - a blank line between the item
    // and its doc comment stops that drain, same gap `declarator_text`
    // already works around for the Group-unit path below. Without this, a
    // commented flat declaration (e.g. d_think.h's `actionf_v`, preceded by
    // a blank-line-then-doc-comment) never even reaches the `typedef`
    // prefix check and falls to `Item::Raw`.
    let text = declarator_text(&unit);
    let kind = if let Some(td) = try_parse_typedef_flat(&text) {
        ItemKind::Typedef(td)
    } else if let Some(vd) = try_parse_var_flat(&text) {
        ItemKind::Var(vd)
    } else if let Some(sig) = text
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
            Chunk::Flat(toks) => render_tokens_no_comments(toks),
            other => other.render(),
        })
        .collect()
}

fn classify_group_unit(unit: &[Chunk], raw: &str, known: &KnownTypeNames) -> ItemKind {
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
        let body = super::stmt::parse_function_body(inner, body_raw, known);
        return ItemKind::FunctionDef(sig, body);
    }

    if header_trim.ends_with('=')
        && let Some(vd) = try_parse_var_braced(header_trim, &inner)
    {
        return ItemKind::Var(vd);
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

/// Splits `inner` tokens on top-level `,` (respecting `(`/`[`/`{` nesting
/// depth, since an enum value can itself be a parenthesized/braced
/// expression referencing an earlier constant, e.g.
/// `INVULNTICS = (30*TICRATE)`), reattaching same-line trailing comments to
/// the variant they describe (same convention as top-level items - see
/// `reattach_trailing_comments`) and any other leading comment run to the
/// variant that follows it.
fn parse_enum_variants(inner: &[RawToken]) -> Vec<EnumVariant> {
    let (mut pieces, mut leftover) = split_tokens_top_level(inner, ',');
    reattach_trailing_comments(&mut pieces, &mut leftover);
    // Unlike struct fields (always `;`-terminated), an enum's last variant
    // usually has no trailing comma, so whatever remains after the final
    // split point is itself a real variant, not just discardable trailing
    // whitespace - fold it in as one more piece.
    if !leftover.is_empty() {
        pieces.push(leftover);
    }

    let mut variants = Vec::new();
    for piece in pieces {
        let (leading, trailing_comment, text) = extract_piece_trivia(piece);
        let text = text.trim();
        if text.is_empty() {
            continue;
        }
        let (name, value) = match text.split_once('=') {
            Some((n, v)) => (n.trim().to_string(), Some(v.trim().to_string())),
            None => (text.to_string(), None),
        };
        variants.push(EnumVariant {
            name,
            value,
            trivia: Trivia { leading },
            trailing_comment,
        });
    }
    variants
}

/// Splits `inner` tokens (a struct/union body) on top-level `;` into
/// fields, reattaching same-line trailing comments (e.g. `int health; //
/// hit points`) and leading doc-comment runs to the field they describe.
///
/// A nested anonymous struct/union (a literal `{` inside the body, e.g.
/// `p_local.h`'s `intercept_t.d`) is recursively parsed rather than kept
/// opaque: `pending_header` accumulates tokens across chunk boundaries the
/// same way `build_items`'s own `unit` accumulator does at the top level,
/// so when a `Group` is reached whose preceding (comment-stripped) text
/// ends in `struct`/`union`, its declarator can still be recovered from
/// whatever `Flat` chunk follows the closing `}` (the field's name, and any
/// array dims). A `Group` not preceded by that keyword (or a nested
/// *enum*, not supported here) falls back to the old opaque-field
/// behavior, unchanged.
fn parse_fields(inner: &[RawToken]) -> Vec<Field> {
    let chunks = super::brace::group_braces(inner.to_vec());
    let mut fields = Vec::new();
    let mut pending_header: Vec<RawToken> = Vec::new();
    let mut pending_nested: Option<PendingNested> = None;

    for chunk in chunks {
        match chunk {
            Chunk::Group {
                open,
                inner: ginner,
                close,
            } => {
                let header_text = render_tokens_no_comments(&pending_header);
                if let Some((kw, kw_pos)) = find_record_keyword(header_text.trim())
                    && kw != "enum"
                {
                    let header_trim = header_text.trim();
                    let after_kw = header_trim[kw_pos + kw.len()..].trim();
                    let tag = if after_kw.is_empty() {
                        None
                    } else {
                        Some(after_kw.to_string())
                    };
                    let kind = if kw == "union" {
                        RecordKind::Union
                    } else {
                        RecordKind::Struct
                    };
                    let (leading, _, _) = extract_piece_trivia(std::mem::take(&mut pending_header));
                    pending_nested = Some(PendingNested {
                        kind,
                        tag,
                        open_text: open.text,
                        inner: ginner,
                        close_text: close.text,
                        leading,
                    });
                    continue;
                }
                // Not a recognized nested-record header: preserve the old
                // per-chunk-independent behavior exactly - whatever's
                // pending is tried as its own (usually unparseable, silently
                // dropped) field, and this group becomes its own opaque
                // field.
                let (leading, trailing_comment, text) =
                    extract_piece_trivia(std::mem::take(&mut pending_header));
                push_plain_field(&text, leading, trailing_comment, &mut fields);
                push_opaque_group_field(&open.text, &ginner, &close.text, &mut fields);
            }
            Chunk::Flat(toks) => {
                let mut combined = std::mem::take(&mut pending_header);
                combined.extend(toks);
                let (mut pieces, mut leftover) = split_tokens_top_level(&combined, ';');
                reattach_trailing_comments(&mut pieces, &mut leftover);

                if let Some(pending) = pending_nested.take() {
                    // This chunk must supply the nested record's trailing
                    // declarator ("NAME;", "NAME[dims];", ...) - only the
                    // first piece belongs to it; anything after starts a
                    // fresh field, same as any other boundary. `pieces`
                    // being empty here (no `;` at all yet) can't actually
                    // happen for a valid struct body - `group_braces`
                    // always alternates Flat/Group, so this Flat chunk is
                    // the *only* place the trailer can come from - but
                    // degrade to the opaque fallback rather than dropping
                    // the group if it somehow does.
                    if pieces.is_empty() {
                        push_opaque_group_field(
                            &pending.open_text,
                            &pending.inner,
                            &pending.close_text,
                            &mut fields,
                        );
                        pending_header = leftover;
                        continue;
                    }
                    let trailer = pieces.remove(0);
                    let (_, trailing_comment, trailer_text) = extract_piece_trivia(trailer);
                    push_nested_record_field(pending, &trailer_text, trailing_comment, &mut fields);
                }
                for piece in pieces {
                    let (leading, trailing_comment, text) = extract_piece_trivia(piece);
                    push_plain_field(&text, leading, trailing_comment, &mut fields);
                }
                pending_header = leftover;
            }
        }
    }
    match pending_nested {
        // EOF with a nested record never followed by a trailer at all
        // (malformed/truncated input) - degrade to the opaque fallback
        // rather than dropping it silently.
        Some(pending) => {
            push_opaque_group_field(
                &pending.open_text,
                &pending.inner,
                &pending.close_text,
                &mut fields,
            );
        }
        None => {
            let (leading, trailing_comment, text) = extract_piece_trivia(pending_header);
            push_plain_field(&text, leading, trailing_comment, &mut fields);
        }
    }
    fields
}

fn push_plain_field(
    text: &str,
    leading: Vec<Comment>,
    trailing_comment: Option<Comment>,
    fields: &mut Vec<Field>,
) {
    let text = text.trim();
    if text.is_empty() {
        return;
    }
    if let Some(mut f) = parse_field(text) {
        f.trivia = Trivia { leading };
        f.trailing_comment = trailing_comment;
        fields.push(f);
    }
}

fn push_opaque_group_field(
    open_text: &str,
    inner: &[RawToken],
    close_text: &str,
    fields: &mut Vec<Field>,
) {
    let raw = format!("{open_text}{}{close_text}", render_tokens(inner));
    fields.push(Field {
        ty: Type::Named(raw),
        name: String::new(),
        storage: Vec::new(),
        bitfield: None,
        nested: None,
        trivia: Trivia::default(),
        trailing_comment: None,
    });
}

fn record_kw(kind: RecordKind) -> &'static str {
    match kind {
        RecordKind::Struct => "struct",
        RecordKind::Union => "union",
    }
}

/// A recognized-but-not-yet-complete nested anonymous struct/union: the
/// `struct`/`union` keyword and optional tag already seen before its `{`,
/// plus the group's own open/inner/close and any leading doc comment, all
/// held until the declarator that names it (`NAME;` after the closing `}`)
/// arrives in a later chunk.
struct PendingNested {
    kind: RecordKind,
    tag: Option<String>,
    open_text: String,
    inner: Vec<RawToken>,
    close_text: String,
    leading: Vec<Comment>,
}

fn push_nested_record_field(
    pending: PendingNested,
    trailer_text: &str,
    trailing_comment: Option<Comment>,
    fields: &mut Vec<Field>,
) {
    let PendingNested {
        kind,
        tag,
        open_text,
        inner: ginner,
        close_text,
        leading,
    } = pending;
    let Some((name, array_dims)) = parse_nested_trailer(trailer_text) else {
        push_opaque_group_field(&open_text, &ginner, &close_text, fields);
        return;
    };
    let nested_fields = parse_fields(&ginner);
    let ty_text = match &tag {
        Some(t) => format!("{} {t}", record_kw(kind)),
        None => record_kw(kind).to_string(),
    };
    let ty = wrap_array_dims(Type::Named(ty_text), &array_dims);
    fields.push(Field {
        ty,
        name,
        storage: Vec::new(),
        bitfield: None,
        nested: Some(Box::new(RecordDecl {
            kind,
            tag,
            fields: nested_fields,
            names: Vec::new(),
            typedef_name: None,
        })),
        trivia: Trivia { leading },
        trailing_comment,
    });
}

/// Parses the declarator following a nested anonymous struct/union's closing
/// `}` (e.g. the `d` in `union { ... } d;`) - just a bare identifier plus
/// optional array dims. A bitfield suffix is never valid here (bitfields
/// apply to integer members, not struct/union-typed ones), so unlike
/// `parse_field` this doesn't look for one.
fn parse_nested_trailer(text: &str) -> Option<(String, Vec<Option<String>>)> {
    let mut base = text.trim();
    let mut dims: Vec<Option<String>> = Vec::new();
    while base.ends_with(']') {
        let open = base.rfind('[')?;
        let dim = base[open + 1..base.len() - 1].trim();
        dims.push(if dim.is_empty() {
            None
        } else {
            Some(dim.to_string())
        });
        base = base[..open].trim_end();
    }
    dims.reverse();
    let name = base.trim();
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let first_char = name.chars().next()?;
    if !(first_char.is_alphabetic() || first_char == '_') {
        return None;
    }
    Some((name.to_string(), dims))
}

fn parse_field(decl_text: &str) -> Option<Field> {
    let (decl_text, bitfield) = match decl_text.rsplit_once(':') {
        Some((d, b)) if !b.trim().is_empty() && b.trim().chars().all(|c| c.is_ascii_digit()) => {
            (d.trim(), Some(b.trim().to_string()))
        }
        _ => (decl_text, None),
    };
    let (storage, ty, name) = parse_declarator(decl_text)?;
    Some(Field {
        ty,
        name,
        storage,
        bitfield,
        nested: None,
        trivia: Trivia::default(),
        trailing_comment: None,
    })
}

/// Splits `tokens` on top-level occurrences of `sep` (a `Code`-token
/// character at bracket depth 0 - `(`/`[`/`{` increase depth, `)`/`]`/`}`
/// decrease it). Comments are *kept* in place (as their own tokens,
/// attached to whichever piece they fall in) rather than dropped, so
/// callers can recover per-piece trivia instead of just per-piece
/// declarator text. Returns the completed pieces plus a final `leftover`
/// run with no terminating `sep` (mirrors `split_into_pieces`'s `(pieces,
/// leftover)` shape).
fn split_tokens_top_level(tokens: &[RawToken], sep: char) -> (Vec<Vec<RawToken>>, Vec<RawToken>) {
    let mut pieces: Vec<Vec<RawToken>> = Vec::new();
    let mut cur: Vec<RawToken> = Vec::new();
    let mut depth: i32 = 0;

    for tok in tokens {
        match tok {
            RawToken::Code(span) => {
                let text = span.text.as_str();
                let mut byte_start = 0usize;
                let mut pos = span.start;
                for (i, c) in text.char_indices() {
                    match c {
                        '(' | '[' | '{' => depth += 1,
                        ')' | ']' | '}' => depth -= 1,
                        _ => {}
                    }
                    if c == sep && depth <= 0 {
                        let piece_text = &text[byte_start..i];
                        if !piece_text.is_empty() {
                            let piece_start = pos;
                            pos = piece_start.advance(piece_text);
                            cur.push(RawToken::Code(Span {
                                start: piece_start,
                                end: pos,
                                text: piece_text.to_string(),
                            }));
                        }
                        pos = pos.advance(&text[i..i + c.len_utf8()]);
                        byte_start = i + c.len_utf8();
                        pieces.push(std::mem::take(&mut cur));
                    }
                }
                if byte_start < text.len() {
                    let piece_text = &text[byte_start..];
                    let piece_start = pos;
                    cur.push(RawToken::Code(Span {
                        start: piece_start,
                        end: piece_start.advance(piece_text),
                        text: piece_text.to_string(),
                    }));
                }
            }
            other => cur.push(other.clone()),
        }
    }
    (pieces, cur)
}

/// Splits a piece (as produced by `split_tokens_top_level`) into its leading
/// comment run, a trailing same-line comment (if `reattach_trailing_comments`
/// moved one onto the end of this piece), and the remaining declarator text
/// with all comments filtered out.
fn extract_piece_trivia(mut toks: Vec<RawToken>) -> (Vec<Comment>, Option<Comment>, String) {
    let mut leading = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        match &toks[i] {
            RawToken::Code(s) if s.text.trim().is_empty() => i += 1,
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

    let trailing = match toks.last() {
        Some(RawToken::LineComment(_)) | Some(RawToken::BlockComment(_)) => {
            match toks.pop().unwrap() {
                RawToken::LineComment(s) => Some(Comment::Line(s.text)),
                RawToken::BlockComment(s) => Some(Comment::Block(s.text)),
                _ => unreachable!(),
            }
        }
        _ => None,
    };

    (leading, trailing, render_tokens_no_comments(&toks))
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
    // Unlike a plain declarator, a function signature's storage set doesn't
    // include const/register/volatile - `const char *foo()`'s "const"
    // qualifies the *return type*, not the function itself, so it's left in
    // `ty_parts` (part of `ret_ty`) rather than pulled into `storage` here.
    let mut storage = Vec::new();
    let mut ty_parts = Vec::new();
    for t in tokens {
        match Storage::from_keyword(t) {
            Some(kw @ (Storage::Static | Storage::Extern | Storage::Inline))
                if ty_parts.is_empty() =>
            {
                storage.push(kw);
            }
            _ => ty_parts.push(t),
        }
    }
    let ret_ty = parse_type_text(&ty_parts.join(" "));
    let (params, variadic) = parse_params(&params_raw);
    Some(FnSig {
        storage,
        ret_ty,
        name,
        params,
        variadic,
    })
}

/// Splits a function signature's parameter-list text on top-level `,` into
/// individual parameters, e.g. `"int x, char *s"` -> two `Param`s. `()` and
/// `(void)` both collapse to an empty list (see `FnSig::params`'s doc). A
/// trailing `...` sets the variadic flag instead of becoming a `Param`.
fn parse_params(params_raw: &str) -> (Vec<Param>, bool) {
    let trimmed = params_raw.trim();
    if trimmed.is_empty() || trimmed == "void" {
        return (Vec::new(), false);
    }
    let mut variadic = false;
    let mut params = Vec::new();
    for part in split_top_level(trimmed, ',') {
        let part = part.trim();
        if part == "..." {
            variadic = true;
            continue;
        }
        if part.is_empty() {
            continue;
        }
        params.push(parse_param(part));
    }
    (params, variadic)
}

/// Parses one parameter's text via the same declarator grammar used
/// elsewhere (`parse_declarator`), which requires a trailing name. Old-style
/// forward declarations can have anonymous parameters (a bare type, e.g.
/// `int foo(int, char*);`) that grammar can't name - `parse_declarator`
/// fails on those (nothing after the type to read as an identifier), so
/// this falls back to reading the parameter's whole text as a bare type
/// (`parse_type_text`, same fallback `parse_fnptr_params` uses) with an
/// empty `name` rather than guessing.
fn parse_param(text: &str) -> Param {
    match parse_declarator(text) {
        Some((storage, ty, name)) => Param { ty, name, storage },
        None => Param {
            ty: parse_type_text(text),
            name: String::new(),
            storage: Vec::new(),
        },
    }
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
