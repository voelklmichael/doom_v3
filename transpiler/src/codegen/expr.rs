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
use crate::parser::ast::Type;
use crate::parser::stmt::expr::{AssignOp, BinaryOp, Expr, PostfixOp, SizeofArg, UnaryOp};
use std::collections::HashMap;

/// Renders `expr` as Rust expression text, or `None` if it (or any
/// subexpression) contains an `Expr::Raw` leaf - the grammar's own
/// genuine "couldn't parse this" fallback, which by construction has no
/// Rust equivalent to fall back to here either. `None` propagates outward
/// through any containing subtree; callers decide the final fallback
/// (flag the whole containing declaration, matching `codegen::items`'
/// `is_malformed`/`type_is_malformed` precedent). `known_globals` (a global
/// var name -> its declared `Type`) is only consulted by `Expr::Index` (see
/// there) - every other case ignores it, threaded through purely so a
/// nested `Index` anywhere in the tree can still reach it.
pub fn render_expr(expr: &Expr, known_globals: &HashMap<String, Type>) -> Option<String> {
    Some(match expr {
        Expr::Ident(name) => render_ident(name),
        Expr::IntLit(text) => sanitize_int_literal(text),
        Expr::FloatLit(text) => render_float_lit(text),
        // `text` already includes the surrounding quotes (see
        // `scan::scan`'s `StringLit`/`CharLit` handling) - prefixing `c`
        // turns a C string literal into a Rust C-string literal directly.
        // Escape-sequence compatibility (e.g. a C octal escape, not valid in
        // a Rust literal) is an accepted gap, same class as every other
        // "best-effort, visible compile error if wrong" case in this
        // module - *except* an embedded NUL escape (`\0`), which Rust's
        // `c"..."` syntax rejects outright as a hard parse error (a C
        // string's terminator is always implicit, never spelled) rather
        // than merely producing a wrong value - see `render_str_lit`.
        Expr::StrLit(text) => render_str_lit(text)?,
        // A C char literal's *type* is `int`, not `char` (integer
        // promotion applies even to the literal itself) - real corpus
        // proof: `am_map.h`'s `AM_MSGHEADER` is `('a'<<24)+('m'<<16)`,
        // shifting well past a byte's range, only well-defined because
        // `'a'` is really an `int`. A bare Rust `b'a'` (type `u8`) doesn't
        // compose with surrounding `std::ffi::c_int` arithmetic (shift
        // amounts, other macro consts), so it's cast right where it's
        // produced rather than left for every call site to remember.
        Expr::CharLit(text) => format!("(b{text} as std::ffi::c_int)"),
        Expr::Paren(inner) => format!("({})", render_expr(inner, known_globals)?),
        Expr::Unary { op, expr } => render_unary(*op, expr, known_globals)?,
        Expr::Postfix { op, expr } => render_postfix(*op, expr, known_globals)?,
        Expr::Binary { op, lhs, rhs } => {
            let lhs_text = render_expr(lhs, known_globals)?;
            let rhs_text = render_expr(rhs, known_globals)?;
            // C's "usual arithmetic conversions": mixing a floating-point
            // operand with an integer one implicitly promotes the integer
            // side to floating-point before the operation - real corpus
            // cases: `am_map.c`'s `INITSCALEMTOF` (`.2*FRACUNIT`) and its
            // `mline_t` scaling-factor tables (`-0.867*R`, `0.5*R`, ...).
            // Rust has no implicit numeric coercion at all (`{float} *
            // i32` doesn't compile), so whichever side isn't float-shaped
            // (see `is_float_expr`) needs an explicit widening cast. `f64`
            // matches every real corpus float literal (none carry an
            // `f`/`F` suffix, so they'd default to `f64` anyway - this only
            // needs to widen the *other*, otherwise-int-inferred side to
            // match it).
            let lhs_float = is_float_expr(lhs);
            let rhs_float = is_float_expr(rhs);
            let lhs_text = if rhs_float && !lhs_float {
                format!("(({lhs_text}) as f64)")
            } else {
                lhs_text
            };
            let rhs_text = if lhs_float && !rhs_float {
                format!("(({rhs_text}) as f64)")
            } else {
                rhs_text
            };
            format!("({lhs_text} {} {rhs_text})", binary_op_text(*op))
        }
        Expr::Assign { op, lhs, rhs } => {
            format!(
                "{} {} {}",
                render_expr(lhs, known_globals)?,
                assign_op_text(*op),
                render_expr(rhs, known_globals)?
            )
        }
        Expr::Ternary {
            cond,
            then_expr,
            else_expr,
        } => format!(
            "(if ({}) != 0 {{ {} }} else {{ {} }})",
            render_expr(cond, known_globals)?,
            render_expr(then_expr, known_globals)?,
            render_expr(else_expr, known_globals)?
        ),
        Expr::Comma(items) => {
            let mut parts = Vec::with_capacity(items.len());
            for e in items {
                parts.push(render_expr(e, known_globals)?);
            }
            format!("{{ {} }}", parts.join("; "))
        }
        Expr::Call { callee, args } => {
            let callee = render_expr(callee, known_globals)?;
            let mut rendered_args = Vec::with_capacity(args.len());
            for a in args {
                rendered_args.push(render_expr(a, known_globals)?);
            }
            format!("{callee}({})", rendered_args.join(", "))
        }
        // Direct Rust indexing syntax whenever `base` mapped to a
        // fixed-size array (the common case for corpus globals, via
        // `codegen::types::map_type`'s `Array` case). When `base` is a bare
        // identifier known (via `known_globals`) to be an *unsized* array
        // (`Type::Array(_, None)`, which `map_type` always maps to a raw
        // pointer, never a real Rust array/slice - real corpus case:
        // `m_misc.c`'s `extern char* chat_macros[];`, whose real sized
        // definition lives in a *different* module, `hu_stuff.c` - see
        // `codegen::module`'s own documented "extern declarations never
        // reconcile types across modules" gap) - `[]` doesn't compile
        // (`*mut T` isn't `Index`-able) - pointer-arithmetic indexing
        // (`*base.add(i)`, a dereferenced place expression) is the real
        // Rust equivalent, and composes correctly both as a plain value
        // read and under `&`/`AddrOf` (`&*base.add(i)` is exactly "address
        // of the i-th element", matching a real corpus case:
        // `m_misc.c`'s `&chat_macros[i]`). Any other `base` shape (a known
        // *sized* array, an unresolvable expression, or simply not found in
        // `known_globals`) keeps the ordinary subscript syntax unchanged -
        // the safe default this whole codebase already uses elsewhere.
        Expr::Index { base, index } => {
            let base_text = render_expr(base, known_globals)?;
            let index_text = render_expr(index, known_globals)?;
            if let Expr::Ident(name) = base.as_ref()
                && matches!(known_globals.get(name.as_str()), Some(Type::Array(_, None)))
            {
                format!("(*{base_text}.add(({index_text}) as usize))")
            } else {
                format!("{base_text}[({index_text}) as usize]")
            }
        }
        Expr::Member { base, name } => {
            format!("{}.{}", render_expr(base, known_globals)?, ident(name))
        }
        // `base` is a raw pointer (this codebase's transliteration keeps
        // every C pointer a raw `*mut T`), so `->` becomes deref-then-field.
        Expr::Arrow { base, name } => {
            format!("(*{}).{}", render_expr(base, known_globals)?, ident(name))
        }
        Expr::Cast { ty, expr } => {
            format!(
                "(({}) as {})",
                render_expr(expr, known_globals)?,
                map_type(ty)
            )
        }
        Expr::Sizeof(SizeofArg::Type(ty)) => format!("std::mem::size_of::<{}>()", map_type(ty)),
        Expr::Sizeof(SizeofArg::Expr(e)) => {
            format!(
                "std::mem::size_of_val(&({}))",
                render_expr(e, known_globals)?
            )
        }
        Expr::Raw(_) => return None,
    })
}

