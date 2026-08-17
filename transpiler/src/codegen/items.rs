//! Per-`ItemKind` Rust emission rules - the core of the codegen "skeleton"
//! phase. Consumes a single already-parsed `ast::File`'s items directly (no
//! cross-file merge yet - see `codegen::module`, a later PR); cross-module
//! type references just pass through as bare names for now (`map_type`
//! already handles that uniformly - resolving them via `use` imports is a
//! module-assembly concern, not an item-emission one).

use super::ident::{ident, synthesize_nested_name};
use super::types::{format_return_suffix, map_type, sanitize_int_literal};
use crate::parser::ast::{
    ActiveBranch, CondGroup, EnumDecl, Field, FnSig, Item, ItemKind, RecordDecl, RecordKind,
    Storage, TypedefDecl, VarDecl,
};

/// Emits Rust source text for one top-level `Item`. Never panics, never
/// silently drops data: an unresolved conditional or an unparsed `Raw` item
/// becomes a loudly-flagged comment rather than being skipped invisibly.
pub fn emit_item(item: &Item) -> String {
    match &item.kind {
        ItemKind::Typedef(td) => emit_typedef(td),
        ItemKind::Record(rd) => emit_record(rd),
        ItemKind::Enum(ed) => emit_enum(ed),
        ItemKind::Var(vd) => emit_var(vd),
        ItemKind::FunctionDecl(sig) => emit_function_decl(sig),
        ItemKind::FunctionDef(sig, _body) => emit_function_def(sig),
        ItemKind::Conditional(group) => emit_conditional(group),
        ItemKind::Raw => emit_raw(&item.raw),
        // #include has no Rust equivalent (cross-module refs are handled via
        // `use` imports at module-assembly time, see codegen::module); bare
        // #define/#undef/#pragma/#error/#other are macros, out of scope this
        // phase (any downstream use of one becomes a visible compile error,
        // an acceptable signal for this phase rather than a silent gap).
        ItemKind::Preproc(_) => String::new(),
    }
}

fn emit_typedef(td: &TypedefDecl) -> String {
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
    // self-referential `pub type X = X;` alias.
    for extra in &rd.names {
        if rd.typedef_name.as_deref() == Some(extra.as_str()) {
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
            let ty_text = map_type(&field.ty);
            if ty_text.contains(',') {
                // A comma inside a mapped type means `record::parse_fields`
                // couldn't split a real multi-declarator field (e.g.
                // info.h's `long misc1, misc2;`) into separate `Field`s -
                // a known, rare (5 occurrences corpus-wide) parser gap, not
                // a codegen bug. Emitting the raw text as-is would break
                // the *whole containing struct's* Rust syntax (a stray
                // comma inside a field's type position derails brace/comma
                // parsing for every field after it), not just this one
                // field - comment the whole field out instead so the rest
                // of the struct stays syntactically valid.
                return (
                    String::new(),
                    format!(
                        "    // TODO: unparsed multi-declarator field, needs manual translation: {fname}: {ty_text}\n"
                    ),
                );
            }
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
    // (see the matching comment in `emit_record` for why) - skip it here too.
    for extra in &ed.names {
        if ed.typedef_name.as_deref() == Some(extra.as_str()) {
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

fn emit_var(vd: &VarDecl) -> String {
    let name = ident(&vd.name);
    let ty = map_type(&vd.ty);
    let vis = if is_static(&vd.storage) { "" } else { "pub " };
    match &vd.initializer {
        None => format!("unsafe extern \"C\" {{\n    {vis}static mut {name}: {ty};\n}}\n\n"),
        Some(_) => format!(
            "{vis}static mut {name}: {ty} = unsafe {{ std::mem::zeroed() }}; // TODO: initializer not yet translated\n\n"
        ),
    }
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
            format!("{name}: {}", map_type(&p.ty))
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
    format!(
        "{vis}unsafe extern \"C\" fn {name}({params}){ret} {{ todo!(\"body not yet translated\") }}{variadic_comment}\n\n"
    )
}

fn emit_conditional(group: &CondGroup) -> String {
    match group.active {
        ActiveBranch::Branch(n) => emit_items(&group.branches[n].body),
        ActiveBranch::Else => group
            .else_body
            .as_ref()
            .map(|b| emit_items(b))
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

/// Emits every item in `items` in order, concatenated - the driver both
/// `codegen::mod`'s whole-module emission and `emit_conditional`'s recursive
/// branch-body emission share.
pub fn emit_items(items: &[(Item, crate::parser::ast::Trivia)]) -> String {
    items.iter().map(|(item, _)| emit_item(item)).collect()
}

#[cfg(test)]
mod tests;
