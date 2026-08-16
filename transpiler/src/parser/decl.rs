//! Step 4: constants with initializers, e.g.
//! `static const char rcsid[] = "...";` or
//! `default_t defaults[] = { ... };`.
//!
//! No operator grammar: a scalar initializer's own expression text
//! (`(30*TICRATE)`, `"a"`, `&foo`, ...) is always kept as raw text - no
//! attempt is made to parse casts/operators/precedence. A *braced*
//! initializer's top-level comma-separated shape is structured (see
//! `parse_braced_init`), which alone is enough to make `mobjinfo[]`/
//! `states[]`-style tables inspectable per-row/per-field instead of one
//! opaque string, because step 1 already found the matching `{`/`}`.

use super::ast::{
    Chunk, ConstDecl, Init, RawToken, TypedefDecl, render_tokens_no_comments, split_top_level,
};

/// Parses a plain `;`-terminated statement with no top-level brace group,
/// e.g. `static const char rcsid[] = "...";`. Returns `None` if there's no
/// top-level `=` (not a constant-with-initializer) or the declarator
/// doesn't parse.
pub fn try_parse_const_flat(stmt: &str) -> Option<ConstDecl> {
    let s = stmt.trim();
    let s = s.strip_suffix(';').unwrap_or(s).trim();
    let (decl_part, init_part) = split_top_level_eq(s)?;
    let (storage, ty, name, array_dims) = parse_declarator(decl_part.trim())?;
    Some(ConstDecl {
        storage,
        ty,
        name,
        array_dims,
        initializer: Some(Init::Expr(init_part.trim().to_string())),
    })
}

/// Parses the `TYPE NAME[dims] =` header preceding a brace-initializer
/// group, e.g. `mobjinfo_t mobjinfo[NUMMOBJTYPES] =` before `{ ... };`.
/// Caller (record.rs) has already confirmed `header` ends with `=`. `inner`
/// is the group's contents (excluding the `{`/`}` themselves).
pub fn try_parse_const_braced(header: &str, inner: &[RawToken]) -> Option<ConstDecl> {
    let decl_part = header.trim().strip_suffix('=')?.trim();
    let (storage, ty, name, array_dims) = parse_declarator(decl_part)?;
    Some(ConstDecl {
        storage,
        ty,
        name,
        array_dims,
        initializer: Some(Init::Braced(parse_braced_init(inner))),
    })
}

/// Splits a `{ ... }` initializer's contents on top-level `,` into its
/// elements. Each element is either a nested `Init::Braced` sub-list (a
/// literal `{` at this level, e.g. one row of a `mobjinfo[]`/`states[]`-
/// style table) - recursively parsed the same way, so arbitrarily nested
/// tables work for free - or a scalar `Init::Expr` kept as raw text.
/// Comments are dropped, same reasoning as `render_tokens_no_comments`
/// everywhere else: a trailing `// comment, with a comma` must not fracture
/// the split.
fn parse_braced_init(inner: &[RawToken]) -> Vec<Init> {
    let chunks = super::brace::group_braces(inner.to_vec());
    let mut elements = Vec::new();
    let mut pending = String::new();

    for chunk in chunks {
        match chunk {
            Chunk::Flat(toks) => {
                let text = render_tokens_no_comments(&toks);
                let combined = format!("{pending}{text}");
                let complete = combined.trim_end().ends_with(',') || combined.trim().is_empty();
                let mut parts = split_top_level(&combined, ',');
                let leftover = if complete { None } else { parts.pop() };
                for part in &parts {
                    push_expr_element(part, &mut elements);
                }
                pending = leftover.unwrap_or_default();
            }
            Chunk::Group { inner: ginner, .. } => {
                // A brace group at the top level of an initializer list is
                // always itself one element (a nested braced sub-list), not
                // something to split further at *this* level. Whatever's
                // pending right before it is normally just whitespace (the
                // separating `,`), but if it's something else - a shape
                // this parser doesn't otherwise expect here - it's kept as
                // its own best-effort scalar element rather than dropped.
                push_expr_element(&pending, &mut elements);
                pending.clear();
                elements.push(Init::Braced(parse_braced_init(&ginner)));
            }
        }
    }
    push_expr_element(&pending, &mut elements);
    elements
}

fn push_expr_element(text: &str, elements: &mut Vec<Init>) {
    let text = text.trim();
    if !text.is_empty() {
        elements.push(Init::Expr(text.to_string()));
    }
}

/// Parses a plain, brace-free `typedef TYPE NAME;`, e.g.
/// `typedef bool boolean;` or `typedef unsigned char byte;`.
/// `typedef struct/union/enum { ... } NAME;` goes through record.rs instead
/// since it involves a brace group.
pub fn try_parse_typedef_flat(stmt: &str) -> Option<TypedefDecl> {
    let s = stmt.trim().strip_suffix(';')?.trim();
    let rest = s.strip_prefix("typedef")?;
    if rest.starts_with(|c: char| c.is_alphanumeric() || c == '_') {
        return None; // e.g. "typedefFoo" is not the "typedef" keyword
    }
    let (_, ty, name, dims) = parse_declarator(rest.trim())?;
    let mut underlying = ty;
    for d in &dims {
        underlying.push('[');
        if let Some(n) = d {
            underlying.push_str(n);
        }
        underlying.push(']');
    }
    Some(TypedefDecl { underlying, name })
}