/// Whether `expr` is C-floating-point-shaped - not a real type-checker, just
/// enough to recognize the real corpus shapes (a float literal, an
/// arithmetic combination where either side is float per C's own promotion
/// rule, or an explicit cast to `float`/`double`) so `render_expr`'s
/// `Binary` case and `codegen::macros::infer_scalar_type` can each decide
/// where an int/float mismatch needs an explicit cast. Used both here and
/// from `codegen::macros` (a macro whose value is a bare `Binary` like
/// `.2*FRACUNIT` is genuinely `double`-typed in C - there's no top-level
/// `Cast`/`FloatLit` for `infer_scalar_type`'s own shallower check to see).
pub(crate) fn is_float_expr(expr: &Expr) -> bool {
    match expr {
        Expr::FloatLit(_) => true,
        Expr::Paren(inner) => is_float_expr(inner),
        Expr::Unary { expr, .. } => is_float_expr(expr),
        Expr::Binary { lhs, rhs, .. } => is_float_expr(lhs) || is_float_expr(rhs),
        Expr::Cast { ty, .. } => {
            matches!(
                map_type(ty).as_str(),
                "std::ffi::c_float" | "std::ffi::c_double"
            )
        }
        _ => false,
    }
}

/// Whether `expr` involves a C `sizeof` anywhere reachable through `Paren`/
/// `Unary`/`Binary` wrapping - not a real type-checker, just enough to
/// recognize the real corpus shape (`am_map.c`'s `NUMPLYRLINES`-style
/// macros: `sizeof(player_arrow)/sizeof(mline_t)`, no explicit cast) so
/// `codegen::macros::emit_define_object` can add the truncating cast a
/// `usize`-producing `std::mem::size_of[_val]` call needs when the macro's
/// own inferred type defaults to `std::ffi::c_int` (real C consumes these
/// purely as plain int counts - `Expr::Sizeof` itself has no Rust-side
/// `Cast` to signal that, unlike the `is_float_expr` case above).
pub(crate) fn is_sizeof_shaped(expr: &Expr) -> bool {
    match expr {
        Expr::Sizeof(_) => true,
        Expr::Paren(inner) => is_sizeof_shaped(inner),
        Expr::Unary { expr, .. } => is_sizeof_shaped(expr),
        Expr::Binary { lhs, rhs, .. } => is_sizeof_shaped(lhs) || is_sizeof_shaped(rhs),
        _ => false,
    }
}

