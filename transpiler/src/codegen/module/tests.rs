use super::*;
use crate::parser::ast::{CondBranch, CondGroup, FnSig, RecordKind, Type, VarDecl};

fn named(s: &str) -> Type {
    Type::Named(s.to_string())
}

fn item(kind: ItemKind) -> (Item, Trivia) {
    (
        Item {
            kind,
            raw: String::new(),
        },
        Trivia::default(),
    )
}

fn fn_decl(name: &str) -> ItemKind {
    ItemKind::FunctionDecl(FnSig {
        storage: vec![],
        ret_ty: named("void"),
        name: name.to_string(),
        params: vec![],
        variadic: false,
    })
}

fn fn_def(name: &str) -> ItemKind {
    use crate::parser::stmt::ast::{Block, FnBody};
    ItemKind::FunctionDef(
        FnSig {
            storage: vec![],
            ret_ty: named("void"),
            name: name.to_string(),
            params: vec![],
            variadic: false,
        },
        FnBody {
            block: Block { stmts: vec![] },
            raw: "{}".to_string(),
        },
    )
}

/// `has_init: false` represents a mere `extern` declaration (the dedup
/// logic's real signal is `Storage::Extern`, not the presence of an
/// initializer - see `collect_stronger_names`'s doc comment for why); `true`
/// represents a real (non-`extern`) definition.
fn var(name: &str, has_init: bool) -> ItemKind {
    ItemKind::Var(vec![VarDecl {
        storage: if has_init {
            vec![]
        } else {
            vec![Storage::Extern]
        },
        ty: named("int"),
        name: name.to_string(),
        initializer: has_init.then(|| crate::parser::ast::Init::Expr("0".to_string())),
    }])
}

fn typedef(name: &str) -> ItemKind {
    ItemKind::Typedef(crate::parser::ast::TypedefDecl {
        underlying: named("int"),
        name: name.to_string(),
    })
}

fn record(typedef_name: &str) -> ItemKind {
    ItemKind::Record(RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![],
        names: vec![],
        typedef_name: Some(typedef_name.to_string()),
    })
}

fn include(path: &str) -> ItemKind {
    ItemKind::Preproc(Directive::Include {
        path: path.to_string(),
        angled: false,
    })
}

// ---- group_into_modules ----

#[test]
fn groups_c_and_h_pairs_by_shared_basename() {
    let paths = vec![
        PathBuf::from("linuxdoom-1.10/m_misc.c"),
        PathBuf::from("linuxdoom-1.10/m_misc.h"),
        PathBuf::from("linuxdoom-1.10/doomtype.h"),
        PathBuf::from("linuxdoom-1.10/i_main.c"),
    ];
    let modules = group_into_modules(&paths);
    assert_eq!(modules.len(), 3);
    let m_misc = modules.iter().find(|m| m.name == "m_misc").unwrap();
    assert!(m_misc.header.is_some());
    assert!(m_misc.source.is_some());
    let doomtype = modules.iter().find(|m| m.name == "doomtype").unwrap();
    assert!(doomtype.header.is_some());
    assert!(doomtype.source.is_none());
    let i_main = modules.iter().find(|m| m.name == "i_main").unwrap();
    assert!(i_main.header.is_none());
    assert!(i_main.source.is_some());
}

#[test]
fn module_name_strips_either_extension() {
    assert_eq!(module_name_for_file("m_misc.c"), "m_misc");
    assert_eq!(module_name_for_file("m_misc.h"), "m_misc");
    assert_eq!(module_name_for_file("doomtype.h"), "doomtype");
}

// ---- direct_local_includes ----

#[test]
fn collects_direct_includes_and_ignores_angled() {
    let items = vec![
        item(include("d_think.h")),
        item(ItemKind::Preproc(Directive::Include {
            path: "stdio.h".to_string(),
            angled: true,
        })),
        item(var("x", false)),
    ];
    assert_eq!(direct_local_includes(&items), vec!["d_think.h".to_string()]);
}

#[test]
fn collects_includes_only_from_active_conditional_branch() {
    let group = CondGroup {
        branches: vec![CondBranch {
            directive: Directive::IfDef {
                name: "FOO".to_string(),
                negate: false,
            },
            body: vec![item(include("active.h"))],
        }],
        else_body: Some(vec![item(include("dead.h"))]),
        active: ActiveBranch::Branch(0),
    };
    let items = vec![item(ItemKind::Conditional(group))];
    assert_eq!(direct_local_includes(&items), vec!["active.h".to_string()]);
}

// ---- transitively_visible_files ----

