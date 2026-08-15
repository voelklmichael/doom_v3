use super::*;
use crate::parser::ast::Init;
use crate::parser::preproc::Directive;
use crate::parser::scan::scan;

fn build(src: &str) -> Vec<(Item, Trivia)> {
    build_items(scan(src))
}

fn round_trip(src: &str) {
    let items = build(src);
    let mut out = String::new();
    for (item, trivia) in &items {
        for c in &trivia.leading {
            out.push_str(c.text());
        }
        out.push_str(&item.raw);
    }
    assert_eq!(out, src, "round trip mismatch");
}

#[test]
fn simple_struct_with_typedef() {
    let src = "typedef struct\n{\n    char* name;\n    int value;\n} default_t;\n";
    round_trip(src);
    let items = build(src);
    // items[1] is the trailing "\n" after the closing ';' - an empty
    // Raw item, not a real declaration, but still needed for round trip.
    assert_eq!(items.len(), 2);
    match &items[0].0.kind {
        ItemKind::Record(r) => {
            assert_eq!(r.kind, RecordKind::Struct);
            assert_eq!(r.typedef_name.as_deref(), Some("default_t"));
            assert_eq!(r.fields.len(), 2);
            assert_eq!(r.fields[0].name, "name");
            assert_eq!(r.fields[1].name, "value");
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn enum_with_values() {
    let src = "typedef enum\n{\n    INVULNTICS = (30*TICRATE),\n    INVISTICS = (60*TICRATE)\n} powerduration_t;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Enum(e) => {
            assert_eq!(e.typedef_name.as_deref(), Some("powerduration_t"));
            assert_eq!(e.variants.len(), 2);
            assert_eq!(e.variants[0].0, "INVULNTICS");
            assert_eq!(e.variants[0].1.as_deref(), Some("(30*TICRATE)"));
        }
        other => panic!("expected Enum, got {other:?}"),
    }
}

#[test]
fn function_body_stays_opaque() {
    let src = "int\nM_DrawText\n( int x,\n  int y )\n{\n    return x + y;\n}\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDef(sig, body) => {
            assert_eq!(sig.name, "M_DrawText");
            assert!(body.contains("return x + y;"));
        }
        other => panic!("expected FunctionDef, got {other:?}"),
    }
}

#[test]
fn doc_comment_mentioning_struct_does_not_misclassify_a_function() {
    // Same shape as i_sound.c's I_StartSound: a blank line before the doc
    // comment stops `drain_leading_comments` from stripping it out first,
    // so the comment text used to leak into the header used for
    // struct/union/enum keyword detection. A comment saying "struct" must
    // never turn a function into a Record.
    let src =
        "\n// The SFX info struct contains a pointer.\nint\nfoo\n( int x )\n{\n    return x;\n}\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDef(sig, _) => assert_eq!(sig.name, "foo"),
        other => panic!("expected FunctionDef, got {other:?}"),
    }
}

#[test]
fn braced_array_const() {
    let src = "default_t defaults[] =\n{\n    {\"a\", &a, 1},\n};\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Const(cd) => {
            assert_eq!(cd.name, "defaults");
            assert!(matches!(cd.initializer, Some(Init::Braced(_))));
        }
        other => panic!("expected Const, got {other:?}"),
    }
}

#[test]
fn function_body_does_not_absorb_what_follows_it() {
    // Same shape as m_misc.c: a function definition immediately followed
    // (after only blank lines and a comment) by a standalone directive. A
    // function body never has trailing `;`-terminated content, unlike a
    // struct/enum/braced-const-initializer unit, so `build_items` must
    // flush right after the closing `}` instead of waiting for one and
    // absorbing whatever comes next into this item's `raw`.
    let src = "int\nfoo\n( void )\n{\n    return 0;\n}\n\n\n//\n// bar\n//\n#ifndef X\n#define X 1\n#endif\n\nint\nbar\n( void )\n{\n    return 1;\n}\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDef(sig, _) => {
            assert_eq!(sig.name, "foo");
            assert!(items[0].0.raw.trim_end().ends_with('}'));
        }
        other => panic!("expected FunctionDef, got {other:?}"),
    }
    assert!(
        items.iter().any(|(item, _)| matches!(
            &item.kind,
            ItemKind::Preproc(Directive::IfDef { name, .. }) if name == "X"
        )),
        "expected #ifndef X to be its own item, not absorbed into `foo`"
    );
}

#[test]
fn directive_and_comment_and_const_all_round_trip_together() {
    let src = "// banner\n#define X 1\nstatic const char rcsid[] = \"id\";\n";
    round_trip(src);
    let items = build(src);
    // items[2] is the trailing "\n" after the ';' - see simple_struct_with_typedef.
    assert_eq!(items.len(), 3);
    assert!(matches!(items[0].0.kind, ItemKind::Preproc(_)));
    assert!(matches!(items[1].0.kind, ItemKind::Const(_)));
    assert_eq!(items[1].1.leading.len(), 0);
}
