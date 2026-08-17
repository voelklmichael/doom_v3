//! `stmt::expr::Expr` -> Rust expression text. Reused by `codegen::macros`
//! for macro bodies now, and intended to be reused again whenever real
//! function-body codegen happens (see the roadmap note in
//! `/home/michael/.claude/plans/refactored-dazzling-treehouse.md`).
//!
//! This is a syntactic transliteration, not a type-checked rewrite: this
//! codebase has no full C type system, so most operators map straight to
//! their Rust look-alike and a resulting type mismatch (e.g. `&&`/`||`
//! expecting `bool` where a `std::ffi::c_int` operand appears) is accepted
//! as a visible compile error - the same precedent `codegen::types`'
//! module doc already establishes. One operator gets special-cased instead
//! of a direct mapping: C's logical `!` (`!x` means `x == 0`, yielding an
//! int) has a Rust look-alike (`!`) that instead means *bitwise* NOT for an
//! integer operand - unlike a type-mismatch, that would silently compile to
//! the wrong value rather than fail loudly, so it's translated to its real
//! meaning explicitly (see `render_unary`).

use super::ident::ident;
use super::types::{map_type, sanitize_int_literal};
use crate::parser::stmt::expr::{AssignOp, BinaryOp, Expr, PostfixOp, SizeofArg, UnaryOp};

/// Renders `expr` as Rust expression text, or `None` if it (or any
/// subexpression) contains an `Expr::Raw` leaf - the grammar's own
/// genuine "couldn't parse this" fallback, which by construction has no
/// Rust equivalent to fall back to here either. `None` propagates outward
/// through any containing subtree; callers decide the final fallback
/// (flag the whole containing declaration, matching `codegen::items`'
/// `is_malformed`/`type_is_malformed` precedent).
pub fn render_expr(expr: &Expr) -> Option<String> {
    Some(match expr {
        Expr::Ident(name) => render_ident(name),
        Expr::IntLit(text) => sanitize_int_literal(text),
        Expr::FloatLit(text) => render_float_lit(text),
        // `text` already includes the surrounding quotes (see
        // `scan::scan`'s `StringLit`/`CharLit` handling) - prefixing `c`/`b`
        // turns a C string/char literal into a Rust C-string/byte literal
        // directly. Escape-sequence compatibility (e.g. a C octal escape,
        // not valid in a Rust literal) is an accepted gap, same class as
        // every other "best-effort, visible compile error if wrong" case
        // in this module.
        Expr::StrLit(text) => format!("(c{text}).as_ptr()"),
        // A C char literal's *type* is `int`, not `char` (integer
        // promotion applies even to the literal itself) - real corpus
        // proof: `am_map.h`'s `AM_MSGHEADER` is `('a'<<24)+('m'<<16)`,
        // shifting well past a byte's range, only well-defined because
        // `'a'` is really an `int`. A bare Rust `b'a'` (type `u8`) doesn't
        // compose with surrounding `std::ffi::c_int` arithmetic (shift
        // amounts, other macro consts), so it's cast right where it's
        // produced rather than left for every call site to remember.
        Expr::CharLit(text) => format!("(b{text} as std::ffi::c_int)"),
        Expr::Paren(inner) => format!("({})", render_expr(inner)?),
        Expr::Unary { op, expr } => render_unary(*op, expr)?,
        Expr::Postfix { op, expr } => render_postfix(*op, expr)?,
        Expr::Binary { op, lhs, rhs } => {
            format!(
                "({} {} {})",
                render_expr(lhs)?,
                binary_op_text(*op),
                render_expr(rhs)?
            )
        }
        Expr::Assign { op, lhs, rhs } => {
            format!(
                "{} {} {}",
                render_expr(lhs)?,
                assign_op_text(*op),
                render_expr(rhs)?
            )
        }
        Expr::Ternary {
            cond,
            then_expr,
            else_expr,
        } => format!(
            "(if ({}) != 0 {{ {} }} else {{ {} }})",
            render_expr(cond)?,
            render_expr(then_expr)?,
            render_expr(else_expr)?
        ),
        Expr::Comma(items) => {
            let mut parts = Vec::with_capacity(items.len());
            for e in items {
                parts.push(render_expr(e)?);
            }
            format!("{{ {} }}", parts.join("; "))
        }
        Expr::Call { callee, args } => {
            let callee = render_expr(callee)?;
            let mut rendered_args = Vec::with_capacity(args.len());
            for a in args {
                rendered_args.push(render_expr(a)?);
            }
            format!("{callee}({})", rendered_args.join(", "))
        }
        // Direct Rust indexing syntax - correct whenever `base` mapped to a
        // fixed-size array (the common case for corpus globals, via
        // `codegen::types::map_type`'s `Array` case); when `base` is
        // actually a raw pointer (`*mut T` doesn't implement `Index`), this
        // is the accepted-compile-error case, same as everywhere else here.
        Expr::Index { base, index } => {
            format!("{}[({}) as usize]", render_expr(base)?, render_expr(index)?)
        }
        Expr::Member { base, name } => format!("{}.{}", render_expr(base)?, ident(name)),
        // `base` is a raw pointer (this codebase's transliteration keeps
        // every C pointer a raw `*mut T`), so `->` becomes deref-then-field.
        Expr::Arrow { base, name } => format!("(*{}).{}", render_expr(base)?, ident(name)),
        Expr::Cast { ty, expr } => format!("(({}) as {})", render_expr(expr)?, map_type(ty)),
        Expr::Sizeof(SizeofArg::Type(ty)) => format!("std::mem::size_of::<{}>()", map_type(ty)),
        Expr::Sizeof(SizeofArg::Expr(e)) => {
            format!("std::mem::size_of_val(&({}))", render_expr(e)?)
        }
        Expr::Raw(_) => return None,
    })
}

