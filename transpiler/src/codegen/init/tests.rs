use super::{render_array_init, render_scalar_init, render_struct_init};
use crate::parser::ast::{Init, RecordDecl, Type};
use crate::parser::stmt::expr::KnownTypeNames;
use std::collections::{BTreeSet, HashMap};

fn known() -> KnownTypeNames {
    KnownTypeNames::new()
}

fn no_records() -> HashMap<String, RecordDecl> {
    HashMap::new()
}

fn no_typedefs() -> HashMap<String, Type> {
    HashMap::new()
}

fn named(s: &str) -> Type {
    Type::Named(s.to_string())
}

#[test]
fn simple_int_literal() {
    assert_eq!(
        render_scalar_init("0", &named("int"), &known(), &no_typedefs()).unwrap(),
        "0"
    );
}

#[test]
fn hex_literal_with_suffix() {
    assert_eq!(
        render_scalar_init("0xc000000", &named("int"), &known(), &no_typedefs()).unwrap(),
        "0xc000000"
    );
}

#[test]
fn bool_literal_ident() {
    // doomtype.h's real boolean typedef is `enum {false, true} boolean;`
    // (pre-C99 code defining its own boolean via enum constants literally
    // named false/true, not the C keywords) - `emit_enum` already escapes
    // these to `false_`/`true_` at the definition site via `ident()`, so a
    // reference here must resolve to the same escaped name to compile.
    assert_eq!(
        render_scalar_init("true", &named("boolean"), &known(), &no_typedefs()).unwrap(),
        "true_"
    );
}

#[test]
fn references_another_already_emitted_const() {
    // d_englsh.h's real e1text = E1TEXT (a string macro const).
    assert_eq!(
        render_scalar_init(
            "E1TEXT",
            &Type::Pointer(Box::new(named("char"))),
            &known(),
            &no_typedefs()
        )
        .unwrap(),
        "E1TEXT"
    );
}

#[test]
fn null_pointer_literal_becomes_null_mut() {
    let ty = Type::Pointer(Box::new(named("Display")));
    assert_eq!(
        render_scalar_init("0", &ty, &known(), &no_typedefs()).unwrap(),
        "std::ptr::null_mut()"
    );
}

#[test]
fn non_pointer_zero_is_not_touched() {
    assert_eq!(
        render_scalar_init("0", &named("int"), &known(), &no_typedefs()).unwrap(),
        "0"
    );
}

#[test]
fn address_of_expression() {
    // m_menu.c-style: `&mousearray[1]`.
    let ty = Type::Pointer(Box::new(named("boolean")));
    let out = render_scalar_init("&mousearray[1]", &ty, &known(), &no_typedefs()).unwrap();
    assert!(out.contains("mousearray"), "got: {out}");
    assert!(!out.contains("null_mut"), "got: {out}");
}

// ---- render_array_init ----

fn arr(elem: Type, dim: Option<&str>) -> Type {
    Type::Array(Box::new(elem), dim.map(str::to_string))
}

