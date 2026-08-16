//! Step 4: variable declarations, both with an initializer, e.g.
//! `static const char rcsid[] = "...";` / `default_t defaults[] = { ... };`,
//! and without one, e.g. `extern int key_right;` / `static byte *wipe_scr;`.
//!
//! No operator grammar: a scalar initializer's own expression text
//! (`(30*TICRATE)`, `"a"`, `&foo`, ...) is always kept as raw text - no
//! attempt is made to parse casts/operators/precedence. A *braced*
//! initializer's top-level comma-separated shape is structured (see
//! `parse_braced_init`), which alone is enough to make `mobjinfo[]`/
//! `states[]`-style tables inspectable per-row/per-field instead of one
//! opaque string, because step 1 already found the matching `{`/`}`.

use super::ast::{
    Chunk, Init, RawToken, Storage, Type, TypedefDecl, VarDecl, render_tokens_no_comments,
    split_top_level,
};

/// Parses a plain `;`-terminated statement with no top-level brace group,
/// e.g. `static const char rcsid[] = "...";` (initializer) or
/// `extern int key_right;` (no initializer - `initializer` comes back
/// `None`). Returns `None` only if the declarator itself doesn't parse.
pub fn try_parse_var_flat(stmt: &str) -> Option<VarDecl> {
    let s = stmt.trim();
    let s = s.strip_suffix(';').unwrap_or(s).trim();
    match split_top_level_eq(s) {
        Some((decl_part, init_part)) => {
            let (storage, ty, name) = parse_declarator(decl_part.trim())?;
            Some(VarDecl {
                storage,
                ty,
                name,
                initializer: Some(Init::Expr(init_part.trim().to_string())),
            })
        }
        None => {
            let (storage, ty, name) = parse_declarator(s)?;
            Some(VarDecl {
                storage,
                ty,
                name,
                initializer: None,
            })
        }
    }
}

