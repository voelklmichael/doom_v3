use super::render_scalar_init;
use crate::parser::ast::Type;
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
