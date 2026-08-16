use super::*;

fn corpus_path(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../linuxdoom-1.10")
        .join(file)
}

#[test]
fn harvests_plain_typedef_names() {
    // doomtype.h's `boolean`/`byte` - both real typedefs, and both sit
    // inside an #ifdef/#else conditional branch (the classic __BYTEBOOL__
    // guard), which is exactly why this must recurse into CondGroup
    // branches rather than only scanning flat top-level items.
    let known = collect_known_type_names(&[corpus_path("doomtype.h")]);
    assert!(known.contains("boolean"));
    assert!(known.contains("byte"));
}

#[test]
fn harvests_typedef_struct_name() {
    // p_mobj.h: `typedef struct mobj_s { ... } mobj_t;` - both the tag
    // (mobj_s) and the typedef name (mobj_t) should be recognized as types.
    let known = collect_known_type_names(&[corpus_path("p_mobj.h")]);
    assert!(known.contains("mobj_t"));
    assert!(known.contains("mobj_s"));
}

#[test]
fn unrelated_identifiers_are_not_known_types() {
    let known = collect_known_type_names(&[corpus_path("doomtype.h")]);
    assert!(!known.contains("some_totally_unrelated_name"));
}

#[test]
fn skips_unreadable_paths_without_failing() {
    let known = collect_known_type_names(&[PathBuf::from("/nonexistent/path/does_not_exist.h")]);
    assert!(!known.contains("boolean"));
}
