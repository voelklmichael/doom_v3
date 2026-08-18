use super::{render_condition, render_expr, unescape_c_string};
use crate::parser::ast::Type;
use crate::parser::stmt::expr::{AssignOp, BinaryOp, Expr, SizeofArg, UnaryOp};
use std::collections::HashMap;

fn ident(name: &str) -> Box<Expr> {
    Box::new(Expr::Ident(name.to_string()))
}

fn no_globals() -> HashMap<String, Type> {
    HashMap::new()
}

#[test]
fn int_lit_strips_c_suffix() {
    assert_eq!(
        render_expr(&Expr::IntLit("0x000000ffU".into()), &no_globals()).unwrap(),
        "0x000000ff"
    );
}

#[test]
fn float_lit_gets_leading_zero() {
    assert_eq!(
        render_expr(&Expr::FloatLit(".2".into()), &no_globals()).unwrap(),
        "0.2"
    );
    assert_eq!(
        render_expr(&Expr::FloatLit("1.02".into()), &no_globals()).unwrap(),
        "1.02"
    );
}

#[test]
fn str_lit_becomes_c_string_ptr() {
    assert_eq!(
        render_expr(&Expr::StrLit("\"hi\"".into()), &no_globals()).unwrap(),
        "(c\"hi\").as_ptr()"
    );
}

#[test]
fn all_nul_str_lit_becomes_empty_c_string() {
    // p_switch.c's real alphSwitchList[] sentinel row: `{"\0","\0",0}` -
    // Rust's `c"..."` literal syntax rejects an embedded NUL outright
    // (E0729-style hard parse error), unlike every other C escape, which
    // passes through as valid Rust syntax unchanged.
    assert_eq!(
        render_expr(&Expr::StrLit("\"\\0\"".into()), &no_globals()).unwrap(),
        "(c\"\").as_ptr()"
    );
}

#[test]
fn nul_elsewhere_in_str_lit_bails_rather_than_guess() {
    // Never confirmed to occur in this corpus - no safe translation exists,
    // so this must degrade the containing declaration rather than emit
    // invalid Rust syntax.
    assert!(render_expr(&Expr::StrLit("\"foo\\0bar\"".into()), &no_globals()).is_none());
}

#[test]
fn short_hex_escape_gets_padded_to_rust_width() {
    // r_data.c's real `printf("\x8...")` (a backspace-erase idiom, 11
    // sites) - C's `\x` escape consumes as many hex digits as follow (no
    // fixed width), but Rust's requires exactly two; reusing the original
    // C text verbatim is a hard parse error here, not just a possible
    // value mismatch.
    assert_eq!(
        render_expr(&Expr::StrLit("\"\\x8\"".into()), &no_globals()).unwrap(),
        "(c\"\\x08\").as_ptr()"
    );
}

#[test]
fn already_two_digit_hex_escape_is_unchanged() {
    assert_eq!(
        render_expr(&Expr::StrLit("\"\\x1b\"".into()), &no_globals()).unwrap(),
        "(c\"\\x1b\").as_ptr()"
    );
}

#[test]
fn unescape_decodes_hex_escapes_to_their_real_byte() {
    assert_eq!(unescape_c_string("\\x8"), vec![0x08]);
    assert_eq!(unescape_c_string("\\x1b"), vec![0x1b]);
}

#[test]
fn char_lit_becomes_byte_lit_cast_to_c_int() {
    // C char literals are int-typed, so this must compose with surrounding
    // std::ffi::c_int arithmetic (see am_map.h's AM_MSGHEADER, which shifts
    // 'a' left by 24 bits - only well-defined for an int-sized value).
    assert_eq!(
        render_expr(&Expr::CharLit("'g'".into()), &no_globals()).unwrap(),
        "(b'g' as std::ffi::c_int)"
    );
}

#[test]
fn binary_add() {
    let e = Expr::Binary {
        op: BinaryOp::Add,
        lhs: Box::new(Expr::Ident("GRAYS".into())),
        rhs: Box::new(Expr::Ident("GRAYSRANGE".into())),
    };
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "(GRAYS + GRAYSRANGE)"
    );
}

