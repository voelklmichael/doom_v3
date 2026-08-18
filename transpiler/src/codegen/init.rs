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
//! Scalar targets only - `render_array_init` below handles an `Array`-typed
//! target instead. `codegen::items::emit_var` routes each `VarDecl` to
//! whichever of the two applies and leaves every other shape (struct/union
//! tables, `Init::Conditional` outside an array) on the existing `zeroed()`
//! stub until a later phase covers them.

use super::expr::render_expr;
use super::types::map_type;
use crate::parser::ast::{Init, Type};
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

/// Renders an `Init` against its declaration's already-known `Array` `Type`,
/// paired with that array's own Rust type text. Unlike every other codegen
/// path, an unsized array's (`Array(_, None)`) real length isn't known from
/// the `Type` alone - only from the initializer itself (the element count of
/// a `Braced` list, or a C string literal's byte count + 1 for the null
/// terminator) - so this returns `(type_text, init_text)` instead of a bare
/// `map_type` call, and `emit_var` must use the paired type text rather than
/// its own generic one when this succeeds.
///
/// `None` when the shape isn't (yet) covered - see `render_array_element`
/// for the struct/union-typed-table-row case this excludes (needs a later
/// phase's record-field-lookup infrastructure), or when a leaf expression
/// contains an `Expr::Raw` (same "no Rust equivalent" fallback every other
/// `render_*` function in this codebase shares).
pub fn render_array_init(
    init: &Init,
    ty: &Type,
    known: &KnownTypeNames,
) -> Option<(String, String)> {
    let Type::Array(elem_ty, dim) = ty else {
        return None;
    };
    match init {
        Init::Expr(text) => render_char_array_from_string(elem_ty, text, dim),
        Init::Braced(elements) => render_scalar_or_nested_array(elements, elem_ty, dim, known),
        // A mid-list `#ifdef` at the array's own top level - never occurs in
        // this corpus outside the struct-typed-table-row case (see
        // `render_array_element`), which is excluded before ever reaching
        // this level. No real shape to model against, so bail rather than
        // guess.
        Init::Conditional(_) => None,
    }
}

/// The `char rcsid[] = "...";` idiom: a scalar `Init::Expr` holding a C
/// string literal against a `char`-elemented array. Unescapes the common C
/// escape sequences (best-effort passthrough for anything else - never
/// panics; the real corpus's 62 occurrences of this exact shape are all
/// plain ASCII RCS/CVS banners with none of the exotic ones) and appends the
/// implicit null terminator, matching C's own char-array-from-string-literal
/// semantics.
fn render_char_array_from_string(
    elem_ty: &Type,
    text: &str,
    dim: &Option<String>,
) -> Option<(String, String)> {
    if map_type(elem_ty) != "std::ffi::c_char" {
        return None;
    }
    let trimmed = text.trim();
    let inner = trimmed.strip_prefix('"')?.strip_suffix('"')?;
    let bytes = unescape_c_string(inner);
    let len_text = match dim {
        Some(d) => format!("({d}) as usize"),
        None => (bytes.len() + 1).to_string(),
    };
    let mut items: Vec<String> = bytes
        .iter()
        .map(|b| format!("{b} as std::ffi::c_char"))
        .collect();
    items.push("0".to_string());
    Some((
        format!("[std::ffi::c_char; {len_text}]"),
        format!("[{}]", items.join(", ")),
    ))
}

fn unescape_c_string(inner: &str) -> Vec<u8> {
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
            // this corpus's real occurrences of this shape) - keep both
            // bytes rather than silently drop the backslash.
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

/// A flat scalar array (`45` real corpus cases, e.g. `sprnames[]`) or a
/// 2-D scalar array (`v_video.c`'s `gammatable[5][256]`, the corpus's only
/// occurrence). `dim: None` gets its real length from `elements.len()`, same
/// reasoning as the string-literal case above.
fn render_scalar_or_nested_array(
    elements: &[Init],
    elem_ty: &Type,
    dim: &Option<String>,
    known: &KnownTypeNames,
) -> Option<(String, String)> {
    let mut rendered_elems = Vec::with_capacity(elements.len());
    let mut elem_type_text = None;
    for element in elements {
        let (this_type, this_init) = render_array_element(element, elem_ty, known)?;
        elem_type_text.get_or_insert(this_type);
        rendered_elems.push(this_init);
    }
    let elem_type_text = elem_type_text.unwrap_or_else(|| map_type(elem_ty));
    let len_text = match dim {
        Some(d) => format!("({d}) as usize"),
        None => rendered_elems.len().to_string(),
    };
    Some((
        format!("[{elem_type_text}; {len_text}]"),
        format!("[{}]", rendered_elems.join(", ")),
    ))
}

/// Renders one element of a braced array initializer against the array's
/// own element type, paired with that element's own Rust type text (needed
/// by the 2-D case, where a row's own length - hence its own type text -
/// isn't known until it's rendered).
fn render_array_element(
    element: &Init,
    elem_ty: &Type,
    known: &KnownTypeNames,
) -> Option<(String, String)> {
    match element {
        // 2-D case: this row is itself an array (e.g. one row of
        // `gammatable[5][256]`) - recurse one `Array` level deeper.
        Init::Braced(_) if matches!(elem_ty, Type::Array(_, _)) => {
            render_array_init(element, elem_ty, known)
        }
        // C allows extra braces around a scalar initializer (`{5}`) - unwrap
        // and recurse.
        Init::Braced(inner) if inner.len() == 1 => render_array_element(&inner[0], elem_ty, known),
        // A struct/union-typed table row (`states[]`/`mobjinfo[]`/the
        // `m_menu.c` menu tables/... - confirmed via a corpus-wide scan to
        // be the *only* real shape matching "nested Braced but elem_ty isn't
        // itself Array") needs a later phase's record-field-lookup
        // infrastructure to zip positionally against the struct's own
        // fields - out of scope here, bail rather than guess. An empty
        // `Braced` (`{}`) falls here too - never confirmed in this corpus,
        // no real shape to render against.
        Init::Braced(_) => None,
        Init::Expr(text) => {
            let rendered = render_scalar_init(text, elem_ty, known)?;
            Some((map_type(elem_ty), rendered))
        }
        // Confirmed via corpus-wide scan: the one real `Init::Conditional`
        // inside an array (`m_misc.c`'s `defaults[]`) sits inside a
        // struct-typed row, already excluded above - this never actually
        // fires against real corpus data, kept as a defensive fallback
        // rather than an unreachable!().
        Init::Conditional(_) => None,
    }
}

#[cfg(test)]
mod tests;
