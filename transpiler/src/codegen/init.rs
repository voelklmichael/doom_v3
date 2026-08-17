//! `ast::Init` -> Rust initializer-expression text. `Init::Expr(String)` is
//! raw, unparsed scalar text (the front-end's own doc comment: "no operator
//! grammar... kept as raw text") - like `codegen::macros`, this module
//! parses it lazily at codegen time, reusing the same `scan::scan` ->
//! `stmt::lex::lex_ctoks` -> `stmt::expr::parse_expr` pipeline, then renders
//! via `codegen::expr::render_expr`.
//!
//! Unlike a macro, a `VarDecl`'s type is never inferred - it's read directly
//! from `ast::VarDecl.ty` (or, for a struct field's initializer element in a
//! later phase, `ast::Field.ty`), so this module's job is narrower than
//! `codegen::macros`': render the scalar text, then apply a small set of
//! target-*type*-aware fixups where C's looser literal rules don't survive
//! a direct transliteration (currently just the bare-`0`-as-null-pointer
//! idiom - see `render_scalar_init`).
//!
//! This first phase only covers a scalar `Init::Expr` against a scalar
//! (non-`Array`, non-record) `Type` - `codegen::items::emit_var` is
//! responsible for routing only that combination here and leaving every
//! other `Init`/`Type` shape (arrays, struct/union tables, conditionals) on
//! the existing `zeroed()` stub until a later phase covers them.

use super::expr::render_expr;
use crate::parser::ast::Type;
use crate::parser::scan;
use crate::parser::stmt::expr::{Expr, KnownTypeNames, parse_expr};
use crate::parser::stmt::lex::lex_ctoks;

fn parse_init_expr(text: &str, known: &KnownTypeNames) -> Expr {
    let tokens = scan::scan(text);
    let ctoks = lex_ctoks(&tokens);
    parse_expr(&ctoks, known)
}

/// Renders a scalar `Init::Expr` against its declaration's already-known
/// scalar `Type`. `None` only when the parsed expression contains an
/// `Expr::Raw` leaf (see `render_expr`'s doc comment) - callers degrade to
/// the existing flagged `zeroed()` stub in that case, never emit broken
/// syntax.
pub fn render_scalar_init(text: &str, ty: &Type, known: &KnownTypeNames) -> Option<String> {
    let expr = parse_init_expr(text, known);
    let rendered = render_expr(&expr)?;
    Some(match (ty, &expr) {
        // A bare `0` used as a null pointer - C's untyped-null-constant
        // idiom has no Rust equivalent via a plain integer literal (`*mut T
        // = 0` doesn't compile; needs an explicit null-pointer
        // constructor). Real corpus cases: `i_video.c`'s `X_display`,
        // `s_sound.c`'s `mus_playing`.
        (Type::Pointer(_), Expr::IntLit(lit)) if lit.trim() == "0" => {
            "std::ptr::null_mut()".to_string()
        }
        _ => rendered,
    })
}

#[cfg(test)]
mod tests;
