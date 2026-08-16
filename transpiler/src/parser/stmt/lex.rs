//! Step 7a: a C token lexer for text *inside* function bodies - identifiers,
//! numeric literals, and punctuators. Only ever run on a `Group`'s `inner`
//! (a function body, or a nested `{...}` block within one), never on a
//! whole file; the coarse whole-file `scan.rs`/`RawToken` split (comments,
//! string/char literals, preprocessor lines, "everything else") already ran
//! first and is reused here unchanged - this only further tokenizes
//! whatever `scan.rs` left as opaque `Code` runs.
//!
//! Mirrors `scan.rs`'s exhaustiveness invariant: every byte of a `Code` run
//! becomes some `CTok`, including whitespace (`CTok::Trivia`) - this is
//! what lets any node built on top of this token stream reconstruct its own
//! exact source text by concatenating `.text()` over a contiguous range,
//! the same way `render_tokens`/`Chunk::render` already do one layer up.

use super::super::ast::{Chunk, Pos, RawToken, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punct {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Colon,
    Question,
    Dot,
    Arrow,
    Ellipsis,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    PlusPlus,
    MinusMinus,
    Amp,
    Pipe,
    Caret,
    Tilde,
    Bang,
    Shl,
    Shr,
    Lt,
    Gt,
    Le,
    Ge,
    EqEq,
    NotEq,
    AmpAmp,
    PipePipe,
    Assign,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AmpEq,
    PipeEq,
    CaretEq,
    ShlEq,
    ShrEq,
}

#[derive(Debug, Clone)]
pub enum CTok {
    /// An identifier or keyword - keyword-ness is for a later step to
    /// decide (mirrors `Storage::from_keyword`'s pattern: this lexer stays
    /// dumb/mechanical, same as `scan.rs`).
    Ident(Span),
    /// Exact literal text kept, no numeric value computed - hex/octal/
    /// decimal, with any `u`/`U`/`l`/`L` suffix.
    IntLit(Span),
    /// Decimal only (`1.0`, `.5`, `1e10`, with an `f`/`F`/`l`/`L` suffix) -
    /// no hex-float, unseen anywhere in the target corpus.
    FloatLit(Span),
    /// 1:1 pass-through of a `RawToken::StringLit`.
    Str(Span),
    /// 1:1 pass-through of a `RawToken::CharLit`.
    Char(Span),
    Punct(Punct, Span),
    /// An opaque `{ ... }` block - produced only by `tokenize_chunks`,
    /// never by `lex_ctoks` itself (which never sees a `{`/`}` byte, since
    /// `group_braces` already extracted every matched pair one level up).
    /// Descending into it means calling `group_braces`/`tokenize_chunks`
    /// again on `inner` - the same one-level-at-a-time primitive
    /// `parser::record::parse_fields` already uses for nested anonymous
    /// struct/union bodies.
    Group {
        open: Span,
        inner: Vec<RawToken>,
        close: Span,
    },
    LineComment(Span),
    BlockComment(Span),
    PreprocLine(Span),
    /// A whitespace run. Kept as a first-class token (not dropped) so that
    /// `render_ctoks` over any contiguous range reproduces the exact
    /// original text - the same "never lose bytes" invariant every other
    /// layer of this parser relies on. Skipped during grammar lookahead by
    /// `is_trivial`, never dropped from the stream itself.
    Trivia(Span),
    /// Defensive catch-all for a byte this lexer doesn't recognize (none
    /// expected in this corpus) - never panics, never drops bytes.
    Unknown(Span),
}

impl CTok {
    pub fn text(&self) -> String {
        match self {
            CTok::Ident(s)
            | CTok::IntLit(s)
            | CTok::FloatLit(s)
            | CTok::Str(s)
            | CTok::Char(s)
            | CTok::Punct(_, s)
            | CTok::LineComment(s)
            | CTok::BlockComment(s)
            | CTok::PreprocLine(s)
            | CTok::Trivia(s)
            | CTok::Unknown(s) => s.text.clone(),
            CTok::Group { open, inner, close } => {
                let mut out = open.text.clone();
                for t in inner {
                    out.push_str(t.text());
                }
                out.push_str(&close.text);
                out
            }
        }
    }

    /// True for tokens grammar lookahead should skip over: whitespace and
    /// comments. Preprocessor lines are deliberately NOT skipped here - a
    /// later statement-parsing step needs to see them explicitly (a
    /// `#ifdef`/`#if 0` mid-body must become its own statement, not vanish
    /// from lookahead).
    pub fn is_trivial(&self) -> bool {
        matches!(
            self,
            CTok::Trivia(_) | CTok::LineComment(_) | CTok::BlockComment(_)
        )
    }
}

pub fn render_ctoks(toks: &[CTok]) -> String {
    toks.iter().map(CTok::text).collect()
}

/// Flattens one `group_braces` level's output into a single `CTok` stream:
/// a `Chunk::Flat` run is lexed via `lex_ctoks`; a `Chunk::Group` becomes
/// one opaque `CTok::Group`.
pub fn tokenize_chunks(chunks: Vec<Chunk>) -> Vec<CTok> {
    let mut out = Vec::new();
    for chunk in chunks {
        match chunk {
            Chunk::Flat(toks) => out.extend(lex_ctoks(&toks)),
            Chunk::Group { open, inner, close } => out.push(CTok::Group { open, inner, close }),
        }
    }
    out
}

/// Tokenizes a run of already-classified `RawToken`s (as produced by
/// `scan.rs`, with no top-level `{`/`}` among them - see `CTok::Group`'s
/// doc comment) into `CTok`s. String/char literals, comments, and
/// preprocessor lines pass through unchanged (already correctly classified
/// one layer up); only `RawToken::Code` runs are lexed further, into
/// identifiers, numeric literals, punctuators, and whitespace.
pub fn lex_ctoks(tokens: &[RawToken]) -> Vec<CTok> {
    let mut out = Vec::new();
    for tok in tokens {
        match tok {
            RawToken::Code(span) => lex_code_span(span, &mut out),
            RawToken::StringLit(span) => out.push(CTok::Str(span.clone())),
            RawToken::CharLit(span) => out.push(CTok::Char(span.clone())),
            RawToken::LineComment(span) => out.push(CTok::LineComment(span.clone())),
            RawToken::BlockComment(span) => out.push(CTok::BlockComment(span.clone())),
            RawToken::PreprocLine(span) => out.push(CTok::PreprocLine(span.clone())),
        }
    }
    out
}

fn lex_code_span(span: &Span, out: &mut Vec<CTok>) {
    let text = span.text.as_str();
    let bytes = text.as_bytes();
    let mut pos = span.start;
    let mut i = 0usize;

    while i < bytes.len() {
        let b = bytes[i];
        let start_i = i;
        let start_pos = pos;

        if b.is_ascii_whitespace() {
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            out.push(CTok::Trivia(make_span(
                text, &mut pos, start_pos, start_i, i,
            )));
            continue;
        }
        if b.is_ascii_digit() || (b == b'.' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit)) {
            let (end, is_float) = lex_number(bytes, i);
            i = end;
            let span = make_span(text, &mut pos, start_pos, start_i, i);
            out.push(if is_float {
                CTok::FloatLit(span)
            } else {
                CTok::IntLit(span)
            });
            continue;
        }
        if b.is_ascii_alphabetic() || b == b'_' {
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            out.push(CTok::Ident(make_span(
                text, &mut pos, start_pos, start_i, i,
            )));
            continue;
        }
        if let Some((punct, len)) = match_punct(&bytes[i..]) {
            i += len;
            let span = make_span(text, &mut pos, start_pos, start_i, i);
            out.push(CTok::Punct(punct, span));
            continue;
        }
        // Unrecognized byte (none expected in this corpus): one-byte
        // Unknown, never a panic, never a dropped byte.
        i += 1;
        out.push(CTok::Unknown(make_span(
            text, &mut pos, start_pos, start_i, i,
        )));
    }
}

