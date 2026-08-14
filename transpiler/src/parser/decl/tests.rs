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
    let cd = try_parse_const_braced("mobjinfo_t mobjinfo[NUMMOBJTYPES] =", "{ /* ... */ }").unwrap();
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
