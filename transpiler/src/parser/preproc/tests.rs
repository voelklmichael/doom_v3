use super::*;
use std::collections::HashMap;

#[test]
fn function_like_macro_needs_no_space_before_paren() {
    match parse_directive("#define FOO(a,b) ((a)+(b))\n") {
        Directive::DefineFunction { name, params, body } => {
            assert_eq!(name, "FOO");
            assert_eq!(params, vec!["a", "b"]);
            assert_eq!(body, "((a)+(b))");
        }
        other => panic!("expected DefineFunction, got {other:?}"),
    }
}

#[test]
fn object_like_macro_with_space_before_paren_is_not_function_like() {
    match parse_directive("#define FOO (1+2)\n") {
        Directive::DefineObject { name, value } => {
            assert_eq!(name, "FOO");
            assert_eq!(value, "(1+2)");
        }
        other => panic!("expected DefineObject, got {other:?}"),
    }
}

#[test]
fn ifdef_ifndef_else_endif() {
    assert!(matches!(
        parse_directive("#ifdef LINUX\n"),
        Directive::IfDef { negate: false, .. }
    ));
    assert!(matches!(
        parse_directive("#ifndef __DOOMTYPE__\n"),
        Directive::IfDef { negate: true, .. }
    ));
    assert!(matches!(parse_directive("#else\n"), Directive::Else));
    assert!(matches!(parse_directive("#endif\n"), Directive::Endif));
}

#[test]
fn include_angled_and_quoted() {
    match parse_directive("#include <stdio.h>\n") {
        Directive::Include { path, angled } => {
            assert_eq!(path, "stdio.h");
            assert!(angled);
        }
        other => panic!("{other:?}"),
    }
    match parse_directive("#include \"doomdef.h\"\n") {
        Directive::Include { path, angled } => {
            assert_eq!(path, "doomdef.h");
            assert!(!angled);
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn eval_ifdef_basic() {
    let mut defines = HashMap::new();
    defines.insert("LINUX".to_string(), String::new());
    assert_eq!(eval_ifdef("LINUX", false, &defines), Tri::True);
    assert_eq!(eval_ifdef("LINUX", true, &defines), Tri::False);
    assert_eq!(eval_ifdef("SGI", false, &defines), Tri::False);
    assert_eq!(eval_ifdef("SGI", true, &defines), Tri::True);
}

#[test]
fn eval_if_expr_bare_literals() {
    let defines = HashMap::new();
    assert_eq!(eval_if_expr("0", &defines), Tri::False);
    assert_eq!(eval_if_expr("1", &defines), Tri::True);
    // Real corpus text: a trailing line comment after the literal.
    assert_eq!(eval_if_expr("0\t// UNUSED - debug?", &defines), Tri::False);
    assert_eq!(eval_if_expr("0 // UNUSED", &defines), Tri::False);
}

#[test]
fn eval_if_expr_defined_identifier_recurses_on_value() {
    // Real corpus shape: `#define SNDSERV  1` then `#elif SNDINTR` /
    // `#ifdef SNDSERV` elsewhere - a bare identifier condition looks up its
    // #define'd value and evaluates *that*.
    let mut defines = HashMap::new();
    defines.insert("SNDSERV".to_string(), "1".to_string());
    assert_eq!(eval_if_expr("SNDSERV", &defines), Tri::True);
}

#[test]
fn eval_if_expr_undefined_identifier_is_false() {
    // Matches real C: an identifier left over after macro expansion in an
    // `#if` expression is replaced with 0. `SNDINTR` is never `#define`'d
    // anywhere in the real corpus, so `#elif SNDINTR` must resolve False.
    let defines = HashMap::new();
    assert_eq!(eval_if_expr("SNDINTR", &defines), Tri::False);
}

#[test]
fn eval_if_expr_valueless_define_is_unknown_if_ever_referenced_bare() {
    // `#define RANGECHECK` (no value) is only ever used via #ifdef in the
    // real corpus, never bare in an #if expression - but if it were, there's
    // no numeric value to evaluate, so this must not guess.
    let mut defines = HashMap::new();
    defines.insert("RANGECHECK".to_string(), String::new());
    assert_eq!(eval_if_expr("RANGECHECK", &defines), Tri::Unknown);
}

#[test]
fn eval_if_expr_real_expression_is_unknown() {
    // None of these occur anywhere in the real corpus, but must degrade
    // gracefully rather than being guessed at.
    let defines = HashMap::new();
    assert_eq!(eval_if_expr("defined(FOO)", &defines), Tri::Unknown);
    assert_eq!(eval_if_expr("VERSION >= 2", &defines), Tri::Unknown);
    assert_eq!(eval_if_expr("FOO && BAR", &defines), Tri::Unknown);
}