fn make_span(text: &str, pos: &mut Pos, start_pos: Pos, start_i: usize, end_i: usize) -> Span {
    let piece = &text[start_i..end_i];
    *pos = pos.advance(piece);
    Span {
        start: start_pos,
        end: *pos,
        text: piece.to_string(),
    }
}

/// Scans one numeric literal starting at `bytes[start]` (caller has already
/// confirmed this is a valid start: a digit, or `.` followed by a digit).
/// Returns the end index (exclusive) and whether it's a float (contains a
/// `.` or a decimal exponent - hex literals are always integers, and a
/// hex digit run's own trailing `e`/`E` is just a hex digit, never mistaken
/// for an exponent).
fn lex_number(bytes: &[u8], start: usize) -> (usize, bool) {
    let n = bytes.len();
    let mut i = start;

    if bytes[i] == b'0' && matches!(bytes.get(i + 1), Some(b'x') | Some(b'X')) {
        i += 2;
        while i < n && bytes[i].is_ascii_hexdigit() {
            i += 1;
        }
        while i < n && matches!(bytes[i], b'u' | b'U' | b'l' | b'L') {
            i += 1;
        }
        return (i, false);
    }

    let mut is_float = false;
    if bytes[i] == b'.' {
        is_float = true;
        i += 1;
    } else {
        while i < n && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i < n && bytes[i] == b'.' {
            is_float = true;
            i += 1;
        }
    }
    while i < n && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i < n && matches!(bytes[i], b'e' | b'E') {
        let mut j = i + 1;
        if j < n && matches!(bytes[j], b'+' | b'-') {
            j += 1;
        }
        if j < n && bytes[j].is_ascii_digit() {
            while j < n && bytes[j].is_ascii_digit() {
                j += 1;
            }
            is_float = true;
            i = j;
        }
    }
    while i < n && matches!(bytes[i], b'u' | b'U' | b'l' | b'L' | b'f' | b'F') {
        i += 1;
    }
    (i, is_float)
}

