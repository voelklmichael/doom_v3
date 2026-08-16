//! Step 7c: the statement AST - the shapes `stmt/parse.rs`'s recursive
//! descent produces, one level up from `expr.rs`'s `Expr`.

use super::super::preproc::Directive;
use super::decl::DeclStmt;
use super::expr::Expr;
use serde::Serialize;

/// Mirrors `Vec<(Item, Trivia)>` at the top level exactly - a function body
/// is just a nested list of statements, each with its own leading-comment
/// trivia.
#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub stmts: Vec<(Stmt, super::super::ast::Trivia)>,
}

/// One statement. `raw` is the exact original text this statement's tokens
/// span (mirrors `Item.raw`) - the round-trip source of truth at this
/// granularity; `kind` (and everything it owns - `Expr`, `DeclStmt`, ...)
/// is best-effort structure on top of it, same relationship `Item.raw` has
/// to `ItemKind` one level up.
#[derive(Debug, Clone, Serialize)]
pub struct Stmt {
    pub kind: StmtKind,
    /// Stacked prefix labels (`case A: case B: stmt;`, or a plain
    /// `name: stmt;` goto target) - a `Vec` rather than separate sibling
    /// statements, so grouped-empty-case fallthrough (the dominant `switch`
    /// pattern in this corpus) falls out of the grammar for free.
    pub labels: Vec<Label>,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum Label {
    Case(Expr),
    Default,
    Named(String),
}

#[derive(Debug, Clone, Serialize)]
pub enum StmtKind {
    Expr(Expr),
    Decl(DeclStmt),
    Block(Block),
    If {
        cond: Expr,
        then_branch: Box<Stmt>,
        /// `else if` is just an `If` nested here - the correct C grammar
        /// reading, no separate `ElseIf` variant needed.
        else_branch: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    DoWhile {
        body: Box<Stmt>,
        cond: Expr,
    },
    For {
        init: Option<ForInit>,
        cond: Option<Expr>,
        step: Option<Expr>,
        body: Box<Stmt>,
    },
    /// `body` stays `Box<Stmt>` (uniform with if/while/for) rather than
    /// forcing `Block` - real usage in this corpus is always a `Block` in
    /// practice, but the grammar doesn't require it.
    Switch {
        scrutinee: Expr,
        body: Box<Stmt>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    Goto(String),
    Empty,
    /// A standalone preprocessor directive at statement position that isn't
    /// part of an `#if`/`#ifdef`/.../`#endif` run - `stmt::cond` folds the
    /// conditional family into `Conditional` below; whatever's left (a
    /// stray `#define`/`#undef`/`#pragma` mid-body) stays a flat sibling
    /// here, same as `ItemKind::Preproc` does one level up.
    Preproc(Directive),
    /// A folded `#if`/`#ifdef`/`#ifndef`...`#endif` run - the statement-level
    /// counterpart to `ItemKind::Conditional`. See `stmt::cond::fold_conditionals`.
    Conditional(StmtCondGroup),
    /// Fallback for anything this grammar doesn't recognize; `raw` (on the
    /// containing `Stmt`) already holds the exact text.
    Raw,
}

#[derive(Debug, Clone, Serialize)]
pub enum ForInit {
    Decl(DeclStmt),
    Expr(Expr),
}

/// One `#if`/`#ifdef`/`#ifndef` branch (or a following `#elif`) at statement
/// position - mirrors `ast::CondBranch` one layer down (`Block` instead of
/// `Vec<(Item, Trivia)>` for the body).
#[derive(Debug, Clone, Serialize)]
pub struct StmtCondBranch {
    pub directive: Directive,
    pub body: Block,
}

/// A folded `#if...#endif` run at statement position - mirrors
/// `ast::CondGroup` one layer down. `branches[0]` is the opening
/// `#if`/`#ifdef`/`#ifndef`; further entries are `#elif`s. `else_body` is
/// the `#else` body, if present.
#[derive(Debug, Clone, Serialize)]
pub struct StmtCondGroup {
    pub branches: Vec<StmtCondBranch>,
    pub else_body: Option<Block>,
    /// Which branch (if any) would actually compile - reuses
    /// `ast::ActiveBranch`, `Unknown` until `stmt::cond::resolve_conditionals`
    /// is run with a `#define` environment. See `ast::CondGroup::active`.
    pub active: super::super::ast::ActiveBranch,
}

/// A function body: the structured `block` alongside its own independently-
/// computed `raw` (exactly as `classify_group_unit` builds it today,
/// unchanged) rather than deriving one from the other. `File::render()`
/// never reads `FnBody` at all - file-level round-trip stays guaranteed by
/// `Item.raw` regardless of `block`'s correctness. Keeping both gives a free
/// corpus-wide self-check instead: `render_block(&block) == raw` should
/// hold for every function; a mismatch is a pure bug signal in this parser,
/// never a threat to `File::render()`.
#[derive(Debug, Clone, Serialize)]
pub struct FnBody {
    pub block: Block,
    pub raw: String,
}
