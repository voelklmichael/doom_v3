use super::render_expr;
use crate::parser::ast::Type;
use crate::parser::stmt::expr::{AssignOp, BinaryOp, Expr, SizeofArg, UnaryOp};

fn ident(name: &str) -> Box<Expr> {
    Box::new(Expr::Ident(name.to_string()))
}

#[test]
fn int_lit_strips_c_suffix() {
    assert_eq!(
        render_expr(&Expr::IntLit("0x000000ffU".into())).unwrap(),
        "0x000000ff"
    );
}

#[test]
fn float_lit_gets_leading_zero() {
    assert_eq!(render_expr(&Expr::FloatLit(".2".into())).unwrap(), "0.2");
    assert_eq!(render_expr(&Expr::FloatLit("1.02".into())).unwrap(), "1.02");
}

#[test]
fn str_lit_becomes_c_string_ptr() {
    assert_eq!(
        render_expr(&Expr::StrLit("\"hi\"".into())).unwrap(),
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
        render_expr(&Expr::StrLit("\"\\0\"".into())).unwrap(),
        "(c\"\").as_ptr()"
    );
}

#[test]
fn nul_elsewhere_in_str_lit_bails_rather_than_guess() {
    // Never confirmed to occur in this corpus - no safe translation exists,
    // so this must degrade the containing declaration rather than emit
    // invalid Rust syntax.
    assert!(render_expr(&Expr::StrLit("\"foo\\0bar\"".into())).is_none());
}

#[test]
fn char_lit_becomes_byte_lit_cast_to_c_int() {
    // C char literals are int-typed, so this must compose with surrounding
    // std::ffi::c_int arithmetic (see am_map.h's AM_MSGHEADER, which shifts
    // 'a' left by 24 bits - only well-defined for an int-sized value).
    assert_eq!(
        render_expr(&Expr::CharLit("'g'".into())).unwrap(),
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
    assert_eq!(render_expr(&e).unwrap(), "(GRAYS + GRAYSRANGE)");
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
        render_expr(&e).unwrap(),
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
    assert_eq!(render_expr(&e).unwrap(), "(&(x) as *const _ as *mut _)");
}

#[test]
fn bitwise_not_maps_directly() {
    let e = Expr::Unary {
        op: UnaryOp::BitNot,
        expr: ident("a"),
    };
    assert_eq!(render_expr(&e).unwrap(), "(!(a))");
}

#[test]
fn cast_to_short_uses_map_type() {
    let e = Expr::Cast {
        ty: Type::Named("short".into()),
        expr: ident("x"),
    };
    assert_eq!(render_expr(&e).unwrap(), "((x) as std::ffi::c_short)");
}

#[test]
fn sizeof_type() {
    let e = Expr::Sizeof(SizeofArg::Type(Type::Named("mline_t".into())));
    assert_eq!(render_expr(&e).unwrap(), "std::mem::size_of::<mline_t>()");
}

#[test]
fn sizeof_expr() {
    let e = Expr::Sizeof(SizeofArg::Expr(ident("player_arrow")));
    assert_eq!(
        render_expr(&e).unwrap(),
        "std::mem::size_of_val(&(player_arrow))"
    );
}

#[test]
fn file_and_line_builtins() {
    assert_eq!(
        render_expr(&Expr::Ident("__FILE__".into())).unwrap(),
        "file!()"
    );
    assert_eq!(
        render_expr(&Expr::Ident("__LINE__".into())).unwrap(),
        "line!()"
    );
}

#[test]
fn plain_ident_escapes_keywords() {
    assert_eq!(render_expr(&Expr::Ident("type".into())).unwrap(), "type_");
}

#[test]
fn assign_renders_operator() {
    let e = Expr::Assign {
        op: AssignOp::OrEq,
        lhs: ident("oc"),
        rhs: Box::new(Expr::IntLit("1".into())),
    };
    assert_eq!(render_expr(&e).unwrap(), "oc |= 1");
}

#[test]
fn raw_leaf_propagates_none() {
    assert!(render_expr(&Expr::Raw("???".into())).is_none());
    let wrapped = Expr::Binary {
        op: BinaryOp::Add,
        lhs: Box::new(Expr::Raw("???".into())),
        rhs: Box::new(Expr::IntLit("1".into())),
    };
    assert!(render_expr(&wrapped).is_none());
}

#[test]
fn arrow_derefs_then_fields() {
    let e = Expr::Arrow {
        base: ident("p"),
        name: "id".into(),
    };
    assert_eq!(render_expr(&e).unwrap(), "(*p).id");
}

#[test]
fn ternary_coerces_condition_to_bool_check() {
    let e = Expr::Ternary {
        cond: ident("dofrags"),
        then_expr: Box::new(Expr::IntLit("1".into())),
        else_expr: Box::new(Expr::IntLit("0".into())),
    };
    assert_eq!(
        render_expr(&e).unwrap(),
        "(if (dofrags) != 0 { 1 } else { 0 })"
    );
}
