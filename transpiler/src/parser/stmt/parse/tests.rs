use super::*;
use crate::parser::ast::RawToken;
use crate::parser::scan::scan;
use crate::parser::stmt::expr::Expr;

fn body(src: &str) -> Block {
    body_with(src, &KnownTypeNames::new())
}

fn body_with(src: &str, known: &KnownTypeNames) -> Block {
    let toks: Vec<RawToken> = scan(src);
    parse_block(toks, known)
}

fn round_trips(src: &str) {
    let block = body(src);
    assert_eq!(render_block(&block), src, "round trip mismatch");
}

#[test]
fn simple_expr_and_decl_statements_round_trip() {
    let src = "int x = 1;\nx = x + 1;\nreturn x;";
    round_trips(src);
    let block = body(src);
    assert_eq!(block.stmts.len(), 3);
    assert!(matches!(block.stmts[0].0.kind, StmtKind::Decl(_)));
    assert!(matches!(block.stmts[1].0.kind, StmtKind::Expr(_)));
    assert!(matches!(block.stmts[2].0.kind, StmtKind::Return(Some(_))));
}

#[test]
fn else_if_chain_is_nested_if() {
    let src = "if (a) {\n    x = 1;\n} else if (b) {\n    x = 2;\n} else {\n    x = 3;\n}\n";
    round_trips(src);
    let block = body(src);
    match &block.stmts[0].0.kind {
        StmtKind::If { else_branch, .. } => {
            let else_branch = else_branch.as_ref().expect("expected else branch");
            match &else_branch.kind {
                StmtKind::If { else_branch, .. } => {
                    assert!(else_branch.is_some(), "expected nested else if's own else");
                }
                other => panic!("expected nested If for else-if, got {other:?}"),
            }
        }
        other => panic!("expected If, got {other:?}"),
    }
}

#[test]
fn dangling_else_binds_to_nearest_if() {
    // if (x) if (y) z=1; else w=2; - the `else` must attach to the inner
    // `if (y)`, not the outer `if (x)` - exactly what a top-level ';'
    // pre-scan would get wrong and true recursive descent gets right.
    let src = "if (x)\n    if (y)\n        z = 1;\n    else\n        w = 2;\n";
    round_trips(src);
    let block = body(src);
    match &block.stmts[0].0.kind {
        StmtKind::If {
            then_branch,
            else_branch,
            ..
        } => {
            assert!(else_branch.is_none(), "outer if must not get the else");
            match &then_branch.kind {
                StmtKind::If { else_branch, .. } => {
                    assert!(else_branch.is_some(), "inner if must get the else");
                }
                other => panic!("expected nested If, got {other:?}"),
            }
        }
        other => panic!("expected If, got {other:?}"),
    }
}

#[test]
fn do_while_trailer_is_consumed() {
    let src = "do {\n    x++;\n} while (x < 10);";
    round_trips(src);
    let block = body(src);
    assert_eq!(block.stmts.len(), 1);
    assert!(matches!(block.stmts[0].0.kind, StmtKind::DoWhile { .. }));
}

#[test]
fn stacked_case_default_labels() {
    let src = "switch (x) {\ncase 1:\ncase 2:\n    y = 1;\n    break;\ndefault:\n    y = 2;\n    break;\n}\n";
    round_trips(src);
    let block = body(src);
    match &block.stmts[0].0.kind {
        StmtKind::Switch { body, .. } => match &body.kind {
            StmtKind::Block(inner) => {
                // First real statement carries both stacked case labels.
                let first = &inner.stmts[0].0;
                assert_eq!(first.labels.len(), 2);
                assert!(matches!(first.labels[0], Label::Case(_)));
                assert!(matches!(first.labels[1], Label::Case(_)));
                // The default: label sits on its own following statement.
                let default_stmt = inner
                    .stmts
                    .iter()
                    .find(|(s, _)| s.labels.iter().any(|l| matches!(l, Label::Default)));
                assert!(
                    default_stmt.is_some(),
                    "expected a statement carrying Label::Default"
                );
            }
            other => panic!("expected Block body, got {other:?}"),
        },
        other => panic!("expected Switch, got {other:?}"),
    }
}

#[test]
fn goto_and_label_pair() {
    let src = "goto done;\nx = 1;\ndone:\n    return x;\n";
    round_trips(src);
    let block = body(src);
    assert!(matches!(block.stmts[0].0.kind, StmtKind::Goto(ref n) if n == "done"));
    let labeled = &block.stmts[2].0;
    assert_eq!(labeled.labels.len(), 1);
    assert!(matches!(&labeled.labels[0], Label::Named(n) if n == "done"));
}

