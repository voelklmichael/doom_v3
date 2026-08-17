use super::*;

#[test]
fn plain_identifier_is_unchanged() {
    assert_eq!(ident("mobj_t"), "mobj_t");
    assert_eq!(ident("player"), "player");
}

#[test]
fn real_corpus_keyword_collisions_get_suffixed() {
    // Confirmed real identifiers in this corpus: mobjtype_t `type`
    // (p_mobj.c), M_ClearBox's `box` param (m_bbox.c), p_user.c's `move`
    // param, buttonlist[i].`where` (p_switch.c).
    assert_eq!(ident("type"), "type_");
    assert_eq!(ident("box"), "box_");
    assert_eq!(ident("move"), "move_");
    assert_eq!(ident("where"), "where_");
}

#[test]
fn unescapable_path_keywords_are_suffixed_uniformly() {
    // r#self/r#Self/r#super/r#crate/r#extern are all rejected by rustc -
    // the uniform suffix rule sidesteps that special case entirely.
    assert_eq!(ident("self"), "self_");
    assert_eq!(ident("Self"), "Self_");
    assert_eq!(ident("super"), "super_");
    assert_eq!(ident("crate"), "crate_");
    assert_eq!(ident("extern"), "extern_");
}

#[test]
fn other_common_keywords_are_suffixed() {
    for kw in [
        "fn", "struct", "enum", "impl", "let", "match", "loop", "if", "pub", "unsafe",
    ] {
        assert_eq!(
            ident(kw),
            format!("{kw}_"),
            "keyword {kw} should be suffixed"
        );
    }
}

#[test]
fn empty_and_underscore_are_not_keywords() {
    assert_eq!(ident(""), "");
    assert_eq!(ident("_"), "_");
}

#[test]
fn synthesizes_nested_record_names() {
    assert_eq!(synthesize_nested_name("intercept_t", "d"), "intercept_t_d");
}

#[test]
fn nested_synthesis_composes_for_recursion() {
    let once = synthesize_nested_name("parent", "field");
    let twice = synthesize_nested_name(&once, "subfield");
    assert_eq!(twice, "parent_field_subfield");
}
