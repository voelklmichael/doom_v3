//! `ast::Init` -> Rust initializer-expression text. `Init::Expr(String)` is
//! raw, unparsed scalar text (the front-end's own doc comment: "no operator
//! grammar... kept as raw text") - like `codegen::macros`, this module
//! parses it lazily at codegen time, reusing the same `scan::scan` ->
//! `stmt::lex::lex_ctoks` -> `stmt::expr::parse_expr` pipeline, then renders
//! via `codegen::expr::render_expr`.
//!
//! Unlike a macro, a `VarDecl`'s type is never inferred - it's read directly
//! from `ast::VarDecl.ty` (or, for a struct field's initializer element,
//! `ast::Field.ty`), so this module's job is narrower than
//! `codegen::macros`': render the scalar text, then apply a small set of
//! target-*type*-aware fixups where C's looser literal rules don't survive
//! a direct transliteration (the bare-`0`-as-null-pointer idiom, and the
//! bare-`0`/`NULL`-as-`None` idiom for a function-pointer-typed target - see
//! `render_scalar_init`).
//!
//! Three target shapes: scalar (`render_scalar_init`), `Array`
//! (`render_array_init`), and struct/union (`render_struct_init`) -
//! `codegen::items::emit_var` routes each `VarDecl` to whichever applies and
//! leaves anything `render_*` can't handle (an unresolved mid-list `#ifdef`
//! branch, more initializer values than a struct has fields, an
//! `Expr::Raw` leaf) on the existing `zeroed()` stub, never a guess.

use super::expr::{render_expr, unescape_c_string};
use super::ident::ident;
use super::types::{map_type, normalize, strip_tag_keyword, type_is_malformed};
use crate::parser::ast::{ActiveBranch, Init, RecordDecl, RecordKind, Type};
use crate::parser::scan;
use crate::parser::stmt::expr::{Expr, KnownTypeNames, parse_expr};
use crate::parser::stmt::lex::lex_ctoks;
use std::collections::{BTreeSet, HashMap, HashSet};

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
pub fn render_scalar_init(
    text: &str,
    ty: &Type,
    known: &KnownTypeNames,
    known_typedefs: &HashMap<String, Type>,
) -> Option<String> {
    let expr = parse_init_expr(text, known);
    let rendered = render_expr(&expr)?;
    // A target spelled via a typedef name (real corpus case: `info.h`'s
    // `state_t.action` field is `Named("actionf_t")`, itself a *union* of
    // further typedef'd function-pointer types - `actionf_p1`/`actionf_v`/
    // `actionf_p2` - so the field-pointer fixups below need to see through
    // one level of aliasing to recognize the real shape, not just a literal
    // `Type::FunctionPointer` spelled directly).
    let resolved_ty = resolve_typedef(ty, known_typedefs);
    Some(match (resolved_ty, &expr) {
        // A bare `0` used as a null pointer - C's untyped-null-constant
        // idiom has no Rust equivalent via a plain integer literal (`*mut T
        // = 0` doesn't compile; needs an explicit null-pointer
        // constructor). Real corpus cases: `i_video.c`'s `X_display`,
        // `s_sound.c`'s `mus_playing`.
        (Type::Pointer(_), Expr::IntLit(lit)) if lit.trim() == "0" => {
            "std::ptr::null_mut()".to_string()
        }
        // A function-pointer-typed target maps to `Option<unsafe extern "C"
        // fn(...)>` (see `codegen::types::map_type`), so a bare `0`/`NULL`
        // (C's "no function" idiom - real corpus case: `info.c`'s `states[]`
        // rows like `{NULL}` for the `action` field) must become `None`, and
        // a bare function-name reference (`{A_Light0}`) must be wrapped in
        // `Some(...)` to typecheck against that `Option`.
        (Type::FunctionPointer { .. }, Expr::IntLit(lit)) if lit.trim() == "0" => {
            "None".to_string()
        }
        (Type::FunctionPointer { .. }, Expr::Ident(name)) if name == "NULL" => "None".to_string(),
        (Type::FunctionPointer { .. }, Expr::Ident(_)) => format!("Some({rendered})"),
        _ => rendered,
    })
}

