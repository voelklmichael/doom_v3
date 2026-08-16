use super::*;

#[test]
fn rcsid_style() {
    let cd = try_parse_const_flat("static const char rcsid[] = \"$Id$\";").unwrap();
    assert_eq!(cd.storage, vec!["static", "const"]);
    assert_eq!(cd.ty, "char");
    assert_eq!(cd.name, "rcsid");
    assert_eq!(cd.array_dims, vec![None]);
    assert!(matches!(cd.initializer, Some(Init::Expr(_))));
}

#[test]
fn braced_array_style() {
    let cd =
        try_parse_const_braced("mobjinfo_t mobjinfo[NUMMOBJTYPES] =", "{ /* ... */ }").unwrap();
    assert_eq!(cd.ty, "mobjinfo_t");
    assert_eq!(cd.name, "mobjinfo");
    assert_eq!(cd.array_dims, vec![Some("NUMMOBJTYPES".to_string())]);
}

#[test]
fn no_equals_is_not_a_const() {
    assert!(try_parse_const_flat("extern patch_t* hu_font[HU_FONTSIZE];").is_none());
}

#[test]
fn comparison_is_not_mistaken_for_assignment() {
    // not realistic top-level syntax, but guards the == skip logic
    assert!(split_top_level_eq("a == b").is_none());
}

#[test]
fn fnptr_typedef_with_named_param() {
    // p_local.h's traverser_t. Before the fnptr-declarator shape was
    // recognized, this fell through to the plain whitespace-token parser
    // and produced garbage: name "in)", underlying "boolean (*traverser_t)
    // (intercept_t *". Round-trip stayed exact (raw is untouched) but the
    // structured fields were nonsense.
    let td = try_parse_typedef_flat("typedef boolean (*traverser_t) (intercept_t *in);").unwrap();
    assert_eq!(td.name, "traverser_t");
    assert_eq!(td.underlying, "boolean (*)(intercept_t *in)");
}

#[test]
fn fnptr_typedef_with_empty_params() {
    // d_think.h's actionf_v.
    let td = try_parse_typedef_flat("typedef  void (*actionf_v)();").unwrap();
    assert_eq!(td.name, "actionf_v");
    assert_eq!(td.underlying, "void (*)()");
}

#[test]
fn fnptr_typedef_with_anonymous_params() {
    // d_think.h's actionf_p2.
    let td = try_parse_typedef_flat("typedef  void (*actionf_p2)( void*, void* );").unwrap();
    assert_eq!(td.name, "actionf_p2");
    assert_eq!(td.underlying, "void (*)(void*, void*)");
}

#[test]
fn fnptr_array_declarator_with_sized_dim() {
    let (storage, ty, name, dims) = parse_declarator("void (*table[4])(int)").unwrap();
    assert!(storage.is_empty());
    assert_eq!(ty, "void (*)(int)");
    assert_eq!(name, "table");
    assert_eq!(dims, vec![Some("4".to_string())]);
}

#[test]
fn fnptr_array_declarator_with_unsized_dim() {
    // f_wipe.c's `wipes[]`: `static int (*wipes[])(int, int, int) = { ... };`
    let cd = try_parse_const_braced(
        "static int (*wipes[])(int, int, int) =",
        "{ wipe_initColorXForm, wipe_doColorXForm }",
    )
    .unwrap();
    assert_eq!(cd.storage, vec!["static"]);
    assert_eq!(cd.ty, "int (*)(int, int, int)");
    assert_eq!(cd.name, "wipes");
    assert_eq!(cd.array_dims, vec![None]);
}

#[test]
fn fnptr_array_declarator_with_multiple_dims() {
    let (_, _, name, dims) = parse_declarator("void (*table[4][2])(int)").unwrap();
    assert_eq!(name, "table");
    assert_eq!(dims, vec![Some("4".to_string()), Some("2".to_string())]);
}

#[test]
fn fnptr_array_declarator_rejects_non_identifier_name() {
    // Guards against a malformed name (e.g. a stray `*`) inside the array
    // brackets being silently accepted.
    assert!(parse_declarator("void (*1table[4])(int)").is_none());
}
