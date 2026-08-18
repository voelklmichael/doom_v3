use super::{render_array_init, render_scalar_init};
use crate::parser::ast::{Init, Type};
use crate::parser::stmt::expr::KnownTypeNames;

fn known() -> KnownTypeNames {
    KnownTypeNames::new()
}

fn named(s: &str) -> Type {
    Type::Named(s.to_string())
}

#[test]
fn simple_int_literal() {
    assert_eq!(
        render_scalar_init("0", &named("int"), &known()).unwrap(),
        "0"
    );
}

#[test]
fn hex_literal_with_suffix() {
    assert_eq!(
        render_scalar_init("0xc000000", &named("int"), &known()).unwrap(),
        "0xc000000"
    );
}

#[test]
fn bool_literal_ident() {
    // doomtype.h's real boolean typedef is `enum {false, true} boolean;`
    // (pre-C99 code defining its own boolean via enum constants literally
    // named false/true, not the C keywords) - `emit_enum` already escapes
    // these to `false_`/`true_` at the definition site via `ident()`, so a
    // reference here must resolve to the same escaped name to compile.
    assert_eq!(
        render_scalar_init("true", &named("boolean"), &known()).unwrap(),
        "true_"
    );
}

#[test]
fn references_another_already_emitted_const() {
    // d_englsh.h's real e1text = E1TEXT (a string macro const).
    assert_eq!(
        render_scalar_init("E1TEXT", &Type::Pointer(Box::new(named("char"))), &known()).unwrap(),
        "E1TEXT"
    );
}

#[test]
fn null_pointer_literal_becomes_null_mut() {
    let ty = Type::Pointer(Box::new(named("Display")));
    assert_eq!(
        render_scalar_init("0", &ty, &known()).unwrap(),
        "std::ptr::null_mut()"
    );
}

#[test]
fn non_pointer_zero_is_not_touched() {
    assert_eq!(
        render_scalar_init("0", &named("int"), &known()).unwrap(),
        "0"
    );
}

#[test]
fn address_of_expression() {
    // m_menu.c-style: `&mousearray[1]`.
    let ty = Type::Pointer(Box::new(named("boolean")));
    let out = render_scalar_init("&mousearray[1]", &ty, &known()).unwrap();
    assert!(out.contains("mousearray"), "got: {out}");
    assert!(!out.contains("null_mut"), "got: {out}");
}

// ---- render_array_init ----

fn arr(elem: Type, dim: Option<&str>) -> Type {
    Type::Array(Box::new(elem), dim.map(str::to_string))
}

#[test]
fn char_array_from_unsized_string_literal_infers_length() {
    let ty = arr(named("char"), None);
    let (ty_text, init_text) =
        render_array_init(&Init::Expr("\"hi\"".to_string()), &ty, &known()).unwrap();
    assert_eq!(ty_text, "[std::ffi::c_char; 3]");
    assert_eq!(
        init_text,
        "[104 as std::ffi::c_char, 105 as std::ffi::c_char, 0]"
    );
}

#[test]
fn char_array_from_string_literal_with_explicit_dim_keeps_dim() {
    let ty = arr(named("char"), Some("8"));
    let (ty_text, _) = render_array_init(&Init::Expr("\"hi\"".to_string()), &ty, &known()).unwrap();
    assert_eq!(ty_text, "[std::ffi::c_char; (8) as usize]");
}

#[test]
fn char_array_from_string_literal_unescapes_common_escapes() {
    let ty = arr(named("char"), None);
    let (_, init_text) =
        render_array_init(&Init::Expr("\"a\\nb\\\\\\\"\"".to_string()), &ty, &known()).unwrap();
    // "a\nb\\\"" -> bytes: a, \n, b, \\, "
    assert_eq!(
        init_text,
        "[97 as std::ffi::c_char, 10 as std::ffi::c_char, 98 as std::ffi::c_char, \
         92 as std::ffi::c_char, 34 as std::ffi::c_char, 0]"
    );
}

#[test]
fn non_char_array_rejects_string_literal_init() {
    let ty = arr(named("int"), None);
    assert!(render_array_init(&Init::Expr("\"hi\"".to_string()), &ty, &known()).is_none());
}

#[test]
fn flat_scalar_array_infers_length_from_element_count() {
    let ty = arr(named("int"), None);
    let init = Init::Braced(vec![
        Init::Expr("0".to_string()),
        Init::Expr("8".to_string()),
        Init::Expr("109".to_string()),
    ]);
    let (ty_text, init_text) = render_array_init(&init, &ty, &known()).unwrap();
    assert_eq!(ty_text, "[std::ffi::c_int; 3]");
    assert_eq!(init_text, "[0, 8, 109]");
}

#[test]
fn flat_scalar_array_with_explicit_dim_keeps_dim() {
    let ty = arr(named("int"), Some("NUMFOO"));
    let init = Init::Braced(vec![Init::Expr("0".to_string())]);
    let (ty_text, _) = render_array_init(&init, &ty, &known()).unwrap();
    assert_eq!(ty_text, "[std::ffi::c_int; (NUMFOO) as usize]");
}

#[test]
fn nested_2d_scalar_array_recurses_one_level() {
    // v_video.c's gammatable[5][256]-style shape, shrunk for the test.
    let ty = arr(arr(named("int"), None), None);
    let init = Init::Braced(vec![
        Init::Braced(vec![
            Init::Expr("1".to_string()),
            Init::Expr("2".to_string()),
        ]),
        Init::Braced(vec![
            Init::Expr("3".to_string()),
            Init::Expr("4".to_string()),
        ]),
    ]);
    let (ty_text, init_text) = render_array_init(&init, &ty, &known()).unwrap();
    assert_eq!(ty_text, "[[std::ffi::c_int; 2]; 2]");
    assert_eq!(init_text, "[[1, 2], [3, 4]]");
}

#[test]
fn single_element_brace_around_scalar_is_unwrapped() {
    let ty = arr(named("int"), None);
    let init = Init::Braced(vec![Init::Braced(vec![Init::Expr("5".to_string())])]);
    let (_, init_text) = render_array_init(&init, &ty, &known()).unwrap();
    assert_eq!(init_text, "[5]");
}

#[test]
fn struct_typed_array_row_bails_out() {
    // states[]/mobjinfo[]-style: each row is itself a multi-item Braced
    // group against a non-Array element type - out of scope this phase.
    let ty = arr(named("state_t"), None);
    let init = Init::Braced(vec![Init::Braced(vec![
        Init::Expr("SPR_TROO".to_string()),
        Init::Expr("0".to_string()),
    ])]);
    assert!(render_array_init(&init, &ty, &known()).is_none());
}

#[test]
fn scalar_target_type_is_rejected() {
    assert!(render_array_init(&Init::Expr("0".to_string()), &named("int"), &known()).is_none());
}
