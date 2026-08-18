use super::{emit_define_function, emit_define_object};
use crate::parser::stmt::expr::KnownTypeNames;

fn known() -> KnownTypeNames {
    KnownTypeNames::new()
}

#[test]
fn empty_value_flag_macro_emits_nothing() {
    assert_eq!(emit_define_object("RANGECHECK", "", &known()), "");
}

#[test]
fn simple_int_const() {
    assert_eq!(
        emit_define_object("MAXPLAYERS", "4", &known()),
        "pub const MAXPLAYERS: std::ffi::c_int = 4;\n\n"
    );
}

#[test]
fn hex_const_with_suffix() {
    assert_eq!(
        emit_define_object("NCMD_KILL", "0x10000000", &known()),
        "pub const NCMD_KILL: std::ffi::c_int = 0x10000000;\n\n"
    );
}

#[test]
fn float_const_gets_double_type() {
    assert_eq!(
        emit_define_object("INV_ASPECT_RATIO", "0.625", &known()),
        "pub const INV_ASPECT_RATIO: std::ffi::c_double = 0.625;\n\n"
    );
}

#[test]
fn bare_float_arithmetic_infers_double_not_int() {
    // am_map.c's real INITSCALEMTOF: `(.2*FRACUNIT)` - no top-level Cast/
    // FloatLit of its own for `infer_scalar_type`'s shallow check to see
    // directly (it's a Binary node), but it's genuinely double-typed in C
    // via the usual arithmetic conversions (mixing float and int promotes
    // the whole expression to float) - must not default to c_int.
    assert_eq!(
        emit_define_object("INITSCALEMTOF", "(.2*FRACUNIT)", &known()),
        "pub const INITSCALEMTOF: std::ffi::c_double = ((0.2 * ((FRACUNIT) as f64)));\n\n"
    );
}

#[test]
fn bare_sizeof_division_gets_a_truncating_cast() {
    // am_map.c's real NUMPLYRLINES: `(sizeof(player_arrow)/sizeof(mline_t))`
    // - no explicit cast, so it infers (correctly) as std::ffi::c_int, but
    // `std::mem::size_of[_val]` always produces a `usize`, which doesn't
    // typecheck against that const's own declared `c_int` type without an
    // explicit cast. `mline_t` must be a known type name for `sizeof` to
    // parse it as `SizeofArg::Type` rather than a plain identifier
    // expression - mirrors how the real corpus's own `KnownTypeNames`
    // environment (via `am_map.c`'s own typedef) would resolve it.
    let mut k = known();
    k.insert("mline_t");
    assert_eq!(
        emit_define_object("NUMPLYRLINES", "(sizeof(player_arrow)/sizeof(mline_t))", &k),
        "pub const NUMPLYRLINES: std::ffi::c_int = (((std::mem::size_of_val(&((player_arrow))) / std::mem::size_of::<mline_t>()))) as std::ffi::c_int;\n\n"
    );
}

#[test]
fn char_const_gets_c_int_type() {
    // A C char literal is int-typed, not char-typed (integer promotion
    // applies to the literal itself) - see `render_expr`'s `CharLit` case.
    assert_eq!(
        emit_define_object("AM_ZOOMINKEY", "'='", &known()),
        "pub const AM_ZOOMINKEY: std::ffi::c_int = (b'=' as std::ffi::c_int);\n\n"
    );
}

#[test]
fn string_const_gets_c_char_ptr_type() {
    assert_eq!(
        emit_define_object("STSTR_FAADDED", "\"Ammo Added\"", &known()),
        "pub const STSTR_FAADDED: *const std::ffi::c_char = (c\"Ammo Added\").as_ptr();\n\n"
    );
}

#[test]
fn cast_shaped_value_uses_cast_target_type() {
    // Mirrors am_map.c's M_ZOOMIN: `((int) (1.02*FRACUNIT))`.
    assert_eq!(
        emit_define_object("M_ZOOMIN", "((int) (1*FRACUNIT))", &known()),
        "pub const M_ZOOMIN: std::ffi::c_int = (((((1 * FRACUNIT))) as std::ffi::c_int));\n\n"
    );
}

#[test]
fn arithmetic_referencing_other_macros() {
    // Mirrors am_map.c's GRIDCOLORS: `(GRAYS + GRAYSRANGE/2)`.
    assert_eq!(
        emit_define_object("GRIDCOLORS", "(GRAYS + GRAYSRANGE/2)", &known()),
        "pub const GRIDCOLORS: std::ffi::c_int = ((GRAYS + (GRAYSRANGE / 2)));\n\n"
    );
}

#[test]
fn keyword_colliding_name_gets_escaped() {
    assert_eq!(
        emit_define_object("type", "1", &known()),
        "pub const type_: std::ffi::c_int = 1;\n\n"
    );
}

// ---- emit_define_function ----

#[test]
fn simple_expression_macro_becomes_a_function() {
    // am_map.c's MTOF: `(FixedMul((x),scale_mtof)>>16)`.
    let out = emit_define_function(
        "MTOF",
        &["x".to_string()],
        "(FixedMul((x),scale_mtof)>>16)",
        &known(),
    );
    assert_eq!(
        out,
        "pub unsafe extern \"C\" fn MTOF(x: std::ffi::c_int) -> std::ffi::c_int { ((FixedMul((x), scale_mtof) >> 16)) }\n\n"
    );
}