#[test]
fn transitive_closure_follows_chained_includes() {
    let mut graph = HashMap::new();
    graph.insert("a.h".to_string(), vec!["b.h".to_string()]);
    graph.insert("b.h".to_string(), vec!["c.h".to_string()]);
    graph.insert("c.h".to_string(), vec![]);
    let visible = transitively_visible_files(&graph, "a.h");
    assert!(visible.contains("b.h"));
    assert!(visible.contains("c.h"));
    assert_eq!(visible.len(), 2);
}

#[test]
fn transitive_closure_handles_a_cycle_without_looping_forever() {
    // Mirrors the real corpus's r_data.h<->r_state.h mutual #include.
    let mut graph = HashMap::new();
    graph.insert("r_data.h".to_string(), vec!["r_state.h".to_string()]);
    graph.insert("r_state.h".to_string(), vec!["r_data.h".to_string()]);
    let visible = transitively_visible_files(&graph, "r_data.h");
    assert!(visible.contains("r_state.h"));
}

// ---- use_statements_for_module ----

#[test]
fn use_statements_cover_transitive_includes_not_just_direct() {
    // X.h includes Y.h, Y.h includes Z.h - X's module must glob-`use` Z
    // directly too, since `use crate::y::*;` doesn't re-export Y's own
    // private `use crate::z::*;`.
    let mut graph = HashMap::new();
    graph.insert("x.h".to_string(), vec!["y.h".to_string()]);
    graph.insert("y.h".to_string(), vec!["z.h".to_string()]);
    graph.insert("z.h".to_string(), vec![]);
    let uses = use_statements_for_module(&graph, "x", &["x.h".to_string()]);
    assert_eq!(uses, vec!["use crate::y::*;\n", "use crate::z::*;\n"]);
}

#[test]
fn use_statements_never_include_own_module() {
    let mut graph = HashMap::new();
    graph.insert("m_misc.h".to_string(), vec![]);
    graph.insert("m_misc.c".to_string(), vec!["m_misc.h".to_string()]);
    let uses = use_statements_for_module(
        &graph,
        "m_misc",
        &["m_misc.h".to_string(), "m_misc.c".to_string()],
    );
    assert!(uses.is_empty());
}

#[test]
fn use_statements_skip_a_quoted_include_of_a_non_corpus_file() {
    // Real corpus case: m_fixed.c's `#include "stdlib.h"` - quoted, but a
    // real system header, not one of the 124 corpus files, so it never got
    // its own entry in `graph` (only real corpus files do - see
    // `use_statements_for_module`'s doc comment). Must not produce a
    // `use crate::stdlib::*;` referencing a module that was never
    // generated.
    let mut graph = HashMap::new();
    graph.insert(
        "m_fixed.c".to_string(),
        vec!["stdlib.h".to_string(), "doomtype.h".to_string()],
    );
    graph.insert("doomtype.h".to_string(), vec![]);
    // Deliberately no entry for "stdlib.h" - it's not a corpus file.
    let uses = use_statements_for_module(&graph, "m_fixed", &["m_fixed.c".to_string()]);
    assert_eq!(uses, vec!["use crate::doomtype::*;\n"]);
}

// ---- merge_items dedup ----

#[test]
fn function_decl_is_dropped_when_a_same_name_def_exists() {
    let header = vec![item(fn_decl("P_Init"))];
    let source = vec![item(fn_def("P_Init"))];
    let merged = merge_items(Some(&header), Some(&source));
    assert_eq!(merged.len(), 1);
    assert!(matches!(merged[0].0.kind, ItemKind::FunctionDef(..)));
}

#[test]
fn function_decl_survives_when_no_def_exists() {
    let header = vec![item(fn_decl("I_Error"))];
    let merged = merge_items(Some(&header), None);
    assert_eq!(merged.len(), 1);
    assert!(matches!(merged[0].0.kind, ItemKind::FunctionDecl(_)));
}

#[test]
fn extern_var_is_dropped_when_a_real_definition_exists() {
    let header = vec![item(var("key_right", false))];
    let source = vec![item(var("key_right", true))];
    let merged = merge_items(Some(&header), Some(&source));
    assert_eq!(merged.len(), 1);
    assert!(matches!(&merged[0].0.kind, ItemKind::Var(vds) if vds[0].initializer.is_some()));
}

