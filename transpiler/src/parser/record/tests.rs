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
fn multi_declarator_field_splits_into_separate_fields() {
    // info.h's real state_t shape: `long misc1, misc2;` - previously
    // collapsed into one malformed Field with an embedded comma.
    let src = "typedef struct\n{\n    long frame;\n    long misc1, misc2;\n    long nextstate;\n} state_t;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(r) => {
            assert_eq!(r.fields.len(), 4);
            assert_eq!(r.fields[0].name, "frame");
            assert_eq!(r.fields[1].name, "misc1");
            assert!(!r.fields[1].name.contains(','), "got: {:?}", r.fields[1]);
            assert_eq!(r.fields[2].name, "misc2");
            assert_eq!(r.fields[3].name, "nextstate");
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn multi_declarator_field_shares_pointer_and_array_decoration_independently() {
    // am_map.c's real mpoint_t/fpoint_t shape: `fixed_t x, y;` (no stars),
    // but the general grammar (mirroring stmt::decl's local-multi-declarator
    // handling) must also let each declarator apply its own `*`/`[]` on top
    // of the shared base type, e.g. `int *a, b[4];`.
    let src = "struct\n{\n    int *a, b[4];\n} s;\n";
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(r) => {
            assert_eq!(r.fields.len(), 2);
            assert_eq!(r.fields[0].name, "a");
            assert!(
                matches!(r.fields[0].ty, Type::Pointer(_)),
                "got: {:?}",
                r.fields[0].ty
            );
            assert_eq!(r.fields[1].name, "b");
            assert!(
                matches!(r.fields[1].ty, Type::Array(_, _)),
                "got: {:?}",
                r.fields[1].ty
            );
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn multi_declarator_field_leading_and_trailing_comments_attach_to_first_and_last() {
    let src = "struct\n{\n    // doc\n    long misc1, misc2; // trailing\n} s;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(r) => {
            assert_eq!(r.fields.len(), 2);
            assert!(!r.fields[0].trivia.leading.is_empty());
            assert!(r.fields[1].trivia.leading.is_empty());
            assert!(r.fields[0].trailing_comment.is_none());
            assert!(r.fields[1].trailing_comment.is_some());
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn single_declarator_field_unaffected_by_multi_declarator_change() {
    // No top-level comma at all - must behave exactly as before.
    let src = "struct\n{\n    int x;\n} s;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(r) => {
            assert_eq!(r.fields.len(), 1);
            assert_eq!(r.fields[0].name, "x");
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn function_body_is_parsed_and_raw_preserved() {
    let src = "int\nM_DrawText\n( int x,\n  int y )\n{\n    return x + y;\n}\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDef(sig, body) => {
            assert_eq!(sig.name, "M_DrawText");
            assert!(body.raw.contains("return x + y;"));
            assert_eq!(sig.params.len(), 2);
            assert_eq!(sig.params[0].ty, Type::Named("int".to_string()));
            assert_eq!(sig.params[0].name, "x");
            assert_eq!(sig.params[1].ty, Type::Named("int".to_string()));
            assert_eq!(sig.params[1].name, "y");
            assert!(!sig.variadic);
        }
        other => panic!("expected FunctionDef, got {other:?}"),
    }
}

#[test]
fn void_param_list_is_empty() {
    let src = "void M_ScreenShot(void);\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDecl(sig) => {
            assert_eq!(sig.name, "M_ScreenShot");
            assert!(sig.params.is_empty());
        }
        other => panic!("expected FunctionDecl, got {other:?}"),
    }
}

#[test]
fn empty_param_list_is_empty() {
    let src = "void I_InitSound();\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDecl(sig) => {
            assert_eq!(sig.name, "I_InitSound");
            assert!(sig.params.is_empty());
        }
        other => panic!("expected FunctionDecl, got {other:?}"),
    }
}

#[test]
fn anonymous_params_keep_their_type_with_no_name() {
    // m_swap.h's SwapSHORT / p_inter.h's P_GivePower: old K&R-style forward
    // declarations can have bare-type, no-name parameters.
    let src = "short SwapSHORT(short);\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDecl(sig) => {
            assert_eq!(sig.params.len(), 1);
            assert_eq!(sig.params[0].ty, Type::Named("short".to_string()));
            assert_eq!(sig.params[0].name, "");
        }
        other => panic!("expected FunctionDecl, got {other:?}"),
    }
}