/// Parses the `TYPE NAME[dims] =` header preceding a brace-initializer
/// group, e.g. `mobjinfo_t mobjinfo[NUMMOBJTYPES] =` before `{ ... };`.
/// Caller (record.rs) has already confirmed `header` ends with `=`. `inner`
/// is the group's contents (excluding the `{`/`}` themselves). Always
/// produces `Some` initializer - a braced group with no leading `=` isn't
/// this function's shape at all (record.rs never calls it in that case).
pub fn try_parse_var_braced(header: &str, inner: &[RawToken]) -> Option<VarDecl> {
    let decl_part = header.trim().strip_suffix('=')?.trim();
    let (storage, ty, name) = parse_declarator(decl_part)?;
    Some(VarDecl {
        storage,
        ty,
        name,
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
    // Storage-class keywords on a typedef (e.g. `typedef const int foo;`)
    // are discarded here, same as before `Type` existed - `TypedefDecl` has
    // nowhere to put them and no real typedef in this corpus uses one.
    let (_, underlying, name) = parse_declarator(rest.trim())?;
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
/// the target files without parsing expressions. Array dims and pointer
/// stars are folded into the returned `Type` (`Array`/`Pointer` wrapping a
/// base `Named`) rather than returned as separate fields.
pub(crate) fn parse_declarator(s: &str) -> Option<(Vec<Storage>, Type, String)> {
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
    // function-pointer shape (`TYPE (*NAME)(PARAMS)`, or its array variant
    // `TYPE (*NAME[N])(PARAMS)`) - falling through to the plain
    // whitespace-token parsing below for such text would misread e.g.
    // `boolean (*traverser_t) (intercept_t *in)` as a garbage `*`-prefixed
    // name (`in)`) instead of correctly failing or extracting `traverser_t`.
    // So this shape is tried first, and if it doesn't match, parsing fails
    // outright rather than falling through. The `while base.ends_with(']')`
    // loop above never fires for this shape (a fn-pointer declarator ends in
    // `)`, not `]`), so `dims` is always empty here - the array dims, if
    // any, live *inside* the name parens instead and are already wrapped
    // into the `Type` `parse_fnptr_declarator` returns.
    if base.ends_with(')') {
        return parse_fnptr_declarator(base);
    }

    let tokens: Vec<&str> = base.split_whitespace().collect();
    let last = *tokens.last()?;
    let star_count = last.len() - last.trim_start_matches('*').len();
    let name = last.trim_start_matches('*').to_string();
    let first_char = name.chars().next()?;
    if !(first_char.is_alphabetic() || first_char == '_') {
        return None;
    }

    let mut storage = Vec::new();
    let mut ty_parts: Vec<&str> = Vec::new();
    for t in &tokens[..tokens.len() - 1] {
        match Storage::from_keyword(t) {
            Some(kw) if ty_parts.is_empty() => storage.push(kw),
            _ => ty_parts.push(t),
        }
    }
    let ty_text = ty_parts.join(" ");
    if ty_text.is_empty() {
        return None;
    }
    let mut ty = parse_type_text(&ty_text);
    for _ in 0..star_count {
        ty = Type::Pointer(Box::new(ty));
    }
    let ty = wrap_array_dims(ty, &dims);
    Some((storage, ty, name))
}

/// Recognizes the function-pointer declarator shape `RETTYPE (*NAME) (PARAMS)`,
/// e.g. `void (*actionf_v)()` or `boolean (*traverser_t) (intercept_t *in)`,
/// as well as its array-of-function-pointers variant
/// `RETTYPE (*NAME[N])(PARAMS)`, e.g. `int (*wipes[])(int, int, int)`
/// (`f_wipe.c`). Returns `(storage, ty, name)` with `ty` a
/// `Type::FunctionPointer` (wrapped in `Type::Array` for the `(*NAME[N])`
/// variant) so it composes with the plain `ty`/`name` fields
/// `VarDecl`/`TypedefDecl`/`Field` already use. Each parameter in
/// `PARAMS` is itself parsed via `parse_declarator` (falling back to a bare
/// `parse_type_text` read if it has no name, e.g. `traverser_t`'s
/// `intercept_t *in` *does* have a name "in" that gets discarded here -
/// `Type::FunctionPointer::params` is types only, see its doc comment).
fn parse_fnptr_declarator(s: &str) -> Option<(Vec<Storage>, Type, String)> {
    let params_open = matching_open_paren(s)?;
    let params_text = s[params_open + 1..s.len() - 1].trim();
    let before_params = s[..params_open].trim();
    if !before_params.ends_with(')') {
        return None;
    }
    let name_open = matching_open_paren(before_params)?;
    let name_group = before_params[name_open + 1..before_params.len() - 1].trim();
    let mut name_base = name_group.strip_prefix('*')?.trim();
    let mut dims: Vec<Option<String>> = Vec::new();
    while name_base.ends_with(']') {
        let open = name_base.rfind('[')?;
        let dim = name_base[open + 1..name_base.len() - 1].trim();
        dims.push(if dim.is_empty() {
            None
        } else {
            Some(dim.to_string())
        });
        name_base = name_base[..open].trim_end();
    }
    dims.reverse();
    let name = name_base.to_string();
    let first_char = name.chars().next()?;
    if !(first_char.is_alphabetic() || first_char == '_')
        || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
    {
        return None;
    }

    let ret_raw = before_params[..name_open].trim();
    let tokens: Vec<&str> = ret_raw.split_whitespace().collect();
    let mut storage = Vec::new();
    let mut ty_parts = Vec::new();
    for t in &tokens {
        match Storage::from_keyword(t) {
            Some(kw) if ty_parts.is_empty() => storage.push(kw),
            _ => ty_parts.push(*t),
        }
    }
    let ret_text = ty_parts.join(" ");
    if ret_text.is_empty() {
        return None;
    }
    let ret = parse_type_text(&ret_text);
    let params = parse_fnptr_params(params_text);
    let ty = wrap_array_dims(
        Type::FunctionPointer {
            ret: Box::new(ret),
            params,
        },
        &dims,
    );
    Some((storage, ty, name))
}

/// Splits a function-pointer type's own parameter-list text (e.g. `"int,
/// int, int"` or `"intercept_t *in"`) on top-level `,` into `Type`s. `()`
/// and `(void)` both mean no parameters. Each piece is tried as a named
/// declarator first (discarding the name - see `Type::FunctionPointer`'s
/// doc comment) and falls back to a bare-type read if it has none.
fn parse_fnptr_params(params_text: &str) -> Vec<Type> {
    let trimmed = params_text.trim();
    if trimmed.is_empty() || trimmed == "void" {
        return Vec::new();
    }
    split_top_level(trimmed, ',')
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| match parse_declarator(p) {
            Some((_, ty, _)) => ty,
            None => parse_type_text(p),
        })
        .collect()
}

/// Reads a bare type-text with no declarator name attached (e.g. a
/// function-pointer's return type, or an anonymous parameter's type) into a
/// `Type`, by counting any trailing `*` characters as pointer depth. Works
/// whether the star is glued to the base (`"char*"`) or separated by
/// whitespace (`"char *"`, `"char * *"`) - both this codebase's styles.
pub(crate) fn parse_type_text(text: &str) -> Type {
    let text = text.trim();
    let base = text.trim_end_matches(['*', ' ']);
    let star_count = text[base.len()..].matches('*').count();
    let mut ty = Type::Named(base.to_string());
    for _ in 0..star_count {
        ty = Type::Pointer(Box::new(ty));
    }
    ty
}

/// Wraps `base` in one `Type::Array` per entry of `dims`, outermost bracket
/// first - see `Type::Array`'s doc comment for why this means iterating
/// `dims` in *reverse*.
pub(crate) fn wrap_array_dims(base: Type, dims: &[Option<String>]) -> Type {
    let mut ty = base;
    for dim in dims.iter().rev() {
        ty = Type::Array(Box::new(ty), dim.clone());
    }
    ty
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
