use super::*;
use crate::parser::scan::scan;

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
    let cd = try_parse_const_braced("mobjinfo_t mobjinfo[NUMMOBJTYPES] =", &scan(" /* ... */ "))
        .unwrap();
    assert_eq!(cd.ty, "mobjinfo_t");
    assert_eq!(cd.name, "mobjinfo");
    assert_eq!(cd.array_dims, vec![Some("NUMMOBJTYPES".to_string())]);
}

#[test]
fn braced_init_splits_scalar_elements() {
    // rndtable-style: a flat list of scalar elements, no nesting.
    let cd = try_parse_const_braced("int rndtable[] =", &scan(" 0, 8, 109, DI_NODIR ")).unwrap();
    match cd.initializer {
        Some(Init::Braced(elements)) => {
            let texts: Vec<&str> = elements
                .iter()
                .map(|e| match e {
                    Init::Expr(s) => s.as_str(),
                    Init::Braced(_) => panic!("expected a scalar element"),
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
    let cd = try_parse_const_braced("default_t defaults[] =", &scan(src)).unwrap();
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
                            })
                            .collect();
                        assert_eq!(texts, expected);
                    }
                    Init::Expr(_) => panic!("expected a nested Braced row"),
                }
            }
        }
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn braced_init_no_trailing_comma_still_captures_last_element() {
    let cd = try_parse_const_braced("int xs[] =", &scan(" 1, 2, 3 ")).unwrap();
    match cd.initializer {
        Some(Init::Braced(elements)) => assert_eq!(elements.len(), 3),
        other => panic!("expected Braced, got {other:?}"),
    }
}

#[test]
fn braced_init_drops_comment_with_comma_without_fracturing() {
    let cd = try_parse_const_braced(
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
                })
                .collect();
            assert_eq!(texts, vec!["1", "2", "3"]);
        }
        other => panic!("expected Braced, got {other:?}"),
    }
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
        &scan(" wipe_initColorXForm, wipe_doColorXForm "),
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
