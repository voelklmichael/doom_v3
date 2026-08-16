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
            assert_eq!(e.variants[0].name, "INVULNTICS");
            assert_eq!(e.variants[0].value.as_deref(), Some("(30*TICRATE)"));
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
fn blank_line_before_doc_comment_does_not_block_flat_classification() {
    // Same shape as d_think.h's actionf_v: a blank line before the doc
    // comment stops `drain_leading_comments` from stripping it (same gap as
    // the struct-keyword test above), but for a *flat* (brace-free) unit
    // this used to make classification see the raw text starting with "//"
    // instead of "typedef", so the typedef was never recognized at all and
    // fell all the way back to Item::Raw.
    let src = "\n// Function pointer type.\ntypedef void (*fn_t)();\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Typedef(td) => {
            assert_eq!(td.name, "fn_t");
            assert_eq!(td.underlying, "void (*)()");
        }
        other => panic!("expected Typedef, got {other:?}"),
    }
}

#[test]
fn equals_sign_in_comment_does_not_fake_a_const_initializer() {
    // Same shape as doomstat.h's viewangleoffset: the comment text
    // "ANG90 = left side" contains a top-level '=', which used to be
    // mistaken for a real initializer's '=' when classification scanned
    // the whole (comment-inclusive) raw text, producing a garbage
    // ConstDecl with the comment fragment as its type/name. A plain
    // extern declaration with no initializer must fall to Item::Raw
    // (by design - see the "no initializer" gap), not a fake Const.
    let src = "\n// ANG90 = left side, ANG270 = right\nextern int viewangleoffset;\n";
    round_trip(src);
    let items = build(src);
    assert!(matches!(items[0].0.kind, ItemKind::Raw));
}

#[test]
fn same_line_trailing_comment_stays_with_its_own_item() {
    // p_local.h's `linetarget`: a comment on the same line as the `;` that
    // ends a top-level declaration describes *that* declaration, not
    // whatever follows. Piece-splitting only scans forward for `;`, so
    // without reattachment the comment lands as a prefix of the next item's
    // raw instead of a suffix of this one's.
    let src = "extern int x; // who got hit\nint y;\n";
    round_trip(src);
    let items = build(src);
    assert_eq!(items[0].0.raw, "extern int x; // who got hit\n");
    assert_eq!(items[1].0.raw, "int y;");
}

#[test]
fn trailing_comment_after_newline_is_not_reattached() {
    // A comment on its *own* line, even right after a `;`, is a leading
    // doc comment for what follows - not a trailing comment for what came
    // before - and must be left alone (the existing `drain_leading_comments`
    // / blank-line-before-doc-comment case, unaffected by reattachment).
    let src = "extern int x;\n// about y\nint y;\n";
    round_trip(src);
    let items = build(src);
    assert_eq!(items[0].0.raw, "extern int x;");
    assert!(items[1].0.raw.contains("about y"));
}

#[test]
fn struct_field_trailing_comment_does_not_leak_into_next_field() {
    // d_player.h-style: nearly every field has a trailing comment.
    // Rendering comments into the text that gets split on `;` used to glue
    // a comment (and, if it began right after the `;` with no space, even
    // fragments of it) onto the *next* field's declarator text.
    let src = "typedef struct\n{\n    int score;\t// current score\n    int epsd;\t// episode #\n} player_t;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(rd) => {
            assert_eq!(rd.fields.len(), 2);
            assert_eq!(rd.fields[0].ty, "int");
            assert_eq!(rd.fields[0].name, "score");
            assert_eq!(rd.fields[1].ty, "int");
            assert_eq!(rd.fields[1].name, "epsd");
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn struct_field_trailing_comment_is_captured_structurally() {
    let src = "typedef struct\n{\n    int score;\t// current score\n    int epsd;\t// episode #\n} player_t;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(rd) => {
            assert_eq!(
                rd.fields[0].trailing_comment.as_ref().map(Comment::text),
                Some("// current score\n")
            );
            assert_eq!(
                rd.fields[1].trailing_comment.as_ref().map(Comment::text),
                Some("// episode #\n")
            );
            assert!(rd.fields[0].trivia.leading.is_empty());
            assert!(rd.fields[1].trivia.leading.is_empty());
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn struct_field_leading_doc_comment_is_captured_structurally() {
    // A blank line ahead of the doc comment (this codebase's common style)
    // must not stop it from being recognized as leading trivia for the
    // field that follows - same gap class as
    // `blank_line_before_doc_comment_does_not_block_flat_classification`,
    // but for a field inside a struct body rather than a top-level item.
    let src = "typedef struct\n{\n\n    // hit points\n    int health;\n} mobj_t;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(rd) => {
            assert_eq!(rd.fields.len(), 1);
            assert_eq!(rd.fields[0].name, "health");
            assert_eq!(rd.fields[0].trivia.leading.len(), 1);
            assert_eq!(rd.fields[0].trivia.leading[0].text(), "// hit points\n");
            assert!(rd.fields[0].trailing_comment.is_none());
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn enum_variant_trailing_comment_is_captured_structurally() {
    let src = "enum\n{\n    BT_ATTACK\t= 1,\t// fire weapon\n    BT_USE\t= 2 // open doors\n};\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Enum(ed) => {
            assert_eq!(ed.variants.len(), 2);
            assert_eq!(ed.variants[0].name, "BT_ATTACK");
            assert_eq!(
                ed.variants[0].trailing_comment.as_ref().map(Comment::text),
                Some("// fire weapon\n")
            );
            assert_eq!(ed.variants[1].name, "BT_USE");
            assert_eq!(
                ed.variants[1].trailing_comment.as_ref().map(Comment::text),
                Some("// open doors\n")
            );
        }
        other => panic!("expected Enum, got {other:?}"),
    }
}

#[test]
fn enum_variant_comment_with_a_comma_does_not_fracture_variants() {
    // d_event.h's BT_ATTACK: a trailing comment containing a comma (e.g.
    // "// Use button, to open doors") used to be rendered straight into the
    // text `parse_enum_variants` splits on top-level ',', so the comma
    // *inside the comment* fractured it into bogus extra variants.
    let src = "enum\n{\n    BT_ATTACK\t= 1,\t// Use button, to open doors\n    BT_USE\t= 2\n};\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Enum(ed) => {
            let simplified: Vec<(String, Option<String>)> = ed
                .variants
                .iter()
                .map(|v| (v.name.clone(), v.value.clone()))
                .collect();
            assert_eq!(
                simplified,
                vec![
                    ("BT_ATTACK".to_string(), Some("1".to_string())),
                    ("BT_USE".to_string(), Some("2".to_string())),
                ]
            );
        }
        other => panic!("expected Enum, got {other:?}"),
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
            match &cd.initializer {
                Some(Init::Braced(rows)) => {
                    assert_eq!(rows.len(), 1);
                    match &rows[0] {
                        Init::Braced(cells) => {
                            let texts: Vec<&str> = cells
                                .iter()
                                .map(|e| match e {
                                    Init::Expr(s) => s.as_str(),
                                    Init::Braced(_) => panic!("expected a scalar cell"),
                                })
                                .collect();
                            assert_eq!(texts, vec!["\"a\"", "&a", "1"]);
                        }
                        Init::Expr(_) => panic!("expected a nested Braced row"),
                    }
                }
                other => panic!("expected Braced, got {other:?}"),
            }
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
