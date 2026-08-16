//! Step 7e: the `Block`/`Stmt` recursive-descent driver - the last layer of
//! the function-body-parsing feature, sitting on top of `lex.rs` (tokens),
//! `expr.rs` (expressions) and `decl.rs` (local declarations).
//!
//! Statement-splitting can't reuse the existing "split on top-level `;`
//! first, classify second" pattern `record::build_items`/`parse_fields` use:
//! dangling-else-style nesting (`if (x) if (y) z=1; else w=2;`) means a
//! top-level `;` pre-scan would misattach `else w=2;` to the wrong `if`.
//! This is true single-token-lookahead recursive descent over a flattened
//! `CTok` stream instead.

use super::super::ast::{Comment, RawToken, Trivia};
use super::super::preproc::parse_directive;
use super::ast::{Block, ForInit, Label, Stmt, StmtKind};
use super::decl::{looks_like_decl_start, split_ctoks_top_level, trim_trivia, try_parse_decl_stmt};
use super::expr::{KnownTypeNames, parse_expr};
use super::lex::{CTok, Punct, render_ctoks, tokenize_chunks};
use std::ops::Range;

/// Parses one `{...}` level's contents (a function body, or a nested
/// explicit block within one) into a `Block`. Calls `group_braces` for this
/// one level (same "call it again on a Group's inner" primitive
/// `record::parse_fields` already uses for nested anonymous struct/union
/// bodies), flattens via `tokenize_chunks` into one `CTok` stream (a nested
/// `{...}` becomes one opaque `CTok::Group`), then does ordinary recursive
/// descent. Whenever the grammar needs a compound statement, it consumes
/// the next `CTok::Group` and recurses back into this same function on its
/// `inner`.
pub fn parse_block(inner: Vec<RawToken>, known: &KnownTypeNames) -> Block {
    let chunks = super::super::brace::group_braces(inner);
    let toks = tokenize_chunks(chunks);
    Block {
        stmts: parse_stmt_list(&toks, known),
    }
}

/// Reconstructs a `Block`'s exact original text (mirrors `ast::render_items`
/// one layer up) - used for the corpus-wide self-check that `FnBody.raw`'s
/// braces wrapped around this equal `FnBody.raw` itself; never used by
/// `File::render()`, which doesn't touch `Block`/`Stmt` at all.
pub fn render_block(block: &Block) -> String {
    let mut out = String::new();
    for (stmt, trivia) in &block.stmts {
        for c in &trivia.leading {
            out.push_str(c.text());
        }
        out.push_str(&stmt.raw);
    }
    out
}

struct Cursor<'a> {
    toks: &'a [CTok],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(toks: &'a [CTok]) -> Self {
        Cursor { toks, pos: 0 }
    }

    /// Index of the next non-trivial token at or after `self.pos`, without
    /// mutating `self.pos` - unlike `expr::Cursor` (whose nodes carry no
    /// `raw` and so can afford to skip-and-forget), `parse_stmt` needs
    /// `self.pos` to stay exactly where the caller left it (e.g. right
    /// after `drain_leading_comments`) until something actually `bump`s
    /// past intervening trivia, or that trivia's bytes silently vanish from
    /// every `Stmt.raw` instead of landing in the right one.
    fn next_real_idx(&self) -> Option<usize> {
        let mut i = self.pos;
        while self.toks.get(i).is_some_and(CTok::is_trivial) {
            i += 1;
        }
        if i < self.toks.len() { Some(i) } else { None }
    }

    fn peek(&self) -> Option<&'a CTok> {
        self.next_real_idx().map(|i| &self.toks[i])
    }

    fn bump(&mut self) -> Option<&'a CTok> {
        match self.next_real_idx() {
            Some(i) => {
                self.pos = i + 1;
                Some(&self.toks[i])
            }
            None => {
                self.pos = self.toks.len();
                None
            }
        }
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn save(&self) -> usize {
        self.pos
    }

    fn restore(&mut self, saved: usize) {
        self.pos = saved;
    }

    fn eat_punct(&mut self, p: Punct) -> bool {
        if matches!(self.peek(), Some(CTok::Punct(pp, _)) if *pp == p) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn ident_text(&mut self) -> Option<&'a str> {
        match self.peek() {
            Some(CTok::Ident(span)) => Some(span.text.as_str()),
            _ => None,
        }
    }
}

