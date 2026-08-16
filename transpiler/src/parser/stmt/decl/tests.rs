use super::*;
use crate::parser::scan::scan;
use crate::parser::stmt::lex::tokenize_chunks;

fn toks(src: &str) -> Vec<CTok> {
    tokenize_chunks(crate::parser::brace::group_braces(scan(src)))
}

fn decl(src: &str) -> DeclStmt {
    decl_with(src, &KnownTypeNames::new())
}

fn decl_with(src: &str, known: &KnownTypeNames) -> DeclStmt {
    try_parse_decl_stmt(&toks(src), known).unwrap_or_else(|| panic!("failed to parse: {src}"))
}

#[test]
fn single_declarator_with_initializer() {
    let d = decl("int x = 5");
    assert_eq!(d.base_ty, Type::Named("int".to_string()));
    assert_eq!(d.declarators.len(), 1);
    assert_eq!(d.declarators[0].name, "x");
    assert_eq!(d.declarators[0].ty, Type::Named("int".to_string()));
    assert!(matches!(
        d.declarators[0].initializer,
        Some(LocalInit::Expr(Expr::IntLit(ref s))) if s == "5"
    ));
}

#[test]
fn multi_declarator_shares_base_type_with_own_decoration() {
    // int *a, b[4]; - `a` is a pointer, `b` is an array of 4, both `int`.
    let d = decl("int *a, b[4]");
    assert_eq!(d.base_ty, Type::Named("int".to_string()));
    assert_eq!(d.declarators.len(), 2);
    assert_eq!(d.declarators[0].name, "a");
    assert_eq!(
        d.declarators[0].ty,
        Type::Pointer(Box::new(Type::Named("int".to_string())))
    );
    assert_eq!(d.declarators[1].name, "b");
    assert_eq!(
        d.declarators[1].ty,
        Type::Array(
            Box::new(Type::Named("int".to_string())),
            Some("4".to_string())
        )
    );
}

#[test]
fn multi_declarator_with_mixed_initializers() {
    let d = decl("int a = 1, b, c = 3");
    assert_eq!(d.declarators.len(), 3);
    assert!(d.declarators[0].initializer.is_some());
    assert!(d.declarators[1].initializer.is_none());
    assert!(d.declarators[2].initializer.is_some());
}

#[test]
fn storage_keyword_recognized() {
    let d = decl("static int count = 0");
    assert_eq!(d.storage, vec![Storage::Static]);
}

#[test]
fn known_typedef_name_as_base_type() {
    let mut known = KnownTypeNames::new();
    known.insert("mobj_t");
    let d = decl_with("mobj_t *thing", &known);
    assert_eq!(d.base_ty, Type::Named("mobj_t".to_string()));
    assert_eq!(
        d.declarators[0].ty,
        Type::Pointer(Box::new(Type::Named("mobj_t".to_string())))
    );
}

#[test]
fn braced_initializer_recurses_per_element() {
    let d = decl("int arr[3] = {1, 2, 3}");
    match &d.declarators[0].initializer {
        Some(LocalInit::Braced(elems)) => {
            assert_eq!(elems.len(), 3);
            for (elem, expected) in elems.iter().zip(["1", "2", "3"]) {
                assert!(matches!(elem, LocalInit::Expr(Expr::IntLit(s)) if s == expected));
            }
        }
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn nested_braced_initializer() {
    let d = decl("int m[2][2] = {{1, 2}, {3, 4}}");
    match &d.declarators[0].initializer {
        Some(LocalInit::Braced(rows)) => {
            assert_eq!(rows.len(), 2);
            assert!(matches!(&rows[0], LocalInit::Braced(r) if r.len() == 2));
        }
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn looks_like_decl_start_recognizes_storage_and_base_keywords() {
    let known = KnownTypeNames::new();
    assert!(looks_like_decl_start(&toks("int x"), &known));
    assert!(looks_like_decl_start(&toks("static int x"), &known));
    assert!(looks_like_decl_start(&toks("struct foo *x"), &known));
    assert!(!looks_like_decl_start(&toks("x = 5"), &known));
    assert!(!looks_like_decl_start(&toks("foo()"), &known));
}

#[test]
fn looks_like_decl_start_recognizes_known_typedef() {
    let mut known = KnownTypeNames::new();
    known.insert("mobj_t");
    assert!(looks_like_decl_start(&toks("mobj_t *x"), &known));
    assert!(!looks_like_decl_start(
        &toks("mobj_t = 5"),
        &KnownTypeNames::new()
    ));
}
