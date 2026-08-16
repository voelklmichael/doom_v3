//! Step 7: statement/expression parsing for function bodies. `lex.rs`
//! tokenizes, `expr.rs` parses expressions, `decl.rs` parses local
//! declarations, `parse.rs` drives the statement-level recursive descent,
//! `cond.rs` folds mid-body `#if`/`#ifdef`/.../`#endif` runs into a tree,
//! and `ast.rs` holds the resulting `Block`/`Stmt`/`FnBody` shapes.
//! `parse_function_body` (below) is the single entry point `record.rs`
//! calls to turn a function's opaque body tokens into a structured
//! `FnBody`. `scope.rs` is a separate, not-yet-wired-in piece: scoped
//! identifier resolution over an already-parsed `Block`, laid groundwork
//! for call-site analysis (see `plans/humming-knitting-simon.md`).

pub mod ast;
pub mod cond;
pub mod decl;
pub mod expr;
pub mod lex;
pub mod parse;
pub mod scope;

use super::ast::RawToken;
use ast::FnBody;
use expr::KnownTypeNames;

/// Parses one function's body. `inner` is the body's tokens (excluding the
/// surrounding `{`/`}`, same as `ItemKind::FunctionDef`'s previous opaque
/// payload was built from); `raw` is the exact original text *including*
/// the braces (computed exactly as before, independently of `block` - see
/// `FnBody`'s doc comment for why round-trip safety never depends on
/// `block` being correct).
pub fn parse_function_body(inner: Vec<RawToken>, raw: String, known: &KnownTypeNames) -> FnBody {
    let block = cond::fold_conditionals(parse::parse_block(inner, known));
    FnBody { block, raw }
}