/// Finds the first top-level `=` in a brace-free expression, skipping `==`,
/// `!=`, `<=`, `>=` and compound assignment operators (`+=`, `-=`, ...).
fn split_top_level_eq(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'=' {
            let prev = if i > 0 { Some(bytes[i - 1]) } else { None };
            let next = bytes.get(i + 1).copied();
            let compound_prev = matches!(
                prev,
                Some(
                    b'=' | b'!'
                        | b'<'
                        | b'>'
                        | b'+'
                        | b'-'
                        | b'*'
                        | b'/'
                        | b'%'
                        | b'&'
                        | b'|'
                        | b'^'
                )
            );
            if next == Some(b'=') || compound_prev {
                i += 1;
                continue;
            }
            return Some((&s[..i], &s[i + 1..]));
        }
        i += 1;
    }
    None
}

/// Heuristic C declarator parser: `[storage...] TYPE [*]NAME (['[' dim ']'])*`,
/// or the function-pointer shape `[storage...] TYPE (*NAME) (PARAMS)`. Not a
/// full C grammar - good enough to pull apart the shapes actually used in
/// the target files without parsing expressions.
pub(crate) fn parse_declarator(
    s: &str,
) -> Option<(Vec<String>, String, String, Vec<Option<String>>)> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let mut base = s;
    let mut dims: Vec<Option<String>> = Vec::new();
    while base.ends_with(']') {
        let open = base.rfind('[')?;
        let dim = base[open + 1..base.len() - 1].trim();
        dims.push(if dim.is_empty() {
            None
        } else {
            Some(dim.to_string())
        });
        base = base[..open].trim_end();
    }
    dims.reverse();

    // A declarator never legitimately ends in `)` other than the
    // function-pointer shape (`TYPE (*NAME)(PARAMS)`) - falling through to
    // the plain whitespace-token parsing below for such text would misread
    // e.g. `boolean (*traverser_t) (intercept_t *in)` as a garbage
    // `*`-prefixed name (`in)`) instead of correctly failing or extracting
    // `traverser_t`. So this shape is tried first, and if it doesn't match,
    // parsing fails outright rather than falling through.
    if base.ends_with(')') {
        let (storage, ty, name) = parse_fnptr_declarator(base)?;
        return Some((storage, ty, name, dims));
    }

    let tokens: Vec<&str> = base.split_whitespace().collect();
    let last = *tokens.last()?;
    let star_count = last.len() - last.trim_start_matches('*').len();
    let name = last.trim_start_matches('*').to_string();
    let first_char = name.chars().next()?;
    if !(first_char.is_alphabetic() || first_char == '_') {
        return None;
    }

    const STORAGE_KW: &[&str] = &["static", "extern", "const", "register", "volatile"];
    let mut storage = Vec::new();
    let mut ty_parts: Vec<&str> = Vec::new();
    for t in &tokens[..tokens.len() - 1] {
        if STORAGE_KW.contains(t) && ty_parts.is_empty() {
            storage.push((*t).to_string());
        } else {
            ty_parts.push(t);
        }
    }
    let mut ty = ty_parts.join(" ");
    if ty.is_empty() {
        return None;
    }
    if star_count > 0 {
        ty.push(' ');
        ty.extend(std::iter::repeat('*').take(star_count));
    }
    Some((storage, ty, name, dims))
}

/// Recognizes the function-pointer declarator shape `RETTYPE (*NAME) (PARAMS)`,
/// e.g. `void (*actionf_v)()` or `boolean (*traverser_t) (intercept_t *in)`.
/// `PARAMS` is kept as raw text, same as `try_parse_fn_sig` does for
/// ordinary function signatures. Returns `(storage, ty, name)` with `ty`
/// re-spelled as `RETTYPE (*)(PARAMS)` (name elided) so it composes with the
/// plain `ty`/`name` fields `ConstDecl`/`TypedefDecl`/`Field` already use.
fn parse_fnptr_declarator(s: &str) -> Option<(Vec<String>, String, String)> {
    let params_open = matching_open_paren(s)?;
    let params = s[params_open + 1..s.len() - 1].trim();
    let before_params = s[..params_open].trim();
    if !before_params.ends_with(')') {
        return None;
    }
    let name_open = matching_open_paren(before_params)?;
    let name_group = before_params[name_open + 1..before_params.len() - 1].trim();
    let name = name_group.strip_prefix('*')?.trim().to_string();
    let first_char = name.chars().next()?;
    if !(first_char.is_alphabetic() || first_char == '_')
        || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
    {
        return None;
    }

    let ret_raw = before_params[..name_open].trim();
    let tokens: Vec<&str> = ret_raw.split_whitespace().collect();
    const STORAGE_KW: &[&str] = &["static", "extern", "const", "register", "volatile"];
    let mut storage = Vec::new();
    let mut ty_parts = Vec::new();
    for t in &tokens {
        if STORAGE_KW.contains(t) && ty_parts.is_empty() {
            storage.push((*t).to_string());
        } else {
            ty_parts.push(*t);
        }
    }
    let ret_ty = ty_parts.join(" ");
    if ret_ty.is_empty() {
        return None;
    }
    let ty = format!("{ret_ty} (*)({params})");
    Some((storage, ty, name))
}

/// Finds the index of the `(` matching the final `)` of `s`, or `None` if
/// `s` doesn't end in `)`.
fn matching_open_paren(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    if bytes.last() != Some(&b')') {
        return None;
    }
    let mut depth = 0i32;
    for (i, &b) in bytes.iter().enumerate().rev() {
        match b {
            b')' => depth += 1,
            b'(' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests;