fn is_control_keyword(w: &str) -> bool {
    matches!(
        w,
        "if" | "while"
            | "do"
            | "for"
            | "switch"
            | "return"
            | "break"
            | "continue"
            | "goto"
            | "case"
            | "default"
            | "else"
    )
}

// ---------------------------------------------------------------------
// Statement-list driver: leading-comment / same-line-trailing-comment
// handling mirrors `record::split_into_pieces`/`reattach_trailing_comments`
// one layer up (`CTok` instead of `RawToken`), so a `Block`'s own
// `(Stmt, Trivia)` list is byte-exact-reconstructable the same way a
// `File`'s `(Item, Trivia)` list already is.
// ---------------------------------------------------------------------

fn parse_stmt_list(toks: &[CTok], known: &KnownTypeNames) -> Vec<(Stmt, Trivia)> {
    let mut cur = Cursor::new(toks);
    let mut out = Vec::new();
    loop {
        let scan_start = cur.pos();
        // `peek()` skips both whitespace *and* comments (see
        // `CTok::is_trivial`) - if it finds nothing, everything remaining
        // is pure trivia/comments with no real statement to attach it to.
        // Fold it all into one final catch-all rather than draining
        // comments into a `Trivia` that would then have nowhere to go.
        if cur.peek().is_none() {
            if scan_start < toks.len() {
                out.push(raw_stmt(render_ctoks(&toks[scan_start..])));
            }
            break;
        }
        let leading = drain_leading_comments(&mut cur, toks);
        let stmt = parse_stmt(&mut cur, known);
        let stmt = absorb_trailing_same_line_comment(&mut cur, toks, stmt);
        out.push((stmt, Trivia { leading }));
    }
    out
}

fn raw_stmt(raw: String) -> (Stmt, Trivia) {
    (
        Stmt {
            kind: StmtKind::Raw,
            labels: Vec::new(),
            raw,
        },
        Trivia::default(),
    )
}

/// Drains a run of comment tokens that are literally the very next tokens
/// (no intervening whitespace) into a leading-comment list - same "must be
/// the first token(s), a preceding blank line stops the drain" policy
/// `record::drain_leading_comments` already applies one layer up.
fn drain_leading_comments(cur: &mut Cursor, toks: &[CTok]) -> Vec<Comment> {
    let mut leading = Vec::new();
    loop {
        match toks.get(cur.pos) {
            Some(CTok::LineComment(s)) => {
                leading.push(Comment::Line(s.text.clone()));
                cur.pos += 1;
            }
            Some(CTok::BlockComment(s)) => {
                leading.push(Comment::Block(s.text.clone()));
                cur.pos += 1;
            }
            _ => break,
        }
    }
    leading
}

/// A comment on the same line as a statement's own terminator describes
/// that statement, not whatever follows - mirrors
/// `record::take_leading_same_line_comment`. If the tokens right after
/// `stmt`'s own raw range are (optional single no-newline `Trivia`) +
/// (a comment), that span is appended onto `stmt.raw` instead of being left
/// for the *next* statement's leading-comment drain.
fn absorb_trailing_same_line_comment(cur: &mut Cursor, toks: &[CTok], mut stmt: Stmt) -> Stmt {
    let end = cur.pos();
    let mut probe = end;
    if let Some(CTok::Trivia(s)) = toks.get(probe)
        && !s.text.contains('\n')
    {
        probe += 1;
    }
    if matches!(
        toks.get(probe),
        Some(CTok::LineComment(_)) | Some(CTok::BlockComment(_))
    ) {
        let new_end = probe + 1;
        stmt.raw.push_str(&render_ctoks(&toks[end..new_end]));
        cur.pos = new_end;
    }
    stmt
}