/// `__FILE__`/`__LINE__` are C preprocessor builtins with no real `Expr`
/// representation upstream (they lex as plain identifiers) - map them to
/// their Rust macro equivalents here, the one place every macro body's
/// identifiers pass through.
fn render_ident(name: &str) -> String {
    match name {
        "__FILE__" => "file!()".to_string(),
        "__LINE__" => "line!()".to_string(),
        _ => ident(name),
    }
}

fn render_float_lit(text: &str) -> String {
    let mut t = text
        .trim()
        .trim_end_matches(['f', 'F', 'l', 'L'])
        .to_string();
    if let Some(rest) = t.strip_prefix('.') {
        t = format!("0.{rest}");
    } else if let Some(rest) = t.strip_prefix("-.") {
        t = format!("-0.{rest}");
    }
    t
}

fn render_unary(op: UnaryOp, expr: &Expr) -> Option<String> {
    let e = render_expr(expr)?;
    Some(match op {
        // See this module's doc comment - `!` is C logical-not (yields
        // 0/1), not Rust's bitwise-not look-alike.
        UnaryOp::Not => format!("((({e}) == 0) as std::ffi::c_int)"),
        UnaryOp::BitNot => format!("(!({e}))"),
        UnaryOp::Neg => format!("(-({e}))"),
        // C's unary `+` is a no-op; Rust has no unary `+` operator at all.
        UnaryOp::Plus => format!("({e})"),
        // A shared reference can't cast directly to `*mut _` (E0606, real
        // corpus example: g_game.c's `mousebuttons = &mousearray[1]`) -
        // must go through `*const _` first, matching how `map_type` already
        // always emits `*mut` for every C pointer.
        UnaryOp::AddrOf => format!("(&({e}) as *const _ as *mut _)"),
        UnaryOp::Deref => format!("(*({e}))"),
        UnaryOp::PreInc => format!("{{ {e} += 1; {e} }}"),
        UnaryOp::PreDec => format!("{{ {e} -= 1; {e} }}"),
    })
}

fn render_postfix(op: PostfixOp, expr: &Expr) -> Option<String> {
    let e = render_expr(expr)?;
    Some(match op {
        PostfixOp::PostInc => format!("{{ let __macro_tmp = {e}; {e} += 1; __macro_tmp }}"),
        PostfixOp::PostDec => format!("{{ let __macro_tmp = {e}; {e} -= 1; __macro_tmp }}"),
    })
}

fn binary_op_text(op: BinaryOp) -> &'static str {
    use BinaryOp::*;
    match op {
        Add => "+",
        Sub => "-",
        Mul => "*",
        Div => "/",
        Mod => "%",
        Shl => "<<",
        Shr => ">>",
        Lt => "<",
        Gt => ">",
        Le => "<=",
        Ge => ">=",
        EqEq => "==",
        NotEq => "!=",
        BitAnd => "&",
        BitXor => "^",
        BitOr => "|",
        LogAnd => "&&",
        LogOr => "||",
    }
}

fn assign_op_text(op: AssignOp) -> &'static str {
    use AssignOp::*;
    match op {
        Assign => "=",
        AddEq => "+=",
        SubEq => "-=",
        MulEq => "*=",
        DivEq => "/=",
        ModEq => "%=",
        AndEq => "&=",
        OrEq => "|=",
        XorEq => "^=",
        ShlEq => "<<=",
        ShrEq => ">>=",
    }
}

#[cfg(test)]
mod tests;
