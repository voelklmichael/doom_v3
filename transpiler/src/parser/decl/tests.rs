use super::*;
use crate::parser::scan::scan;

#[test]
fn rcsid_style() {
    let cds = try_parse_var_flat("static const char rcsid[] = \"$Id$\";").unwrap();
    assert_eq!(cds.len(), 1);
    let cd = &cds[0];
    assert_eq!(cd.storage, vec![Storage::Static, Storage::Const]);
    assert_eq!(
        cd.ty,
        Type::Array(Box::new(Type::Named("char".to_string())), None)
    );
    assert_eq!(cd.name, "rcsid");
    assert!(matches!(cd.initializer, Some(Init::Expr(_))));
}

#[test]
fn bare_forward_declaration_is_not_a_variable() {
    // r_defs.h's real `struct line_s;` - a bodyless forward declaration,
    // not a variable named after its own tag. Must fail to parse here so
    // classification falls through to `ItemKind::Raw` instead of producing
    // a bogus `Var` with type `Named("struct")` (renders as `struct_`).
    assert!(try_parse_var_flat("struct line_s;").is_none());
    assert!(try_parse_var_flat("union foo;").is_none());
    assert!(try_parse_var_flat("enum bar;").is_none());
}

#[test]
fn braced_array_style() {
    let cd =
        try_parse_var_braced("mobjinfo_t mobjinfo[NUMMOBJTYPES] =", &scan(" /* ... */ ")).unwrap();
    assert_eq!(
        cd.ty,
        Type::Array(
            Box::new(Type::Named("mobjinfo_t".to_string())),
            Some("NUMMOBJTYPES".to_string())
        )
    );
    assert_eq!(cd.name, "mobjinfo");
}