#[test]
fn binary_mixing_float_and_int_casts_the_int_side() {
    // am_map.c's real `.2*FRACUNIT` idiom: C's usual arithmetic conversions
    // implicitly promote FRACUNIT to double before multiplying, but Rust has
    // no implicit numeric coercion at all (`{float} * i32` doesn't compile).
    let e = Expr::Binary {
        op: BinaryOp::Mul,
        lhs: Box::new(Expr::FloatLit(".2".into())),
        rhs: Box::new(Expr::Ident("FRACUNIT".into())),
    };
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "(0.2 * ((FRACUNIT) as f64))"
    );
}

#[test]
fn binary_mixing_int_and_float_casts_the_int_side_regardless_of_order() {
    // am_map.c's real `FRACUNIT/1.02` idiom (M_ZOOMOUT) - the float operand
    // can be on either side.
    let e = Expr::Binary {
        op: BinaryOp::Div,
        lhs: Box::new(Expr::Ident("FRACUNIT".into())),
        rhs: Box::new(Expr::FloatLit("1.02".into())),
    };
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "(((FRACUNIT) as f64) / 1.02)"
    );
}

#[test]
fn binary_both_float_needs_no_cast() {
    let e = Expr::Binary {
        op: BinaryOp::Add,
        lhs: Box::new(Expr::FloatLit("1.0".into())),
        rhs: Box::new(Expr::FloatLit("2.0".into())),
    };
    assert_eq!(render_expr(&e, &no_globals()).unwrap(), "(1.0 + 2.0)");
}

#[test]
fn binary_both_int_needs_no_cast() {
    let e = Expr::Binary {
        op: BinaryOp::Add,
        lhs: Box::new(Expr::Ident("A".into())),
        rhs: Box::new(Expr::Ident("B".into())),
    };
    assert_eq!(render_expr(&e, &no_globals()).unwrap(), "(A + B)");
}

#[test]
fn logical_not_uses_c_semantics_not_bitwise() {
    // `!dofrags` (wi_stuff.c's NG_STATSX) must become "== 0", not Rust's
    // bitwise-not look-alike, which would silently compile to a different
    // value for an integer operand.
    let e = Expr::Unary {
        op: UnaryOp::Not,
        expr: ident("dofrags"),
    };
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "(((dofrags) == 0) as std::ffi::c_int)"
    );
}

#[test]
fn addr_of_goes_through_const_cast_first() {
    // g_game.c's real mousebuttons = &mousearray[1]. A bare `&(x) as *mut _`
    // is invalid Rust (E0606, casting a shared reference straight to a
    // mutable pointer) - must go through `*const _` first.
    let e = Expr::Unary {
        op: UnaryOp::AddrOf,
        expr: ident("x"),
    };
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "(&(x) as *const _ as *mut _)"
    );
}

#[test]
fn addr_of_known_global_spells_out_the_pointee_type() {
    // m_misc.c's real `&sndserver_filename` (a `char*` global) - a bare `_`
    // placeholder normally infers fine, but real corpus proof it sometimes
    // can't (see this case's own doc comment in `render_unary`): once known,
    // spell the identifier's own declared type out explicitly instead.
    let mut globals = HashMap::new();
    globals.insert(
        "sndserver_filename".to_string(),
        Type::Pointer(Box::new(Type::Named("char".to_string()))),
    );
    let e = Expr::Unary {
        op: UnaryOp::AddrOf,
        expr: ident("sndserver_filename"),
    };
    assert_eq!(
        render_expr(&e, &globals).unwrap(),
        "(&(sndserver_filename) as *const *mut std::ffi::c_char as *mut *mut std::ffi::c_char)"
    );
}

#[test]
fn bitwise_not_maps_directly() {
    let e = Expr::Unary {
        op: UnaryOp::BitNot,
        expr: ident("a"),
    };
    assert_eq!(render_expr(&e, &no_globals()).unwrap(), "(!(a))");
}

#[test]
fn cast_to_short_uses_map_type() {
    let e = Expr::Cast {
        ty: Type::Named("short".into()),
        expr: ident("x"),
    };
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "((x) as std::ffi::c_short)"
    );
}

#[test]
fn sizeof_type() {
    let e = Expr::Sizeof(SizeofArg::Type(Type::Named("mline_t".into())));
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "std::mem::size_of::<mline_t>()"
    );
}

