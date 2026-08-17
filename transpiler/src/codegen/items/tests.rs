use super::*;
use crate::parser::ast::{CondBranch, Trivia, Type};
use crate::parser::preproc::Directive;

fn named(s: &str) -> Type {
    Type::Named(s.to_string())
}

fn item(kind: ItemKind) -> Item {
    Item {
        kind,
        raw: String::new(),
    }
}

fn field(name: &str, ty: Type) -> Field {
    Field {
        ty,
        name: name.to_string(),
        storage: vec![],
        bitfield: None,
        nested: None,
        trivia: Trivia::default(),
        trailing_comment: None,
    }
}

// ---- Typedef ----

#[test]
fn typedef_simple() {
    let td = TypedefDecl {
        underlying: named("int"),
        name: "myint_t".to_string(),
    };
    assert_eq!(emit_typedef(&td), "pub type myint_t = std::ffi::c_int;\n\n");
}

#[test]
fn typedef_function_pointer() {
    // d_think.h's actionf_v: typedef void (*actionf_v)();
    let td = TypedefDecl {
        underlying: Type::FunctionPointer {
            ret: Box::new(named("void")),
            params: vec![],
        },
        name: "actionf_v".to_string(),
    };
    assert_eq!(
        emit_typedef(&td),
        "pub type actionf_v = Option<unsafe extern \"C\" fn()>;\n\n"
    );
}

// ---- Record ----

#[test]
fn record_simple_struct_with_typedef_only() {
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![field("x", named("int")), field("y", named("int"))],
        names: vec![],
        typedef_name: Some("point_t".to_string()),
    };
    let out = emit_record(&rd);
    assert!(out.contains("#[repr(C)]"));
    assert!(out.contains("#[derive(Copy, Clone)]"));
    assert!(out.contains("pub struct point_t {"));
    assert!(out.contains("pub x: std::ffi::c_int,"));
    assert!(out.contains("pub y: std::ffi::c_int,"));
}

#[test]
fn record_with_tag_and_typedef_gets_an_alias_for_the_tag() {
    // p_mobj.h-style: typedef struct mobj_s { struct mobj_s* snext; ... } mobj_t;
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: Some("mobj_s".to_string()),
        fields: vec![field(
            "snext",
            Type::Pointer(Box::new(named("struct mobj_s"))),
        )],
        names: vec![],
        typedef_name: Some("mobj_t".to_string()),
    };
    let out = emit_record(&rd);
    assert!(out.contains("pub struct mobj_t {"));
    assert!(out.contains("pub snext: *mut mobj_s,"));
    assert!(out.contains("pub type mobj_s = mobj_t;"));
}

#[test]
fn record_with_only_a_tag_no_typedef() {
    let rd = RecordDecl {
        kind: RecordKind::Union,
        tag: Some("line_s".to_string()),
        fields: vec![field("a", named("int"))],
        names: vec![],
        typedef_name: None,
    };
    let out = emit_record(&rd);
    assert!(out.contains("pub union line_s {"));
    // No typedef name, so there's nothing to alias against.
    assert!(!out.contains("pub type line_s = "));
}

#[test]
fn record_bitfield_gets_a_todo_comment() {
    let mut f = field("flags", named("int"));
    f.bitfield = Some("4".to_string());
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![f],
        names: vec![],
        typedef_name: Some("flags_t".to_string()),
    };
    let out = emit_record(&rd);
    assert!(out.contains("pub flags: std::ffi::c_int,"));
    assert!(out.contains("TODO: bitfield width 4"));
}

#[test]
fn record_anonymous_nested_union_field_gets_synthesized_type() {
    // p_local.h's intercept_t: { fixed_t frac; boolean isaline; union {
    // mobj_t* thing; line_t* line; } d; }
    let nested = RecordDecl {
        kind: RecordKind::Union,
        tag: None,
        fields: vec![
            field("thing", Type::Pointer(Box::new(named("mobj_t")))),
            field("line", Type::Pointer(Box::new(named("line_t")))),
        ],
        names: vec![],
        typedef_name: None,
    };
    let mut d_field = field("d", named("union"));
    d_field.nested = Some(Box::new(nested));
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![field("frac", named("fixed_t")), d_field],
        names: vec![],
        typedef_name: Some("intercept_t".to_string()),
    };
    let out = emit_record(&rd);
    // The nested type is emitted before the parent (it references it).
    let nested_pos = out
        .find("pub union intercept_t_d {")
        .expect("nested union emitted");
    let parent_pos = out
        .find("pub struct intercept_t {")
        .expect("parent struct emitted");
    assert!(
        nested_pos < parent_pos,
        "nested type must precede its parent"
    );
    assert!(out.contains("pub thing: *mut mobj_t,"));
    assert!(out.contains("pub line: *mut line_t,"));
    assert!(out.contains("pub d: intercept_t_d,"));
}