#[test]
fn braced_init_splits_scalar_elements() {
    // rndtable-style: a flat list of scalar elements, no nesting.
    let cd = try_parse_var_braced("int rndtable[] =", &scan(" 0, 8, 109, DI_NODIR ")).unwrap();
    match cd.initializer {
        Some(Init::Braced(elements)) => {
            let texts: Vec<&str> = elements
                .iter()
                .map(|e| match e {
                    Init::Expr(s) => s.as_str(),
                    Init::Braced(_) => panic!("expected a scalar element"),
                    Init::Conditional(_) => panic!("expected a scalar element"),
                })
                .collect();
            assert_eq!(texts, vec!["0", "8", "109", "DI_NODIR"]);
        }
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn braced_init_splits_nested_rows() {
    // m_misc.c's defaults[]-style table: each row is its own nested Braced
    // sub-list, e.g. {"mouse_sensitivity", &mouseSensitivity, 5}.
    let src = " {\"a\", &a, 1},\n    {\"b\", &b, 0},\n";
    let cd = try_parse_var_braced("default_t defaults[] =", &scan(src)).unwrap();
    match cd.initializer {
        Some(Init::Braced(rows)) => {
            assert_eq!(rows.len(), 2);
            for (row, expected) in rows
                .iter()
                .zip([["\"a\"", "&a", "1"], ["\"b\"", "&b", "0"]])
            {
                match row {
                    Init::Braced(cells) => {
                        let texts: Vec<&str> = cells
                            .iter()
                            .map(|e| match e {
                                Init::Expr(s) => s.as_str(),
                                Init::Braced(_) => panic!("expected a scalar cell"),
                                Init::Conditional(_) => panic!("expected a scalar cell"),
                            })
                            .collect();
                        assert_eq!(texts, expected);
                    }
                    Init::Expr(_) => panic!("expected a nested Braced row"),
                    Init::Conditional(_) => panic!("expected a nested Braced row"),
                }
            }
        }
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn braced_init_no_trailing_comma_still_captures_last_element() {
    let cd = try_parse_var_braced("int xs[] =", &scan(" 1, 2, 3 ")).unwrap();
    match cd.initializer {
        Some(Init::Braced(elements)) => assert_eq!(elements.len(), 3),
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn braced_init_drops_comment_with_comma_without_fracturing() {
    let cd = try_parse_var_braced(
        "int xs[] =",
        &scan(" 1, 2, // a comment, with a comma\n 3 "),
    )
    .unwrap();
    match cd.initializer {
        Some(Init::Braced(elements)) => {
            let texts: Vec<&str> = elements
                .iter()
                .map(|e| match e {
                    Init::Expr(s) => s.as_str(),
                    Init::Braced(_) => panic!("expected a scalar element"),
                    Init::Conditional(_) => panic!("expected a scalar element"),
                })
                .collect();
            assert_eq!(texts, vec!["1", "2", "3"]);
        }
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn braced_init_folds_ifdef_between_rows() {
    // m_misc.c's defaults[]-style gap: a whole run of rows gated by a
    // mid-initializer #ifdef, never reachable as its own top-level Item.
    let src = " {\"a\", &a, 1},\n\
#ifdef FOO\n\
    {\"b\", &b, 0},\n\
    {\"c\", &c, 2},\n\
#endif\n\
    {\"d\", &d, 3},\n";
    let cd = try_parse_var_braced("default_t defaults[] =", &scan(src)).unwrap();
    match cd.initializer {
        Some(Init::Braced(elements)) => {
            assert_eq!(elements.len(), 3, "a-row, folded conditional, d-row");
            assert!(matches!(&elements[0], Init::Braced(cells) if cells.len() == 3));
            assert!(matches!(&elements[2], Init::Braced(cells) if cells.len() == 3));
            match &elements[1] {
                Init::Conditional(group) => {
                    assert!(group.else_body.is_none());
                    assert_eq!(group.active, ActiveBranch::Unknown);
                    assert_eq!(group.branches.len(), 1);
                    assert!(matches!(
                        &group.branches[0].directive,
                        Directive::IfDef { name, negate: false } if name == "FOO"
                    ));
                    assert_eq!(group.branches[0].body.len(), 2);
                }
                other => panic!("expected Conditional, got {other:?}"),
            }
        }
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn braced_init_folds_nested_ifdef() {
    // The exact defaults[] shape: an #ifdef SNDSERV block nested inside the
    // #ifdef NORMALUNIX block, both gating rows of the same table.
    let src = " {\"a\", &a, 1},\n\
#ifdef NORMALUNIX\n\
    {\"b\", &b, 0},\n\
#ifdef SNDSERV\n\
    {\"c\", &c, 2},\n\
#endif\n\
#endif\n";
    let cd = try_parse_var_braced("default_t defaults[] =", &scan(src)).unwrap();
    match cd.initializer {
        Some(Init::Braced(elements)) => {
            assert_eq!(elements.len(), 2);
            match &elements[1] {
                Init::Conditional(outer) => {
                    assert!(matches!(
                        &outer.branches[0].directive,
                        Directive::IfDef { name, .. } if name == "NORMALUNIX"
                    ));
                    // one row + the nested SNDSERV conditional
                    assert_eq!(outer.branches[0].body.len(), 2);
                    match &outer.branches[0].body[1] {
                        Init::Conditional(inner) => {
                            assert!(matches!(
                                &inner.branches[0].directive,
                                Directive::IfDef { name, .. } if name == "SNDSERV"
                            ));
                            assert_eq!(inner.branches[0].body.len(), 1);
                        }
                        other => panic!("expected nested Conditional, got {other:?}"),
                    }
                }
                other => panic!("expected Conditional, got {other:?}"),
            }
        }
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn braced_init_unterminated_ifdef_flattens_at_eof() {
    // Malformed input (never seen in the real corpus, which is fully
    // balanced): a #ifdef with no matching #endif shouldn't lose its rows.
    let src = " {\"a\", &a, 1},\n#ifdef FOO\n    {\"b\", &b, 0},\n";
    let cd = try_parse_var_braced("default_t defaults[] =", &scan(src)).unwrap();
    match cd.initializer {
        Some(Init::Braced(elements)) => {
            assert_eq!(
                elements.len(),
                2,
                "both rows recovered, no Conditional wrapper"
            );
            assert!(elements.iter().all(|e| matches!(e, Init::Braced(_))));
        }
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn no_equals_means_no_initializer() {
    let vds = try_parse_var_flat("extern patch_t* hu_font[HU_FONTSIZE];").unwrap();
    assert_eq!(vds.len(), 1);
    let vd = &vds[0];
    assert_eq!(vd.storage, vec![Storage::Extern]);
    assert_eq!(
        vd.ty,
        Type::Array(
            Box::new(Type::Pointer(Box::new(Type::Named("patch_t".to_string())))),
            Some("HU_FONTSIZE".to_string())
        )
    );
    assert_eq!(vd.name, "hu_font");
    assert!(vd.initializer.is_none());
}

#[test]
fn fnptr_decl_without_initializer() {
    // i_video.c-style: `extern void (*colfunc)(void);` - a function-pointer
    // declarator with no initializer, exercising the same `parse_declarator`
    // fn-pointer path as the initialized case but through the "no top-level
    // `=`" branch of `try_parse_var_flat`.
    let vds = try_parse_var_flat("extern void (*colfunc)(void);").unwrap();
    assert_eq!(vds.len(), 1);
    let vd = &vds[0];
    assert_eq!(vd.storage, vec![Storage::Extern]);
    assert_eq!(
        vd.ty,
        Type::FunctionPointer {
            ret: Box::new(Type::Named("void".to_string())),
            params: vec![],
        }
    );
    assert_eq!(vd.name, "colfunc");
    assert!(vd.initializer.is_none());
}

#[test]
fn multi_declarator_var_splits_into_multiple_var_decls() {
    // am_map.c's real `static fixed_t m_x, m_y;` - previously collapsed
    // into one malformed VarDecl (`ty: Named("fixed_t m_x,")`, `name:
    // "m_y"`), silently dropping `m_x` from the AST entirely.
    let vds = try_parse_var_flat("static fixed_t m_x, m_y;").unwrap();
    assert_eq!(vds.len(), 2);
    assert_eq!(vds[0].storage, vec![Storage::Static]);
    assert_eq!(vds[0].ty, Type::Named("fixed_t".to_string()));
    assert_eq!(vds[0].name, "m_x");
    assert!(vds[0].initializer.is_none());
    assert_eq!(vds[1].storage, vec![Storage::Static]);
    assert_eq!(vds[1].ty, Type::Named("fixed_t".to_string()));
    assert_eq!(vds[1].name, "m_y");
    assert!(vds[1].initializer.is_none());
}

#[test]
fn multi_declarator_var_with_three_names() {
    // g_game.c's real `int totalkills, totalitems, totalsecret;`.
    let vds = try_parse_var_flat("int totalkills, totalitems, totalsecret;").unwrap();
    let names: Vec<&str> = vds.iter().map(|v| v.name.as_str()).collect();
    assert_eq!(names, vec!["totalkills", "totalitems", "totalsecret"]);
}

#[test]
fn multi_declarator_var_with_per_declarator_initializers_and_pointer_star() {
    // Not a real corpus case, but valid C (`int *a, b;` - the pointer star
    // applies per-declarator, not to the shared base type) and per-
    // declarator initializers must each attach to their own VarDecl, not
    // leak into a sibling's.
    let vds = try_parse_var_flat("int *a = 0, b = 1;").unwrap();
    assert_eq!(vds.len(), 2);
    assert_eq!(
        vds[0].ty,
        Type::Pointer(Box::new(Type::Named("int".to_string())))
    );
    assert_eq!(vds[0].name, "a");
    assert!(matches!(&vds[0].initializer, Some(Init::Expr(s)) if s == "0"));
    assert_eq!(vds[1].ty, Type::Named("int".to_string()));
    assert_eq!(vds[1].name, "b");
    assert!(matches!(&vds[1].initializer, Some(Init::Expr(s)) if s == "1"));
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
    // structured fields were nonsense. The param's own name ("in") is
    // discarded - Type::FunctionPointer::params is types only.
    let td = try_parse_typedef_flat("typedef boolean (*traverser_t) (intercept_t *in);").unwrap();
    assert_eq!(td.name, "traverser_t");
    assert_eq!(
        td.underlying,
        Type::FunctionPointer {
            ret: Box::new(Type::Named("boolean".to_string())),
            params: vec![Type::Pointer(Box::new(Type::Named(
                "intercept_t".to_string()
            )))],
        }
    );
}

#[test]
fn fnptr_typedef_with_empty_params() {
    // d_think.h's actionf_v.
    let td = try_parse_typedef_flat("typedef  void (*actionf_v)();").unwrap();
    assert_eq!(td.name, "actionf_v");
    assert_eq!(
        td.underlying,
        Type::FunctionPointer {
            ret: Box::new(Type::Named("void".to_string())),
            params: vec![],
        }
    );
}

#[test]
fn fnptr_typedef_with_anonymous_params() {
    // d_think.h's actionf_p2.
    let td = try_parse_typedef_flat("typedef  void (*actionf_p2)( void*, void* );").unwrap();
    assert_eq!(td.name, "actionf_p2");
    assert_eq!(
        td.underlying,
        Type::FunctionPointer {
            ret: Box::new(Type::Named("void".to_string())),
            params: vec![
                Type::Pointer(Box::new(Type::Named("void".to_string()))),
                Type::Pointer(Box::new(Type::Named("void".to_string()))),
            ],
        }
    );
}

#[test]
fn fnptr_array_declarator_with_sized_dim() {
    let (storage, ty, name) = parse_declarator("void (*table[4])(int)").unwrap();
    assert!(storage.is_empty());
    assert_eq!(name, "table");
    assert_eq!(
        ty,
        Type::Array(
            Box::new(Type::FunctionPointer {
                ret: Box::new(Type::Named("void".to_string())),
                params: vec![Type::Named("int".to_string())],
            }),
            Some("4".to_string())
        )
    );
}

#[test]
fn fnptr_array_declarator_with_unsized_dim() {
    // f_wipe.c's `wipes[]`: `static int (*wipes[])(int, int, int) = { ... };`
    let cd = try_parse_var_braced(
        "static int (*wipes[])(int, int, int) =",
        &scan(" wipe_initColorXForm, wipe_doColorXForm "),
    )
    .unwrap();
    assert_eq!(cd.storage, vec![Storage::Static]);
    assert_eq!(cd.name, "wipes");
    assert_eq!(
        cd.ty,
        Type::Array(
            Box::new(Type::FunctionPointer {
                ret: Box::new(Type::Named("int".to_string())),
                params: vec![
                    Type::Named("int".to_string()),
                    Type::Named("int".to_string()),
                    Type::Named("int".to_string()),
                ],
            }),
            None
        )
    );
}

#[test]
fn fnptr_array_declarator_with_multiple_dims() {
    // The outer Array corresponds to the *first* bracket - `table[4][2]` is
    // an array of 4 (array of 2 fn-pointer).
    let (_, ty, name) = parse_declarator("void (*table[4][2])(int)").unwrap();
    assert_eq!(name, "table");
    let fn_ptr = Type::FunctionPointer {
        ret: Box::new(Type::Named("void".to_string())),
        params: vec![Type::Named("int".to_string())],
    };
    assert_eq!(
        ty,
        Type::Array(
            Box::new(Type::Array(Box::new(fn_ptr), Some("2".to_string()))),
            Some("4".to_string())
        )
    );
}

#[test]
fn fnptr_array_declarator_rejects_non_identifier_name() {
    // Guards against a malformed name (e.g. a stray `*`) inside the array
    // brackets being silently accepted.
    assert!(parse_declarator("void (*1table[4])(int)").is_none());
}

#[test]
fn pointer_stars_fold_into_type_regardless_of_spacing() {
    // "char*" (glued to the type) and "char *x" (glued to the name) must
    // both produce the same Pointer(Named("char")) shape - previously they
    // diverged into differently-spelled strings ("char*" vs "char *").
    let (_, ty1, _) = parse_declarator("char *rcsid").unwrap();
    let (_, ty2, _) = parse_declarator("char* rcsid").unwrap();
    let expected = Type::Pointer(Box::new(Type::Named("char".to_string())));
    assert_eq!(ty1, expected);
    assert_eq!(ty2, expected);
}

#[test]
fn double_pointer_is_nested() {
    let (_, ty, name) = parse_declarator("patch_t **p").unwrap();
    assert_eq!(name, "p");
    assert_eq!(
        ty,
        Type::Pointer(Box::new(Type::Pointer(Box::new(Type::Named(
            "patch_t".to_string()
        )))))
    );
}

#[test]
fn parse_type_text_handles_bare_and_pointer_types() {
    assert_eq!(parse_type_text("int"), Type::Named("int".to_string()));
    assert_eq!(
        parse_type_text("player_t*"),
        Type::Pointer(Box::new(Type::Named("player_t".to_string())))
    );
    assert_eq!(
        parse_type_text("char * *"),
        Type::Pointer(Box::new(Type::Pointer(Box::new(Type::Named(
            "char".to_string()
        )))))
    );
}
