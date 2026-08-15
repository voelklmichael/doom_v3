use super::*;
use crate::parser::record::build_items;
use crate::parser::scan::scan;
use std::path::PathBuf;

fn build(src: &str) -> Vec<(Item, Trivia)> {
    fold_conditionals(build_items(scan(src)))
}

fn round_trip(src: &str) {
    let items = build(src);
    assert_eq!(render_items(&items), src, "round trip mismatch");
}

fn body_text(body: &[(Item, Trivia)]) -> String {
    render_items(body)
}

#[test]
fn plain_ifdef_endif() {
    let src = "#ifdef FOO\nint x;\n#endif\n";
    round_trip(src);
    let items = build(src);
    assert_eq!(items.len(), 1);
    match &items[0].0.kind {
        ItemKind::Conditional(g) => {
            assert_eq!(g.branches.len(), 1);
            assert!(g.else_body.is_none());
            assert!(matches!(
                &g.branches[0].directive,
                Directive::IfDef { name, negate: false } if name == "FOO"
            ));
            assert!(body_text(&g.branches[0].body).contains("int x;"));
        }
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn if_else_endif() {
    let src = "#if FOO\nint a;\n#else\nint b;\n#endif\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Conditional(g) => {
            assert_eq!(g.branches.len(), 1);
            assert!(body_text(&g.branches[0].body).contains("int a;"));
            let else_body = g.else_body.as_ref().expect("expected else body");
            assert!(body_text(else_body).contains("int b;"));
        }
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn if_elif_elif_else_endif() {
    let src = "#if A\nint a;\n#elif B\nint b;\n#elif C\nint c;\n#else\nint d;\n#endif\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Conditional(g) => {
            assert_eq!(g.branches.len(), 3);
            assert!(matches!(&g.branches[0].directive, Directive::If { expr } if expr == "A"));
            assert!(matches!(&g.branches[1].directive, Directive::Elif { expr } if expr == "B"));
            assert!(matches!(&g.branches[2].directive, Directive::Elif { expr } if expr == "C"));
            assert!(body_text(&g.branches[2].body).contains("int c;"));
            assert!(body_text(g.else_body.as_ref().unwrap()).contains("int d;"));
        }
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn nested_conditional_proves_recursion() {
    let src = "#ifdef OUTER\nint a;\n#ifdef INNER\nint b;\n#endif\nint c;\n#endif\n";
    round_trip(src);
    let items = build(src);
    match &items[0].0.kind {
        ItemKind::Conditional(outer) => {
            let body = &outer.branches[0].body;
            let nested = body
                .iter()
                .find_map(|(item, _)| match &item.kind {
                    ItemKind::Conditional(g) => Some(g),
                    _ => None,
                })
                .expect("expected a nested Conditional item in the outer body");
            assert!(matches!(
                &nested.branches[0].directive,
                Directive::IfDef { name, .. } if name == "INNER"
            ));
            assert!(body_text(&nested.branches[0].body).contains("int b;"));
            assert!(body_text(body).contains("int c;"));
        }
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn stray_endif_with_no_open_group_is_left_flat() {
    let src = "#endif\nint x;\n";
    round_trip(src);
    let items = build(src);
    assert!(matches!(
        items[0].0.kind,
        ItemKind::Preproc(Directive::Endif)
    ));
}

#[test]
fn unterminated_if_at_eof_flattens_back_to_plain_items() {
    let src = "#ifdef FOO\nint x;\n";
    round_trip(src);
    let items = build(src);
    // No matching #endif - nothing should be folded into a Conditional.
    assert!(
        !items
            .iter()
            .any(|(item, _)| matches!(item.kind, ItemKind::Conditional(_))),
        "expected no Conditional item for an unterminated #if"
    );
    assert!(matches!(
        items[0].0.kind,
        ItemKind::Preproc(Directive::IfDef { .. })
    ));
}

#[test]
fn mid_array_initializer_ifdef_stays_opaque_unaffected_by_folding() {
    // Same shape as m_misc.c's `defaults[]` table: an #ifdef inside a braced
    // array initializer never becomes a sibling Item in the first place, so
    // folding must be a complete no-op here - same behavior as before this
    // step existed.
    let src = "default_t defaults[] =\n{\n    {\"a\", &a, 1},\n#ifdef FOO\n    {\"b\", &b, 2},\n#endif\n};\n";
    let before = build_items(scan(src));
    let after = fold_conditionals(build_items(scan(src)));
    assert_eq!(before.len(), after.len());
    match &after[0].0.kind {
        ItemKind::Const(cd) => {
            assert_eq!(cd.name, "defaults");
        }
        other => panic!("expected Const, got {other:?}"),
    }
    assert!(after[0].0.raw.contains("#ifdef FOO"));
}

#[test]
fn include_guard_strips_real_header_shape() {
    let src = "#ifndef __FOO_H__\n#define __FOO_H__\n\nint x;\n\n#endif\n";
    let file = File {
        path: PathBuf::from("foo.h"),
        items: build(src),
    };
    assert_eq!(include_guard_name(&file).as_deref(), Some("__FOO_H__"));

    let mut file = file;
    let stripped = strip_include_guard(&mut file);
    assert_eq!(stripped.as_deref(), Some("__FOO_H__"));
    let rendered = render_items(&file.items);
    assert!(rendered.contains("int x;"));
    assert!(!rendered.contains("#ifndef"));
    assert!(!rendered.contains("#define"));
    assert!(!rendered.contains("#endif"));
}

#[test]
fn include_guard_none_when_shape_differs() {
    // Plain #ifdef (not #ifndef) at the top isn't the include-guard idiom.
    let src = "#ifdef FOO\nint x;\n#endif\n";
    let mut file = File {
        path: PathBuf::from("foo.h"),
        items: build(src),
    };
    assert_eq!(include_guard_name(&file), None);
    assert_eq!(strip_include_guard(&mut file), None);
    assert!(render_items(&file.items).contains("#ifdef FOO"));
}

#[test]
fn include_guard_none_when_it_has_an_else() {
    let src = "#ifndef __FOO_H__\n#define __FOO_H__\nint x;\n#else\nint y;\n#endif\n";
    let file = File {
        path: PathBuf::from("foo.h"),
        items: build(src),
    };
    assert_eq!(include_guard_name(&file), None);
}
