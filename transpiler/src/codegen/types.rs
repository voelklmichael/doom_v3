//! `ast::Type` -> Rust type-text mapping. Unsafe transliteration, not
//! idiomatic Rust: pointers stay raw pointers, base types map to `std::ffi`
//! C-ABI aliases, and an unrecognized base name (any of this corpus's own
//! hundreds of typedef'd/struct-tagged types, e.g. `mobj_t`) passes straight
//! through verbatim as the same (keyword-escaped) Rust identifier - resolved
//! at compile time via the `use crate::other_mod::*;` glob imports codegen
//! emits per module (see `codegen::module`, added in a later PR), or a plain
//! compile error if genuinely undefined. This mirrors how the parser itself
//! never guesses past what's decidable - an unmapped name is a visible
//! compile error, not a silently wrong guess.

use super::ident::ident;
use crate::parser::ast::Type;

/// Maps a C `Type` to Rust type text.
///
/// Pointers are always `*mut` (never `*const`) this phase - `Storage::Const`
/// doesn't distinguish pointer-const from pointee-const in this AST yet, so
/// there is no sound basis to place `*const` correctly.
///
/// `Array(T, Some(dim))` reuses `dim`'s raw C text verbatim as a Rust
/// const-expression - this will not compile when `dim` is a macro name
/// (e.g. `NUMMOBJTYPES`), since macros are out of scope this phase; an
/// expected, accepted gap, not something this function tries to work around.
///
/// `Array(T, None)` (an unsized array, e.g. `int xs[];`) has no Rust
/// value-type equivalent, so it falls back to `*mut T` with an inline
/// block comment flagging the substitution.
pub fn map_type(ty: &Type) -> String {
    match ty {
        Type::Named(name) => map_named(name),
        Type::Pointer(inner) => {
            if is_void(inner) {
                "*mut std::ffi::c_void".to_string()
            } else {
                format!("*mut {}", map_type(inner))
            }
        }
        // `dim` is reused verbatim as a const-expr, but Rust array lengths
        // must be `usize` specifically - this corpus's dims are usually
        // either a plain integer literal (untyped, infers fine either way)
        // or a symbolic name (typically one of our own `std::ffi::c_int`-
        // typed enum constants, e.g. `NUMAMMO`) that would otherwise fail
        // to typecheck (`expected usize, found i32`) - a real, deterministic
        // corpus-wide error class, not a deferred-feature gap, confirmed via
        // a real `cargo build -p doom_rs` run. `as usize` is a legal const-
        // context cast either way and fixes both cases uniformly.
        Type::Array(elem, Some(dim)) => format!("[{}; ({dim}) as usize]", map_type(elem)),
        Type::Array(elem, None) => {
            format!("*mut {} /* TODO: was unsized array */", map_type(elem))
        }
        Type::FunctionPointer { ret, params } => {
            let params = params.iter().map(map_type).collect::<Vec<_>>().join(", ");
            format!(
                "Option<unsafe extern \"C\" fn({params}){}>",
                format_return_suffix(ret)
            )
        }
    }
}

/// Formats a function's return-type suffix: `""` for `void` (so callers omit
/// `-> ()` entirely, matching idiomatic Rust style even though this is a
/// transliteration elsewhere), else `" -> {mapped}"`.
pub fn format_return_suffix(ret: &Type) -> String {
    if is_void(ret) {
        String::new()
    } else {
        format!(" -> {}", map_type(ret))
    }
}

fn is_void(ty: &Type) -> bool {
    matches!(ty, Type::Named(name) if normalize(name) == "void")
}