#[test]
fn multi_declarator_local_in_real_statement_position() {
    let src = "int *a, b[4];\n";
    round_trips(src);
    let block = body(src);
    match &block.stmts[0].0.kind {
        StmtKind::Decl(d) => assert_eq!(d.declarators.len(), 2),
        other => panic!("expected Decl, got {other:?}"),
    }
}

#[test]
fn nested_explicit_block() {
    // `src` is a function body's *contents* (not including the function's
    // own outer braces, matching `parse_block`'s contract) - the inner
    // `{ ... }` here is a genuine nested compound statement, not the
    // function body's own delimiters.
    let src = "int x = 1;\n{\n    int y = 2;\n    x = y;\n}";
    round_trips(src);
    let block = body(src);
    assert_eq!(block.stmts.len(), 2);
    match &block.stmts[1].0.kind {
        // 2 real statements + 1 trailing catch-all `Raw` for the blank-line
        // whitespace before the closing `}` (same "the leftover trailing
        // bytes become their own final Raw entry" pattern
        // `record::build_items` already uses at the top level).
        StmtKind::Block(inner) => assert_eq!(inner.stmts.len(), 3),
        other => panic!("expected nested Block, got {other:?}"),
    }
}

#[test]
fn comma_expression_for_loop() {
    let src = "for (i = 0, j = 10; i < j; i++, j--) {\n    x = i;\n}\n";
    round_trips(src);
    let block = body(src);
    match &block.stmts[0].0.kind {
        StmtKind::For {
            init: Some(ForInit::Expr(Expr::Comma(init_parts))),
            step: Some(Expr::Comma(step_parts)),
            ..
        } => {
            assert_eq!(init_parts.len(), 2);
            assert_eq!(step_parts.len(), 2);
        }
        other => panic!("expected For with comma init/step, got {other:?}"),
    }
}

#[test]
fn for_loop_with_declaration_init() {
    let src = "for (int i = 0; i < 10; i++) {\n    x = i;\n}\n";
    round_trips(src);
    let block = body(src);
    match &block.stmts[0].0.kind {
        StmtKind::For {
            init: Some(ForInit::Decl(_)),
            ..
        } => {}
        other => panic!("expected For with Decl init, got {other:?}"),
    }
}

#[test]
fn mid_body_ifdef_becomes_flat_preproc_sibling() {
    let src = "x = 1;\n#ifdef RANGECHECK\ny = 2;\n#endif\nz = 3;";
    round_trips(src);
    let block = body(src);
    assert_eq!(block.stmts.len(), 5);
    assert!(matches!(block.stmts[0].0.kind, StmtKind::Expr(_)));
    assert!(matches!(block.stmts[1].0.kind, StmtKind::Preproc(_)));
    assert!(matches!(block.stmts[2].0.kind, StmtKind::Expr(_)));
    assert!(matches!(block.stmts[3].0.kind, StmtKind::Preproc(_)));
    assert!(matches!(block.stmts[4].0.kind, StmtKind::Expr(_)));
}

#[test]
fn same_line_trailing_comment_attaches_to_its_own_statement() {
    let src = "x = 1; // meaning\ny = 2;\n";
    round_trips(src);
    let block = body(src);
    assert!(block.stmts[0].0.raw.contains("// meaning"));
    assert!(!block.stmts[1].0.raw.contains("// meaning"));
}

#[test]
fn leading_doc_comment_becomes_trivia_not_raw() {
    let src = "// note about y\ny = 2;\n";
    round_trips(src);
    let block = body(src);
    assert_eq!(block.stmts[0].1.leading.len(), 1);
    assert!(!block.stmts[0].0.raw.contains("// note about y"));
}

#[test]
fn empty_block_round_trips() {
    round_trips("");
}

#[test]
fn trailing_comment_with_nothing_after_is_preserved() {
    let src = "x = 1;\n// trailing note\n";
    round_trips(src);
}

#[test]
fn cast_disambiguation_uses_known_types_inside_statements() {
    let mut known = KnownTypeNames::new();
    known.insert("mobj_t");
    let block = body_with("x = (mobj_t *) y;\n", &known);
    match &block.stmts[0].0.kind {
        StmtKind::Expr(Expr::Assign { rhs, .. }) => {
            assert!(
                matches!(**rhs, Expr::Cast { .. }),
                "expected Cast, got {rhs:?}"
            );
        }
        other => panic!("expected Expr(Assign), got {other:?}"),
    }
}