// ---------------------------------------------------------------------
// One statement: an optional stacked-label prefix, then its real content.
// ---------------------------------------------------------------------

fn parse_stmt(cur: &mut Cursor, known: &KnownTypeNames) -> Stmt {
    // `cur.pos` is exactly where the caller (`parse_stmt_list`, after
    // `drain_leading_comments`) left it - `peek()` no longer mutates it, so
    // any leading trivia here is correctly still unconsumed and becomes
    // part of `raw` below, not lost.
    let start = cur.pos();
    let mut labels = Vec::new();
    loop {
        match cur.ident_text() {
            Some("case") => {
                cur.bump();
                let range = scan_to_top_level_punct(cur, Punct::Colon);
                labels.push(Label::Case(parse_expr(&cur.toks[range], known)));
                continue;
            }
            Some("default") => {
                let saved = cur.save();
                cur.bump();
                if cur.eat_punct(Punct::Colon) {
                    labels.push(Label::Default);
                    continue;
                }
                cur.restore(saved);
            }
            _ => {}
        }
        if let Some(name) = try_parse_named_label(cur) {
            labels.push(Label::Named(name));
            continue;
        }
        break;
    }
    let kind = parse_stmt_kind(cur, known);
    let end = cur.pos();
    let raw = render_ctoks(&cur.toks[start..end]);
    Stmt { kind, labels, raw }
}

/// A plain `NAME :` goto-target label. Speculative: saves position, and
/// restores it if the colon never follows (so whatever this wasn't gets
/// tried as an ordinary statement instead).
fn try_parse_named_label(cur: &mut Cursor) -> Option<String> {
    let saved = cur.save();
    let name = cur.ident_text()?.to_string();
    if is_control_keyword(&name) {
        return None;
    }
    cur.bump();
    if cur.eat_punct(Punct::Colon) {
        Some(name)
    } else {
        cur.restore(saved);
        None
    }
}

fn parse_stmt_kind(cur: &mut Cursor, known: &KnownTypeNames) -> StmtKind {
    match cur.peek() {
        Some(CTok::PreprocLine(span)) => {
            let text = span.text.clone();
            cur.bump();
            StmtKind::Preproc(parse_directive(&text))
        }
        Some(CTok::Group { .. }) => {
            let inner = match cur.bump() {
                Some(CTok::Group { inner, .. }) => inner.clone(),
                _ => unreachable!("just peeked a Group"),
            };
            StmtKind::Block(parse_block(inner, known))
        }
        Some(CTok::Punct(Punct::Semicolon, _)) => {
            cur.bump();
            StmtKind::Empty
        }
        Some(CTok::Ident(span)) => match span.text.as_str() {
            "if" => parse_if(cur, known),
            "while" => parse_while(cur, known),
            "do" => parse_do_while(cur, known),
            "for" => parse_for(cur, known),
            "switch" => parse_switch(cur, known),
            "return" => parse_return(cur, known),
            "break" => {
                cur.bump();
                eat_semi(cur);
                StmtKind::Break
            }
            "continue" => {
                cur.bump();
                eat_semi(cur);
                StmtKind::Continue
            }
            "goto" => parse_goto(cur),
            _ => parse_decl_or_expr_stmt(cur, known),
        },
        None => StmtKind::Empty,
        _ => parse_decl_or_expr_stmt(cur, known),
    }
}

fn eat_semi(cur: &mut Cursor) {
    cur.eat_punct(Punct::Semicolon);
}

fn parse_if(cur: &mut Cursor, known: &KnownTypeNames) -> StmtKind {
    cur.bump(); // "if"
    let range = scan_bracketed(cur, Punct::LParen, Punct::RParen);
    let cond = parse_expr(&cur.toks[range], known);
    let then_branch = Box::new(parse_stmt(cur, known));
    // `else if` is just an `If` nested here via the recursive `parse_stmt`
    // call - no separate handling needed.
    let else_branch = if cur.ident_text() == Some("else") {
        cur.bump();
        Some(Box::new(parse_stmt(cur, known)))
    } else {
        None
    };
    StmtKind::If {
        cond,
        then_branch,
        else_branch,
    }
}