/// A C base-type name can be multiple words (`"unsigned char"`,
/// `"short int"`) and `decl::parse_type_text` keeps whatever internal
/// whitespace the source happened to have (only trailing `*`/space get
/// trimmed) - normalize to single-space-separated words before matching
/// against the builtin table, so irregular source spacing can't cause a
/// real builtin to fall through to the passthrough case.
pub(crate) fn normalize(name: &str) -> String {
    name.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Strips a leading `struct `/`union `/`enum ` keyword from a tag-based type
/// reference (already-`normalize`d, single-space-separated text), e.g.
/// `"struct mobj_s"` -> `Some("mobj_s")`. Returns `None` for a bare
/// `"struct"`/`"union"` with no following tag (that shape only appears as
/// `record::push_nested_record_field`'s synthesized placeholder for an
/// *anonymous* nested field, which codegen substitutes with a synthesized
/// name before ever calling `map_type` - see `codegen::items::emit_field`
/// - so it should never actually reach this function).
pub(crate) fn strip_tag_keyword(normalized: &str) -> Option<&str> {
    for kw in ["struct ", "union ", "enum "] {
        if let Some(rest) = normalized.strip_prefix(kw) {
            let rest = rest.trim();
            if !rest.is_empty() {
                return Some(rest);
            }
        }
    }
    None
}

fn map_named(name: &str) -> String {
    match classify_named(name) {
        Some(mapped) => mapped,
        // Not recognized as any known shape (see `classify_named`) - e.g.
        // this corpus's one real `"char const"` case (a trailing `const`
        // qualifier `decl.rs` only recognizes when *leading*, so it stays
        // glued into the type text instead of being separated into
        // `storage`). Emitting a bare keyword like `const` in the middle of
        // a type position doesn't just fail to typecheck, it fails to
        // *parse*, which can cascade into every later item in the same
        // file - flag it via an embedded comment+comma rather than emit it
        // verbatim. `codegen::items` never needs to detect *this* signal in
        // the final mapped text, though (which could legitimately contain
        // commas for unrelated reasons, e.g. a `FunctionPointer`'s own
        // parameter list) - it calls `type_is_malformed` instead, which
        // checks the same `classify_named` logic against the raw `Type`
        // tree's `Named` leaves directly.
        None => format!("/* unrecognized type: {} */,", normalize(name)),
    }
}

/// Attempts to classify and map a single `Type::Named` leaf's text.
/// `None` for either of two malformed shapes, never guessed at:
/// 1. A comma - the signature of `record::parse_fields`/
///    `decl::parse_declarator` failing to split a real multi-declarator
///    declaration (e.g. `long misc1, misc2;`) into separate declarations,
///    leaving one `Type::Named` with the next declarator's text embedded.
/// 2. Text that isn't a recognized builtin/tag shape *and* isn't a single
///    clean identifier either (e.g. `"char const"` - see `map_named`).
fn classify_named(name: &str) -> Option<String> {
    let normalized = normalize(name);
    if normalized.contains(',') {
        return None;
    }
    // A tag-based type reference (e.g. `struct mobj_s`) keeps the `struct`/
    // `union`/`enum` keyword glued into the Named text verbatim (the parser
    // never strips it - `decl::parse_type_text` keeps whatever precedes the
    // trailing `*`s as-is). This is common in this corpus: self-referential
    // linked-structure fields are routinely declared via the tag, not the
    // typedef, because the typedef isn't complete yet at that point in the
    // struct's own body (e.g. p_mobj.h's `mobj_s`'s own `struct mobj_s*
    // snext` field, inside `mobj_s`'s own definition). Codegen emits a type
    // alias from the bare tag name to the real struct/union/enum definition
    // whenever both a tag and a typedef name exist (see `codegen::items`),
    // so stripping the keyword here and using just the tag identifier
    // resolves correctly.
    if let Some(tag) = strip_tag_keyword(&normalized) {
        return Some(ident(tag));
    }
    let builtin = match normalized.as_str() {
        "void" => "()",
        "int" | "signed" | "signed int" => "std::ffi::c_int",
        "unsigned" | "unsigned int" => "std::ffi::c_uint",
        "char" => "std::ffi::c_char",
        "signed char" => "std::ffi::c_schar",
        "unsigned char" => "std::ffi::c_uchar",
        "short" | "short int" | "signed short" | "signed short int" => "std::ffi::c_short",
        "unsigned short" | "unsigned short int" => "std::ffi::c_ushort",
        "long" | "long int" | "signed long" | "signed long int" => "std::ffi::c_long",
        "unsigned long" | "unsigned long int" => "std::ffi::c_ulong",
        "long long" | "long long int" | "signed long long" => "std::ffi::c_longlong",
        "unsigned long long" | "unsigned long long int" => "std::ffi::c_ulonglong",
        "float" => "std::ffi::c_float",
        "double" => "std::ffi::c_double",
        // Not a builtin - one of this corpus's own typedefs/tags (or a raw
        // unparsed-text fallback blob) - pass straight through verbatim,
        // *if* it actually looks like a single clean identifier.
        other => {
            return if looks_like_identifier(other) {
                Some(ident(other))
            } else {
                None
            };
        }
    };
    Some(builtin.to_string())
}

/// True if `ty` contains a `Type::Named` leaf `map_type` can't map cleanly
/// (see `classify_named`'s two cases). Checked structurally against the raw
/// `Type` tree - recursing into `Pointer`/`Array`/`FunctionPointer` -
/// *never* against final mapped text, since that can legitimately contain
/// commas for unrelated reasons (e.g. a `FunctionPointer`'s own parameter
/// list joined with `", "` - checking the composed *output* string instead
/// of the raw *input* leaves was a real bug: it flagged `d_think.h`'s
/// perfectly valid `actionf_p2` typedef, whose underlying
/// `Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>` mapping just
/// happens to contain a comma). Callers (`codegen::items`) use this to
/// decide whether to emit a declaration normally or degrade to a flagged
/// comment.
pub fn type_is_malformed(ty: &Type) -> bool {
    match ty {
        Type::Named(name) => classify_named(name).is_none(),
        Type::Pointer(inner) => type_is_malformed(inner),
        Type::Array(elem, _) => type_is_malformed(elem),
        Type::FunctionPointer { ret, params } => {
            type_is_malformed(ret) || params.iter().any(type_is_malformed)
        }
    }
}

/// Whether `s` is a single, plain identifier-shaped token (starts with a
/// letter/underscore, all remaining characters alphanumeric/underscore) -
/// i.e. genuinely safe to pass through as a Rust identifier. A `Named`
/// string that fails this after every known builtin/tag shape has already
/// been tried is not a real type name, just leftover unparsed text.
/// `pub` since `codegen::items` reuses it for the same kind of validation
/// on `RecordDecl.names`/`EnumDecl.names` entries (also raw parser-provided
/// strings that aren't guaranteed to be real identifiers - see
/// `record::split_names`'s doc comment / the real `m_bbox.h` case where a
/// bare `};    // comment` with no real extra declarator leaks the trailing
/// `;` and comment text into `names` as a single garbage "name").
pub fn looks_like_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Strips trailing C integer-literal suffixes (`u`/`U`/`l`/`L` in any
/// combination, e.g. `UL`/`ull`) that aren't valid Rust integer-literal
/// syntax (Rust has no literal suffixes for these). No real corpus enum
/// variant value currently has one (verified directly against the whole
/// corpus), added defensively since it's a one-function, zero-risk addition
/// that keeps `Enum` codegen correct if that ever changes.
pub fn sanitize_int_literal(text: &str) -> String {
    text.trim()
        .trim_end_matches(['u', 'U', 'l', 'L'])
        .to_string()
}

#[cfg(test)]
mod tests;
