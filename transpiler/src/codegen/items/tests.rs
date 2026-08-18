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
fn typedef_malformed_multi_declarator_underlying_is_flagged_not_emitted_broken() {
    let td = TypedefDecl {
        underlying: named("int a,"),
        name: "b".to_string(),
    };
    let out = emit_typedef(&td);
    assert!(!out.contains("pub type"));
    assert!(out.contains("TODO"));
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

#[test]
fn typedef_function_pointer_with_multiple_params_is_not_mistaken_for_malformed() {
    // Regression test: d_think.h's real actionf_p2 typedef (two void*
    // params) was a real false-positive bug - an earlier version of the
    // malformed-type check ran against the *final mapped text*
    // (`Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>`), which
    // legitimately contains a comma from the parameter list, and wrongly
    // flagged this perfectly valid typedef as an unparsed multi-declarator
    // artifact. The check must run against the raw `Type` tree's `Named`
    // leaves instead (see `types::type_is_malformed`).
    let td = TypedefDecl {
        underlying: Type::FunctionPointer {
            ret: Box::new(named("void")),
            params: vec![
                Type::Pointer(Box::new(named("void"))),
                Type::Pointer(Box::new(named("void"))),
            ],
        },
        name: "actionf_p2".to_string(),
    };
    let out = emit_typedef(&td);
    assert!(out.starts_with("pub type actionf_p2 ="), "got: {out}");
    assert!(!out.contains("TODO"));
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
fn record_multi_declarator_field_bug_is_commented_out_not_emitted_broken() {
    // Real corpus bug (info.h's `long misc1, misc2;`, 5 occurrences total):
    // record::parse_fields doesn't split a multi-declarator field into
    // separate Fields, so `field.ty` ends up as Named("long misc1,") with a
    // literal embedded comma. Emitting that verbatim would put a stray
    // comma inside the field's type position, breaking the whole struct's
    // syntax (every field after it) - must degrade to a flagged comment
    // covering just this one field instead.
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![
            field("frame", named("long")),
            field("misc2", named("long misc1,")),
            field("nextstate", named("statenum_t")),
        ],
        names: vec![],
        typedef_name: Some("state_t".to_string()),
    };
    let out = emit_record(&rd);
    assert!(
        !out.contains("pub misc2:"),
        "must not emit a broken field declaration"
    );
    assert!(out.contains("TODO"));
    assert!(out.contains("unparsed multi-declarator field"));
    // Neighboring, correctly-parsed fields must still emit normally.
    assert!(out.contains("pub frame: std::ffi::c_long,"));
    assert!(out.contains("pub nextstate: statenum_t,"));
}

#[test]
fn record_multi_declarator_bug_landing_on_the_name_half_is_also_caught() {
    // Real corpus shape (am_map.c's mpoint_t: `int x, y;`): the stray comma
    // from an unsplit multi-declarator field can land on the *name* instead
    // of the type, depending on exactly where the parser's token stream got
    // cut - both variants must be caught, not just the type-side one above.
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![field("x,y", named("fixed_t"))],
        names: vec![],
        typedef_name: Some("mpoint_t".to_string()),
    };
    let out = emit_record(&rd);
    assert!(
        !out.contains("pub x"),
        "must not emit a broken field declaration"
    );
    assert!(out.contains("TODO"));
    assert!(out.contains("unparsed multi-declarator field"));
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
fn enum_garbage_name_from_trailing_comment_is_silently_skipped() {
    // Real corpus case (m_bbox.h's anonymous bbox enum): a bare `};	// bbox
    // coordinates` with no real extra declarator leaks the trailing
    // `;`/comment text into `names` as one garbage, non-identifier "name" -
    // must not be emitted as a type alias (broken syntax) nor as a TODO
    // comment (it's noise, not real content - same precedent as
    // `emit_raw`'s whitespace-only case).
    let ed = EnumDecl {
        tag: None,
        variants: vec![crate::parser::ast::EnumVariant {
            name: "BOXTOP".to_string(),
            value: None,
            trivia: Trivia::default(),
            trailing_comment: None,
        }],
        names: vec![";\t// bbox coordinates".to_string()],
        typedef_name: None,
    };
    let out = emit_enum(&ed);
    assert!(!out.contains("pub type ;"));
    assert!(!out.contains("bbox coordinates"));
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
fn var_malformed_multi_declarator_type_is_flagged_not_emitted_broken() {
    // Real corpus bug (am_map.c's `fixed_t m_x2, m_y2;`, a top-level
    // multi-declarator variable - same parser gap as the struct-field case,
    // just at file scope): decl::try_parse_var_flat doesn't split it into
    // separate VarDecls, so `ty` ends up with a literal embedded comma.
    let vd = VarDecl {
        storage: vec![],
        ty: named("fixed_t m_x2,"),
        name: "m_y2".to_string(),
        initializer: None,
    };
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert!(!out.contains("static mut m_y2: fixed_t m_x2,"));
    assert!(out.contains("TODO"));
}

#[test]
fn var_without_initializer_becomes_extern_block() {
    let vd = VarDecl {
        storage: vec![Storage::Extern],
        ty: named("int"),
        name: "key_right".to_string(),
        initializer: None,
    };
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert!(out.contains("unsafe extern \"C\" {"));
    assert!(out.contains("pub static mut key_right: std::ffi::c_int;"));
}

#[test]
fn var_without_initializer_still_becomes_zeroed_stub() {
    let vd = VarDecl {
        storage: vec![],
        ty: named("int"),
        name: "modifiedgame".to_string(),
        initializer: None,
    };
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert!(
        out.contains(
            "pub static mut modifiedgame: std::ffi::c_int = unsafe { std::mem::zeroed() };"
        )
    );
    assert!(out.contains("TODO"));
}

#[test]
fn var_with_scalar_initializer_is_translated() {
    let vd = VarDecl {
        storage: vec![],
        ty: named("int"),
        name: "usegamma".to_string(),
        initializer: Some(crate::parser::ast::Init::Expr("0".to_string())),
    };
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert_eq!(
        out,
        "pub static mut usegamma: std::ffi::c_int = unsafe { 0 };\n\n"
    );
}

#[test]
fn var_char_array_from_string_literal_is_translated() {
    // char rcsid[] = "..." - the corpus's real rcsid idiom (62 occurrences),
    // now translated to a real fixed-size char array with the length
    // inferred from the string's own byte count + a null terminator.
    let vd = VarDecl {
        storage: vec![],
        ty: Type::Array(Box::new(named("char")), None),
        name: "rcsid".to_string(),
        initializer: Some(crate::parser::ast::Init::Expr("\"hi\"".to_string())),
    };
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert!(!out.contains("std::mem::zeroed()"), "got: {out}");
    assert!(!out.contains("TODO"), "got: {out}");
    assert!(
        out.contains("pub static mut rcsid: [std::ffi::c_char; 3]"),
        "got: {out}"
    );
    assert!(
        out.contains("[104 as std::ffi::c_char, 105 as std::ffi::c_char, 0]"),
        "got: {out}"
    );
}

#[test]
fn var_flat_scalar_array_is_translated() {
    // A flat scalar table, e.g. `int rndtable[] = {0, 8, 109};`.
    let vd = VarDecl {
        storage: vec![],
        ty: Type::Array(Box::new(named("int")), None),
        name: "rndtable".to_string(),
        initializer: Some(crate::parser::ast::Init::Braced(vec![
            crate::parser::ast::Init::Expr("0".to_string()),
            crate::parser::ast::Init::Expr("8".to_string()),
            crate::parser::ast::Init::Expr("109".to_string()),
        ])),
    };
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert!(!out.contains("std::mem::zeroed()"), "got: {out}");
    assert!(
        out.contains("pub static mut rndtable: [std::ffi::c_int; 3] = unsafe { [0, 8, 109] };"),
        "got: {out}"
    );
}

#[test]
fn var_struct_typed_array_stays_a_stub_this_phase() {
    // states[]/mobjinfo[]-style: each row is itself a multi-item Braced
    // group against a non-Array element type - needs a later phase's
    // record-field-lookup infrastructure, still deferred.
    let vd = VarDecl {
        storage: vec![],
        ty: Type::Array(Box::new(named("state_t")), None),
        name: "states".to_string(),
        initializer: Some(crate::parser::ast::Init::Braced(vec![
            crate::parser::ast::Init::Braced(vec![
                crate::parser::ast::Init::Expr("SPR_TROO".to_string()),
                crate::parser::ast::Init::Expr("0".to_string()),
            ]),
        ])),
    };
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert!(out.contains("std::mem::zeroed()"), "got: {out}");
    assert!(out.contains("TODO"));
}

#[test]
fn var_null_pointer_literal_becomes_null_mut() {
    // i_video.c's real X_display: `Display *X_display = 0;`
    let vd = VarDecl {
        storage: vec![],
        ty: Type::Pointer(Box::new(named("Display"))),
        name: "X_display".to_string(),
        initializer: Some(crate::parser::ast::Init::Expr("0".to_string())),
    };
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert!(
        out.contains("= unsafe { std::ptr::null_mut() };"),
        "got: {out}"
    );
}

#[test]
fn static_var_drops_pub() {
    let vd = VarDecl {
        storage: vec![Storage::Static],
        ty: named("int"),
        name: "internal_counter".to_string(),
        initializer: Some(crate::parser::ast::Init::Expr("0".to_string())),
    };
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert!(!out.contains("pub static mut"));
    assert!(out.contains("static mut internal_counter"));
}

#[test]
fn bare_struct_var_is_translated_to_a_named_field_literal() {
    // m_menu.c-style: `menu_t MainDef = { numitems, ..., routine };`
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![field("x", named("int")), field("y", named("int"))],
        names: vec![],
        typedef_name: Some("point_t".to_string()),
    };
    let mut records = HashMap::new();
    records.insert("point_t".to_string(), rd);
    let vd = VarDecl {
        storage: vec![],
        ty: named("point_t"),
        name: "origin".to_string(),
        initializer: Some(crate::parser::ast::Init::Braced(vec![
            crate::parser::ast::Init::Expr("1".to_string()),
            crate::parser::ast::Init::Expr("2".to_string()),
        ])),
    };
    let mut needed = BTreeSet::new();
    let out = emit_var(
        &vd,
        &KnownTypeNames::new(),
        &records,
        &HashMap::new(),
        &HashMap::new(),
        &mut needed,
    );
    assert!(!out.contains("std::mem::zeroed()"), "got: {out}");
    assert!(
        out.contains("pub static mut origin: point_t = unsafe { point_t { x: 1, y: 2 } };"),
        "got: {out}"
    );
    assert!(needed.is_empty());
}

#[test]
fn emit_items_appends_one_deduped_zeroed_const_per_module() {
    // Two vars, both partial rows against the *same* struct type - the
    // const must be emitted exactly once for the whole module, not once
    // per var, and it must appear regardless of declaration order (Rust
    // doesn't require const items to precede their use site).
    let rd = RecordDecl {
        kind: RecordKind::Struct,
        tag: None,
        fields: vec![field("x", named("int")), field("y", named("int"))],
        names: vec![],
        typedef_name: Some("point_t".to_string()),
    };
    let mut records = HashMap::new();
    records.insert("point_t".to_string(), rd);

    let make_var = |name: &str| {
        item(ItemKind::Var(vec![VarDecl {
            storage: vec![],
            ty: named("point_t"),
            name: name.to_string(),
            initializer: Some(crate::parser::ast::Init::Braced(vec![
                crate::parser::ast::Init::Expr("1".to_string()),
            ])),
        }]))
    };
    let items: Vec<(Item, Trivia)> = vec![
        (make_var("a"), Trivia::default()),
        (make_var("b"), Trivia::default()),
    ];
    let out = emit_items(
        &items,
        &KnownTypeNames::new(),
        &records,
        &HashMap::new(),
        &HashMap::new(),
    );
    assert_eq!(
        out.matches("const ZEROED_point_t: point_t = unsafe { std::mem::zeroed() };")
            .count(),
        1,
        "got: {out}"
    );
    assert!(out.contains("static mut a: point_t"));
    assert!(out.contains("static mut b: point_t"));
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
fn function_def_malformed_param_type_gets_a_placeholder_not_broken_syntax() {
    // Real corpus case (m_misc.c's M_ReadFile/M_WriteFile): a `char const
    // *` param maps to a malformed-type signal, not a clean Rust type. A
    // param can't be commented out like a struct field without breaking
    // the enclosing parenthesized list - must substitute a placeholder
    // type instead so the whole signature stays syntactically valid.
    let sig = FnSig {
        storage: vec![],
        ret_ty: named("boolean"),
        name: "M_WriteFile".to_string(),
        params: vec![crate::parser::ast::Param {
            ty: named("char const"),
            name: "name".to_string(),
            storage: vec![],
        }],
        variadic: false,
    };
    let out = emit_function_def(&sig);
    assert!(!out.contains("char const"));
    assert!(out.contains("name: ()"));
    assert!(out.contains("TODO"));
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
        item(ItemKind::Var(vec![VarDecl {
            storage: vec![],
            ty: named("int"),
            name: name.to_string(),
            initializer: None,
        }]))
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
    let out = emit_conditional(
        &group,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
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
                item(ItemKind::Var(vec![VarDecl {
                    storage: vec![],
                    ty: named("int"),
                    name: "a".to_string(),
                    initializer: None,
                }])),
                Trivia::default(),
            )],
        )],
        else_body: None,
        active: ActiveBranch::None,
    };
    assert_eq!(
        emit_conditional(
            &group,
            &KnownTypeNames::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &mut BTreeSet::new()
        ),
        ""
    );
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
    let out = emit_conditional(
        &group,
        &KnownTypeNames::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
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
    let known = KnownTypeNames::new();
    let out = emit_item(
        &item(ItemKind::Preproc(Directive::Include {
            path: "foo.h".to_string(),
            angled: false,
        })),
        &known,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert_eq!(out, "");
}

#[test]
fn define_object_item_emits_a_const() {
    // Full coverage of value parsing/type-inference lives in
    // `codegen::macros::tests` - this just checks `emit_item` wires the
    // `DefineObject` case through instead of dropping it.
    let known = KnownTypeNames::new();
    let out = emit_item(
        &item(ItemKind::Preproc(Directive::DefineObject {
            name: "FOO".to_string(),
            value: "1".to_string(),
        })),
        &known,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &mut BTreeSet::new(),
    );
    assert_eq!(out, "pub const FOO: std::ffi::c_int = 1;\n\n");
}
