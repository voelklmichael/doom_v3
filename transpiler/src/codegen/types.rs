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
        Type::Array(elem, Some(dim)) => format!("[{}; {dim}]", map_type(elem)),
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
fn normalize(name: &str) -> String {
    name.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn map_named(name: &str) -> String {
    match normalize(name).as_str() {
        "void" => "()".to_string(),
        "int" | "signed" | "signed int" => "std::ffi::c_int".to_string(),
        "unsigned" | "unsigned int" => "std::ffi::c_uint".to_string(),
        "char" => "std::ffi::c_char".to_string(),
        "signed char" => "std::ffi::c_schar".to_string(),
        "unsigned char" => "std::ffi::c_uchar".to_string(),
        "short" | "short int" | "signed short" | "signed short int" => {
            "std::ffi::c_short".to_string()
        }
        "unsigned short" | "unsigned short int" => "std::ffi::c_ushort".to_string(),
        "long" | "long int" | "signed long" | "signed long int" => "std::ffi::c_long".to_string(),
        "unsigned long" | "unsigned long int" => "std::ffi::c_ulong".to_string(),
        "long long" | "long long int" | "signed long long" => "std::ffi::c_longlong".to_string(),
        "unsigned long long" | "unsigned long long int" => "std::ffi::c_ulonglong".to_string(),
        "float" => "std::ffi::c_float".to_string(),
        "double" => "std::ffi::c_double".to_string(),
        // Not a builtin - one of this corpus's own typedefs/tags (or a raw
        // unparsed-text fallback blob) - pass straight through verbatim.
        other => ident(other),
    }
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