/// Resolves `ty` through named typedefs (bounded by a cycle guard, though
/// this corpus's own typedef chains never nest more than one level deep) to
/// whatever `Type` shape it ultimately spells - stops as soon as a `Named`
/// leaf isn't itself a known typedef (an ordinary builtin/tag/opaque name),
/// matching every other "don't guess past what's decidable" rule in this
/// codebase.
fn resolve_typedef<'a>(ty: &'a Type, known_typedefs: &'a HashMap<String, Type>) -> &'a Type {
    let mut current = ty;
    let mut seen = HashSet::new();
    while let Type::Named(name) = current {
        if !seen.insert(name.as_str()) {
            break;
        }
        match known_typedefs.get(name) {
            Some(next) => current = next,
            None => break,
        }
    }
    current
}

/// Resolves a `Type::Named` leaf to the struct/union it refers to, if any -
/// `known_records` is keyed by both a record's `tag` and its `typedef_name`
/// (see `corpus::compute_known_records`), so a tag-based reference (`struct
/// mobj_s`, common for self-referential fields - see `codegen::types`'s own
/// tag-stripping note) resolves exactly like a typedef-name reference.
/// `None` for every other `Type` shape (`Pointer`/`Array`/`FunctionPointer`,
/// or a `Named` leaf that isn't a known record at all - an ordinary scalar
/// type).
fn resolve_record<'a>(
    ty: &Type,
    known_records: &'a HashMap<String, RecordDecl>,
) -> Option<&'a RecordDecl> {
    let Type::Named(name) = ty else {
        return None;
    };
    let normalized = normalize(name);
    let key = strip_tag_keyword(&normalized).unwrap_or(normalized.as_str());
    known_records.get(key)
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
/// `known_records`/`needed_zeroed` exist only for the struct/union-typed
/// table-row case (`states[]`/`mobjinfo[]`/...) - see `render_value`.
/// `needed_zeroed` accumulates the Rust type names of every struct/union
/// referenced by a *partial* row (fewer initializer values than fields) -
/// callers emit one `const ZEROED_{name}: {name} = unsafe {
/// std::mem::zeroed() };` per accumulated name (deduped by the `BTreeSet`
/// itself), used as the `..ZEROED_{name}` struct-update-syntax source for
/// every field the row didn't specify.
///
/// `None` when the shape isn't covered - see `render_value` for the one
/// struct-row case that still isn't (a row with more values than the
/// struct has fields, or a nested field this parser's own struct-field
/// grammar doesn't recognize), an unresolved mid-list `#ifdef` branch (see
/// `flatten_active_elements`), or when a leaf expression contains an
/// `Expr::Raw` (same "no Rust equivalent" fallback every other `render_*`
/// function in this codebase shares).
pub fn render_array_init(
    init: &Init,
    ty: &Type,
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    needed_zeroed: &mut BTreeSet<String>,
) -> Option<(String, String)> {
    let Type::Array(elem_ty, dim) = ty else {
        return None;
    };
    match init {
        Init::Expr(text) => render_char_array_from_string(elem_ty, text, dim),
        Init::Braced(elements) => render_scalar_or_nested_array(
            elements,
            elem_ty,
            dim,
            known,
            known_records,
            known_typedefs,
            needed_zeroed,
        ),
        // A mid-list `#ifdef` at the array's own top level - never occurs in
        // this corpus (the one real occurrence, `m_misc.c`'s `defaults[]`,
        // sits one level *inside* a `Braced` list - see
        // `flatten_active_elements`). No real shape to model against here,
        // so bail rather than guess.
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
    let real_len = bytes.len() + 1; // + implicit null terminator
    // A literal integer dim (real corpus case: a struct field like
    // `menuitem_t.name: char[10]`, always a plain bracket literal in this
    // corpus - never a symbolic macro name for a *field*, unlike a
    // top-level var's array dim) needs the initializer *literal* padded out
    // to match, since Rust array literals need an exact element count and
    // this field's own type was already declared with the full length
    // (`emit_field`/`map_type` reuse the dim text verbatim). A symbolic dim
    // (never seen for a field in this corpus) falls back to reusing it
    // verbatim as a const-expr, same accepted-gap precedent `map_type`
    // itself already documents for a `VarDecl`'s own array dim.
    let declared_len = dim.as_ref().and_then(|d| d.trim().parse::<usize>().ok());
    let len_text = match (declared_len, dim) {
        (Some(n), _) => n.to_string(),
        (None, Some(d)) => format!("({d}) as usize"),
        (None, None) => real_len.to_string(),
    };
    let mut items: Vec<String> = bytes
        .iter()
        .map(|b| format!("{b} as std::ffi::c_char"))
        .collect();
    items.push("0".to_string());
    if let Some(n) = declared_len {
        // C's own "the rest default to zero" initializer rule. More real
        // bytes than the declared length isn't real/safe C - left as a
        // visible length-mismatch compile error rather than silently
        // truncated, same "accepted, never guessed" precedent as
        // everywhere else in this module.
        for _ in real_len..n {
            items.push("0".to_string());
        }
    }
    Some((
        format!("[std::ffi::c_char; {len_text}]"),
        format!("[{}]", items.join(", ")),
    ))
}

/// A flat scalar array (`45` real corpus cases, e.g. `sprnames[]`), a 2-D
/// scalar array (`v_video.c`'s `gammatable[5][256]`, the corpus's only
/// occurrence), or an array of struct/union rows (`states[]`/`mobjinfo[]`/
/// the `m_menu.c` menu tables/`m_misc.c`'s `defaults[]`/...). `dim: None`
/// gets its real length from the (post-`#ifdef`-flattening) element count,
/// same reasoning as the string-literal case above.
fn render_scalar_or_nested_array(
    elements: &[Init],
    elem_ty: &Type,
    dim: &Option<String>,
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    needed_zeroed: &mut BTreeSet<String>,
) -> Option<(String, String)> {
    let mut flat = Vec::with_capacity(elements.len());
    flatten_active_elements(elements, &mut flat)?;

    let mut rendered_elems = Vec::with_capacity(flat.len());
    let mut elem_type_text = None;
    for element in flat {
        let (this_type, this_init) = render_value(
            element,
            elem_ty,
            known,
            known_records,
            known_typedefs,
            needed_zeroed,
        )?;
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

/// Expands every mid-list `#ifdef` (`Init::Conditional`) in `elements` into
/// its already-resolved active branch's own elements, spliced in place -
/// recursing, since a resolved branch's body can itself contain a nested
/// `#ifdef` (real corpus case: `m_misc.c`'s `defaults[]` has a `SNDSERV`
/// block nested inside its `NORMALUNIX` block). `None` (bail the *whole*
/// array, never silently drop rows) if any `Conditional` encountered has no
/// resolved branch (`ActiveBranch::None`/`Unknown`, or an `Else` with no
/// `else_body`) - a shorter-than-real array is a silent semantic corruption
/// this codebase never risks, unlike a merely-unrenderable *value* (which
/// only ever fails that one row/var, never changes another row's meaning).
fn flatten_active_elements<'a>(elements: &'a [Init], out: &mut Vec<&'a Init>) -> Option<()> {
    for element in elements {
        match element {
            Init::Conditional(group) => match &group.active {
                ActiveBranch::Branch(n) => {
                    flatten_active_elements(&group.branches[*n].body, out)?;
                }
                ActiveBranch::Else => {
                    flatten_active_elements(group.else_body.as_ref()?, out)?;
                }
                ActiveBranch::None | ActiveBranch::Unknown => return None,
            },
            other => out.push(other),
        }
    }
    Some(())
}

/// Renders one value (an array element, or a struct field's own value)
/// against its already-known target `Type`, paired with that value's own
/// Rust type text - needed by the 2-D array case (a row's own length, hence
/// its own type text, isn't known until it's rendered) and reused as-is by
/// `render_struct_literal` for a field's value (a field's *declared* type
/// text never depends on any one row's content, so the pairing is simply
/// discarded there).
fn render_value(
    element: &Init,
    target_ty: &Type,
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    needed_zeroed: &mut BTreeSet<String>,
) -> Option<(String, String)> {
    match element {
        // An `Array`-typed target - covers a nested array (e.g. one row of
        // `gammatable[5][256]`, `Init::Braced`) *and* a fixed char-array
        // field given a bare string literal with no extra braces (real
        // corpus case: `m_menu.c`'s `menuitem_t.name` rows, e.g.
        // `{1,"M_NGAME",M_NewGame,'n'}` - `"M_NGAME"` is `Init::Expr`, not
        // `Init::Braced`, same shape `render_array_init` already handles
        // for a top-level `char rcsid[] = "...";` var). Delegate to
        // `render_array_init` either way - it dispatches on `Init::Expr`
        // vs. `Init::Braced` internally.
        _ if matches!(target_ty, Type::Array(_, _)) => render_array_init(
            element,
            target_ty,
            known,
            known_records,
            known_typedefs,
            needed_zeroed,
        ),
        // A struct/union-typed row/field (`states[]`'s own rows; a nested
        // struct-typed field's own value, e.g. `am_map.c`'s `mline_t { a,
        // b }` where `a`/`b` are each `mpoint_t`-typed) - zip positionally
        // against the resolved record's own fields.
        Init::Braced(inner) if resolve_record(target_ty, known_records).is_some() => {
            let rd = resolve_record(target_ty, known_records)?;
            let type_name = map_type(target_ty);
            let rendered = render_struct_literal(
                inner,
                &type_name,
                rd,
                known,
                known_records,
                known_typedefs,
                needed_zeroed,
            )?;
            Some((type_name, rendered))
        }
        // C allows extra braces around a scalar/function-pointer initializer
        // (`{5}`, `info.c`'s real `state_t.action` rows like `{A_Light0}`/
        // `{NULL}`) - unwrap and recurse.
        Init::Braced(inner) if inner.len() == 1 => render_value(
            &inner[0],
            target_ty,
            known,
            known_records,
            known_typedefs,
            needed_zeroed,
        ),
        // Anything else `Braced` (0 or >1 items against a target that isn't
        // an `Array` or a known record) - no real shape to render against,
        // bail rather than guess.
        Init::Braced(_) => None,
        Init::Expr(text) => {
            let rendered = render_scalar_init(text, target_ty, known, known_typedefs)?;
            Some((map_type(target_ty), rendered))
        }
        // A mid-list `#ifdef` sitting directly in field-value position (not
        // between array elements, where `flatten_active_elements` already
        // handles it) - never occurs in this corpus, kept as a defensive
        // fallback rather than an unreachable!().
        Init::Conditional(_) => None,
    }
}

/// Renders `elements` (a struct/union row's own comma-separated values, C's
/// positional initializer order) as a Rust struct/union literal against
/// `rd`'s own field order. `type_name` is the already-mapped Rust type name
/// (from `map_type`, so it also covers a tag-based reference like `struct
/// foo` resolving to the bare tag identifier). A `RecordKind::Union` target
/// delegates to `render_union_literal` instead - a union's positional
/// init/Rust-literal-syntax rules are different enough (see there) that
/// zipping it through this struct path would emit invalid Rust.
///
/// A row with fewer values than `rd` has fields (C's own "the rest default
/// to zero" rule - the common case, e.g. `wi_stuff.c`'s `anim_t` rows only
/// ever specify their first 4 of 11 fields) gets a trailing `..ZEROED_{type_name}`
/// (struct-update syntax) and records `type_name` into `needed_zeroed` so
/// the caller emits that placeholder const once per module (see
/// `codegen::items::emit_items`). A row with *more* values than fields, or
/// containing a field this parser's own struct-field grammar couldn't
/// classify (`Field.name`/`Field.ty` embedding a stray comma - see
/// `codegen::items::is_malformed`/`type_is_malformed`), isn't real/safe C
/// to positionally zip - bails (`None`) rather than guess, degrading the
/// whole containing var to the existing `zeroed()` stub.
#[allow(clippy::too_many_arguments)]
fn render_struct_literal(
    elements: &[Init],
    type_name: &str,
    rd: &RecordDecl,
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    needed_zeroed: &mut BTreeSet<String>,
) -> Option<String> {
    if rd.kind == RecordKind::Union {
        return render_union_literal(
            elements,
            type_name,
            rd,
            known,
            known_records,
            known_typedefs,
        );
    }
    if elements.len() > rd.fields.len() {
        return None;
    }
    let mut parts = Vec::with_capacity(rd.fields.len());
    for (element, field) in elements.iter().zip(&rd.fields) {
        if field.name.contains(',') || type_is_malformed(&field.ty) {
            return None;
        }
        let (_, value) = render_value(
            element,
            &field.ty,
            known,
            known_records,
            known_typedefs,
            needed_zeroed,
        )?;
        parts.push(format!("{}: {value}", ident(&field.name)));
    }
    if elements.len() < rd.fields.len() {
        needed_zeroed.insert(type_name.to_string());
        parts.push(format!("..ZEROED_{type_name}"));
    }
    Some(format!("{type_name} {{ {} }}", parts.join(", ")))
}

/// A union's positional brace-init sets *only* its first named member (real
/// C semantics - unlike a struct, a union's members all overlap the same
/// storage, so there's no "rest" left to default). Rust mirrors this
/// exactly: a union literal must name exactly one field and forbids `..`
/// update syntax entirely - so, unlike `render_struct_literal`, there's no
/// zeroed-placeholder-const case here at all (no `needed_zeroed` parameter).
///
/// Real corpus shape: `info.h`'s `actionf_t` (a union of function-pointer
/// typedefs) via `state_t.action`'s rows (`{NULL}`, `{A_Light0}`, ...) -
/// always exactly one value, confirmed via corpus-wide scan. A row with zero
/// or more than one value isn't a shape this corpus's own source uses for a
/// union, so bails (`None`) rather than guess which member (if any) is meant.
fn render_union_literal(
    elements: &[Init],
    type_name: &str,
    rd: &RecordDecl,
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
) -> Option<String> {
    let [element] = elements else {
        return None;
    };
    let first_field = rd.fields.first()?;
    if first_field.name.contains(',') || type_is_malformed(&first_field.ty) {
        return None;
    }
    // A union's own value never itself needs a `..ZEROED_x` placeholder (see
    // this function's doc comment), but a *nested* struct-typed field could
    // in principle still need one - pass a scratch accumulator through
    // rather than `None`/a dummy, so that nested case isn't silently lost.
    let mut nested_needed = BTreeSet::new();
    let (_, value) = render_value(
        element,
        &first_field.ty,
        known,
        known_records,
        known_typedefs,
        &mut nested_needed,
    )?;
    // Never actually populated by any real corpus shape (a union field
    // that's itself a partially-initialized struct doesn't occur here), but
    // silently dropping it if it ever did would be a real (if currently
    // unreachable) correctness gap - bail loudly instead of guessing.
    if !nested_needed.is_empty() {
        return None;
    }
    Some(format!(
        "{type_name} {{ {}: {value} }}",
        ident(&first_field.name)
    ))
}

/// Renders a *bare* struct/union-typed variable's own brace-init (not
/// wrapped in an `Array` - e.g. `m_menu.c`'s `MainDef`/`st_stuff.c`'s
/// `cheat_god`, ~19 real corpus cases), reusing the exact same positional
/// field-zip `render_struct_literal` already provides for one row of an
/// array of structs.
pub fn render_struct_init(
    init: &Init,
    ty: &Type,
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    needed_zeroed: &mut BTreeSet<String>,
) -> Option<String> {
    let Init::Braced(elements) = init else {
        return None;
    };
    let rd = resolve_record(ty, known_records)?;
    let type_name = map_type(ty);
    render_struct_literal(
        elements,
        &type_name,
        rd,
        known,
        known_records,
        known_typedefs,
        needed_zeroed,
    )
}

#[cfg(test)]
mod tests;