#[test]
fn extern_var_is_dropped_by_a_tentative_definition_with_no_explicit_initializer() {
    // Real corpus bug: doomstat.h's `extern boolean modifiedgame;` +
    // doomstat.c's `boolean modifiedgame;` (a tentative definition - real
    // storage, implicitly zero-initialized, but with NO explicit `=` in the
    // source either) both have `initializer: None`. An earlier version of
    // this dedup keyed on `initializer.is_some()`, so it treated *both* as
    // mere declarations and dropped neither - a real `cargo build -p
    // doom_rs` failure (duplicate `static mut modifiedgame`). The real
    // signal is `Storage::Extern`, not the initializer.
    let header = vec![item(ItemKind::Var(vec![VarDecl {
        storage: vec![Storage::Extern],
        ty: named("boolean"),
        name: "modifiedgame".to_string(),
        initializer: None,
    }]))];
    let source = vec![item(ItemKind::Var(vec![VarDecl {
        storage: vec![],
        ty: named("boolean"),
        name: "modifiedgame".to_string(),
        initializer: None, // tentative definition - no explicit `=` in the source either
    }]))];
    let merged = merge_items(Some(&header), Some(&source));
    assert_eq!(merged.len(), 1);
    assert!(
        matches!(&merged[0].0.kind, ItemKind::Var(vds) if !vds[0].storage.contains(&Storage::Extern))
    );
}

#[test]
fn two_extern_declarations_of_the_same_name_with_no_definition_keep_only_one() {
    // e.g. two different headers both declaring `extern int foo;` for a
    // variable actually defined in some other .c file outside this merged
    // pair - must not produce a literal duplicate extern block entry.
    let header = vec![item(var("g_only_declared", false))];
    let source = vec![item(var("g_only_declared", false))];
    let merged = merge_items(Some(&header), Some(&source));
    assert_eq!(merged.len(), 1);
}

#[test]
fn duplicate_typedef_keeps_only_the_first_occurrence() {
    let header = vec![item(typedef("foo_t"))];
    let source = vec![item(typedef("foo_t"))];
    let merged = merge_items(Some(&header), Some(&source));
    assert_eq!(merged.len(), 1);
}

#[test]
fn duplicate_record_keeps_only_the_first_occurrence() {
    let header = vec![item(record("mobj_t"))];
    let source = vec![item(record("mobj_t"))];
    let merged = merge_items(Some(&header), Some(&source));
    assert_eq!(merged.len(), 1);
}

#[test]
fn unrelated_items_are_all_kept() {
    let header = vec![item(typedef("a_t")), item(fn_decl("Foo"))];
    let source = vec![item(typedef("b_t")), item(var("g_count", true))];
    let merged = merge_items(Some(&header), Some(&source));
    assert_eq!(merged.len(), 4);
}

#[test]
fn header_only_module_merges_fine_with_no_source() {
    let header = vec![item(typedef("boolean"))];
    let merged = merge_items(Some(&header), None);
    assert_eq!(merged.len(), 1);
}

#[test]
fn dedup_recurses_into_the_active_conditional_branch() {
    // A FunctionDecl inside an active #ifdef guard (the realistic shape -
    // real headers wrap almost everything in their own include guard) must
    // still be dropped when the .c file's matching FunctionDef exists.
    let group = CondGroup {
        branches: vec![CondBranch {
            directive: Directive::IfDef {
                name: "GUARD".to_string(),
                negate: true,
            },
            body: vec![item(fn_decl("P_Init"))],
        }],
        else_body: None,
        active: ActiveBranch::Branch(0),
    };
    let header = vec![item(ItemKind::Conditional(group))];
    let source = vec![item(fn_def("P_Init"))];
    let merged = merge_items(Some(&header), Some(&source));
    // The Conditional wrapper survives (never itself removed), but its
    // active branch's now-superseded FunctionDecl is gone.
    let ItemKind::Conditional(g) = &merged[0].0.kind else {
        panic!("expected Conditional to survive");
    };
    assert!(g.branches[0].body.is_empty());
}

#[test]
fn a_definition_in_a_dead_branch_does_not_wrongly_drop_a_real_declaration() {
    // If a FunctionDef only exists in a branch that resolved *inactive*, it
    // will never actually be emitted - so a same-name FunctionDecl
    // elsewhere must NOT be treated as superseded (that would drop the only
    // reference that will ever actually appear in the output).
    let dead_group = CondGroup {
        branches: vec![CondBranch {
            directive: Directive::IfDef {
                name: "NEVER_DEFINED".to_string(),
                negate: false,
            },
            body: vec![item(fn_def("Helper"))],
        }],
        else_body: None,
        active: ActiveBranch::None,
    };
    let header = vec![
        item(fn_decl("Helper")),
        item(ItemKind::Conditional(dead_group)),
    ];
    let merged = merge_items(Some(&header), None);
    assert!(
        merged
            .iter()
            .any(|(i, _)| matches!(&i.kind, ItemKind::FunctionDecl(s) if s.name == "Helper"))
    );
}
