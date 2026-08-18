use super::*;
use crate::parser::ast::RecordDecl;
use crate::parser::preproc::Directive;
use crate::parser::stmt::ast::{Label, StmtCondBranch, StmtCondGroup, StmtKind};
use crate::parser::stmt::decl::Declarator;
use crate::parser::stmt::expr::Expr;

/// Bundles empty (or caller-populated) `known_*` maps so individual tests
/// don't each repeat six empty-map declarations - `ctx()` borrows into a
/// `BodyCtx` on demand.
#[derive(Default)]
struct Fixture {
    known: KnownTypeNames,
    records: HashMap<String, RecordDecl>,
    typedefs: HashMap<String, Type>,
    functions: HashMap<String, FnSig>,
    globals: HashMap<String, Type>,
    defines: HashMap<String, String>,
}

impl Fixture {
    fn ctx(&self) -> BodyCtx<'_> {
        BodyCtx {
            known: &self.known,
            known_records: &self.records,
            known_typedefs: &self.typedefs,
            known_functions: &self.functions,
            known_globals: &self.globals,
            known_defines: &self.defines,
        }
    }
}

fn stmt(kind: StmtKind) -> Stmt {
    Stmt {
        kind,
        labels: vec![],
        raw: "<test>".to_string(),
    }
}

fn block(stmts: Vec<Stmt>) -> Block {
    Block {
        stmts: stmts
            .into_iter()
            .map(|s| (s, crate::parser::ast::Trivia::default()))
            .collect(),
    }
}

fn named(text: &str) -> Type {
    Type::Named(text.to_string())
}

#[test]
fn expr_statement_renders_through_render_expr() {
    let f = Fixture::default();
    let s = stmt(StmtKind::Expr(Expr::Ident("foo".to_string())));
    assert_eq!(render_stmt(&s, &f.ctx()), "foo;\n");
}

#[test]
fn return_with_value() {
    let f = Fixture::default();
    let s = stmt(StmtKind::Return(Some(Expr::IntLit("1".to_string()))));
    assert_eq!(render_stmt(&s, &f.ctx()), "return 1;\n");
}

#[test]
fn return_without_value() {
    let f = Fixture::default();
    let s = stmt(StmtKind::Return(None));
    assert_eq!(render_stmt(&s, &f.ctx()), "return;\n");
}

#[test]
fn empty_statement_renders_nothing() {
    let f = Fixture::default();
    let s = stmt(StmtKind::Empty);
    assert_eq!(render_stmt(&s, &f.ctx()), "");
}

#[test]
fn raw_statement_degrades_with_comment_and_todo() {
    let f = Fixture::default();
    let mut s = stmt(StmtKind::Raw);
    s.raw = "weird_stmt();".to_string();
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("// weird_stmt();"));
    assert!(out.contains("todo!(\"statement not yet translated\");"));
}

#[test]
fn goto_degrades_with_label_name_in_reason() {
    let f = Fixture::default();
    let s = stmt(StmtKind::Goto("retry".to_string()));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("goto retry not translated"));
}

#[test]
fn named_label_becomes_a_comment_and_statement_still_translates() {
    // Only the jump itself is untranslatable - control that reaches this
    // statement by ordinary fall-through in C still reaches it here, so
    // the labeled statement itself must NOT degrade.
    let f = Fixture::default();
    let mut s = stmt(StmtKind::Expr(Expr::Ident("foo".to_string())));
    s.labels = vec![Label::Named("retry".to_string())];
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("// C label retry:"));
    assert!(out.contains("foo;\n"));
    assert!(!out.contains("todo!"));
}

#[test]
fn case_label_outside_switch_degrades_defensively() {
    let f = Fixture::default();
    let mut s = stmt(StmtKind::Expr(Expr::Ident("foo".to_string())));
    s.labels = vec![Label::Default];
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("todo!"));
}

#[test]
fn control_flow_degrades_for_now() {
    let f = Fixture::default();
    let s = stmt(StmtKind::Break);
    assert!(render_stmt(&s, &f.ctx()).contains("todo!(\"break statement not yet translated\")"));
}

#[test]
fn block_statement_renders_nested_statements() {
    let f = Fixture::default();
    let inner = stmt(StmtKind::Return(None));
    let s = stmt(StmtKind::Block(block(vec![inner])));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.starts_with("{\n"));
    assert!(out.contains("return;\n"));
    assert!(out.trim_end().ends_with('}'));
}

#[test]
fn preproc_statement_becomes_a_comment() {
    let f = Fixture::default();
    let mut s = stmt(StmtKind::Preproc(Directive::Undef {
        name: "FOO".to_string(),
    }));
    s.raw = "#undef FOO\n".to_string();
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("// #undef FOO"));
    assert!(!out.contains("todo!"));
}

// ---- Decl ----