#[test]
fn sizeof_expr() {
    let e = Expr::Sizeof(SizeofArg::Expr(ident("player_arrow")));
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "std::mem::size_of_val(&(player_arrow))"
    );
}

#[test]
fn file_and_line_builtins() {
    assert_eq!(
        render_expr(&Expr::Ident("__FILE__".into()), &no_globals()).unwrap(),
        "file!()"
    );
    assert_eq!(
        render_expr(&Expr::Ident("__LINE__".into()), &no_globals()).unwrap(),
        "line!()"
    );
}

#[test]
fn system_value_ident_resolves_to_its_crate_path() {
    // i_sound.c's real `sig = SIGALRM` - never defined anywhere in
    // linuxdoom-1.10's own headers.
    assert_eq!(
        render_expr(&Expr::Ident("SIGALRM".into()), &no_globals()).unwrap(),
        "libc::SIGALRM"
    );
}

#[test]
fn plain_ident_escapes_keywords() {
    assert_eq!(
        render_expr(&Expr::Ident("type".into()), &no_globals()).unwrap(),
        "type_"
    );
}

#[test]
fn assign_renders_operator() {
    let e = Expr::Assign {
        op: AssignOp::OrEq,
        lhs: ident("oc"),
        rhs: Box::new(Expr::IntLit("1".into())),
    };
    assert_eq!(render_expr(&e, &no_globals()).unwrap(), "oc |= 1");
}

#[test]
fn raw_leaf_propagates_none() {
    assert!(render_expr(&Expr::Raw("???".into()), &no_globals()).is_none());
    let wrapped = Expr::Binary {
        op: BinaryOp::Add,
        lhs: Box::new(Expr::Raw("???".into())),
        rhs: Box::new(Expr::IntLit("1".into())),
    };
    assert!(render_expr(&wrapped, &no_globals()).is_none());
}

#[test]
fn arrow_derefs_then_fields() {
    let e = Expr::Arrow {
        base: ident("p"),
        name: "id".into(),
    };
    assert_eq!(render_expr(&e, &no_globals()).unwrap(), "(*p).id");
}

#[test]
fn index_into_unknown_base_uses_plain_subscript() {
    // The safe default when `base` isn't a known global at all (e.g. a
    // local variable, or an identifier this pass never saw a declaration
    // for).
    let e = Expr::Index {
        base: ident("arr"),
        index: Box::new(Expr::IntLit("0".into())),
    };
    assert_eq!(render_expr(&e, &no_globals()).unwrap(), "arr[(0) as usize]");
}

#[test]
fn index_into_sized_array_uses_plain_subscript() {
    // `map_type` maps a *sized* array (`Type::Array(_, Some(_))`) to a real
    // Rust array/slice, which supports `[]` fine.
    let mut globals = HashMap::new();
    globals.insert(
        "sprnames".to_string(),
        Type::Array(
            Box::new(Type::Named("char".to_string())),
            Some("5".to_string()),
        ),
    );
    let e = Expr::Index {
        base: ident("sprnames"),
        index: Box::new(Expr::IntLit("0".into())),
    };
    assert_eq!(render_expr(&e, &globals).unwrap(), "sprnames[(0) as usize]");
}

#[test]
fn index_into_unsized_array_uses_pointer_arithmetic() {
    // `m_misc.c`'s real `extern char* chat_macros[];` - an *unsized* array
    // (`Type::Array(_, None)`) maps to a raw pointer (`map_type`'s own
    // fallback rule), which doesn't implement `Index` at all (`[]` fails to
    // compile, E0608) - `*base.add(i)` is the real pointer-arithmetic
    // equivalent, and composes correctly under `&` too (see
    // `addr_of_unsized_array_index_derefs_then_takes_address` below).
    let mut globals = HashMap::new();
    globals.insert(
        "chat_macros".to_string(),
        Type::Array(
            Box::new(Type::Pointer(Box::new(Type::Named("char".to_string())))),
            None,
        ),
    );
    let e = Expr::Index {
        base: ident("chat_macros"),
        index: Box::new(Expr::IntLit("0".into())),
    };
    assert_eq!(
        render_expr(&e, &globals).unwrap(),
        "(*chat_macros.add((0) as usize))"
    );
}

