//! `#define` -> Rust `const`/`fn` emission. Object-like macros (no parameter
//! list) become `pub const`; function-like macros become `pub fn`. Macro
//! bodies are stored as plain text by the front end (`parser::preproc::
//! Directive` deliberately keeps them unparsed - see that module's doc
//! comment); this module parses them lazily, at codegen time, reusing the
//! exact same lex/parse pipeline `parser::stmt::mod::parse_function_body`
//! already uses for real function bodies.

use super::expr::{is_float_expr, is_sizeof_shaped, render_expr};
use super::ident::ident;
use crate::parser::ast::{FnSig, Type};
use crate::parser::scan;
use crate::parser::stmt::expr::{Expr, KnownTypeNames, UnaryOp, parse_expr};
use crate::parser::stmt::lex::{CTok, Punct, lex_ctoks};
use std::collections::HashMap;

/// Joins a C line-continuation (`\` immediately followed by a newline,
/// possibly with a trailing `\r`) by deleting both bytes - exactly what a
/// real C preprocessor does before anything else ever sees the text. Only
/// macro bodies can contain one outside a string literal in this corpus
/// (`preproc::parse_define` keeps a multi-line `#define`'s continuation
/// bytes verbatim in `value`/`body` - see that module's doc comment); a
/// continuation *inside* a string literal (e.g. `d_englsh.h`'s long dialogue
/// strings) is already handled correctly by `scan::scan`'s string-literal
/// tokenizer treating `\<newline>` as an ordinary escape pair, so this only
/// needs to run over the macro's raw text once, up front, before either
/// lexer sees it - a bare `\` byte outside a string has no other meaning in
/// this grammar and would otherwise lex as an unrecognized token, degrading
/// the whole macro to `Expr::Raw` (confirmed via `i_net.c`'s real `ntohl`/
/// `ntohs`, both continued across 4-5 physical lines).
fn join_line_continuations(text: &str) -> String {
    text.replace("\\\r\n", "").replace("\\\n", "")
}

/// Lexes a macro's raw C text (a `DefineObject`'s value or a
/// `DefineFunction`'s body) into `CTok`s - the shared first step before
/// either `parse_expr`-ing it directly or checking `is_single_expression_body`.
fn lex_macro_text(text: &str) -> Vec<CTok> {
    let tokens = scan::scan(&join_line_continuations(text));
    lex_ctoks(&tokens)
}