#[test]
fn record_extra_declarator_names_become_type_aliases() {
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![field("x", named("int"))],
        names: vec!["foo".to_string(), "bar".to_string()],
        typedef_name: Some("primary_t".to_string()),
    };
    let out = emit_record(&rd);
    assert!(out.contains("pub type foo = primary_t;"));
    assert!(out.contains("pub type bar = primary_t;"));
}

#[test]
fn record_names_includes_the_typedef_name_itself_and_must_not_self_alias() {
    // record::classify_record_or_enum sets `typedef_name = names.first()`
    // without removing it from `names` - e.g. `typedef union { ... }
    // actionf_t;` (d_think.h) parses to `names: ["actionf_t"], typedef_name:
    // Some("actionf_t")`. A naive "alias every name in `names`" loop
    // produces a bogus `pub type actionf_t = actionf_t;` self-reference -
    // regression test for that exact bug.
    let rd = RecordDecl {
        kind: RecordKind::Union,
        tag: None,
        fields: vec![field("x", named("int"))],
        names: vec!["actionf_t".to_string()],
        typedef_name: Some("actionf_t".to_string()),
    };
    let out = emit_record(&rd);
    assert!(!out.contains("pub type actionf_t = actionf_t;"));
    // Same overlap, but with a distinct tag also present (p_mobj.h-style):
    // `names` still shouldn't produce a redundant second alias for the
    // typedef name once the tag-alias line has already covered it.
    let rd2 = RecordDecl {
        kind: RecordKind::Struct,
        tag: Some("thinker_s".to_string()),
        fields: vec![field("x", named("int"))],
        names: vec!["thinker_t".to_string()],
        typedef_name: Some("thinker_t".to_string()),
    };
    let out2 = emit_record(&rd2);
    assert!(!out2.contains("pub type thinker_t = thinker_t;"));
    assert!(out2.contains("pub type thinker_s = thinker_t;"));
}

#[test]
fn record_anonymous_with_no_name_at_all_is_flagged_not_dropped() {
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![field("x", named("int"))],
        names: vec![],
        typedef_name: None,
    };
    let out = emit_record(&rd);
    assert!(out.contains("TODO"));
    assert!(out.contains("anonymous"));
}

// ---- Enum ----

#[test]
fn enum_explicit_and_implicit_values() {
    fn variant(name: &str, value: Option<&str>) -> crate::parser::ast::EnumVariant {
        crate::parser::ast::EnumVariant {
            name: name.to_string(),
            value: value.map(str::to_string),
            trivia: Trivia::default(),
            trailing_comment: None,
        }
    }
    let ed = EnumDecl {
        tag: None,
        variants: vec![
            variant("FOO", None),
            variant("BAR", None),
            variant("BAZ", Some("5")),
            variant("QUX", None),
        ],
        names: vec![],
        typedef_name: Some("myenum_t".to_string()),
    };
    let out = emit_enum(&ed);
    assert!(out.contains("pub const FOO: std::ffi::c_int = 0;"));
    assert!(out.contains("pub const BAR: std::ffi::c_int = FOO + 1;"));
    assert!(out.contains("pub const BAZ: std::ffi::c_int = 5;"));
    assert!(out.contains("pub const QUX: std::ffi::c_int = BAZ + 1;"));
    assert!(out.contains("pub type myenum_t = std::ffi::c_int;"));
}

