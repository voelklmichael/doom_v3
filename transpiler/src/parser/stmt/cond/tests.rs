use super::*;
use crate::parser::ast::RawToken;
use crate::parser::scan::scan;
use crate::parser::stmt::expr::KnownTypeNames;
use crate::parser::stmt::parse::{parse_block, render_block};

fn build(src: &str) -> Block {
    let toks: Vec<RawToken> = scan(src);
    fold_conditionals(parse_block(toks, &KnownTypeNames::new()))
}

fn defines(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn group_of(block: &Block) -> &StmtCondGroup {
    match &block.stmts[0].0.kind {
        StmtKind::Conditional(g) => g,
        other => panic!("expected Conditional, got {other:?}"),
    }
}

fn round_trip(src: &str) {
    let block = build(src);
    assert_eq!(render_block(&block), src, "round trip mismatch");
}

#[test]
fn plain_ifdef_endif() {
    let src = "x = 1;\n#ifdef FOO\ny = 2;\n#endif";
    round_trip(src);
    let block = build(src);
    assert_eq!(block.stmts.len(), 2);
    match &block.stmts[1].0.kind {
        StmtKind::Conditional(g) => {
            assert_eq!(g.branches.len(), 1);
            assert!(g.else_body.is_none());
            assert!(matches!(
                &g.branches[0].directive,
                Directive::IfDef { name, negate: false } if name == "FOO"
            ));
            assert!(render_block(&g.branches[0].body).contains("y = 2;"));
        }
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn if_else_endif() {
    let src = "#if FOO\na = 1;\n#else\nb = 2;\n#endif";
    round_trip(src);
    let block = build(src);
    match &block.stmts[0].0.kind {
        StmtKind::Conditional(g) => {
            assert_eq!(g.branches.len(), 1);
            assert!(render_block(&g.branches[0].body).contains("a = 1;"));
            let else_body = g.else_body.as_ref().expect("expected else body");
            assert!(render_block(else_body).contains("b = 2;"));
        }
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn if_elif_else_endif() {
    let src = "#if A\na = 1;\n#elif B\nb = 2;\n#else\nc = 3;\n#endif";
    round_trip(src);
    let block = build(src);
    match &block.stmts[0].0.kind {
        StmtKind::Conditional(g) => {
            assert_eq!(g.branches.len(), 2);
            assert!(matches!(&g.branches[0].directive, Directive::If { expr } if expr == "A"));
            assert!(matches!(&g.branches[1].directive, Directive::Elif { expr } if expr == "B"));
            assert!(g.else_body.is_some());
        }
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn nested_ifdef_inside_ifdef() {
    let src = "#ifdef OUTER\nx = 1;\n#ifdef INNER\ny = 2;\n#endif\nz = 3;\n#endif";
    round_trip(src);
    let block = build(src);
    match &block.stmts[0].0.kind {
        StmtKind::Conditional(outer) => {
            let body = &outer.branches[0].body;
            assert_eq!(body.stmts.len(), 3);
            assert!(matches!(body.stmts[1].0.kind, StmtKind::Conditional(_)));
        }
        other => panic!("expected Conditional, got {other:?}"),
    }
}

#[test]
fn ifdef_inside_if_body_block() {
    // The #ifdef sits inside an `if`'s own nested Block, not the function's
    // top-level statement list - this is the case that needs recursion into
    // nested Blocks, unlike the top-level (flat-only) `cond::fold_conditionals`.
    let src = "if (cond) {\n    x = 1;\n#ifdef FOO\n    y = 2;\n#endif\n}";
    round_trip(src);
    let block = build(src);
    match &block.stmts[0].0.kind {
        StmtKind::If { then_branch, .. } => match &then_branch.kind {
            StmtKind::Block(inner) => {
                assert!(
                    inner
                        .stmts
                        .iter()
                        .any(|(s, _)| matches!(s.kind, StmtKind::Conditional(_))),
                    "expected a Conditional statement inside the if-body block"
                );
            }
            other => panic!("expected Block, got {other:?}"),
        },
        other => panic!("expected If, got {other:?}"),
    }
}

#[test]
fn unterminated_ifdef_degrades_to_flat() {
    // No matching #endif - must not lose bytes or panic, just stay flat.
    let src = "x = 1;\n#ifdef FOO\ny = 2;";
    round_trip(src);
    let block = build(src);
    assert!(
        !block
            .stmts
            .iter()
            .any(|(s, _)| matches!(s.kind, StmtKind::Conditional(_))),
        "an unterminated #ifdef must not be folded"
    );
}

#[test]
fn resolves_ifdef_true() {
    let mut block = build("#ifdef FOO\nx = 1;\n#endif");
    resolve_conditionals(&mut block, &defines(&[("FOO", "")]));
    assert_eq!(group_of(&block).active, ActiveBranch::Branch(0));
}

#[test]
fn resolves_ifdef_false_falls_to_else() {
    let mut block = build("#ifdef FOO\nx = 1;\n#else\ny = 2;\n#endif");
    resolve_conditionals(&mut block, &defines(&[]));
    assert_eq!(group_of(&block).active, ActiveBranch::Else);
}

#[test]
fn unresolved_before_resolve_conditionals_runs() {
    let block = build("#ifdef FOO\nx = 1;\n#endif");
    assert_eq!(group_of(&block).active, ActiveBranch::Unknown);
}

#[test]
fn resolves_inside_nested_block() {
    // The #ifdef sits inside an `if`'s own nested Block - resolution must
    // recurse into it, same as folding already does.
    let src = "if (cond) {\n    x = 1;\n#ifdef FOO\n    y = 2;\n#endif\n}";
    let mut block = build(src);
    resolve_conditionals(&mut block, &defines(&[("FOO", "1")]));
    match &block.stmts[0].0.kind {
        StmtKind::If { then_branch, .. } => match &then_branch.kind {
            StmtKind::Block(inner) => {
                let (cond_stmt, _) = inner
                    .stmts
                    .iter()
                    .find(|(s, _)| matches!(s.kind, StmtKind::Conditional(_)))
                    .expect("expected a Conditional statement inside the if-body block");
                match &cond_stmt.kind {
                    StmtKind::Conditional(g) => assert_eq!(g.active, ActiveBranch::Branch(0)),
                    _ => unreachable!(),
                }
            }
            other => panic!("expected Block, got {other:?}"),
        },
        other => panic!("expected If, got {other:?}"),
    }
}

#[test]
fn resolve_conditionals_does_not_change_render_output() {
    let src = "#ifdef FOO\nx = 1;\n#else\ny = 2;\n#endif";
    let mut block = build(src);
    resolve_conditionals(&mut block, &defines(&[("FOO", "1")]));
    assert_eq!(
        render_block(&block),
        src,
        "active-branch resolution must not touch bytes"
    );
}