#[test]
fn cast_shaped_body_infers_return_type_from_cast() {
    // m_swap.h's SHORT: `((short)SwapSHORT((unsigned short) (x)))`.
    let out = emit_define_function(
        "SHORT",
        &["x".to_string()],
        "((short)SwapSHORT((unsigned short) (x)))",
        &known(),
    );
    assert!(
        out.starts_with(
            "pub unsafe extern \"C\" fn SHORT(x: std::ffi::c_int) -> std::ffi::c_short"
        ),
        "got: {out}"
    );
}

#[test]
fn zero_param_macro_mutating_a_global_still_emits() {
    // p_saveg.c's PADSAVEP: `save_p += (4 - ((int) save_p & 3)) & 3`.
    let out = emit_define_function(
        "PADSAVEP",
        &[],
        "save_p += (4 - ((int) save_p & 3)) & 3",
        &known(),
    );
    assert!(out.starts_with("pub unsafe extern \"C\" fn PADSAVEP() -> std::ffi::c_int {"));
    assert!(out.contains("save_p +="));
}

#[test]
fn assigned_param_gets_mut_binding() {
    // am_map.c's PUTDOT: `fb[(yy)*f_w+(xx)]=(cc)` - doesn't reassign a
    // param itself, so this is a synthetic case exercising the `mut`
    // inference directly: a param assigned to in the body needs `mut`.
    let out = emit_define_function("SET", &["x".to_string()], "x = 1", &known());
    assert!(out.contains("fn SET(mut x: std::ffi::c_int)"), "got: {out}");
}

#[test]
fn pointer_cast_param_gets_pointer_type() {
    // z_zone.h's Z_ChangeTag casts `p` to `(memblock_t *)` - synthesized
    // here as a single-expression body to isolate the pointer-param
    // inference from the (separate) statement-shaped-body rejection below.
    // `memblock_t` must be in `known` for cast-vs-paren disambiguation to
    // even recognize `(memblock_t *)` as a cast rather than an expression
    // (same requirement real function-body parsing already has).
    let mut known = known();
    known.insert("memblock_t");
    let out = emit_define_function(
        "TAGOF",
        &["p".to_string()],
        "((memblock_t *)(p))->id",
        &known,
    );
    assert!(
        out.contains("fn TAGOF(p: *mut std::ffi::c_void)"),
        "got: {out}"
    );
}

#[test]
fn statement_shaped_body_is_flagged_not_mis_parsed() {
    // am_map.c's real DOOUTCODE body - an if/else-if chain, not a single
    // expression. Must not silently emit a function containing only the
    // first statement.
    let out = emit_define_function(
        "DOOUTCODE",
        &["oc".to_string(), "mx".to_string(), "my".to_string()],
        "(oc) = 0; if ((my) < 0) (oc) |= 1; else if ((my) >= 2) (oc) |= 2;",
        &known(),
    );
    assert!(out.contains("TODO"));
    assert!(!out.contains("fn DOOUTCODE"));
}

#[test]
fn block_shaped_body_is_flagged_not_mis_parsed() {
    // z_zone.h's real Z_ChangeTag body starts with `{`.
    let out = emit_define_function(
        "Z_ChangeTag",
        &["p".to_string(), "t".to_string()],
        "{ if (1) I_Error(\"x\"); Z_ChangeTag2(p,t); }",
        &known(),
    );
    assert!(out.contains("TODO"));
    assert!(!out.contains("fn Z_ChangeTag"));
}

#[test]
fn line_continued_body_is_joined_before_parsing() {
    // i_net.c's real ntohl - a line-continued arithmetic expression (no
    // string literals involved, unlike the multi-line macros that already
    // worked before this fix). Must parse and render as one function, not
    // degrade to a TODO comment.
    let out = emit_define_function(
        "ntohl",
        &["x".to_string()],
        "((unsigned long int)((((unsigned long int)(x) & 0x000000ffU) << 24) | \\\n(((unsigned long int)(x) & 0x0000ff00U) << 8)))",
        &known(),
    );
    assert!(
        out.starts_with("pub unsafe extern \"C\" fn ntohl"),
        "got: {out}"
    );
    assert!(!out.contains('\\'), "got: {out}");
}

#[test]
fn multiline_fallback_comment_is_block_style_not_line_style() {
    // A `//` line comment would only cover the first physical line, leaking
    // every line after it as raw (almost certainly invalid) Rust source -
    // this is a real bug that hit i_net.c's ntohl/ntohs before both the
    // line-continuation join above and this fix landed. Force the fallback
    // path with a genuinely statement-shaped, multi-line body.
    let out = emit_define_function(
        "DOOUTCODE",
        &["oc".to_string()],
        "(oc) = 0;\nif (1) (oc) |= 1;",
        &known(),
    );
    assert!(out.trim_start().starts_with("/*"), "got: {out}");
    assert!(out.trim_end().ends_with("*/"), "got: {out}");
}
