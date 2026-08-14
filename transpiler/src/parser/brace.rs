//! Step 1b: brace grouping. Flat, non-recursive: finds matched top-level
//! `{ ... }` pairs and stores each one's contents as an opaque token list.
//! Braces nested inside a group are still depth-counted (so the *matching*
//! outer close is found correctly) but are not themselves turned into
//! nested `Chunk::Group`s - callers that need one more level call
//! `group_braces` again on a group's `inner`.
//!
//! Operates on the `Code` tokens produced by `scan` only; comments,
//! string/char literals and preprocessor lines are opaque to it (a `{`
//! inside a block comment or string literal is just text).

use super::ast::{Chunk, RawToken, Span};

pub fn group_braces(tokens: Vec<RawToken>) -> Vec<Chunk> {
    let mut top: Vec<Chunk> = Vec::new();
    let mut flat: Vec<RawToken> = Vec::new();

    let mut depth: usize = 0;
    let mut group_open: Option<Span> = None;
    let mut inner: Vec<RawToken> = Vec::new();

    for tok in tokens {
        if let RawToken::Code(span) = &tok {
            let text = span.text.as_str();
            let bytes = text.as_bytes();
            let mut seg_start = 0usize;
            let mut pos = span.start;

            for (i, &b) in bytes.iter().enumerate() {
                if b != b'{' && b != b'}' {
                    continue;
                }

                if i > seg_start {
                    let piece = &text[seg_start..i];
                    let piece_start = pos;
                    pos = pos.advance(piece);
                    let code_tok = RawToken::Code(Span {
                        start: piece_start,
                        end: pos,
                        text: piece.to_string(),
                    });
                    if depth == 0 {
                        flat.push(code_tok);
                    } else {
                        inner.push(code_tok);
                    }
                }

                let brace_start = pos;
                let brace_str = &text[i..i + 1];
                pos = pos.advance(brace_str);
                let brace_span = Span {
                    start: brace_start,
                    end: pos,
                    text: brace_str.to_string(),
                };

                if b == b'{' {
                    if depth == 0 {
                        if !flat.is_empty() {
                            top.push(Chunk::Flat(std::mem::take(&mut flat)));
                        }
                        group_open = Some(brace_span);
                    } else {
                        inner.push(RawToken::Code(brace_span));
                    }
                    depth += 1;
                } else {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        let open = group_open.take().unwrap_or_else(|| brace_span.clone());
                        top.push(Chunk::Group {
                            open,
                            close: brace_span,
                            inner: std::mem::take(&mut inner),
                        });
                    } else {
                        inner.push(RawToken::Code(brace_span));
                    }
                }
                seg_start = i + 1;
            }

            if seg_start < bytes.len() {
                let piece = &text[seg_start..];
                let code_tok = RawToken::Code(Span {
                    start: pos,
                    end: span.end,
                    text: piece.to_string(),
                });
                if depth == 0 {
                    flat.push(code_tok);
                } else {
                    inner.push(code_tok);
                }
            }
        } else if depth == 0 {
            flat.push(tok);
        } else {
            inner.push(tok);
        }
    }

    if !flat.is_empty() {
        top.push(Chunk::Flat(flat));
    }
    // Unclosed group at EOF (malformed input): surface what we have instead
    // of silently dropping it.
    if let Some(open) = group_open {
        top.push(Chunk::Group {
            open: open.clone(),
            close: open,
            inner,
        });
    }
    top
}

#[cfg(test)]
mod tests;