#[test]
fn anonymous_pointer_param_still_folds_its_star_into_type() {
    // p_inter.h's real P_GivePower(player_t*, int): the anonymous-parameter
    // fallback used to wrap the parameter's raw text ("player_t*") straight
    // into Type::Named instead of running it through parse_type_text, so
    // the pointer star leaked into the Named string instead of becoming a
    // proper Type::Pointer wrapper.
    let src = "boolean P_GivePower(player_t*, int);\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDecl(sig) => {
            assert_eq!(sig.params.len(), 2);
            assert_eq!(
                sig.params[0].ty,
                Type::Pointer(Box::new(Type::Named("player_t".to_string())))
            );
            assert_eq!(sig.params[0].name, "");
            assert_eq!(sig.params[1].ty, Type::Named("int".to_string()));
        }
        other => panic!("expected FunctionDecl, got {other:?}"),
    }
}

#[test]
fn variadic_function_sets_flag_without_a_trailing_param() {
    // i_system.h's I_Error.
    let src = "void I_Error (char *error, ...);\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDecl(sig) => {
            assert!(sig.variadic);
            assert_eq!(sig.params.len(), 1);
            assert_eq!(sig.params[0].name, "error");
        }
        other => panic!("expected FunctionDecl, got {other:?}"),
    }
}

#[test]
fn function_pointer_parameter_is_parsed_via_declarator() {
    // p_local.h's P_BlockLinesIterator: a callback parameter is itself a
    // function-pointer declarator, reusing parse_declarator's existing
    // fn-ptr support (decl.rs) rather than anything new here.
    let src = "boolean P_BlockLinesIterator (int x, int y, boolean(*func)(line_t*) );\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDecl(sig) => {
            assert_eq!(sig.params.len(), 3);
            assert_eq!(
                sig.params[2].ty,
                Type::FunctionPointer {
                    ret: Box::new(Type::Named("boolean".to_string())),
                    params: vec![Type::Pointer(Box::new(Type::Named("line_t".to_string())))],
                }
            );
            assert_eq!(sig.params[2].name, "func");
        }
        other => panic!("expected FunctionDecl, got {other:?}"),
    }
}

#[test]
fn function_storage_keeps_static_extern_inline_but_not_const() {
    // A `const` before a function's return type qualifies the *return
    // type*, not the function itself (unlike a plain declarator, where
    // `const` is a real storage-class/qualifier keyword) - it must stay
    // part of ret_ty, not leak into FnSig::storage.
    let src = "static const char *M_Foo(void);\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::FunctionDecl(sig) => {
            assert_eq!(sig.storage, vec![Storage::Static]);
            assert_eq!(
                sig.ret_ty,
                Type::Pointer(Box::new(Type::Named("const char".to_string())))
            );
        }
        other => panic!("expected FunctionDecl, got {other:?}"),
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
            assert_eq!(
                td.underlying,
                Type::FunctionPointer {
                    ret: Box::new(Type::Named("void".to_string())),
                    params: vec![],
                }
            );
        }
        other => panic!("expected Typedef, got {other:?}"),
    }
}