#[test]
fn addr_of_unsized_array_index_uses_add_directly() {
    // `m_misc.c`'s real `&chat_macros[0]` - AddrOf wrapping Index. Naively
    // combining the two generic renderings (`Index`'s own `*base.add(i)`,
    // then wrapping in the generic `AddrOf` -> `&(*base.add(i)) as *const _
    // as *mut _`) round-trips through a deref-then-reference plus a
    // two-step pointer cast - confirmed via the actual `--emit-rust` +
    // build run to hit a real rustc type-inference limitation (`E0641`)
    // that never fires for a plain identifier's `AddrOf`. `base.add(i)`
    // alone already has exactly the needed `*mut T` type, so `AddrOf`
    // special-cases this exact shape to skip the round-trip entirely.
    let mut globals = HashMap::new();
    globals.insert(
        "chat_macros".to_string(),
        Type::Array(
            Box::new(Type::Pointer(Box::new(Type::Named("char".to_string())))),
            None,
        ),
    );
    let e = Expr::Unary {
        op: UnaryOp::AddrOf,
        expr: Box::new(Expr::Index {
            base: ident("chat_macros"),
            index: Box::new(Expr::IntLit("0".into())),
        }),
    };
    assert_eq!(
        render_expr(&e, &globals).unwrap(),
        "chat_macros.add((0) as usize)"
    );
}

#[test]
fn ternary_coerces_condition_to_bool_check() {
    let e = Expr::Ternary {
        cond: ident("dofrags"),
        then_expr: Box::new(Expr::IntLit("1".into())),
        else_expr: Box::new(Expr::IntLit("0".into())),
    };
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "(if (dofrags) != 0 { 1 } else { 0 })"
    );
}

#[test]
fn ternary_with_comparison_condition_does_not_double_compare() {
    // A comparison-shaped condition already renders as a real Rust `bool`
    // (see `render_condition`) - appending a bare `!= 0` unconditionally
    // (the pre-fix behavior) compared a `bool` against an integer, a type
    // error. `dofrags < 5` is a real corpus-adjacent shape.
    let e = Expr::Ternary {
        cond: Box::new(Expr::Binary {
            op: BinaryOp::Lt,
            lhs: ident("dofrags"),
            rhs: Box::new(Expr::IntLit("5".into())),
        }),
        then_expr: Box::new(Expr::IntLit("1".into())),
        else_expr: Box::new(Expr::IntLit("0".into())),
    };
    assert_eq!(
        render_expr(&e, &no_globals()).unwrap(),
        "(if (dofrags < 5) { 1 } else { 0 })"
    );
}

// ---- render_condition ----

#[test]
fn condition_comparison_passes_through_unchanged() {
    let e = Expr::Binary {
        op: BinaryOp::Lt,
        lhs: ident("a"),
        rhs: ident("b"),
    };
    assert_eq!(render_condition(&e, &no_globals()).unwrap(), "(a < b)");
}

#[test]
fn condition_bare_value_gets_not_equal_zero() {
    assert_eq!(
        render_condition(&Expr::Ident("dofrags".into()), &no_globals()).unwrap(),
        "(dofrags) != 0"
    );
}

#[test]
fn condition_logical_and_recurses_both_sides() {
    // `if (a < b && c)` - `c` alone is std::ffi::c_int-typed, not bool;
    // without recursing, `&&`'s right operand would fail to typecheck.
    let e = Expr::Binary {
        op: BinaryOp::LogAnd,
        lhs: Box::new(Expr::Binary {
            op: BinaryOp::Lt,
            lhs: ident("a"),
            rhs: ident("b"),
        }),
        rhs: ident("c"),
    };
    assert_eq!(
        render_condition(&e, &no_globals()).unwrap(),
        "((a < b) && (c) != 0)"
    );
}

#[test]
fn condition_paren_recurses() {
    let e = Expr::Paren(Box::new(Expr::Binary {
        op: BinaryOp::EqEq,
        lhs: ident("a"),
        rhs: ident("b"),
    }));
    assert_eq!(render_condition(&e, &no_globals()).unwrap(), "((a == b))");
}