/// Renders a C string literal (`text` includes its surrounding quotes) as
/// `(c"...").as_ptr()`, with one target-*content*-aware fixup: a literal
/// whose unescaped bytes are *entirely* NUL (C's own explicit "empty string"
/// idiom, `"\0"` - real corpus case: `p_switch.c`'s `alphSwitchList[]`
/// sentinel row `{"\0","\0",0}`) becomes an empty `c""` literal instead -
/// byte-for-byte different from C's `"\0"` (which also carries its own
/// *implicit* terminator, two NUL bytes total) but identical as a
/// NUL-terminated string (`strlen`/`strcmp`, which is the only way this
/// corpus ever reads these fields, see the value as C's own terminator
/// convention already treats both as "the empty string"). A NUL escape
/// anywhere *else* in a literal (never confirmed to occur in this corpus)
/// has no safe translation - Rust's `c"..."` syntax rejects it outright, and
/// there's no other fixed-length literal syntax in expression position - so
/// this bails (`None`) rather than guess or silently truncate.
fn render_str_lit(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if let Some(inner) = trimmed.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        let bytes = unescape_c_string(inner);
        if !bytes.is_empty() && bytes.iter().all(|b| *b == 0) {
            return Some("(c\"\").as_ptr()".to_string());
        }
        if bytes.contains(&0) {
            return None;
        }
    }
    Some(format!("(c{text}).as_ptr()"))
}

/// Unescapes a C string/char literal's inner text (quotes already stripped)
/// into its real bytes: `\n`/`\t`/`\r`/`\0`/`\\`/`\"`/`\'` map to their real
/// byte value, anything else passes through as literal text (best-effort,
/// never panics - keeps the backslash rather than silently dropping it).
/// Shared by `render_str_lit` above and `codegen::init`'s
/// char-array-from-string-literal rendering (the `rcsid[]` idiom).
pub(crate) fn unescape_c_string(inner: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            let mut buf = [0u8; 4];
            bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            continue;
        }
        match chars.next() {
            Some('n') => bytes.push(b'\n'),
            Some('t') => bytes.push(b'\t'),
            Some('r') => bytes.push(b'\r'),
            Some('0') => bytes.push(0),
            Some('\\') => bytes.push(b'\\'),
            Some('"') => bytes.push(b'"'),
            Some('\'') => bytes.push(b'\''),
            // Best-effort passthrough for any other escape (none exist in
            // this corpus's real occurrences of either caller's shape) -
            // keep both bytes rather than silently drop the backslash.
            Some(other) => {
                bytes.push(b'\\');
                let mut buf = [0u8; 4];
                bytes.extend_from_slice(other.encode_utf8(&mut buf).as_bytes());
            }
            None => bytes.push(b'\\'),
        }
    }
    bytes
}

