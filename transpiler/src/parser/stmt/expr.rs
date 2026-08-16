//! Step 7b: a real C expression grammar - precedence-climbing recursive
//! descent over the `CTok` stream from `lex.rs`. Unlike every prior step in
//! this codebase (const initializers, struct-field types, braced-
//! initializer elements all deliberately kept scalar expression text raw),
//! this one *is* a full operator-precedence/associativity grammar - an
//! explicit, larger scope decision made for the function-body-parsing
//! feature specifically.
//!
//! Still inert as of this module: nothing outside `stmt::expr`'s own tests
//! calls into it yet. A later step (statement parsing) will drive it with
//! real `CTok` slices sliced out of function bodies.

use super::super::ast::Type;
use super::lex::{CTok, Punct};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(String),
    /// Exact literal text kept, no value computed (same reasoning `CTok`'s
    /// literal variants already use).
    IntLit(String),
    FloatLit(String),
    StrLit(String),
    CharLit(String),
    /// An explicit `(expr)` - kept as its own node (not collapsed away)
    /// so the tree shape alone still reflects the source's own grouping,
    /// same "never lose information" spirit as the rest of this codebase.
    Paren(Box<Expr>),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Postfix {
        op: PostfixOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Assign {
        op: AssignOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Ternary {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    /// N-ary, not nested binary pairs - matches `for (i=0, j=10; ...)`'s
    /// comma-separated clauses directly.
    Comma(Vec<Expr>),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
    /// `base.name`
    Member {
        base: Box<Expr>,
        name: String,
    },
    /// `base->name`
    Arrow {
        base: Box<Expr>,
        name: String,
    },
    /// Reuses `ast::Type` - no new `Type` variants needed (a cast target is
    /// always `Named`/`Pointer`/`FunctionPointer`; `Array` never appears
    /// here, since casting to an array type isn't legal C).
    Cast {
        ty: Type,
        expr: Box<Expr>,
    },
    Sizeof(SizeofArg),
    /// Graceful-degradation leaf for anything this grammar doesn't
    /// recognize - never loses bytes, never panics.
    Raw(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SizeofArg {
    Type(Type),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    BitNot,
    Neg,
    Plus,
    AddrOf,
    Deref,
    PreInc,
    PreDec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostfixOp {
    PostInc,
    PostDec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    Lt,
    Gt,
    Le,
    Ge,
    EqEq,
    NotEq,
    BitAnd,
    BitXor,
    BitOr,
    LogAnd,
    LogOr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Assign,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    AndEq,
    OrEq,
    XorEq,
    ShlEq,
    ShrEq,
}

/// Type names known to this parser, used to resolve the classic C
/// "typedef-name problem": is `(foo)` a cast or a parenthesized
/// expression? Only decidable by knowing whether `foo` names a type.
/// A fixed baseline of C base-type keywords (`int`, `char`, ...) is always
/// recognized regardless of this set's contents - see `BASE_TYPE_WORDS`.
///
/// Deliberately *not* filesystem-aware itself - `parser::corpus::collect_known_type_names`
/// (kept in a separate module) builds one from real files; this type stays
/// a plain data holder so this module's own tests can build one by hand.
#[derive(Debug, Clone, Default)]
pub struct KnownTypeNames(HashSet<String>);

impl KnownTypeNames {
    pub fn new() -> Self {
        KnownTypeNames(HashSet::new())
    }

    pub fn insert(&mut self, name: impl Into<String>) {
        self.0.insert(name.into());
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.contains(name)
    }
}

impl FromIterator<String> for KnownTypeNames {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        KnownTypeNames(iter.into_iter().collect())
    }
}

/// Parses `toks` as one comma-operator-level expression (the loosest
/// grammar level - `parse_expr` is what a statement-level caller reaches
/// for whenever the grammar calls for a full `expression`, e.g. an
/// expression-statement or a `for(...)` clause).
pub fn parse_expr(toks: &[CTok], known: &KnownTypeNames) -> Expr {
    let mut cur = Cursor::new(toks);
    parse_comma(&mut cur, known)
}

const BASE_TYPE_WORDS: &[&str] = &[
    "int", "char", "short", "long", "unsigned", "signed", "float", "double", "void",
];

/// A cursor over a `CTok` slice that transparently skips whitespace/comment
/// trivia during lookahead (`is_trivial`) without ever dropping it from the
/// underlying slice - this parser just never looks at it. Preprocessor
/// lines are NOT trivia here (a later statement-parsing step needs to see
/// them explicitly); none are expected to appear inside a pure expression
/// token range, but if one did, it falls through to `parse_primary`'s
/// catch-all rather than being silently skipped or causing a panic.
struct Cursor<'a> {
    toks: &'a [CTok],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(toks: &'a [CTok]) -> Self {
        Cursor { toks, pos: 0 }
    }

    fn save(&self) -> usize {
        self.pos
    }

    fn restore(&mut self, saved: usize) {
        self.pos = saved;
    }

    fn skip_trivia(&mut self) {
        while self.toks.get(self.pos).is_some_and(CTok::is_trivial) {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<&'a CTok> {
        self.skip_trivia();
        self.toks.get(self.pos)
    }

    fn bump(&mut self) -> Option<&'a CTok> {
        self.skip_trivia();
        let tok = self.toks.get(self.pos);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
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

// ---------------------------------------------------------------------
// Precedence-climbing chain: comma -> assign -> ternary -> binary(1) ->
// unary -> postfix -> primary. Comments on each level note the C grammar
// production it implements.
// ---------------------------------------------------------------------

fn parse_comma(cur: &mut Cursor, known: &KnownTypeNames) -> Expr {
    let first = parse_assign(cur, known);
    if !matches!(cur.peek(), Some(CTok::Punct(Punct::Comma, _))) {
        return first;
    }
    let mut items = vec![first];
    while cur.eat_punct(Punct::Comma) {
        items.push(parse_assign(cur, known));
    }
    Expr::Comma(items)
}

fn assign_op(cur: &mut Cursor) -> Option<AssignOp> {
    match cur.peek() {
        Some(CTok::Punct(p, _)) => Some(match p {
            Punct::Assign => AssignOp::Assign,
            Punct::PlusEq => AssignOp::AddEq,
            Punct::MinusEq => AssignOp::SubEq,
            Punct::StarEq => AssignOp::MulEq,
            Punct::SlashEq => AssignOp::DivEq,
            Punct::PercentEq => AssignOp::ModEq,
            Punct::AmpEq => AssignOp::AndEq,
            Punct::PipeEq => AssignOp::OrEq,
            Punct::CaretEq => AssignOp::XorEq,
            Punct::ShlEq => AssignOp::ShlEq,
            Punct::ShrEq => AssignOp::ShrEq,
            _ => return None,
        }),
        _ => None,
    }
}

/// Right-associative: `a = b = c` reads as `a = (b = c)`.
fn parse_assign(cur: &mut Cursor, known: &KnownTypeNames) -> Expr {
    let lhs = parse_ternary(cur, known);
    match assign_op(cur) {
        Some(op) => {
            cur.bump();
            let rhs = parse_assign(cur, known);
            Expr::Assign {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
        }
        None => lhs,
    }
}

/// `conditional-expression: logical-OR-expression ('?' expression ':'
/// conditional-expression)?` - note the middle operand is a full
/// (comma-permitting) `expression`, but the else-operand recurses back into
/// `conditional-expression` (no bare top-level comma there without parens)
/// - this is what makes `a ? b : c ? d : e` right-associate the nesting.
fn parse_ternary(cur: &mut Cursor, known: &KnownTypeNames) -> Expr {
    let cond = parse_binary(cur, known, 1);
    if !cur.eat_punct(Punct::Question) {
        return cond;
    }
    let then_expr = parse_comma(cur, known);
    cur.eat_punct(Punct::Colon);
    let else_expr = parse_ternary(cur, known);
    Expr::Ternary {
        cond: Box::new(cond),
        then_expr: Box::new(then_expr),
        else_expr: Box::new(else_expr),
    }
}

/// Precedence, loosest (1) to tightest (10) - matches the standard C table:
/// `|| < && < | < ^ < & < (==,!=) < (<,>,<=,>=) < (<<,>>) < (+,-) < (*,/,%)`.
fn binary_op_and_prec(cur: &mut Cursor) -> Option<(BinaryOp, u8)> {
    use BinaryOp::*;
    match cur.peek() {
        Some(CTok::Punct(p, _)) => Some(match p {
            Punct::PipePipe => (LogOr, 1),
            Punct::AmpAmp => (LogAnd, 2),
            Punct::Pipe => (BitOr, 3),
            Punct::Caret => (BitXor, 4),
            Punct::Amp => (BitAnd, 5),
            Punct::EqEq => (EqEq, 6),
            Punct::NotEq => (NotEq, 6),
            Punct::Lt => (Lt, 7),
            Punct::Gt => (Gt, 7),
            Punct::Le => (Le, 7),
            Punct::Ge => (Ge, 7),
            Punct::Shl => (Shl, 8),
            Punct::Shr => (Shr, 8),
            Punct::Plus => (Add, 9),
            Punct::Minus => (Sub, 9),
            Punct::Star => (Mul, 10),
            Punct::Slash => (Div, 10),
            Punct::Percent => (Mod, 10),
            _ => return None,
        }),
        _ => None,
    }
}

/// Standard precedence-climbing loop; every binary tier here is
/// left-associative, so the recursive call for the RHS climbs to `prec + 1`.
fn parse_binary(cur: &mut Cursor, known: &KnownTypeNames, min_prec: u8) -> Expr {
    let mut lhs = parse_unary(cur, known);
    while let Some((op, prec)) = binary_op_and_prec(cur) {
        if prec < min_prec {
            break;
        }
        cur.bump();
        let rhs = parse_binary(cur, known, prec + 1);
        lhs = Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    lhs
}

/// `unary-expression: postfix-expression | (++|--) unary-expression
/// | (unary-operator) cast-expression | sizeof unary-expression
/// | sizeof '(' type-name ')'`, plus this parser's cast-expression handling
/// (`cast-expression: unary-expression | '(' type-name ')' cast-expression`)
/// folded in here rather than as a separate function, since both dispatch
/// on the same leading tokens.
fn parse_unary(cur: &mut Cursor, known: &KnownTypeNames) -> Expr {
    if let Some(op) = prefix_unary_op(cur) {
        cur.bump();
        let expr = parse_unary(cur, known);
        return Expr::Unary {
            op,
            expr: Box::new(expr),
        };
    }
    if cur.ident_text() == Some("sizeof") {
        cur.bump();
        return parse_sizeof(cur, known);
    }
    if matches!(cur.peek(), Some(CTok::Punct(Punct::LParen, _)))
        && let Some(ty) = try_parse_cast_type(cur, known)
    {
        let expr = parse_unary(cur, known);
        return Expr::Cast {
            ty,
            expr: Box::new(expr),
        };
    }
    parse_postfix(cur, known)
}

fn prefix_unary_op(cur: &mut Cursor) -> Option<UnaryOp> {
    match cur.peek() {
        Some(CTok::Punct(p, _)) => Some(match p {
            Punct::Bang => UnaryOp::Not,
            Punct::Tilde => UnaryOp::BitNot,
            Punct::Minus => UnaryOp::Neg,
            Punct::Plus => UnaryOp::Plus,
            Punct::Amp => UnaryOp::AddrOf,
            Punct::Star => UnaryOp::Deref,
            Punct::PlusPlus => UnaryOp::PreInc,
            Punct::MinusMinus => UnaryOp::PreDec,
            _ => return None,
        }),
        _ => None,
    }
}

fn parse_sizeof(cur: &mut Cursor, known: &KnownTypeNames) -> Expr {
    if matches!(cur.peek(), Some(CTok::Punct(Punct::LParen, _)))
        && let Some(ty) = try_parse_cast_type(cur, known)
    {
        return Expr::Sizeof(SizeofArg::Type(ty));
    }
    let expr = parse_unary(cur, known);
    Expr::Sizeof(SizeofArg::Expr(Box::new(expr)))
}

/// `postfix-expression: primary-expression ( '[' expression ']'
/// | '(' argument-expression-list? ')' | '.' identifier | '->' identifier
/// | '++' | '--' )*`
fn parse_postfix(cur: &mut Cursor, known: &KnownTypeNames) -> Expr {
    let mut expr = parse_primary(cur, known);
    loop {
        match cur.peek() {
            Some(CTok::Punct(Punct::LParen, _)) => {
                cur.bump();
                let args = parse_call_args(cur, known);
                cur.eat_punct(Punct::RParen);
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            }
            Some(CTok::Punct(Punct::LBracket, _)) => {
                cur.bump();
                let index = parse_comma(cur, known);
                cur.eat_punct(Punct::RBracket);
                expr = Expr::Index {
                    base: Box::new(expr),
                    index: Box::new(index),
                };
            }
            Some(CTok::Punct(Punct::Dot, _)) => {
                cur.bump();
                let name = expect_ident(cur);
                expr = Expr::Member {
                    base: Box::new(expr),
                    name,
                };
            }
            Some(CTok::Punct(Punct::Arrow, _)) => {
                cur.bump();
                let name = expect_ident(cur);
                expr = Expr::Arrow {
                    base: Box::new(expr),
                    name,
                };
            }
            Some(CTok::Punct(Punct::PlusPlus, _)) => {
                cur.bump();
                expr = Expr::Postfix {
                    op: PostfixOp::PostInc,
                    expr: Box::new(expr),
                };
            }
            Some(CTok::Punct(Punct::MinusMinus, _)) => {
                cur.bump();
                expr = Expr::Postfix {
                    op: PostfixOp::PostDec,
                    expr: Box::new(expr),
                };
            }
            _ => break,
        }
    }
    expr
}

/// Each call argument is an `assignment-expression` (not a full
/// comma-permitting `expression`) - `foo(a, b)` is two args, not one
/// `Comma` arg; `foo((a, b))` would be needed for that.
fn parse_call_args(cur: &mut Cursor, known: &KnownTypeNames) -> Vec<Expr> {
    if matches!(cur.peek(), Some(CTok::Punct(Punct::RParen, _))) || cur.peek().is_none() {
        return Vec::new();
    }
    let mut args = Vec::new();
    loop {
        args.push(parse_assign(cur, known));
        if !cur.eat_punct(Punct::Comma) {
            break;
        }
    }
    args
}

fn expect_ident(cur: &mut Cursor) -> String {
    match cur.peek() {
        Some(CTok::Ident(span)) => {
            let s = span.text.clone();
            cur.bump();
            s
        }
        // Malformed input (never expected against real corpus text once
        // this is wired up) - best-effort empty name, never panic.
        _ => String::new(),
    }
}

fn parse_primary(cur: &mut Cursor, known: &KnownTypeNames) -> Expr {
    match cur.peek() {
        Some(CTok::Ident(span)) => {
            let text = span.text.clone();
            cur.bump();
            Expr::Ident(text)
        }
        Some(CTok::IntLit(span)) => {
            let text = span.text.clone();
            cur.bump();
            Expr::IntLit(text)
        }
        Some(CTok::FloatLit(span)) => {
            let text = span.text.clone();
            cur.bump();
            Expr::FloatLit(text)
        }
        Some(CTok::Str(span)) => {
            let text = span.text.clone();
            cur.bump();
            Expr::StrLit(text)
        }
        Some(CTok::Char(span)) => {
            let text = span.text.clone();
            cur.bump();
            Expr::CharLit(text)
        }
        Some(CTok::Punct(Punct::LParen, _)) => {
            cur.bump();
            let inner = parse_comma(cur, known);
            cur.eat_punct(Punct::RParen);
            Expr::Paren(Box::new(inner))
        }
        Some(other) => {
            let text = other.text();
            cur.bump();
            Expr::Raw(text)
        }
        None => Expr::Raw(String::new()),
    }
}

// ---------------------------------------------------------------------
// Type-name lookahead, for cast-vs-paren and sizeof(type)-vs-sizeof(expr)
// disambiguation. Backtracks via Cursor::save/restore on any failure -
// never partially consumes tokens on a failed attempt.
// ---------------------------------------------------------------------

/// Tries to read `'(' type-name ')'` starting at the current position
/// (caller has already peeked that the next token is `(`). On success,
/// consumes through the closing `)` and returns the type; on failure,
/// restores the cursor to exactly where it was and returns `None`.
fn try_parse_cast_type(cur: &mut Cursor, known: &KnownTypeNames) -> Option<Type> {
    let saved = cur.save();
    if !cur.eat_punct(Punct::LParen) {
        return None;
    }
    let Some(ty) = try_parse_type_name(cur, known) else {
        cur.restore(saved);
        return None;
    };
    if !cur.eat_punct(Punct::RParen) {
        cur.restore(saved);
        return None;
    }
    Some(ty)
}

/// `type-name := base-type-name ( fnptr-abstract-declarator | '*'* )`
fn try_parse_type_name(cur: &mut Cursor, known: &KnownTypeNames) -> Option<Type> {
    let base = try_parse_base_type_name(cur, known)?;

    // Abstract function-pointer declarator: `BASE '(' '*' ')' '(' params ')'`
    // - e.g. `void (*)(int)` (confirmed live: i_video.c's
    // `(void (*)(int)) I_Quit` cast).
    if matches!(cur.peek(), Some(CTok::Punct(Punct::LParen, _))) {
        let saved = cur.save();
        cur.bump();
        if cur.eat_punct(Punct::Star)
            && cur.eat_punct(Punct::RParen)
            && matches!(cur.peek(), Some(CTok::Punct(Punct::LParen, _)))
        {
            cur.bump();
            let params = parse_abstract_param_types(cur, known);
            if cur.eat_punct(Punct::RParen) {
                return Some(Type::FunctionPointer {
                    ret: Box::new(base),
                    params,
                });
            }
        }
        cur.restore(saved);
    }

    let mut ty = base;
    while cur.eat_punct(Punct::Star) {
        ty = Type::Pointer(Box::new(ty));
    }
    Some(ty)
}

/// `base-type-name := ('struct'|'union'|'enum') IDENT | base-keyword+ |
/// KNOWN_TYPEDEF_IDENT`. C forbids mixing a known-typedef name with base
/// keywords or a tag, so trying these three shapes in order (each
/// exclusive) is sufficient - no case falls through to another.
fn try_parse_base_type_name(cur: &mut Cursor, known: &KnownTypeNames) -> Option<Type> {
    if let Some(kw) = cur.ident_text()
        && (kw == "struct" || kw == "union" || kw == "enum")
    {
        let kw = kw.to_string();
        cur.bump();
        let tag = cur.ident_text()?.to_string();
        cur.bump();
        return Some(Type::Named(format!("{kw} {tag}")));
    }

    let mut words: Vec<String> = Vec::new();
    while let Some(word) = cur.ident_text() {
        if BASE_TYPE_WORDS.contains(&word) {
            words.push(word.to_string());
            cur.bump();
        } else {
            break;
        }
    }
    if !words.is_empty() {
        return Some(Type::Named(words.join(" ")));
    }

    if let Some(name) = cur.ident_text()
        && known.contains(name)
    {
        let name = name.to_string();
        cur.bump();
        return Some(Type::Named(name));
    }
    None
}

/// A function-pointer cast type's own parameter list is types only (no
/// names - there's nowhere for a name to go in an abstract declarator),
/// mirroring `ast::Type::FunctionPointer::params`'s existing convention.
/// `()`/`(void)` both mean no parameters, same as `FnSig::params` elsewhere.
fn parse_abstract_param_types(cur: &mut Cursor, known: &KnownTypeNames) -> Vec<Type> {
    if matches!(cur.peek(), Some(CTok::Punct(Punct::RParen, _))) || cur.peek().is_none() {
        return Vec::new();
    }
    let mut params = Vec::new();
    while let Some(ty) = try_parse_type_name(cur, known) {
        params.push(ty);
        if !cur.eat_punct(Punct::Comma) {
            break;
        }
    }
    if params.len() == 1 && params[0] == Type::Named("void".to_string()) {
        return Vec::new();
    }
    params
}

#[cfg(test)]
mod tests;