#[test]
fn enum_names_includes_the_typedef_name_itself_and_must_not_duplicate_the_alias() {
    // Real corpus shape: doomtype.h's `typedef enum {false, true} boolean;`
    // parses to `names: ["boolean"], typedef_name: Some("boolean")` (same
    // names/typedef_name overlap as the record case above) - a naive loop
    // over `names` produced a duplicate `pub type boolean = std::ffi::c_int;`
    // line. Regression test for that exact bug.
    fn variant(name: &str) -> crate::parser::ast::EnumVariant {
        crate::parser::ast::EnumVariant {
            name: name.to_string(),
            value: None,
            trivia: Trivia::default(),
            trailing_comment: None,
        }
    }
    let ed = EnumDecl {
        tag: None,
        variants: vec![variant("false"), variant("true")],
        names: vec!["boolean".to_string()],
        typedef_name: Some("boolean".to_string()),
    };
    let out = emit_enum(&ed);
    let alias_count = out.matches("pub type boolean = std::ffi::c_int;").count();
    assert_eq!(
        alias_count, 1,
        "expected exactly one alias line, got:\n{out}"
    );
}

#[test]
fn enum_value_with_c_integer_suffix_is_sanitized() {
    let ed = EnumDecl {
        tag: None,
        variants: vec![crate::parser::ast::EnumVariant {
            name: "MASK".to_string(),
            value: Some("0xffffffffu".to_string()),
            trivia: Trivia::default(),
            trailing_comment: None,
        }],
        names: vec![],
        typedef_name: None,
    };
    let out = emit_enum(&ed);
    assert!(out.contains("pub const MASK: std::ffi::c_int = 0xffffffff;"));
}

// ---- Var ----

#[test]
fn var_without_initializer_becomes_extern_block() {
    let vd = VarDecl {
        storage: vec![Storage::Extern],
        ty: named("int"),
        name: "key_right".to_string(),
        initializer: None,
    };
    let out = emit_var(&vd);
    assert!(out.contains("unsafe extern \"C\" {"));
    assert!(out.contains("pub static mut key_right: std::ffi::c_int;"));
}

#[test]
fn var_with_initializer_becomes_zeroed_stub() {
    let vd = VarDecl {
        storage: vec![],
        ty: named("int"),
        name: "usegamma".to_string(),
        initializer: Some(crate::parser::ast::Init::Expr("0".to_string())),
    };
    let out = emit_var(&vd);
    assert!(
        out.contains("pub static mut usegamma: std::ffi::c_int = unsafe { std::mem::zeroed() };")
    );
    assert!(out.contains("TODO"));
}

#[test]
fn static_var_drops_pub() {
    let vd = VarDecl {
        storage: vec![Storage::Static],
        ty: named("int"),
        name: "internal_counter".to_string(),
        initializer: Some(crate::parser::ast::Init::Expr("0".to_string())),
    };
    let out = emit_var(&vd);
    assert!(!out.contains("pub static mut"));
    assert!(out.contains("static mut internal_counter"));
}

// ---- Functions ----

#[test]
fn function_decl_basic() {
    let sig = FnSig {
        storage: vec![],
        ret_ty: named("void"),
        name: "P_Init".to_string(),
        params: vec![],
        variadic: false,
    };
    let out = emit_function_decl(&sig);
    assert!(out.contains("unsafe extern \"C\" {"));
    assert!(out.contains("pub fn P_Init();"));
}

#[test]
fn function_def_stub_body() {
    let sig = FnSig {
        storage: vec![],
        ret_ty: named("int"),
        name: "P_Random".to_string(),
        params: vec![],
        variadic: false,
    };
    let out = emit_function_def(&sig);
    assert!(out.contains("pub unsafe extern \"C\" fn P_Random() -> std::ffi::c_int"));
    assert!(out.contains("todo!(\"body not yet translated\")"));
}

#[test]
fn function_def_anonymous_param_becomes_underscore() {
    let sig = FnSig {
        storage: vec![],
        ret_ty: named("void"),
        name: "SwapSHORT".to_string(),
        params: vec![crate::parser::ast::Param {
            ty: named("short"),
            name: String::new(),
            storage: vec![],
        }],
        variadic: false,
    };
    let out = emit_function_def(&sig);
    assert!(out.contains("_: std::ffi::c_short"));
}

