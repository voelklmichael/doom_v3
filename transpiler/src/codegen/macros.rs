//! `#define` -> Rust `const`/`fn` emission. Object-like macros (no parameter
//! list) become `pub const`; function-like macros become `pub fn` (added in
//! a later phase - see the approved plan). Macro bodies are stored as plain
//! text by the front end (`parser::preproc::Directive` deliberately keeps
//! them unparsed - see that module's doc comment); this module parses them
//! lazily, at codegen time, reusing the exact same lex/parse pipeline
//! `parser::stmt::mod::parse_function_body` already uses for real function
//! bodies.

use super::expr::render_expr;
use super::ident::ident;
use crate::parser::scan;
use crate::parser::stmt::expr::{Expr, KnownTypeNames, parse_expr};
use crate::parser::stmt::lex::lex_ctoks;

/// Parses a macro's raw C text (a `DefineObject`'s value or - in a later
/// phase - a `DefineFunction`'s body) as one expression. Total: `parse_expr`
/// never panics, degrading anything it can't recognize to an `Expr::Raw`
/// leaf rather than failing - see that function's own doc comment.
fn parse_macro_expr(text: &str, known: &KnownTypeNames) -> Expr {
    let tokens = scan::scan(text);
    let ctoks = lex_ctoks(&tokens);
    parse_expr(&ctoks, known)
}

/// Infers a Rust type for an object-like macro's `const` from the shape of
/// its *parsed* value - macros carry no type information at all, so this is
/// necessarily a heuristic, matching every other "default to something
/// reasonable, prefer real structure when available" choice in this codegen
/// backend (e.g. `codegen::items::emit_enum`'s enum constants always being
/// `std::ffi::c_int`). Unwraps `Paren` first so e.g. `((int) (1.02*FRACUNIT))`
/// still sees its own `Cast` node. A `Cast`'s own explicit target type is the
/// most precise signal available and is used verbatim (this is what makes
/// `m_swap.h`-style `((short)...)`-shaped values type correctly); anything
/// else - including a bare char literal, which is `std::ffi::c_int`-typed in
/// real C too (see `render_expr`'s `CharLit` case) - defaults to
/// `std::ffi::c_int` (matches the enum-constant precedent).
fn infer_const_type(expr: &Expr) -> String {
    match unwrap_paren(expr) {
        Expr::FloatLit(_) => "std::ffi::c_double".to_string(),
        Expr::StrLit(_) => "*const std::ffi::c_char".to_string(),
        Expr::Cast { ty, .. } => super::types::map_type(ty),
        _ => "std::ffi::c_int".to_string(),
    }
}

fn unwrap_paren(expr: &Expr) -> &Expr {
    match expr {
        Expr::Paren(inner) => unwrap_paren(inner),
        other => other,
    }
}

/// Emits one object-like `#define` as a Rust `pub const`, or a flagged
/// comment when it can't be (see `render_expr`'s doc comment for what
/// "can't" means here - only an `Expr::Raw` leaf, this grammar's own
/// genuine parse-failure marker). An empty value (a flag-only macro used
/// purely for `#ifdef`/`#if` testing, e.g. `RANGECHECK`/`NORMALUNIX` - their
/// conditionals are already resolved elsewhere, see `parser::cond`) has
/// nothing to constantify and is skipped entirely, same as `#include`
/// already is in `codegen::items::emit_item`.
pub fn emit_define_object(name: &str, value: &str, known: &KnownTypeNames) -> String {
    if value.trim().is_empty() {
        return String::new();
    }
    let expr = parse_macro_expr(value, known);
    match render_expr(&expr) {
        Some(rendered) => {
            let ty = infer_const_type(&expr);
            format!("pub const {}: {ty} = {rendered};\n\n", ident(name))
        }
        None => format!(
            "// TODO: unparsed macro value, needs manual translation: #define {name} {value}\n\n"
        ),
    }
}

#[cfg(test)]
mod tests;