/// Infers a Rust type for a macro's value/return from the shape of its
/// *parsed* expression - macros carry no type information at all, so this is
/// necessarily a heuristic, matching every other "default to something
/// reasonable, prefer real structure when available" choice in this codegen
/// backend (e.g. `codegen::items::emit_enum`'s enum constants always being
/// `std::ffi::c_int`). Unwraps `Paren` first so e.g. `((int) (1.02*FRACUNIT))`
/// still sees its own `Cast` node. A `Cast`'s own explicit target type is the
/// most precise signal available and is used verbatim (this is what makes
/// `m_swap.h`-style `((short)...)`-shaped values - and, once wired into
/// `emit_define_function`, `SHORT`/`LONG`-shaped function-like macros - type
/// correctly); a bare `Binary` combination involving a float literal (real
/// corpus case: `am_map.c`'s `INITSCALEMTOF`, `(.2*FRACUNIT)` - genuinely
/// `double`-typed in C via its own usual-arithmetic-conversion rule, with no
/// top-level `Cast`/`FloatLit` of its own for this function to see directly)
/// is caught via `is_float_expr`; anything else - including a bare char
/// literal, which is `std::ffi::c_int`-typed in real C too (see
/// `render_expr`'s `CharLit` case) - defaults to `std::ffi::c_int` (matches
/// the enum-constant precedent).
fn infer_scalar_type(expr: &Expr) -> String {
    let inner = unwrap_paren(expr);
    match inner {
        Expr::FloatLit(_) => "std::ffi::c_double".to_string(),
        Expr::StrLit(_) => "*const std::ffi::c_char".to_string(),
        Expr::Cast { ty, .. } => super::types::map_type(ty),
        _ if is_float_expr(inner) => "std::ffi::c_double".to_string(),
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
/// genuine parse-failure marker, or an identifier the macro's body
/// references that has no known definition anywhere the emitting module can
/// see - see `has_unresolved_ident`). An empty value (a flag-only macro used
/// purely for `#ifdef`/`#if` testing, e.g. `RANGECHECK`/`NORMALUNIX` - their
/// conditionals are already resolved elsewhere, see `parser::cond`) has
/// nothing to constantify and is skipped entirely, same as `#include`
/// already is in `codegen::items::emit_item`.
pub fn emit_define_object(
    name: &str,
    value: &str,
    known: &KnownTypeNames,
    known_globals: &HashMap<String, Type>,
    known_functions: &HashMap<String, FnSig>,
    known_defines: &HashMap<String, String>,
) -> String {
    if value.trim().is_empty() {
        return String::new();
    }
    let expr = parse_expr(&lex_macro_text(value), known);
    if has_unresolved_ident(&expr, known_globals, known_functions, known_defines) {
        // Real corpus cases (`s_sound.c`'s `NORM_VOLUME`, `st_stuff.c`'s
        // `ST_MAPTITLEX`, `wi_stuff.c`'s `SP_PAR`): a macro whose body
        // references a genuinely undefined identifier - confirmed via a
        // corpus-wide grep that none of these three macros are ever
        // expanded anywhere. Real C never type/scope-checks an unused
        // macro body (pure textual substitution), so the original source
        // compiled fine despite this; unconditionally emitting a `const`
        // for every `#define` regardless of use would force Rust to
        // resolve identifiers C itself never needed to.
        return format!(
            "/* TODO: unparsed macro value, references an identifier with no known definition anywhere in this module's visible corpus (likely dead code never expanded in the original C):\n#define {name} {value}\n*/\n\n"
        );
    }
    match render_expr(&expr, known_globals) {
        Some(rendered) => {
            let ty = infer_scalar_type(&expr);
            // `sizeof`/`sizeof_val` always render as `usize` (see
            // `render_expr`'s `Sizeof` case), but a bare `sizeof(a)/
            // sizeof(b)`-shaped macro with no explicit cast (real corpus
            // case: `am_map.c`'s `NUMPLYRLINES`) infers - correctly, since
            // real C consumes it as a plain int count - as
            // `std::ffi::c_int` here, leaving a `usize`-vs-`i32` mismatch at
            // the assignment itself. A macro whose own `Cast` already named
            // a different target type renders through `Expr::Cast`'s own
            // wrapping instead (see `render_expr`), so this only ever needs
            // to fire for the no-cast, defaulted-to-`c_int` case.
            let rendered = if ty == "std::ffi::c_int" && is_sizeof_shaped(&expr) {
                format!("({rendered}) as std::ffi::c_int")
            } else {
                rendered
            };
            format!("pub const {}: {ty} = {rendered};\n\n", ident(name))
        }
        // Block comment, not `//` - `value` can be multi-line (a real
        // C line-continuation not fully joined, or just an embedded
        // newline), and every physical line after the first would
        // otherwise fall outside a `//` comment's reach and leak as raw,
        // almost-certainly-invalid Rust source (matches `emit_raw`'s own
        // block-comment choice for the same reason).
        None => format!(
            "/* TODO: unparsed macro value, needs manual translation:\n#define {name} {value}\n*/\n\n"
        ),
    }
}

/// True if `ctoks` (a function-like macro's body) is shaped like a single C
/// `expression` - no top-level (outside any `()`/`[]`/`{}` nesting)
/// semicolon, and it doesn't open with a `{` block. Real corpus split: 15 of
/// 17 function-like macros are single expressions (including ones that
/// *read* statement-like at call sites, e.g. `am_map.c`'s `PUTDOT(x,y,c)` -
/// its own body `fb[(yy)*f_w+(xx)]=(cc)` is one assignment *expression*, no
/// semicolon of its own; the caller supplies the `;`). Only `am_map.c`'s
/// `DOOUTCODE` (an if/else-if chain) and `z_zone.h`'s `Z_ChangeTag` (an
/// explicit `{ if (...) ...; ...; }` block) are genuinely statement-shaped -
/// out of scope until real statement codegen exists (`emit_define_function`
/// degrades those two to a flagged comment rather than mis-parsing just
/// their leading expression and silently dropping the rest).
fn is_single_expression_body(ctoks: &[CTok]) -> bool {
    let mut depth: i32 = 0;
    for (i, tok) in ctoks.iter().filter(|t| !t.is_trivial()).enumerate() {
        if i == 0 && matches!(tok, CTok::Punct(Punct::LBrace, _)) {
            return false;
        }
        match tok {
            CTok::Punct(Punct::LParen | Punct::LBracket | Punct::LBrace, _) => depth += 1,
            CTok::Punct(Punct::RParen | Punct::RBracket | Punct::RBrace, _) => depth -= 1,
            CTok::Punct(Punct::Semicolon, _) if depth <= 0 => return false,
            _ => {}
        }
    }
    true
}

/// Visits `expr` and every subexpression, true if any satisfies `pred`.
fn any_subexpr(expr: &Expr, pred: &dyn Fn(&Expr) -> bool) -> bool {
    if pred(expr) {
        return true;
    }
    match expr {
        Expr::Paren(e) | Expr::Unary { expr: e, .. } | Expr::Postfix { expr: e, .. } => {
            any_subexpr(e, pred)
        }
        Expr::Binary { lhs, rhs, .. } | Expr::Assign { lhs, rhs, .. } => {
            any_subexpr(lhs, pred) || any_subexpr(rhs, pred)
        }
        Expr::Ternary {
            cond,
            then_expr,
            else_expr,
        } => {
            any_subexpr(cond, pred) || any_subexpr(then_expr, pred) || any_subexpr(else_expr, pred)
        }
        Expr::Comma(items) => items.iter().any(|e| any_subexpr(e, pred)),
        Expr::Call { callee, args } => {
            any_subexpr(callee, pred) || args.iter().any(|e| any_subexpr(e, pred))
        }
        Expr::Index { base, index } => any_subexpr(base, pred) || any_subexpr(index, pred),
        Expr::Member { base, .. } | Expr::Arrow { base, .. } => any_subexpr(base, pred),
        Expr::Cast { expr: e, .. } => any_subexpr(e, pred),
        Expr::Sizeof(crate::parser::stmt::expr::SizeofArg::Expr(e)) => any_subexpr(e, pred),
        Expr::Sizeof(crate::parser::stmt::expr::SizeofArg::Type(_))
        | Expr::Ident(_)
        | Expr::IntLit(_)
        | Expr::FloatLit(_)
        | Expr::StrLit(_)
        | Expr::CharLit(_)
        | Expr::Raw(_) => false,
    }
}

/// True if `expr` (a parsed, successfully-*rendered* object-macro body)
/// references any bare identifier not resolvable via the emitting module's
/// own visible environment - a real global variable, a real function, or
/// another `#define`d macro (the three ways an object macro's `render_expr`
/// output can actually name-resolve once Rust's own glob imports run at the
/// real `cargo build`). Deliberately does *not* attempt to also recognize
/// enum-variant names (no corpus-wide harvester for those exists) - if that
/// ever produces a false positive, the real `--emit-rust` + `cargo build
/// -p doom_rs` run is what would surface it (same discipline as everywhere
/// else in this codegen backend), not a theoretical soundness argument.
fn has_unresolved_ident(
    expr: &Expr,
    known_globals: &HashMap<String, Type>,
    known_functions: &HashMap<String, FnSig>,
    known_defines: &HashMap<String, String>,
) -> bool {
    any_subexpr(expr, &|e| match e {
        Expr::Ident(name) => {
            !known_globals.contains_key(name)
                && !known_functions.contains_key(name)
                && !known_defines.contains_key(name)
        }
        _ => false,
    })
}

fn expr_is_param(expr: &Expr, param: &str) -> bool {
    matches!(unwrap_paren(expr), Expr::Ident(name) if name == param)
}

/// A macro param's real C type is never known (macros carry no type info at
/// all) - defaults to `std::ffi::c_int`, *except* when `body` shows the
/// param used somewhere that only makes sense for a pointer (cast to a
/// pointer/function-pointer type, `->`-dereferenced, or `*`-dereferenced).
/// The one real corpus case this matters for: `z_zone.h`'s `Z_ChangeTag`'s
/// `p`, cast to `memblock_t *` and `byte *`.
fn is_pointer_param(body: &Expr, param: &str) -> bool {
    any_subexpr(body, &|e| match e {
        Expr::Cast { ty, expr } => {
            matches!(ty, Type::Pointer(_) | Type::FunctionPointer { .. })
                && expr_is_param(expr, param)
        }
        Expr::Arrow { base, .. } => expr_is_param(base, param),
        Expr::Unary {
            op: UnaryOp::Deref,
            expr,
        } => expr_is_param(expr, param),
        _ => false,
    })
}

/// True if `param` is ever assigned to (or incremented/decremented) inside
/// `body` - needs `mut` on the Rust parameter binding for that to compile at
/// all (real corpus cases: `am_map.c`'s `PUTDOT`... actually never reassigns
/// its own params, but `DOOUTCODE`/`Z_ChangeTag`'s statement-shaped bodies
/// would if they were ever wired in here - kept general rather than
/// speculatively narrowed, since it's cheap and correct for any future
/// macro with this shape too).
fn is_mut_param(body: &Expr, param: &str) -> bool {
    any_subexpr(body, &|e| match e {
        Expr::Assign { lhs, .. } => expr_is_param(lhs, param),
        Expr::Unary {
            op: UnaryOp::PreInc | UnaryOp::PreDec,
            expr,
        } => expr_is_param(expr, param),
        Expr::Postfix { expr, .. } => expr_is_param(expr, param),
        _ => false,
    })
}

/// Emits one function-like `#define` as a Rust `pub unsafe extern "C" fn`
/// (matching `codegen::items::emit_function_def`'s convention for every
/// other real function), or a flagged comment when its body isn't a single
/// expression (see `is_single_expression_body`) or isn't renderable (an
/// `Expr::Raw` leaf somewhere in it). The corpus has zero variadic macros
/// (confirmed by census), so `params` is always a fixed arity.
pub fn emit_define_function(
    name: &str,
    params: &[String],
    body: &str,
    known: &KnownTypeNames,
    known_globals: &HashMap<String, Type>,
) -> String {
    let ctoks = lex_macro_text(body);
    // Block comments, not `//` - see `emit_define_object`'s matching note;
    // `body` can be genuinely multi-line (e.g. `am_map.c`'s real `DOOUTCODE`).
    if !is_single_expression_body(&ctoks) {
        return format!(
            "/* TODO: statement-shaped macro body, needs manual translation:\n#define {name}(...) {body}\n*/\n\n"
        );
    }
    let expr = parse_expr(&ctoks, known);
    let Some(rendered) = render_expr(&expr, known_globals) else {
        return format!(
            "/* TODO: unparsed macro body, needs manual translation:\n#define {name}(...) {body}\n*/\n\n"
        );
    };
    let ret_ty = infer_scalar_type(&expr);
    let params_text = params
        .iter()
        .map(|p| {
            let mutkw = if is_mut_param(&expr, p) { "mut " } else { "" };
            let ty = if is_pointer_param(&expr, p) {
                "*mut std::ffi::c_void"
            } else {
                "std::ffi::c_int"
            };
            format!("{mutkw}{}: {ty}", ident(p))
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "pub unsafe extern \"C\" fn {}({params_text}) -> {ret_ty} {{ {rendered} }}\n\n",
        ident(name)
    )
}

#[cfg(test)]
mod tests;