fn parse_while(cur: &mut Cursor, known: &KnownTypeNames) -> StmtKind {
    cur.bump(); // "while"
    let range = scan_bracketed(cur, Punct::LParen, Punct::RParen);
    let cond = parse_expr(&cur.toks[range], known);
    let body = Box::new(parse_stmt(cur, known));
    StmtKind::While { cond, body }
}

fn parse_do_while(cur: &mut Cursor, known: &KnownTypeNames) -> StmtKind {
    cur.bump(); // "do"
    let body = Box::new(parse_stmt(cur, known));
    if cur.ident_text() == Some("while") {
        cur.bump();
    }
    let range = scan_bracketed(cur, Punct::LParen, Punct::RParen);
    let cond = parse_expr(&cur.toks[range], known);
    eat_semi(cur);
    StmtKind::DoWhile { body, cond }
}

fn parse_switch(cur: &mut Cursor, known: &KnownTypeNames) -> StmtKind {
    cur.bump(); // "switch"
    let range = scan_bracketed(cur, Punct::LParen, Punct::RParen);
    let scrutinee = parse_expr(&cur.toks[range], known);
    let body = Box::new(parse_stmt(cur, known));
    StmtKind::Switch { scrutinee, body }
}

fn parse_return(cur: &mut Cursor, known: &KnownTypeNames) -> StmtKind {
    cur.bump(); // "return"
    let range = scan_to_top_level_semicolon(cur);
    let toks = trim_trivia(&cur.toks[range]);
    if toks.is_empty() {
        StmtKind::Return(None)
    } else {
        StmtKind::Return(Some(parse_expr(toks, known)))
    }
}

fn parse_goto(cur: &mut Cursor) -> StmtKind {
    cur.bump(); // "goto"
    let name = cur.ident_text().unwrap_or_default().to_string();
    if !name.is_empty() {
        cur.bump();
    }
    eat_semi(cur);
    StmtKind::Goto(name)
}

fn parse_for(cur: &mut Cursor, known: &KnownTypeNames) -> StmtKind {
    cur.bump(); // "for"
    let range = scan_bracketed(cur, Punct::LParen, Punct::RParen);
    let mut parts = split_ctoks_top_level(&cur.toks[range], Punct::Semicolon);
    // A well-formed `for(init; cond; step)` always has exactly 2 top-level
    // `;`s (3 pieces). Pad defensively rather than indexing out of bounds
    // on malformed input.
    while parts.len() < 3 {
        parts.push(Vec::new());
    }
    let init_toks = trim_trivia(&parts[0]);
    let cond_toks = trim_trivia(&parts[1]);
    let step_toks = trim_trivia(&parts[2]);

    let init = if init_toks.is_empty() {
        None
    } else if looks_like_decl_start(init_toks, known)
        && let Some(decl) = try_parse_decl_stmt(init_toks, known)
    {
        Some(ForInit::Decl(decl))
    } else {
        Some(ForInit::Expr(parse_expr(init_toks, known)))
    };
    let cond = if cond_toks.is_empty() {
        None
    } else {
        Some(parse_expr(cond_toks, known))
    };
    let step = if step_toks.is_empty() {
        None
    } else {
        Some(parse_expr(step_toks, known))
    };
    let body = Box::new(parse_stmt(cur, known));
    StmtKind::For {
        init,
        cond,
        step,
        body,
    }
}

fn parse_decl_or_expr_stmt(cur: &mut Cursor, known: &KnownTypeNames) -> StmtKind {
    let range = scan_to_top_level_semicolon(cur);
    let toks = &cur.toks[range];
    if looks_like_decl_start(toks, known)
        && let Some(decl) = try_parse_decl_stmt(toks, known)
    {
        return StmtKind::Decl(decl);
    }
    if trim_trivia(toks).is_empty() {
        return StmtKind::Empty;
    }
    StmtKind::Expr(parse_expr(toks, known))
}