/// `__FILE__`/`__LINE__` are C preprocessor builtins with no real `Expr`
/// representation upstream (they lex as plain identifiers) - map them to
/// their Rust macro equivalents here, the one place every macro body's
/// identifiers pass through.
fn render_ident(name: &str) -> String {
    match name {
        "__FILE__" => "file!()".to_string(),
        "__LINE__" => "line!()".to_string(),
        _ => super::system_names::system_value(name)
            .or_else(|| super::system_names::system_function(name))
            .map(str::to_string)
            .unwrap_or_else(|| ident(name)),
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

fn render_unary(op: UnaryOp, expr: &Expr, known_globals: &HashMap<String, Type>) -> Option<String> {
    // `&arr[i]` where `arr` is a known unsized-array identifier (real
    // corpus case: `m_misc.c`'s `&chat_macros[i]`) - `Expr::Index` already
    // renders this shape as a dereferenced place expression
    // (`*arr.add(i)`), so the generic `AddrOf` handling below would produce
    // `&(*arr.add(i)) as *const _ as *mut _` (deref, then re-reference,
    // then a two-step cast) - confirmed via the actual `--emit-rust` +
    // build run to hit a real rustc type-inference limitation (`E0641
    // cannot cast to a pointer of an unknown kind`) that a plain
    // identifier's `AddrOf` never hits. `arr.add(i)` alone already has
    // exactly the needed `*mut T` type - no deref, no re-reference, no cast
    // round-trip needed at all, and it's the more direct rendering anyway
    // (`&*p` round-trips back to `p` for any raw pointer `p`).
    if op == UnaryOp::AddrOf
        && let Expr::Index { base, index } = expr
        && let Expr::Ident(name) = base.as_ref()
        && matches!(known_globals.get(name.as_str()), Some(Type::Array(_, None)))
    {
        let base_text = render_expr(base, known_globals)?;
        let index_text = render_expr(index, known_globals)?;
        return Some(format!("{base_text}.add(({index_text}) as usize)"));
    }
    let e = render_expr(expr, known_globals)?;
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
        // always emits `*mut` for every C pointer. The placeholder `_`
        // normally infers fine from the surrounding context, but real
        // corpus proof it sometimes can't: `m_misc.c`'s `defaults[]` config
        // table has dozens of `location: &(SOMEVAR)`-shaped rows sharing one
        // generic pointer-typed struct field, and once enough neighboring
        // rows in the *same* array literal exercise other inference paths
        // (confirmed via the actual `--emit-rust` + build run), a handful of
        // otherwise-unrelated plain-identifier rows (`sndserver_filename`,
        // `mousedev`, `mousetype`) started failing with `E0641 cannot cast
        // to a pointer of an unknown kind` - spelling out the identifier's
        // own already-known declared type (via `known_globals`) instead of
        // `_`, when available, sidesteps the inference dependency entirely;
        // falls back to the placeholder unchanged when the identifier isn't
        // a known global (a local, a param, or an expression more complex
        // than a bare identifier).
        UnaryOp::AddrOf => {
            let pointee = match expr {
                Expr::Ident(name) => known_globals.get(name.as_str()).map(map_type),
                _ => None,
            };
            match pointee {
                Some(ty) => format!("(&({e}) as *const {ty} as *mut {ty})"),
                None => format!("(&({e}) as *const _ as *mut _)"),
            }
        }
        UnaryOp::Deref => format!("(*({e}))"),
        UnaryOp::PreInc => format!("{{ {e} += 1; {e} }}"),
        UnaryOp::PreDec => format!("{{ {e} -= 1; {e} }}"),
    })
}

fn render_postfix(
    op: PostfixOp,
    expr: &Expr,
    known_globals: &HashMap<String, Type>,
) -> Option<String> {
    let e = render_expr(expr, known_globals)?;
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