#[test]
fn char_array_from_unsized_string_literal_infers_length() {
    let ty = arr(named("char"), None);
    let (ty_text, init_text) = render_array_init(
        &Init::Expr("\"hi\"".to_string()),
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(ty_text, "[std::ffi::c_char; 3]");
    assert_eq!(
        init_text,
        "[104 as std::ffi::c_char, 105 as std::ffi::c_char, 0]"
    );
}

#[test]
fn char_array_from_string_literal_with_explicit_dim_keeps_dim() {
    let ty = arr(named("char"), Some("8"));
    let (ty_text, _) = render_array_init(
        &Init::Expr("\"hi\"".to_string()),
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    // A literal integer dim is used directly (needed so the padding logic
    // below can compute how many implicit-zero slots to add) - a symbolic
    // dim (a macro name) still falls back to the verbatim `(d) as usize`
    // cast, see `non_literal_dim_falls_back_to_verbatim_cast` below.
    assert_eq!(ty_text, "[std::ffi::c_char; 8]");
}

#[test]
fn explicit_dim_longer_than_the_string_pads_with_zeros() {
    // Real corpus case: `m_menu.c`'s `menuitem_t.name: char[10]` given a
    // 7-byte string (`"M_NGAME"`) - the initializer literal must have
    // exactly 10 elements to match the field's own already-declared type.
    let ty = arr(named("char"), Some("10"));
    let (_, init_text) = render_array_init(
        &Init::Expr("\"hi\"".to_string()),
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    // "hi" -> 2 bytes + null terminator = 3, padded to 10 with 7 more zeros.
    assert_eq!(
        init_text,
        "[104 as std::ffi::c_char, 105 as std::ffi::c_char, 0, 0, 0, 0, 0, 0, 0, 0]"
    );
}

#[test]
fn non_literal_dim_falls_back_to_verbatim_cast() {
    let ty = arr(named("char"), Some("SOME_MACRO"));
    let (ty_text, _) = render_array_init(
        &Init::Expr("\"hi\"".to_string()),
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(ty_text, "[std::ffi::c_char; (SOME_MACRO) as usize]");
}

#[test]
fn char_array_from_string_literal_unescapes_common_escapes() {
    let ty = arr(named("char"), None);
    let (_, init_text) = render_array_init(
        &Init::Expr("\"a\\nb\\\\\\\"\"".to_string()),
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    // "a\nb\\\"" -> bytes: a, \n, b, \\, "
    assert_eq!(
        init_text,
        "[97 as std::ffi::c_char, 10 as std::ffi::c_char, 98 as std::ffi::c_char, \
         92 as std::ffi::c_char, 34 as std::ffi::c_char, 0]"
    );
}

#[test]
fn non_char_array_rejects_string_literal_init() {
    let ty = arr(named("int"), None);
    assert!(
        render_array_init(
            &Init::Expr("\"hi\"".to_string()),
            &ty,
            &known(),
            &no_records(),
            &no_typedefs(),
            &mut BTreeSet::new()
        )
        .is_none()
    );
}

#[test]
fn flat_scalar_array_infers_length_from_element_count() {
    let ty = arr(named("int"), None);
    let init = Init::Braced(vec![
        Init::Expr("0".to_string()),
        Init::Expr("8".to_string()),
        Init::Expr("109".to_string()),
    ]);
    let (ty_text, init_text) = render_array_init(
        &init,
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(ty_text, "[std::ffi::c_int; 3]");
    assert_eq!(init_text, "[0, 8, 109]");
}

#[test]
fn flat_scalar_array_with_explicit_dim_keeps_dim() {
    let ty = arr(named("int"), Some("NUMFOO"));
    let init = Init::Braced(vec![Init::Expr("0".to_string())]);
    let (ty_text, _) = render_array_init(
        &init,
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(ty_text, "[std::ffi::c_int; (NUMFOO) as usize]");
}

#[test]
fn flat_scalar_array_with_literal_dim_pads_missing_elements_with_zeroed() {
    // g_game.c's `pars[4][10] = { {0}, ... }` idiom - a row shorter than
    // its declared literal length gets the rest zero-defaulted.
    let ty = arr(named("int"), Some("4"));
    let init = Init::Braced(vec![Init::Expr("0".to_string())]);
    let (ty_text, init_text) = render_array_init(
        &init,
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(ty_text, "[std::ffi::c_int; 4]");
    assert_eq!(
        init_text,
        "[0, std::mem::zeroed(), std::mem::zeroed(), std::mem::zeroed()]"
    );
}

#[test]
fn nested_2d_scalar_array_recurses_one_level() {
    // v_video.c's gammatable[5][256]-style shape, shrunk for the test.
    let ty = arr(arr(named("int"), None), None);
    let init = Init::Braced(vec![
        Init::Braced(vec![
            Init::Expr("1".to_string()),
            Init::Expr("2".to_string()),
        ]),
        Init::Braced(vec![
            Init::Expr("3".to_string()),
            Init::Expr("4".to_string()),
        ]),
    ]);
    let (ty_text, init_text) = render_array_init(
        &init,
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(ty_text, "[[std::ffi::c_int; 2]; 2]");
    assert_eq!(init_text, "[[1, 2], [3, 4]]");
}

#[test]
fn single_element_brace_around_scalar_is_unwrapped() {
    let ty = arr(named("int"), None);
    let init = Init::Braced(vec![Init::Braced(vec![Init::Expr("5".to_string())])]);
    let (_, init_text) = render_array_init(
        &init,
        &ty,
        &known(),
        &no_records(),
        &no_typedefs(),
        &mut BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(init_text, "[5]");
}

#[test]
fn struct_typed_array_row_with_unknown_record_bails_out() {
    // A row that's itself a multi-item Braced group against a non-Array
    // element type, but `known_records` doesn't have that type at all
    // (e.g. it's defined in a file this module's environment doesn't
    // reach) - never guess a field order, bail.
    let ty = arr(named("state_t"), None);
    let init = Init::Braced(vec![Init::Braced(vec![
        Init::Expr("SPR_TROO".to_string()),
        Init::Expr("0".to_string()),
    ])]);
    assert!(
        render_array_init(
            &init,
            &ty,
            &known(),
            &no_records(),
            &no_typedefs(),
            &mut BTreeSet::new()
        )
        .is_none()
    );
}

#[test]
fn scalar_target_type_is_rejected() {
    assert!(
        render_array_init(
            &Init::Expr("0".to_string()),
            &named("int"),
            &known(),
            &no_records(),
            &no_typedefs(),
            &mut BTreeSet::new()
        )
        .is_none()
    );
}

// ---- struct/union-typed rows and vars ----

fn field(name: &str, ty: Type) -> crate::parser::ast::Field {
    crate::parser::ast::Field {
        ty,
        name: name.to_string(),
        storage: vec![],
        bitfield: None,
        nested: None,
        trivia: Default::default(),
        trailing_comment: None,
    }
}

fn record(fields: Vec<crate::parser::ast::Field>) -> RecordDecl {
    RecordDecl {
        kind: crate::parser::ast::RecordKind::Struct,
        tag: None,
        fields,
        names: vec![],
        typedef_name: Some("point_t".to_string()),
    }
}

fn records_with(name: &str, rd: RecordDecl) -> HashMap<String, RecordDecl> {
    let mut m = HashMap::new();
    m.insert(name.to_string(), rd);
    m
}

#[test]
fn bare_struct_var_zips_fields_positionally() {
    let rd = record(vec![field("x", named("int")), field("y", named("int"))]);
    let records = records_with("point_t", rd);
    let init = Init::Braced(vec![
        Init::Expr("1".to_string()),
        Init::Expr("2".to_string()),
    ]);
    let mut needed = BTreeSet::new();
    let out = render_struct_init(
        &init,
        &named("point_t"),
        &known(),
        &records,
        &no_typedefs(),
        &mut needed,
    )
    .unwrap();
    assert_eq!(out, "point_t { x: 1, y: 2 }");
    assert!(needed.is_empty());
}

#[test]
fn partial_row_gets_zeroed_update_syntax_and_records_the_type() {
    let rd = record(vec![
        field("a", named("int")),
        field("b", named("int")),
        field("c", named("int")),
    ]);
    let records = records_with("point_t", rd);
    let init = Init::Braced(vec![Init::Expr("1".to_string())]);
    let mut needed = BTreeSet::new();
    let out = render_struct_init(
        &init,
        &named("point_t"),
        &known(),
        &records,
        &no_typedefs(),
        &mut needed,
    )
    .unwrap();
    assert_eq!(out, "point_t { a: 1, ..ZEROED_point_t }");
    assert!(needed.contains("point_t"));
}

#[test]
fn too_many_values_bails_out() {
    let rd = record(vec![field("x", named("int"))]);
    let records = records_with("point_t", rd);
    let init = Init::Braced(vec![
        Init::Expr("1".to_string()),
        Init::Expr("2".to_string()),
    ]);
    let mut needed = BTreeSet::new();
    assert!(
        render_struct_init(
            &init,
            &named("point_t"),
            &known(),
            &records,
            &no_typedefs(),
            &mut needed
        )
        .is_none()
    );
}

#[test]
fn nested_struct_typed_field_recurses() {
    // am_map.c's real mline_t { mpoint_t a, b; } shape: `{ {1,2}, {3,4} }`.
    let point = record(vec![field("x", named("int")), field("y", named("int"))]);
    let mut records = records_with("mpoint_t", point);
    records.insert(
        "mline_t".to_string(),
        RecordDecl {
            kind: crate::parser::ast::RecordKind::Struct,
            tag: None,
            fields: vec![field("a", named("mpoint_t")), field("b", named("mpoint_t"))],
            names: vec![],
            typedef_name: Some("mline_t".to_string()),
        },
    );
    let init = Init::Braced(vec![
        Init::Braced(vec![
            Init::Expr("1".to_string()),
            Init::Expr("2".to_string()),
        ]),
        Init::Braced(vec![
            Init::Expr("3".to_string()),
            Init::Expr("4".to_string()),
        ]),
    ]);
    let mut needed = BTreeSet::new();
    let out = render_struct_init(
        &init,
        &named("mline_t"),
        &known(),
        &records,
        &no_typedefs(),
        &mut needed,
    )
    .unwrap();
    assert_eq!(
        out,
        "mline_t { a: mpoint_t { x: 1, y: 2 }, b: mpoint_t { x: 3, y: 4 } }"
    );
}

#[test]
fn function_pointer_field_wraps_bare_ident_in_some_and_null_in_none() {
    // info.c's real state_t.action rows: `{A_Light0}` / `{NULL}`.
    let fnptr = Type::FunctionPointer {
        ret: Box::new(named("void")),
        params: vec![],
    };
    let rd = record(vec![field("action", fnptr)]);
    let records = records_with("state_t", rd);
    let mut needed = BTreeSet::new();
    let some = render_struct_init(
        &Init::Braced(vec![Init::Braced(vec![Init::Expr("A_Light0".to_string())])]),
        &named("state_t"),
        &known(),
        &records,
        &no_typedefs(),
        &mut needed,
    )
    .unwrap();
    assert_eq!(some, "state_t { action: Some(A_Light0) }");
    let none = render_struct_init(
        &Init::Braced(vec![Init::Braced(vec![Init::Expr("NULL".to_string())])]),
        &named("state_t"),
        &known(),
        &records,
        &no_typedefs(),
        &mut needed,
    )
    .unwrap();
    assert_eq!(none, "state_t { action: None }");
}

#[test]
fn union_of_typedefd_function_pointers_zips_only_the_first_field() {
    // info.h's real actionf_t: `typedef union { actionf_p1 acp1; actionf_v
    // acv; actionf_p2 acp2; } actionf_t;` - a *union*, not itself a
    // FunctionPointer, whose own first field's type is a typedef *alias*
    // for a function pointer (not a literal `Type::FunctionPointer`) - the
    // real shape behind `state_t.action`'s rows. Two things must both hold:
    // (1) a union's positional init sets only its first member, with no
    // `..ZEROED` (Rust unions forbid `..` update syntax entirely); (2) the
    // Some/None wrapping must fire even though `acp1`'s field type is
    // `Named("actionf_p1")`, resolved through `known_typedefs` to the real
    // `FunctionPointer` shape.
    let mut typedefs = HashMap::new();
    typedefs.insert(
        "actionf_p1".to_string(),
        Type::FunctionPointer {
            ret: Box::new(named("void")),
            params: vec![named("void")],
        },
    );
    let union_rd = RecordDecl {
        kind: crate::parser::ast::RecordKind::Union,
        tag: None,
        fields: vec![
            field("acp1", named("actionf_p1")),
            field("acv", named("actionf_v")),
            field("acp2", named("actionf_p2")),
        ],
        names: vec![],
        typedef_name: Some("actionf_t".to_string()),
    };
    let records = records_with("actionf_t", union_rd);
    let mut needed = BTreeSet::new();

    let some = render_struct_init(
        &Init::Braced(vec![Init::Expr("A_Light0".to_string())]),
        &named("actionf_t"),
        &known(),
        &records,
        &typedefs,
        &mut needed,
    )
    .unwrap();
    assert_eq!(some, "actionf_t { acp1: Some(A_Light0) }");

    let none = render_struct_init(
        &Init::Braced(vec![Init::Expr("NULL".to_string())]),
        &named("actionf_t"),
        &known(),
        &records,
        &typedefs,
        &mut needed,
    )
    .unwrap();
    assert_eq!(none, "actionf_t { acp1: None }");
    // Never `..ZEROED_actionf_t` - invalid Rust union syntax.
    assert!(needed.is_empty());
}

#[test]
fn union_with_more_than_one_value_bails_out() {
    let rd = RecordDecl {
        kind: crate::parser::ast::RecordKind::Union,
        tag: None,
        fields: vec![field("a", named("int")), field("b", named("int"))],
        names: vec![],
        typedef_name: Some("u_t".to_string()),
    };
    let records = records_with("u_t", rd);
    let mut needed = BTreeSet::new();
    let init = Init::Braced(vec![
        Init::Expr("1".to_string()),
        Init::Expr("2".to_string()),
    ]);
    assert!(
        render_struct_init(
            &init,
            &named("u_t"),
            &known(),
            &records,
            &no_typedefs(),
            &mut needed
        )
        .is_none()
    );
}

#[test]
fn char_array_field_from_bare_string_literal_without_extra_braces() {
    // m_menu.c's real menuitem_t rows: `{1,"M_NGAME",M_NewGame,'n'}` - the
    // `name` field is `char[10]`, and its value is a *bare* string literal
    // (`Init::Expr`, not wrapped in its own `{}`) - must still route through
    // the char-array-from-string-literal machinery, same shape
    // `render_array_init` already handles for a top-level
    // `char rcsid[] = "...";` var.
    let rd = record(vec![field("name", arr(named("char"), Some("10")))]);
    let records = records_with("menuitem_t", rd);
    let init = Init::Braced(vec![Init::Expr("\"M_NGAME\"".to_string())]);
    let mut needed = BTreeSet::new();
    let out = render_struct_init(
        &init,
        &named("menuitem_t"),
        &known(),
        &records,
        &no_typedefs(),
        &mut needed,
    )
    .unwrap();
    // "M_NGAME" is 7 bytes + a null terminator = 8, but the field's own
    // declared length is 10 - the remaining 2 slots must be padded with
    // implicit zeros (C's own "the rest default to zero" rule), matching
    // the `[std::ffi::c_char; 10]` type this field was already declared
    // with (see `emit_record`'s own field emission).
    assert_eq!(
        out,
        "menuitem_t { name: [77 as std::ffi::c_char, 95 as std::ffi::c_char, \
         78 as std::ffi::c_char, 71 as std::ffi::c_char, 65 as std::ffi::c_char, \
         77 as std::ffi::c_char, 69 as std::ffi::c_char, 0, 0, 0] }"
    );
}

#[test]
fn array_of_struct_rows_uses_the_record_type_as_element_type() {
    let rd = record(vec![field("x", named("int")), field("y", named("int"))]);
    let records = records_with("point_t", rd);
    let ty = arr(named("point_t"), None);
    let init = Init::Braced(vec![
        Init::Braced(vec![
            Init::Expr("1".to_string()),
            Init::Expr("2".to_string()),
        ]),
        Init::Braced(vec![
            Init::Expr("3".to_string()),
            Init::Expr("4".to_string()),
        ]),
    ]);
    let mut needed = BTreeSet::new();
    let (ty_text, init_text) =
        render_array_init(&init, &ty, &known(), &records, &no_typedefs(), &mut needed).unwrap();
    assert_eq!(ty_text, "[point_t; 2]");
    assert_eq!(
        init_text,
        "[point_t { x: 1, y: 2 }, point_t { x: 3, y: 4 }]"
    );
}

#[test]
fn mid_list_ifdef_splices_the_active_branch_rows() {
    // m_misc.c's defaults[]-style shape: a resolved mid-list #ifdef between
    // plain rows.
    use crate::parser::ast::{ActiveBranch, InitCondBranch, InitCondGroup};
    let rd = record(vec![field("x", named("int"))]);
    let records = records_with("point_t", rd);
    let ty = arr(named("point_t"), None);
    let cond = Init::Conditional(InitCondGroup {
        branches: vec![InitCondBranch {
            directive: crate::parser::preproc::Directive::IfDef {
                name: "FOO".to_string(),
                negate: false,
            },
            body: vec![Init::Braced(vec![Init::Expr("2".to_string())])],
        }],
        else_body: None,
        active: ActiveBranch::Branch(0),
    });
    let init = Init::Braced(vec![
        Init::Braced(vec![Init::Expr("1".to_string())]),
        cond,
        Init::Braced(vec![Init::Expr("3".to_string())]),
    ]);
    let mut needed = BTreeSet::new();
    let (_, init_text) =
        render_array_init(&init, &ty, &known(), &records, &no_typedefs(), &mut needed).unwrap();
    assert_eq!(
        init_text,
        "[point_t { x: 1 }, point_t { x: 2 }, point_t { x: 3 }]"
    );
}

#[test]
fn unresolved_mid_list_ifdef_bails_the_whole_array() {
    use crate::parser::ast::{ActiveBranch, InitCondBranch, InitCondGroup};
    let rd = record(vec![field("x", named("int"))]);
    let records = records_with("point_t", rd);
    let ty = arr(named("point_t"), None);
    let cond = Init::Conditional(InitCondGroup {
        branches: vec![InitCondBranch {
            directive: crate::parser::preproc::Directive::IfDef {
                name: "FOO".to_string(),
                negate: false,
            },
            body: vec![Init::Braced(vec![Init::Expr("2".to_string())])],
        }],
        else_body: None,
        active: ActiveBranch::Unknown,
    });
    let init = Init::Braced(vec![Init::Braced(vec![Init::Expr("1".to_string())]), cond]);
    let mut needed = BTreeSet::new();
    assert!(
        render_array_init(&init, &ty, &known(), &records, &no_typedefs(), &mut needed).is_none()
    );
}