#[test]
fn equals_sign_in_comment_does_not_fake_an_initializer() {
    // Same shape as doomstat.h's viewangleoffset: the comment text
    // "ANG90 = left side" contains a top-level '=', which used to be
    // mistaken for a real initializer's '=' when classification scanned
    // the whole (comment-inclusive) raw text, producing a garbage VarDecl
    // with the comment fragment as its type/name. It must instead come back
    // as a genuine no-initializer VarDecl for `viewangleoffset`, with the
    // comment excluded entirely from classification.
    let src = "\n// ANG90 = left side, ANG270 = right\nextern int viewangleoffset;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Var(vds) => {
            let vd = &vds[0];
            assert_eq!(vd.storage, vec![Storage::Extern]);
            assert_eq!(vd.ty, Type::Named("int".to_string()));
            assert_eq!(vd.name, "viewangleoffset");
            assert!(vd.initializer.is_none());
        }
        other => panic!("expected Var, got {other:?}"),
    }
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
            assert_eq!(rd.fields[0].ty, Type::Named("int".to_string()));
            assert_eq!(rd.fields[0].name, "score");
            assert_eq!(rd.fields[1].ty, Type::Named("int".to_string()));
            assert_eq!(rd.fields[1].name, "epsd");
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn nested_anonymous_union_field_is_recursively_parsed() {
    // p_local.h's intercept_t: a nested anonymous union field, previously
    // kept as one opaque Field with the union's raw text as its "type" and
    // its own name ("d") not captured anywhere structurally.
    let src = "typedef struct\n{\n    fixed_t\tfrac;\n    boolean\tisaline;\n    union {\n\tmobj_t*\tthing;\n\tline_t*\tline;\n    }\t\t\td;\n} intercept_t;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(rd) => {
            assert_eq!(rd.fields.len(), 3);
            assert_eq!(rd.fields[0].name, "frac");
            assert_eq!(rd.fields[1].name, "isaline");
            let nested_field = &rd.fields[2];
            assert_eq!(nested_field.name, "d");
            assert_eq!(nested_field.ty, Type::Named("union".to_string()));
            let nested = nested_field
                .nested
                .as_deref()
                .expect("expected nested record");
            assert_eq!(nested.kind, RecordKind::Union);
            assert_eq!(nested.tag, None);
            assert_eq!(nested.fields.len(), 2);
            assert_eq!(
                nested.fields[0].ty,
                Type::Pointer(Box::new(Type::Named("mobj_t".to_string())))
            );
            assert_eq!(nested.fields[0].name, "thing");
            assert_eq!(
                nested.fields[1].ty,
                Type::Pointer(Box::new(Type::Named("line_t".to_string())))
            );
            assert_eq!(nested.fields[1].name, "line");
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn nested_anonymous_struct_field_with_array_dims() {
    let src = "struct\n{\n    int before;\n    struct\n    {\n\tint x;\n    } points[4];\n    int after;\n} foo;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(rd) => {
            assert_eq!(rd.fields.len(), 3);
            let nested_field = &rd.fields[1];
            assert_eq!(nested_field.name, "points");
            assert_eq!(
                nested_field.ty,
                Type::Array(
                    Box::new(Type::Named("struct".to_string())),
                    Some("4".to_string())
                )
            );
            let nested = nested_field
                .nested
                .as_deref()
                .expect("expected nested record");
            assert_eq!(nested.kind, RecordKind::Struct);
            assert_eq!(nested.fields.len(), 1);
            assert_eq!(nested.fields[0].name, "x");
            assert_eq!(rd.fields[2].name, "after");
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn nested_record_with_tag_keeps_its_tag() {
    let src = "struct\n{\n    struct point_s\n    {\n\tint x;\n    } origin;\n} foo;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(rd) => {
            let nested = rd.fields[0].nested.as_deref().unwrap();
            assert_eq!(nested.tag.as_deref(), Some("point_s"));
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn nested_group_without_keyword_falls_back_to_opaque_field() {
    // A `{...}` field body not preceded by struct/union/enum isn't a shape
    // this parser recognizes at all (invalid C, or something stranger) -
    // must degrade to the old opaque-field fallback rather than panicking
    // or silently vanishing.
    let src = "struct\n{\n    int before;\n    weird_macro { 1, 2, 3 } after;\n} foo;\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Record(rd) => {
            assert_eq!(rd.fields[0].name, "before");
            // "weird_macro" (unparseable alone) is dropped, same as before
            // this change; the group itself becomes one opaque field.
            let opaque = rd
                .fields
                .iter()
                .find(|f| f.nested.is_none() && f.name.is_empty());
            assert!(opaque.is_some(), "expected an opaque fallback field");
            match &opaque.unwrap().ty {
                Type::Named(s) => assert!(s.contains("1, 2, 3")),
                other => panic!("expected Type::Named, got {other:?}"),
            }
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
fn braced_array_var() {
    let src = "default_t defaults[] =\n{\n    {\"a\", &a, 1},\n};\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Var(vds) => {
            let cd = &vds[0];
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
                                    Init::Conditional(_) => panic!("expected a scalar cell"),
                                })
                                .collect();
                            assert_eq!(texts, vec!["\"a\"", "&a", "1"]);
                        }
                        Init::Expr(_) => panic!("expected a nested Braced row"),
                        Init::Conditional(_) => panic!("expected a nested Braced row"),
                    }
                }
                other => panic!("expected Braced, got {other:?}"),
            }
        }
        other => panic!("expected Var, got {other:?}"),
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
    assert!(matches!(items[1].0.kind, ItemKind::Var(_)));
    assert_eq!(items[1].1.leading.len(), 0);
}

#[test]
fn top_level_multi_declarator_var_becomes_one_item_with_two_var_decls() {
    // am_map.c's real `static fixed_t m_x, m_y;` - previously collapsed
    // into one malformed VarDecl (`ty: Named("fixed_t m_x,")`, `name:
    // "m_y"`), silently dropping `m_x` from the structured AST entirely
    // (though never from `Item.raw`, so round-trip stayed byte-exact
    // throughout - this is a purely structural gap, same class as the
    // struct/union multi-declarator field bug fixed earlier).
    let src = "static fixed_t \tm_x, m_y;";
    round_trip(src);
    let items = build(src);
    assert_eq!(items.len(), 1, "one statement stays one Item");
    match &items[0].0.kind {
        ItemKind::Var(vds) => {
            assert_eq!(vds.len(), 2);
            assert_eq!(vds[0].name, "m_x");
            assert_eq!(vds[0].ty, Type::Named("fixed_t".to_string()));
            assert_eq!(vds[1].name, "m_y");
            assert_eq!(vds[1].ty, Type::Named("fixed_t".to_string()));
        }
        other => panic!("expected Var, got {other:?}"),
    }
}
