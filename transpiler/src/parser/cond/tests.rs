use super::*;
use crate::parser::record::build_items;
use crate::parser::scan::scan;
use std::collections::HashMap;
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
fn mid_array_initializer_ifdef_item_shape_unaffected_by_top_level_folding() {
    // Same shape as m_misc.c's `defaults[]` table: an #ifdef inside a braced
    // array initializer never becomes a sibling Item in the first place, so
    // *this* pass (top-level Item folding) must be a complete no-op here -
    // same behavior as before this step existed. The initializer's own
    // *internal* structure is a separate story: `decl::parse_braced_init`
    // folds it into `Init::Conditional` on its own (see
    // `decl::tests::braced_init_folds_ifdef_between_rows`) - this test only
    // documents that `cond::fold_conditionals` itself never sees it.
    let src = "default_t defaults[] =\n{\n    {\"a\", &a, 1},\n#ifdef FOO\n    {\"b\", &b, 2},\n#endif\n};\n";
    let before = build_items(scan(src));
    let after = fold_conditionals(build_items(scan(src)));
    assert_eq!(before.len(), after.len());
    match &after[0].0.kind {
        ItemKind::Var(cd) => {
            assert_eq!(cd.name, "defaults");
            let elements = match &cd.initializer {
                Some(Init::Braced(elements)) => elements,
                other => panic!("expected Braced initializer, got {other:?}"),
            };
            assert_eq!(elements.len(), 2, "a-row, folded conditional");
            assert!(matches!(&elements[1], Init::Conditional(_)));
        }
        other => panic!("expected Var, got {other:?}"),
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

fn defines(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn group_of(items: &[(Item, Trivia)]) -> &CondGroup {
    match &items[0].0.kind {
        ItemKind::Conditional(g) => g,
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn resolves_ifdef_true() {
    let mut items = build("#ifdef FOO\nint x;\n#endif\n");
    resolve_conditionals(&mut items, &defines(&[("FOO", "")]));
    assert_eq!(group_of(&items).active, ActiveBranch::Branch(0));
}

#[test]
fn resolves_ifdef_false_falls_to_none_without_else() {
    let mut items = build("#ifdef FOO\nint x;\n#endif\n");
    resolve_conditionals(&mut items, &defines(&[]));
    assert_eq!(group_of(&items).active, ActiveBranch::None);
}

#[test]
fn resolves_ifdef_false_falls_to_else() {
    let mut items = build("#ifdef FOO\nint x;\n#else\nint y;\n#endif\n");
    resolve_conditionals(&mut items, &defines(&[]));
    assert_eq!(group_of(&items).active, ActiveBranch::Else);
}

#[test]
fn resolves_elif_chain_first_true_wins() {
    let mut items = build("#if A\nint a;\n#elif B\nint b;\n#else\nint c;\n#endif\n");
    resolve_conditionals(&mut items, &defines(&[("A", "0"), ("B", "1")]));
    assert_eq!(group_of(&items).active, ActiveBranch::Branch(1));
}

#[test]
fn resolves_bare_0_and_1_literals() {
    let mut off = build("#if 0\nint x;\n#endif\n");
    resolve_conditionals(&mut off, &defines(&[]));
    assert_eq!(group_of(&off).active, ActiveBranch::None);

    let mut on = build("#if 1\nint x;\n#endif\n");
    resolve_conditionals(&mut on, &defines(&[]));
    assert_eq!(group_of(&on).active, ActiveBranch::Branch(0));
}

#[test]
fn elif_bare_identifier_loses_to_earlier_ifdef_true() {
    // Real corpus shape (i_sound.c): #ifdef SNDSERV ... #elif SNDINTR ...
    // #endif - SNDSERV is #define'd (true), so it must win regardless of
    // whether SNDINTR is ever defined.
    let mut items = build("#ifdef SNDSERV\nint a;\n#elif SNDINTR\nint b;\n#endif\n");
    resolve_conditionals(&mut items, &defines(&[("SNDSERV", "1")]));
    assert_eq!(group_of(&items).active, ActiveBranch::Branch(0));
}

#[test]
fn unresolvable_expression_yields_unknown_and_stops() {
    // A bare identifier is always decidable (looked up, defaults False if
    // undefined) - genuine `Unknown` needs an actual expression shape this
    // evaluator doesn't attempt (operators, `defined()`, ...), none of
    // which occur anywhere in the real corpus.
    let mut items = build("#if VERSION >= 2\nint a;\n#else\nint b;\n#endif\n");
    resolve_conditionals(&mut items, &defines(&[]));
    // Can't safely fall through to `#else` either - the condition wasn't
    // decidably false, just undecidable.
    assert_eq!(group_of(&items).active, ActiveBranch::Unknown);
}

#[test]
fn unresolved_before_resolve_conditionals_runs() {
    let items = build("#ifdef FOO\nint x;\n#endif\n");
    assert_eq!(group_of(&items).active, ActiveBranch::Unknown);
}

#[test]
fn nested_conditional_resolves_independently_of_ancestor() {
    // The outer branch is false (never resolves True), but the nested
    // conditional inside the *other* (unreachable) branch should still get
    // its own independent resolution - useful information regardless.
    let src = "#ifdef OUTER\nint a;\n#else\n#ifdef INNER\nint b;\n#endif\n#endif\n";
    let mut items = build(src);
    resolve_conditionals(&mut items, &defines(&[("INNER", "")]));
    let outer = group_of(&items);
    assert_eq!(outer.active, ActiveBranch::Else);
    let else_body = outer.else_body.as_ref().expect("expected else body");
    let inner = match &else_body[0].0.kind {
        ItemKind::Conditional(g) => g,
        other => panic!("expected nested Conditional, got {other:?}"),
    };
    assert_eq!(inner.active, ActiveBranch::Branch(0));
}

#[test]
fn resolves_ifdef_inside_braced_initializer() {
    // m_misc.c's defaults[] shape: resolve_conditionals must reach into a
    // Var's initializer, not just top-level/function-body conditionals.
    let src = "default_t defaults[] =\n{\n    {\"a\", &a, 1},\n#ifdef FOO\n    {\"b\", &b, 2},\n#endif\n};\n";
    let mut items = build(src);
    resolve_conditionals(&mut items, &defines(&[("FOO", "1")]));
    let cd = match &items[0].0.kind {
        ItemKind::Var(cd) => cd,
        other => panic!("expected Var, got {other:?}"),
    };
    let elements = match &cd.initializer {
        Some(Init::Braced(elements)) => elements,
        other => panic!("expected Braced initializer, got {other:?}"),
    };
    match &elements[1] {
        Init::Conditional(group) => assert_eq!(group.active, ActiveBranch::Branch(0)),
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn resolve_conditionals_does_not_change_render_output() {
    let src = "#ifdef FOO\nint x;\n#else\nint y;\n#endif\n";
    let mut items = build(src);
    resolve_conditionals(&mut items, &defines(&[("FOO", "1")]));
    assert_eq!(
        render_items(&items),
        src,
        "active-branch resolution must not touch bytes"
    );
}
