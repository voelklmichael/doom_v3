use super::emit_define_object;
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
