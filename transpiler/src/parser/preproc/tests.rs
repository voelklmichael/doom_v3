use super::*;

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
    assert!(matches!(parse_directive("#ifdef LINUX\n"), Directive::IfDef { negate: false, .. }));
    assert!(matches!(parse_directive("#ifndef __DOOMTYPE__\n"), Directive::IfDef { negate: true, .. }));
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