// ---------------------------------------------------------------------
// Token-range scanners. These only ever feed a sub-parse (`parse_expr`,
// `try_parse_decl_stmt`) whose own output carries no `raw` of its own - the
// enclosing `Stmt.raw` is always independently sliced from `cur`'s own
// index range in `parse_stmt`, so these can freely skip trivia internally
// without any byte-loss risk.
// ---------------------------------------------------------------------

/// Assumes the next token is `open` (not yet consumed); consumes it, then
/// scans forward - `open` nests, `close` un-nests - until the matching
/// `close` at depth 0, consuming that too. Returns the index range strictly
/// between them.
fn scan_bracketed(cur: &mut Cursor, open: Punct, close: Punct) -> Range<usize> {
    cur.eat_punct(open);
    let start = cur.pos();
    let mut depth = 0i32;
    loop {
        match cur.peek() {
            None => return start..cur.pos(),
            Some(CTok::Punct(p, _)) if *p == close && depth == 0 => {
                let end = cur.pos();
                cur.bump();
                return start..end;
            }
            Some(CTok::Punct(p, _)) if *p == open => {
                depth += 1;
                cur.bump();
            }
            Some(CTok::Punct(p, _)) if *p == close => {
                depth -= 1;
                cur.bump();
            }
            _ => {
                cur.bump();
            }
        }
    }
}

/// Scans forward - `(`/`[` nest, `)`/`]` un-nest - until a top-level `;`,
/// consuming it. Terminates at end-of-tokens if none is found (malformed
/// input), never loops forever.
fn scan_to_top_level_semicolon(cur: &mut Cursor) -> Range<usize> {
    let start = cur.pos();
    let mut depth = 0i32;
    loop {
        match cur.peek() {
            None => return start..cur.pos(),
            Some(CTok::Punct(Punct::Semicolon, _)) if depth <= 0 => {
                let end = cur.pos();
                cur.bump();
                return start..end;
            }
            Some(CTok::Punct(Punct::LParen, _)) | Some(CTok::Punct(Punct::LBracket, _)) => {
                depth += 1;
                cur.bump();
            }
            Some(CTok::Punct(Punct::RParen, _)) | Some(CTok::Punct(Punct::RBracket, _)) => {
                depth -= 1;
                cur.bump();
            }
            _ => {
                cur.bump();
            }
        }
    }
}

/// Scans forward - `(`/`[` nest, `)`/`]` un-nest, and (only when `target` is
/// `Colon`) a `?` opens one level of ternary nesting so its own `:` doesn't
/// prematurely end the scan - until a top-level `target`, consuming it.
/// Used for a `case EXPR :` label's expression.
fn scan_to_top_level_punct(cur: &mut Cursor, target: Punct) -> Range<usize> {
    let start = cur.pos();
    let mut depth = 0i32;
    let mut ternary_depth = 0i32;
    loop {
        match cur.peek() {
            None => return start..cur.pos(),
            Some(CTok::Punct(Punct::Question, _)) if target == Punct::Colon => {
                ternary_depth += 1;
                cur.bump();
            }
            Some(CTok::Punct(p, _)) if *p == target && depth <= 0 => {
                if target == Punct::Colon && ternary_depth > 0 {
                    ternary_depth -= 1;
                    cur.bump();
                    continue;
                }
                let end = cur.pos();
                cur.bump();
                return start..end;
            }
            Some(CTok::Punct(Punct::LParen, _)) | Some(CTok::Punct(Punct::LBracket, _)) => {
                depth += 1;
                cur.bump();
            }
            Some(CTok::Punct(Punct::RParen, _)) | Some(CTok::Punct(Punct::RBracket, _)) => {
                depth -= 1;
                cur.bump();
            }
            _ => {
                cur.bump();
            }
        }
    }
}

#[cfg(test)]
mod tests;