#[test]
fn automatic_local_becomes_let_mut() {
    let f = Fixture::default();
    let d = DeclStmt {
        storage: vec![],
        base_ty: named("int"),
        declarators: vec![Declarator {
            ty: named("int"),
            name: "x".to_string(),
            initializer: Some(LocalInit::Expr(Expr::IntLit("5".to_string()))),
        }],
    };
    let s = stmt(StmtKind::Decl(d));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("let mut x: std::ffi::c_int = unsafe { 5 };"));
}

#[test]
fn static_local_becomes_static_mut_not_let() {
    // am_map.c's real `cheatstate`/`bigstate`-style locals depend on
    // persistence across calls - a fresh `let` on every call would
    // silently break that.
    let f = Fixture::default();
    let d = DeclStmt {
        storage: vec![Storage::Static],
        base_ty: named("int"),
        declarators: vec![Declarator {
            ty: named("int"),
            name: "cheatstate".to_string(),
            initializer: None,
        }],
    };
    let s = stmt(StmtKind::Decl(d));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("static mut cheatstate: std::ffi::c_int"));
    assert!(!out.contains("let mut"));
}

#[test]
fn local_with_no_initializer_still_gets_zeroed() {
    let f = Fixture::default();
    let d = DeclStmt {
        storage: vec![],
        base_ty: named("int"),
        declarators: vec![Declarator {
            ty: named("int"),
            name: "x".to_string(),
            initializer: None,
        }],
    };
    let s = stmt(StmtKind::Decl(d));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("std::mem::zeroed()"));
}

#[test]
fn braced_local_init_degrades_to_zeroed_stub() {
    // Local braced initializers are rare (5 sites corpus-wide, all
    // `static`) and need the same array/struct-row machinery
    // `codegen::init` provides at module scope - out of scope here.
    let f = Fixture::default();
    let d = DeclStmt {
        storage: vec![Storage::Static],
        base_ty: named("int"),
        declarators: vec![Declarator {
            ty: Type::Array(Box::new(named("int")), Some("3".to_string())),
            name: "table".to_string(),
            initializer: Some(LocalInit::Braced(vec![
                LocalInit::Expr(Expr::IntLit("1".to_string())),
                LocalInit::Expr(Expr::IntLit("2".to_string())),
            ])),
        }],
    };
    let s = stmt(StmtKind::Decl(d));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("std::mem::zeroed()"));
    assert!(out.contains("TODO"));
}

#[test]
fn malformed_local_type_keeps_a_binding_with_a_flagged_comment() {
    let f = Fixture::default();
    let d = DeclStmt {
        storage: vec![],
        base_ty: named("char const"),
        declarators: vec![Declarator {
            ty: named("char const"),
            name: "x".to_string(),
            initializer: None,
        }],
    };
    let s = stmt(StmtKind::Decl(d));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("let mut x: () = ()"));
    assert!(out.contains("TODO"));
}

#[test]
fn multi_declarator_decl_emits_one_binding_per_declarator() {
    let f = Fixture::default();
    let d = DeclStmt {
        storage: vec![],
        base_ty: named("int"),
        declarators: vec![
            Declarator {
                ty: named("int"),
                name: "a".to_string(),
                initializer: None,
            },
            Declarator {
                ty: named("int"),
                name: "b".to_string(),
                initializer: None,
            },
        ],
    };
    let s = stmt(StmtKind::Decl(d));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("let mut a:"));
    assert!(out.contains("let mut b:"));
}

// ---- Conditional (stmt-level #ifdef) ----

fn cond_branch(body: Vec<Stmt>) -> StmtCondBranch {
    StmtCondBranch {
        directive: Directive::IfDef {
            name: "FOO".to_string(),
            negate: false,
        },
        body: block(body),
    }
}

#[test]
fn conditional_active_branch_renders_its_body() {
    let f = Fixture::default();
    let g = StmtCondGroup {
        branches: vec![cond_branch(vec![stmt(StmtKind::Return(None))])],
        else_body: Some(block(vec![stmt(StmtKind::Break)])),
        active: ActiveBranch::Branch(0),
    };
    let s = stmt(StmtKind::Conditional(g));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("return;"));
    assert!(!out.contains("break statement"));
}

#[test]
fn conditional_none_renders_nothing() {
    let f = Fixture::default();
    let g = StmtCondGroup {
        branches: vec![cond_branch(vec![stmt(StmtKind::Return(None))])],
        else_body: None,
        active: ActiveBranch::None,
    };
    let s = stmt(StmtKind::Conditional(g));
    assert_eq!(render_stmt(&s, &f.ctx()), "");
}

#[test]
fn conditional_unknown_degrades_rather_than_silently_dropping() {
    // Unlike the item-level equivalent (an unresolved #define just fails
    // to define something, loudly, at compile time), silently dropping
    // *statements* would make the function quietly do less.
    let f = Fixture::default();
    let g = StmtCondGroup {
        branches: vec![cond_branch(vec![stmt(StmtKind::Return(None))])],
        else_body: None,
        active: ActiveBranch::Unknown,
    };
    let s = stmt(StmtKind::Conditional(g));
    let out = render_stmt(&s, &f.ctx());
    assert!(out.contains("todo!"));
}
