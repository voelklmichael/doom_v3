//! Per-`ItemKind` Rust emission rules - the core of the codegen "skeleton"
//! phase. Consumes a single already-parsed `ast::File`'s items directly (no
//! cross-file merge yet - see `codegen::module`, a later PR); cross-module
//! type references just pass through as bare names for now (`map_type`
//! already handles that uniformly - resolving them via `use` imports is a
//! module-assembly concern, not an item-emission one).

use super::ident::{ident, synthesize_nested_name};
use super::init::{render_array_init, render_scalar_init, render_struct_init};
use super::macros::{emit_define_function, emit_define_object};
use super::types::{
    format_return_suffix, looks_like_identifier, map_type, sanitize_int_literal, type_is_malformed,
};
use crate::parser::ast::{
    ActiveBranch, CondGroup, EnumDecl, Field, FnSig, Init, Item, ItemKind, RecordDecl, RecordKind,
    Storage, Type, TypedefDecl, VarDecl,
};
use crate::parser::preproc::Directive;
use crate::parser::stmt::expr::KnownTypeNames;
use std::collections::{BTreeSet, HashMap};

/// Emits Rust source text for every item in `items`, concatenated, plus (at
/// the end) one `const ZEROED_{name}: {name} = unsafe { std::mem::zeroed()
/// };` per struct/union type that any partial struct-typed initializer
/// anywhere in `items` referenced via `..ZEROED_{name}` struct-update syntax
/// (see `codegen::init::render_struct_literal`) - collected into a single
/// `BTreeSet` shared across the *whole* walk (including every nested
/// `Conditional` branch, via `emit_items_inner`) so each needed const is
/// defined exactly once per module, regardless of how many rows/vars/branches
/// reference the same struct type. Rust doesn't require a `const` to be
/// declared before its use site, so appending them at the end (rather than
/// the plan's originally-considered "emit before the main loop") needs no
/// two-pass rendering - the accumulator naturally finishes filling by the
/// time the single walk completes.
#[allow(clippy::too_many_arguments)]
pub fn emit_items(
    items: &[(Item, crate::parser::ast::Trivia)],
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    known_functions: &HashMap<String, FnSig>,
    known_globals: &HashMap<String, Type>,
    known_defines: &HashMap<String, String>,
) -> String {
    let mut needed_zeroed = BTreeSet::new();
    let mut out = emit_items_inner(
        items,
        known,
        known_records,
        known_typedefs,
        known_functions,
        known_globals,
        known_defines,
        &mut needed_zeroed,
    );
    for type_name in &needed_zeroed {
        out.push_str(&format!(
            "const ZEROED_{type_name}: {type_name} = unsafe {{ std::mem::zeroed() }};\n"
        ));
    }
    if !needed_zeroed.is_empty() {
        out.push('\n');
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn emit_items_inner(
    items: &[(Item, crate::parser::ast::Trivia)],
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    known_functions: &HashMap<String, FnSig>,
    known_globals: &HashMap<String, Type>,
    known_defines: &HashMap<String, String>,
    needed_zeroed: &mut BTreeSet<String>,
) -> String {
    items
        .iter()
        .map(|(item, _)| {
            emit_item(
                item,
                known,
                known_records,
                known_typedefs,
                known_functions,
                known_globals,
                known_defines,
                needed_zeroed,
            )
        })
        .collect()
}

/// Emits Rust source text for one top-level `Item`. Never panics, never
/// silently drops data: an unresolved conditional or an unparsed `Raw` item
/// becomes a loudly-flagged comment rather than being skipped invisibly.
/// `known` is the emitting module's own `KnownTypeNames` (used only for
/// `#define` bodies - see `codegen::macros` - since a macro's cast
/// disambiguation needs the same corpus-derived type-name environment real
/// function-body parsing already uses). `known_records`/`known_typedefs`/
/// `needed_zeroed` are only used by `Var` (see `emit_var`)/`Conditional`
/// (which may recurse into a branch containing a `Var`).
#[allow(clippy::too_many_arguments)]
fn emit_item(
    item: &Item,
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    known_functions: &HashMap<String, FnSig>,
    known_globals: &HashMap<String, Type>,
    known_defines: &HashMap<String, String>,
    needed_zeroed: &mut BTreeSet<String>,
) -> String {
    match &item.kind {
        ItemKind::Typedef(td) => emit_typedef(td),
        ItemKind::Record(rd) => emit_record(rd),
        ItemKind::Enum(ed) => emit_enum(ed),
        ItemKind::Var(vds) => vds
            .iter()
            .map(|vd| {
                emit_var(
                    vd,
                    known,
                    known_records,
                    known_typedefs,
                    known_functions,
                    known_globals,
                    needed_zeroed,
                )
            })
            .collect(),
        ItemKind::FunctionDecl(sig) => emit_function_decl(sig),
        ItemKind::FunctionDef(sig, _body) => emit_function_def(sig),
        ItemKind::Conditional(group) => emit_conditional(
            group,
            known,
            known_records,
            known_typedefs,
            known_functions,
            known_globals,
            known_defines,
            needed_zeroed,
        ),
        ItemKind::Raw => emit_raw(&item.raw),
        // #include has no Rust equivalent (cross-module refs are handled via
        // `use` imports at module-assembly time, see codegen::module).
        ItemKind::Preproc(Directive::DefineObject { name, value }) => emit_define_object(
            name,
            value,
            known,
            known_globals,
            known_functions,
            known_defines,
        ),
        ItemKind::Preproc(Directive::DefineFunction { name, params, body }) => {
            emit_define_function(name, params, body, known, known_globals)
        }
        // #undef/#pragma/#error/#other: no Rust equivalent, out of scope
        // (any downstream use becomes a visible compile error, an
        // acceptable signal for this phase rather than a silent gap).
        ItemKind::Preproc(_) => String::new(),
    }
}

/// True if `text` (a raw declaration *name*, never a mapped type) contains
/// a comma - the signature of `record::parse_fields`/
/// `decl::parse_declarator` failing to split a real multi-declarator
/// declaration (e.g. a struct field like `long misc1, misc2;`, or a
/// top-level variable like `fixed_t m_x2, m_y2;`) into separate
/// `Field`s/`VarDecl`s. The stray comma can end up glued onto *either*
/// half depending on exactly where the token stream got cut - sometimes
/// the name (`{name: "x,y", ty: "int"}`, checked here), sometimes the type
/// (`{name: "y", ty: "int x,"}`, checked separately via
/// `types::type_is_malformed` on the raw `Type` - never on mapped text,
/// which can legitimately contain commas for unrelated reasons, e.g. a
/// function pointer's own parameter list). Emitting either verbatim would
/// place a stray comma inside a declaration's syntax, breaking the
/// surrounding item (for a struct field, the *whole containing struct*) -
/// callers must detect this and degrade to a flagged comment instead of
/// emitting broken code.
fn is_malformed(text: &str) -> bool {
    text.contains(',')
}

fn emit_typedef(td: &TypedefDecl) -> String {
    if is_malformed(&td.name) || type_is_malformed(&td.underlying) {
        let ty = map_type(&td.underlying);
        return format!(
            "// TODO: unparsed multi-declarator typedef, needs manual translation: {} = {ty}\n\n",
            td.name
        );
    }
    format!(
        "pub type {} = {};\n\n",
        ident(&td.name),
        map_type(&td.underlying)
    )
}

/// The Rust name a record/enum's own type should be defined under: prefer
/// the typedef name, else the tag. `None` only for an anonymous, non-
/// typedef'd top-level record/enum - essentially never valid/useful C at
/// file scope (confirmed zero real occurrences in this corpus), handled by
/// callers via a loud fallback comment rather than a panic or a guessed name.
fn primary_name<'a>(typedef_name: &'a Option<String>, tag: &'a Option<String>) -> Option<&'a str> {
    typedef_name.as_deref().or(tag.as_deref())
}

fn emit_record(rd: &RecordDecl) -> String {
    let Some(primary) = primary_name(&rd.typedef_name, &rd.tag) else {
        return format!(
            "/* TODO: anonymous top-level {:?} record with no tag/typedef name, skipped:\n{:#?}\n*/\n\n",
            rd.kind, rd
        );
    };
    let primary = ident(primary);
    let mut out = emit_record_named(&primary, rd);

    // A record can be referenced either via its typedef name or its tag
    // (e.g. p_mobj.h's `mobj_s`'s own `struct mobj_s* snext` field
    // self-references the tag, while external code uses `mobj_t`) -
    // map_type's tag-stripping rule (see codegen::types) resolves a tag
    // reference to the bare tag identifier, so alias it to the same type
    // whenever both spellings exist and differ.
    if let (Some(typedef_name), Some(tag)) = (&rd.typedef_name, &rd.tag) {
        let tag_ident = ident(tag);
        if tag_ident != ident(typedef_name) {
            out.push_str(&format!("pub type {tag_ident} = {primary};\n\n"));
        }
    }
    // Extra declarator names after the closing brace, e.g. `} foo, *foo_p;`
    // - confirmed zero real *extra* occurrences in this corpus, but the AST
    // supports it generically, so handle it rather than silently drop the
    // data. `names` is *every* declarator name, not just extras beyond the
    // typedef - `record::classify_record_or_enum` sets `typedef_name =
    // names.first().cloned()` without removing it from `names` - so the
    // typedef name itself must be skipped here, or it gets a bogus
    // self-referential `pub type X = X;` alias. Also skip anything that
    // isn't identifier-shaped: a bare `};    // comment` with no real extra
    // declarator can leak the trailing `;`/comment text into `names` as
    // one garbage "name" (real corpus case: m_bbox.h's anonymous bbox
    // enum) - not a real declarator, silently dropped as noise (same
    // "whitespace-only Raw item is noise, not content" precedent as
    // `emit_raw`), not worth a TODO comment.
    for extra in &rd.names {
        if rd.typedef_name.as_deref() == Some(extra.as_str()) || !looks_like_identifier(extra) {
            continue;
        }
        out.push_str(&format!("pub type {} = {primary};\n", ident(extra)));
    }
    out
}

/// Emits one `#[repr(C)]` struct/union definition under `name`, plus (before
/// it) any nested anonymous struct/union field's own synthesized definition.
fn emit_record_named(name: &str, rd: &RecordDecl) -> String {
    let mut extra_defs = String::new();
    let mut field_lines = String::new();
    for field in &rd.fields {
        let (extra, line) = emit_field(name, field);
        extra_defs.push_str(&extra);
        field_lines.push_str(&line);
    }
    let keyword = match rd.kind {
        RecordKind::Struct => "struct",
        RecordKind::Union => "union",
    };
    format!(
        "{extra_defs}#[repr(C)]\n#[derive(Copy, Clone)]\npub {keyword} {name} {{\n{field_lines}}}\n\n"
    )
}

/// Returns (extra top-level definitions needed before the parent, this
/// field's own `pub name: Type,` line).
fn emit_field(parent_name: &str, field: &Field) -> (String, String) {
    if is_malformed(&field.name) {
        // See `is_malformed` - the stray comma from an unsplit multi-
        // declarator field can land on the *name* half instead of the
        // type half (e.g. am_map.c's `mpoint_t`: `int x, y;` -> one Field
        // with name "x,y"). Comment the whole field out so the rest of the
        // struct stays syntactically valid.
        return (
            String::new(),
            format!(
                "    // TODO: unparsed multi-declarator field, needs manual translation: {}\n",
                field.name
            ),
        );
    }
    let fname = ident(&field.name);
    let bitfield_comment = match &field.bitfield {
        Some(width) => format!(" // TODO: bitfield width {width}, needs manual packing"),
        None => String::new(),
    };
    match &field.nested {
        Some(nested) => {
            // Anonymous nested struct/union: no name of its own (`nested`'s
            // own tag/typedef_name are always None - see record.rs) -
            // synthesize one from the parent + this field's name (e.g.
            // p_local.h's `intercept_t`'s union field `d` -> `intercept_t_d`)
            // and substitute it for the Named("union")/Named("struct TAG")
            // placeholder `field.ty` carries, preserving any Array/pointer
            // wrapping around it.
            let nested_name = ident(&synthesize_nested_name(parent_name, &field.name));
            let extra = emit_record_named(&nested_name, nested);
            let ty_text = replace_named_leaf(&field.ty, &nested_name);
            (
                extra,
                format!("    pub {fname}: {ty_text},{bitfield_comment}\n"),
            )
        }
        None => {
            if type_is_malformed(&field.ty) {
                // See `type_is_malformed` - comment the whole field out so
                // the rest of the struct stays syntactically valid.
                return (
                    String::new(),
                    format!(
                        "    // TODO: unparsed multi-declarator field, needs manual translation: {fname}: {}\n",
                        map_type(&field.ty)
                    ),
                );
            }
            let ty_text = map_type(&field.ty);
            (
                String::new(),
                format!("    pub {fname}: {ty_text},{bitfield_comment}\n"),
            )
        }
    }
}

/// Same shape as `types::map_type`, but substitutes `replacement` for the
/// base `Named` leaf instead of mapping it - used only for a nested
/// anonymous field's own outer declarator type, whose `Named` payload is a
/// placeholder (`"union"`/`"struct TAG"`) that must become the synthesized
/// nested-type name instead of being mapped normally.
fn replace_named_leaf(ty: &crate::parser::ast::Type, replacement: &str) -> String {
    use crate::parser::ast::Type;
    match ty {
        Type::Named(_) => replacement.to_string(),
        Type::Pointer(inner) => format!("*mut {}", replace_named_leaf(inner, replacement)),
        Type::Array(elem, Some(dim)) => {
            format!("[{}; {dim}]", replace_named_leaf(elem, replacement))
        }
        Type::Array(elem, None) => {
            format!(
                "*mut {} /* TODO: was unsized array */",
                replace_named_leaf(elem, replacement)
            )
        }
        // Not a shape record.rs's nested-field parsing ever produces for a
        // nested field's own declarator type - defensive fallback only.
        Type::FunctionPointer { .. } => replacement.to_string(),
    }
}

fn emit_enum(ed: &EnumDecl) -> String {
    let mut out = String::new();
    let mut prev: Option<String> = None;
    for variant in &ed.variants {
        let name = ident(&variant.name);
        let value_expr = match &variant.value {
            Some(v) => sanitize_int_literal(v),
            None => match &prev {
                // Zero evaluation on the codegen side - rustc's own
                // const-folding resolves the real value, exactly mirroring
                // C's implicit-successor rule.
                Some(p) => format!("{p} + 1"),
                None => "0".to_string(),
            },
        };
        out.push_str(&format!(
            "pub const {name}: std::ffi::c_int = {value_expr};\n"
        ));
        prev = Some(name);
    }
    out.push('\n');
    // Not a real Rust `enum` (C enum constants are never enum-scoped - see
    // the plan's decision on flat/bare consts), so a field/param typed with
    // the enum's own name (typedef or tag) needs a type alias to resolve.
    if let Some(typedef_name) = &ed.typedef_name {
        out.push_str(&format!(
            "pub type {} = std::ffi::c_int;\n",
            ident(typedef_name)
        ));
    }
    if let Some(tag) = &ed.tag
        && ed.typedef_name.as_deref() != Some(tag.as_str())
    {
        out.push_str(&format!("pub type {} = std::ffi::c_int;\n", ident(tag)));
    }
    // `names` is *every* declarator name, including the typedef name itself
    // (see the matching comment in `emit_record` for why) - skip it here
    // too, and skip anything not identifier-shaped (real corpus case:
    // m_bbox.h's anonymous bbox enum, `};    // bbox coordinates` with no
    // real extra declarator, leaks the trailing `;`/comment text into
    // `names` as one garbage "name" - see the matching comment in
    // `emit_record`).
    for extra in &ed.names {
        if ed.typedef_name.as_deref() == Some(extra.as_str()) || !looks_like_identifier(extra) {
            continue;
        }
        out.push_str(&format!("pub type {} = std::ffi::c_int;\n", ident(extra)));
    }
    out.push('\n');
    out
}

fn is_static(storage: &[Storage]) -> bool {
    storage.contains(&Storage::Static)
}

#[allow(clippy::too_many_arguments)]
fn emit_var(
    vd: &VarDecl,
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    known_functions: &HashMap<String, FnSig>,
    known_globals: &HashMap<String, Type>,
    needed_zeroed: &mut BTreeSet<String>,
) -> String {
    if is_malformed(&vd.name) || type_is_malformed(&vd.ty) {
        // See `is_malformed`/`type_is_malformed` - real corpus example:
        // am_map.c's `fixed_t m_x2, m_y2;` (a multi-declarator top-level
        // variable, same parser gap as the struct-field case, just at file
        // scope).
        return format!(
            "// TODO: unparsed multi-declarator variable, needs manual translation: {}: {}\n\n",
            vd.name,
            map_type(&vd.ty)
        );
    }
    let ty = map_type(&vd.ty);
    let name = ident(&vd.name);
    let vis = if is_static(&vd.storage) { "" } else { "pub " };
    // The real declaration-vs-definition signal is the `extern` keyword,
    // *not* whether an explicit initializer is present: a tentative
    // definition (`boolean modifiedgame;`, no `extern`, no `= value`) is
    // still a real definition in C (implicitly zero-initialized storage),
    // not a mere declaration - confirmed via a real corpus build failure
    // (doomstat.h's `extern boolean modifiedgame;` + doomstat.c's
    // `boolean modifiedgame;` both had `initializer: None`, so checking
    // `initializer.is_some()` here treated *both* as declarations and
    // neither as the "real" one, producing a duplicate `static mut`
    // definition - `merge_items`'s dedup rule in `codegen::module` uses the
    // same `Storage::Extern` signal, for the same reason).
    if vd.storage.contains(&Storage::Extern) {
        return format!("unsafe extern \"C\" {{\n    {vis}static mut {name}: {ty};\n}}\n\n");
    }
    if let Some(init) = &vd.initializer {
        if matches!(vd.ty, Type::Array(_, _)) {
            // An `Array`-typed target's real Rust type depends on the
            // initializer itself whenever the C declaration left the
            // dimension unsized (`Array(_, None)`, e.g. `char rcsid[]`/a
            // flat scalar table) - `render_array_init` returns the paired
            // type text instead of the generic `ty` computed above. Covers
            // both a flat/2-D scalar array and an array of struct/union
            // rows (`states[]`/`mobjinfo[]`/...).
            if let Some((array_ty, rendered)) = render_array_init(
                init,
                &vd.ty,
                known,
                known_records,
                known_typedefs,
                known_functions,
                known_globals,
                needed_zeroed,
            ) {
                return format!(
                    "{vis}static mut {name}: {array_ty} = unsafe {{ {rendered} }};\n\n"
                );
            }
        } else if let Init::Expr(text) = init
            && let Some(rendered) = render_scalar_init(
                text,
                &vd.ty,
                known,
                known_typedefs,
                known_functions,
                known_globals,
            )
        {
            // Always wrapped in `unsafe {}`, matching the zeroed-stub path
            // below - a plain literal doesn't need it, but a reference to
            // another `static mut` (real corpus case: `r_main.c`'s
            // `finecosine = &finesine[FINEANGLES/4]`) does, and this codegen
            // has no cheap way to tell the two apart without a much bigger
            // static-vs-const name lookup - an unnecessary `unsafe` block is
            // harmless, an omitted necessary one is a compile error.
            return format!("{vis}static mut {name}: {ty} = unsafe {{ {rendered} }};\n\n");
        } else if matches!(init, Init::Braced(_))
            && let Some(rendered) = render_struct_init(
                init,
                &vd.ty,
                known,
                known_records,
                known_typedefs,
                known_functions,
                known_globals,
                needed_zeroed,
            )
        {
            // A bare struct/union-typed variable with its own brace-init
            // (e.g. `m_menu.c`'s `MainDef`/`st_stuff.c`'s `cheat_god`) -
            // `render_struct_init` returns `None` immediately for any
            // non-record `vd.ty`, so this never fires for an ordinary
            // scalar's own (invalid, but never seen in this corpus) brace-
            // wrapped init.
            return format!("{vis}static mut {name}: {ty} = unsafe {{ {rendered} }};\n\n");
        }
    }
    // Zero matches C's own implicit-zero-initialization for a tentative
    // definition, and stands in as a documented stub when a real explicit
    // initializer exists but isn't translatable yet (an `Init::Conditional`
    // outside an array, or one this codegen genuinely can't parse).
    format!(
        "{vis}static mut {name}: {ty} = unsafe {{ std::mem::zeroed() }}; // TODO: initializer not yet translated\n\n"
    )
}

fn format_params(sig: &FnSig) -> String {
    sig.params
        .iter()
        .map(|p| {
            // Anonymous K&R-style params (Param.name == "") just become `_`
            // - nothing references them since the body is a stub anyway.
            let name = if p.name.is_empty() {
                "_".to_string()
            } else {
                ident(&p.name)
            };
            if type_is_malformed(&p.ty) {
                // See `type_is_malformed` - real corpus example: m_misc.c's
                // M_ReadFile/M_WriteFile take a `char const *` param (a
                // trailing-qualifier shape `map_type` itself flags). Unlike
                // a struct field, a param can't be commented out without
                // breaking the enclosing parenthesized list's syntax -
                // substitute a placeholder type instead, keeping the
                // signature's arity and overall syntax valid.
                format!("{name}: () /* TODO: unparsed param type, needs manual translation */")
            } else {
                let ty = map_type(&p.ty);
                format!("{name}: {ty}")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn emit_function_decl(sig: &FnSig) -> String {
    let name = ident(&sig.name);
    let vis = if is_static(&sig.storage) { "" } else { "pub " };
    let params = format_params(sig);
    let variadic = if sig.variadic { ", ..." } else { "" };
    let ret = format_return_suffix(&sig.ret_ty);
    format!("unsafe extern \"C\" {{\n    {vis}fn {name}({params}{variadic}){ret};\n}}\n\n")
}

fn emit_function_def(sig: &FnSig) -> String {
    let name = ident(&sig.name);
    let vis = if is_static(&sig.storage) { "" } else { "pub " };
    let params = format_params(sig);
    let ret = format_return_suffix(&sig.ret_ty);
    // Rust doesn't support C-variadic function *definitions* (only extern
    // declarations) - the corpus's one real case (I_Error) drops `...`
    // rather than failing to compile the whole module.
    let variadic_comment = if sig.variadic {
        " // TODO: variadic definition not supported, C variadic marker dropped"
    } else {
        ""
    };
    // Spike step (per the function-body codegen plan): verify the
    // unsafe-wrapping + trailing-todo shape alone doesn't move the
    // `cargo build -p doom_rs` baseline before building the real
    // statement renderer on top of it.
    let tail = if ret.is_empty() {
        String::new()
    } else {
        "todo!(\"fell off the end of a non-void C function\")\n".to_string()
    };
    format!(
        "{vis}unsafe extern \"C\" fn {name}({params}){ret} {{\nunsafe {{ todo!(\"body not yet translated\") }}\n{tail}}}{variadic_comment}\n\n"
    )
}

#[allow(clippy::too_many_arguments)]
fn emit_conditional(
    group: &CondGroup,
    known: &KnownTypeNames,
    known_records: &HashMap<String, RecordDecl>,
    known_typedefs: &HashMap<String, Type>,
    known_functions: &HashMap<String, FnSig>,
    known_globals: &HashMap<String, Type>,
    known_defines: &HashMap<String, String>,
    needed_zeroed: &mut BTreeSet<String>,
) -> String {
    match group.active {
        ActiveBranch::Branch(n) => emit_items_inner(
            &group.branches[n].body,
            known,
            known_records,
            known_typedefs,
            known_functions,
            known_globals,
            known_defines,
            needed_zeroed,
        ),
        ActiveBranch::Else => group
            .else_body
            .as_ref()
            .map(|b| {
                emit_items_inner(
                    b,
                    known,
                    known_records,
                    known_typedefs,
                    known_functions,
                    known_globals,
                    known_defines,
                    needed_zeroed,
                )
            })
            .unwrap_or_default(),
        ActiveBranch::None => String::new(),
        ActiveBranch::Unknown => {
            "// TODO: unresolved #if condition, needs manual review\n\n".to_string()
        }
    }
}

fn emit_raw(raw: &str) -> String {
    if raw.trim().is_empty() {
        // The parser's own "leftover trailing whitespace becomes its own
        // final catch-all item" pattern (see record::build_items) - an
        // empty Raw item is a synthetic placeholder holding trailing
        // trivia, not unparsed C content, so there's nothing to flag.
        return String::new();
    }
    format!("/* TODO: unparsed C construct, needs manual translation:\n{raw}\n*/\n\n")
}

#[cfg(test)]
mod tests;
