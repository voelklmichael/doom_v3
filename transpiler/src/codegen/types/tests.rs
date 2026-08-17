use super::*;
use crate::parser::ast::Type;

fn named(s: &str) -> Type {
    Type::Named(s.to_string())
}

#[test]
fn void_return_position_maps_to_unit() {
    assert_eq!(map_type(&named("void")), "()");
}

#[test]
fn void_pointer_maps_to_c_void_not_unit() {
    assert_eq!(
        map_type(&Type::Pointer(Box::new(named("void")))),
        "*mut std::ffi::c_void"
    );
}

#[test]
fn builtin_base_types() {
    assert_eq!(map_type(&named("int")), "std::ffi::c_int");
    assert_eq!(map_type(&named("char")), "std::ffi::c_char");
    assert_eq!(map_type(&named("unsigned")), "std::ffi::c_uint");
    assert_eq!(map_type(&named("unsigned int")), "std::ffi::c_uint");
    assert_eq!(map_type(&named("short")), "std::ffi::c_short");
    assert_eq!(map_type(&named("unsigned short")), "std::ffi::c_ushort");
    assert_eq!(map_type(&named("long")), "std::ffi::c_long");
    assert_eq!(map_type(&named("unsigned long")), "std::ffi::c_ulong");
    assert_eq!(map_type(&named("float")), "std::ffi::c_float");
    assert_eq!(map_type(&named("double")), "std::ffi::c_double");
    assert_eq!(map_type(&named("unsigned char")), "std::ffi::c_uchar");
    assert_eq!(map_type(&named("signed char")), "std::ffi::c_schar");
}

#[test]
fn irregular_internal_whitespace_still_matches_builtin_table() {
    // decl::parse_type_text only trims trailing */space, so a base type's
    // internal whitespace is whatever the source happened to have.
    assert_eq!(map_type(&named("unsigned   char")), "std::ffi::c_uchar");
    assert_eq!(map_type(&named("short  int")), "std::ffi::c_short");
}

#[test]
fn unrecognized_name_passes_through_verbatim() {
    // Corpus's own typedefs/tags, e.g. mobj_t, player_t, fixed_t.
    assert_eq!(map_type(&named("mobj_t")), "mobj_t");
    assert_eq!(map_type(&named("player_t")), "player_t");
    assert_eq!(map_type(&named("boolean")), "boolean");
}

#[test]
fn unrecognized_name_colliding_with_a_keyword_gets_escaped() {
    assert_eq!(map_type(&named("type")), "type_");
}

#[test]
fn tag_based_type_reference_strips_the_c_keyword() {
    // Real corpus shape (49 occurrences), e.g. p_mobj.h's `mobj_s` struct
    // has a self-referential `struct mobj_s* snext;` field, declared via the
    // tag since the `mobj_t` typedef isn't complete yet at that point.
    assert_eq!(map_type(&named("struct mobj_s")), "mobj_s");
    assert_eq!(map_type(&named("union foo")), "foo");
    assert_eq!(map_type(&named("enum weapontype_e")), "weapontype_e");
    assert_eq!(
        map_type(&Type::Pointer(Box::new(named("struct mobj_s")))),
        "*mut mobj_s"
    );
}

#[test]
fn tag_based_type_reference_with_keyword_colliding_tag_gets_escaped() {
    assert_eq!(map_type(&named("struct type")), "type_");
}

#[test]
fn bare_struct_or_union_with_no_tag_is_not_treated_as_tag_reference() {
    // Only reached in practice via record.rs's synthesized nested-field
    // placeholder, which codegen's item-emission substitutes before calling
    // map_type - but map_type itself must still degrade sanely if it ever
    // does see this shape, rather than producing an empty identifier.
    assert_eq!(map_type(&named("union")), "union");
    assert_eq!(map_type(&named("struct")), "struct_");
}

#[test]
fn pointer_is_always_mut_never_const() {
    assert_eq!(
        map_type(&Type::Pointer(Box::new(named("char")))),
        "*mut std::ffi::c_char"
    );
    assert_eq!(
        map_type(&Type::Pointer(Box::new(Type::Pointer(Box::new(named(
            "char"
        )))))),
        "*mut *mut std::ffi::c_char"
    );
}

#[test]
fn sized_array_reuses_dim_verbatim() {
    assert_eq!(
        map_type(&Type::Array(Box::new(named("int")), Some("4".to_string()))),
        "[std::ffi::c_int; 4]"
    );
}

#[test]
fn multi_dimensional_array_preserves_outermost_first_order() {
    // C's `int m[3][4]` parses as Array(Array(Named("int"), Some("4")),
    // Some("3")) - "array of 3 (array of 4 int)". Rust's own array nesting
    // reads in the same outermost-first bracket order, so this must come
    // out as [[c_int; 4]; 3], NOT [[c_int; 3]; 4] (a silent dimension-order
    // flip would be a real, easy-to-miss bug).
    let ty = Type::Array(
        Box::new(Type::Array(Box::new(named("int")), Some("4".to_string()))),
        Some("3".to_string()),
    );
    assert_eq!(map_type(&ty), "[[std::ffi::c_int; 4]; 3]");
}

#[test]
fn unsized_array_falls_back_to_pointer_with_a_comment() {
    let out = map_type(&Type::Array(Box::new(named("int")), None));
    assert!(out.starts_with("*mut std::ffi::c_int"));
    assert!(out.contains("TODO"));
}

#[test]
fn function_pointer_maps_to_optional_extern_fn() {
    let ty = Type::FunctionPointer {
        ret: Box::new(named("void")),
        params: vec![],
    };
    assert_eq!(map_type(&ty), "Option<unsafe extern \"C\" fn()>");
}

#[test]
fn function_pointer_with_params_and_non_void_return() {
    let ty = Type::FunctionPointer {
        ret: Box::new(named("int")),
        params: vec![
            Type::Pointer(Box::new(named("void"))),
            Type::Pointer(Box::new(named("void"))),
        ],
    };
    assert_eq!(
        map_type(&ty),
        "Option<unsafe extern \"C\" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> std::ffi::c_int>"
    );
}

#[test]
fn format_return_suffix_omits_void() {
    assert_eq!(format_return_suffix(&named("void")), "");
    assert_eq!(format_return_suffix(&named("int")), " -> std::ffi::c_int");
}

#[test]
fn sanitize_int_literal_strips_c_suffixes() {
    assert_eq!(sanitize_int_literal("5"), "5");
    assert_eq!(sanitize_int_literal("5u"), "5");
    assert_eq!(sanitize_int_literal("5U"), "5");
    assert_eq!(sanitize_int_literal("5L"), "5");
    assert_eq!(sanitize_int_literal("5UL"), "5");
    assert_eq!(sanitize_int_literal("5ull"), "5");
    assert_eq!(sanitize_int_literal("0xffffffffu"), "0xffffffff");
    assert_eq!(sanitize_int_literal("0xDEADBEEFL"), "0xDEADBEEF");
}

#[test]
fn sanitize_int_literal_leaves_plain_hex_digits_alone() {
    // 'f'/'F'/'e'/'E' etc. are valid hex digits, not suffix characters -
    // must not be stripped.
    assert_eq!(sanitize_int_literal("0xFEED"), "0xFEED");
    assert_eq!(sanitize_int_literal("0xCAFE"), "0xCAFE");
}