/// Maximal-munch punctuator match at the start of `bytes`, longest first.
fn match_punct(bytes: &[u8]) -> Option<(Punct, usize)> {
    use Punct::*;

    let b0 = bytes[0];
    let b1 = bytes.get(1).copied();
    let b2 = bytes.get(2).copied();

    match (b0, b1, b2) {
        (b'<', Some(b'<'), Some(b'=')) => return Some((ShlEq, 3)),
        (b'>', Some(b'>'), Some(b'=')) => return Some((ShrEq, 3)),
        (b'.', Some(b'.'), Some(b'.')) => return Some((Ellipsis, 3)),
        _ => {}
    }
    match (b0, b1) {
        (b'-', Some(b'>')) => return Some((Arrow, 2)),
        (b'+', Some(b'+')) => return Some((PlusPlus, 2)),
        (b'-', Some(b'-')) => return Some((MinusMinus, 2)),
        (b'<', Some(b'<')) => return Some((Shl, 2)),
        (b'>', Some(b'>')) => return Some((Shr, 2)),
        (b'<', Some(b'=')) => return Some((Le, 2)),
        (b'>', Some(b'=')) => return Some((Ge, 2)),
        (b'=', Some(b'=')) => return Some((EqEq, 2)),
        (b'!', Some(b'=')) => return Some((NotEq, 2)),
        (b'&', Some(b'&')) => return Some((AmpAmp, 2)),
        (b'|', Some(b'|')) => return Some((PipePipe, 2)),
        (b'+', Some(b'=')) => return Some((PlusEq, 2)),
        (b'-', Some(b'=')) => return Some((MinusEq, 2)),
        (b'*', Some(b'=')) => return Some((StarEq, 2)),
        (b'/', Some(b'=')) => return Some((SlashEq, 2)),
        (b'%', Some(b'=')) => return Some((PercentEq, 2)),
        (b'&', Some(b'=')) => return Some((AmpEq, 2)),
        (b'|', Some(b'=')) => return Some((PipeEq, 2)),
        (b'^', Some(b'=')) => return Some((CaretEq, 2)),
        _ => {}
    }
    let p = match b0 {
        b'(' => LParen,
        b')' => RParen,
        b'[' => LBracket,
        b']' => RBracket,
        b'{' => LBrace,
        b'}' => RBrace,
        b',' => Comma,
        b';' => Semicolon,
        b':' => Colon,
        b'?' => Question,
        b'.' => Dot,
        b'+' => Plus,
        b'-' => Minus,
        b'*' => Star,
        b'/' => Slash,
        b'%' => Percent,
        b'&' => Amp,
        b'|' => Pipe,
        b'^' => Caret,
        b'~' => Tilde,
        b'!' => Bang,
        b'<' => Lt,
        b'>' => Gt,
        b'=' => Assign,
        _ => return None,
    };
    Some((p, 1))
}

#[cfg(test)]
mod tests;