#[test]
fn function_def_variadic_drops_ellipsis_with_comment() {
    // The corpus's one real variadic definition: I_Error.
    let sig = FnSig {
        storage: vec![],
        ret_ty: named("void"),
        name: "I_Error".to_string(),
        params: vec![crate::parser::ast::Param {
            ty: Type::Pointer(Box::new(named("char"))),
            name: "error".to_string(),
            storage: vec![],
        }],
        variadic: true,
    };
    let out = emit_function_def(&sig);
    assert!(!out.contains("..."));
    assert!(out.contains("TODO: variadic definition not supported"));
}

#[test]
fn function_decl_variadic_keeps_ellipsis() {
    // extern declarations DO support C-variadic syntax in Rust.
    let sig = FnSig {
        storage: vec![],
        ret_ty: named("void"),
        name: "I_Error".to_string(),
        params: vec![crate::parser::ast::Param {
            ty: Type::Pointer(Box::new(named("char"))),
            name: "error".to_string(),
            storage: vec![],
        }],
        variadic: true,
    };
    let out = emit_function_decl(&sig);
    assert!(out.contains(", ...);"));
}

#[test]
fn static_function_drops_pub() {
    let sig = FnSig {
        storage: vec![Storage::Static],
        ret_ty: named("void"),
        name: "helper".to_string(),
        params: vec![],
        variadic: false,
    };
    let out = emit_function_def(&sig);
    assert!(!out.contains("pub unsafe extern"));
    assert!(out.contains("unsafe extern \"C\" fn helper"));
}

// ---- Conditional ----

fn cond_branch(directive: Directive, body: Vec<(Item, Trivia)>) -> CondBranch {
    CondBranch { directive, body }
}

#[test]
fn conditional_emits_only_the_active_branch() {
    let inner_var = |name: &str| {
        item(ItemKind::Var(VarDecl {
            storage: vec![],
            ty: named("int"),
            name: name.to_string(),
            initializer: None,
        }))
    };
    let group = CondGroup {
        branches: vec![cond_branch(
            Directive::IfDef {
                name: "FOO".to_string(),
                negate: false,
            },
            vec![(inner_var("a"), Trivia::default())],
        )],
        else_body: Some(vec![(inner_var("b"), Trivia::default())]),
        active: ActiveBranch::Branch(0),
    };
    let out = emit_conditional(&group);
    assert!(out.contains(" a:"));
    assert!(!out.contains(" b:"));
}

#[test]
fn conditional_none_emits_nothing() {
    let group = CondGroup {
        branches: vec![cond_branch(
            Directive::IfDef {
                name: "FOO".to_string(),
                negate: false,
            },
            vec![(
                item(ItemKind::Var(VarDecl {
                    storage: vec![],
                    ty: named("int"),
                    name: "a".to_string(),
                    initializer: None,
                })),
                Trivia::default(),
            )],
        )],
        else_body: None,
        active: ActiveBranch::None,
    };
    assert_eq!(emit_conditional(&group), "");
}

#[test]
fn conditional_unknown_is_flagged_not_silently_dropped() {
    let group = CondGroup {
        branches: vec![cond_branch(
            Directive::If {
                expr: "VERSION >= 2".to_string(),
            },
            vec![],
        )],
        else_body: None,
        active: ActiveBranch::Unknown,
    };
    let out = emit_conditional(&group);
    assert!(out.contains("TODO"));
    assert!(out.contains("unresolved"));
}

// ---- Raw / Preproc ----

#[test]
fn raw_item_is_a_flagged_comment_not_dropped() {
    let out = emit_raw("weird construct #here");
    assert!(out.contains("TODO"));
    assert!(out.contains("weird construct #here"));
}

#[test]
fn empty_raw_item_emits_nothing() {
    // record::build_items' own "leftover trailing whitespace becomes its
    // own final catch-all item" pattern produces a Raw item with an empty
    // `raw` string - a synthetic placeholder, not real unparsed C content,
    // so it shouldn't produce a noisy empty TODO comment block.
    assert_eq!(emit_raw(""), "");
    assert_eq!(emit_raw("   \n  "), "");
}

#[test]
fn bare_preproc_item_emits_nothing() {
    let out = emit_item(&item(ItemKind::Preproc(Directive::Include {
        path: "foo.h".to_string(),
        angled: false,
    })));
    assert_eq!(out, "");
    let out = emit_item(&item(ItemKind::Preproc(Directive::DefineObject {
        name: "FOO".to_string(),
        value: "1".to_string(),
    })));
    assert_eq!(out, "");
}
