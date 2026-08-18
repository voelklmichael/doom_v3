use super::*;
use crate::parser::scan::scan;
use crate::parser::stmt::lex::lex_ctoks;

fn parse(src: &str) -> Expr {
    parse_with(src, &KnownTypeNames::new())
}

fn parse_with(src: &str, known: &KnownTypeNames) -> Expr {
    let toks = lex_ctoks(&scan(src));
    parse_expr(&toks, known)
}

fn parse_checked(src: &str) -> Option<Expr> {
    let toks = lex_ctoks(&scan(src));
    parse_expr_checked(&toks, &KnownTypeNames::new())
}

fn ident(s: &str) -> Expr {
    Expr::Ident(s.to_string())
}

fn int(s: &str) -> Expr {
    Expr::IntLit(s.to_string())
}

#[test]
fn multiplicative_binds_tighter_than_additive() {
    assert_eq!(
        parse("a + b * c"),
        Expr::Binary {
            op: BinaryOp::Add,
            lhs: Box::new(ident("a")),
            rhs: Box::new(Expr::Binary {
                op: BinaryOp::Mul,
                lhs: Box::new(ident("b")),
                rhs: Box::new(ident("c")),
            }),
        }
    );
}

#[test]
fn equality_binds_tighter_than_bitwise_and() {
    // The classic C gotcha: `flags & MASK == VALUE` is `flags & (MASK ==
    // VALUE)`, not `(flags & MASK) == VALUE` - == outranks &.
    assert_eq!(
        parse("a & b == c"),
        Expr::Binary {
            op: BinaryOp::BitAnd,
            lhs: Box::new(ident("a")),
            rhs: Box::new(Expr::Binary {
                op: BinaryOp::EqEq,
                lhs: Box::new(ident("b")),
                rhs: Box::new(ident("c")),
            }),
        }
    );
}

#[test]
fn logical_or_is_loosest_of_logical_and_bitwise() {
    assert_eq!(
        parse("a | b || c"),
        Expr::Binary {
            op: BinaryOp::LogOr,
            lhs: Box::new(Expr::Binary {
                op: BinaryOp::BitOr,
                lhs: Box::new(ident("a")),
                rhs: Box::new(ident("b")),
            }),
            rhs: Box::new(ident("c")),
        }
    );
}

#[test]
fn assignment_is_right_associative() {
    assert_eq!(
        parse("a = b = c"),
        Expr::Assign {
            op: AssignOp::Assign,
            lhs: Box::new(ident("a")),
            rhs: Box::new(Expr::Assign {
                op: AssignOp::Assign,
                lhs: Box::new(ident("b")),
                rhs: Box::new(ident("c")),
            }),
        }
    );
}

#[test]
fn ternary_right_associates_nested_else() {
    assert_eq!(
        parse("a ? b : c ? d : e"),
        Expr::Ternary {
            cond: Box::new(ident("a")),
            then_expr: Box::new(ident("b")),
            else_expr: Box::new(Expr::Ternary {
                cond: Box::new(ident("c")),
                then_expr: Box::new(ident("d")),
                else_expr: Box::new(ident("e")),
            }),
        }
    );
}

#[test]
fn comma_operator_is_n_ary_not_nested_pairs() {
    assert_eq!(
        parse("a = 0, b = 1"),
        Expr::Comma(vec![
            Expr::Assign {
                op: AssignOp::Assign,
                lhs: Box::new(ident("a")),
                rhs: Box::new(int("0")),
            },
            Expr::Assign {
                op: AssignOp::Assign,
                lhs: Box::new(ident("b")),
                rhs: Box::new(int("1")),
            },
        ])
    );
}

#[test]
fn call_args_are_assignment_level_not_comma_level() {
    // foo(a, b) is two args, not one Comma arg.
    assert_eq!(
        parse("foo(a, b)"),
        Expr::Call {
            callee: Box::new(ident("foo")),
            args: vec![ident("a"), ident("b")],
        }
    );
}

#[test]
fn member_arrow_index_call_chain() {
    assert_eq!(
        parse("a.b->c[i](x, y)"),
        Expr::Call {
            callee: Box::new(Expr::Index {
                base: Box::new(Expr::Arrow {
                    base: Box::new(Expr::Member {
                        base: Box::new(ident("a")),
                        name: "b".to_string(),
                    }),
                    name: "c".to_string(),
                }),
                index: Box::new(ident("i")),
            }),
            args: vec![ident("x"), ident("y")],
        }
    );
}

#[test]
fn pre_and_post_inc_dec() {
    assert_eq!(
        parse("++a"),
        Expr::Unary {
            op: UnaryOp::PreInc,
            expr: Box::new(ident("a")),
        }
    );
    assert_eq!(
        parse("a++"),
        Expr::Postfix {
            op: PostfixOp::PostInc,
            expr: Box::new(ident("a")),
        }
    );
    assert_eq!(
        parse("--*p"),
        Expr::Unary {
            op: UnaryOp::PreDec,
            expr: Box::new(Expr::Unary {
                op: UnaryOp::Deref,
                expr: Box::new(ident("p")),
            }),
        }
    );
}

#[test]
fn address_of_and_deref() {
    assert_eq!(
        parse("&x"),
        Expr::Unary {
            op: UnaryOp::AddrOf,
            expr: Box::new(ident("x")),
        }
    );
    assert_eq!(
        parse("*p"),
        Expr::Unary {
            op: UnaryOp::Deref,
            expr: Box::new(ident("p")),
        }
    );
}

#[test]
fn chained_unary_operators() {
    assert_eq!(
        parse("!!x"),
        Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(ident("x")),
            }),
        }
    );
}

#[test]
fn sizeof_of_known_type_vs_expr() {
    assert_eq!(
        parse("sizeof(int)"),
        Expr::Sizeof(SizeofArg::Type(Type::Named("int".to_string())))
    );
    // `x` isn't a known type, so `sizeof(x)` reads its operand as a
    // parenthesized expression, not a type-name - the same lookahead used
    // for casts resolves this the same way.
    assert_eq!(
        parse("sizeof(x)"),
        Expr::Sizeof(SizeofArg::Expr(Box::new(Expr::Paren(Box::new(ident("x"))))))
    );
    // No parens at all is also legal C for a non-type sizeof operand.
    assert_eq!(
        parse("sizeof x"),
        Expr::Sizeof(SizeofArg::Expr(Box::new(ident("x"))))
    );
}

#[test]
fn cast_to_known_typedef_pointer() {
    let mut known = KnownTypeNames::new();
    known.insert("mobj_t");
    assert_eq!(
        parse_with("(mobj_t*)ptr", &known),
        Expr::Cast {
            ty: Type::Pointer(Box::new(Type::Named("mobj_t".to_string()))),
            expr: Box::new(ident("ptr")),
        }
    );
}

#[test]
fn cast_to_base_keyword_type() {
    assert_eq!(
        parse("(unsigned)x"),
        Expr::Cast {
            ty: Type::Named("unsigned".to_string()),
            expr: Box::new(ident("x")),
        }
    );
}

#[test]
fn cast_applies_to_the_whole_postfix_chain() {
    // (unsigned)actor->movedir >= 8, from p_enemy.c-style code: the cast
    // wraps the entire `actor->movedir` postfix chain, not just `actor`,
    // and the whole cast is the looser >= operator's left operand.
    assert_eq!(
        parse("(unsigned)actor->movedir >= 8"),
        Expr::Binary {
            op: BinaryOp::Ge,
            lhs: Box::new(Expr::Cast {
                ty: Type::Named("unsigned".to_string()),
                expr: Box::new(Expr::Arrow {
                    base: Box::new(ident("actor")),
                    name: "movedir".to_string(),
                }),
            }),
            rhs: Box::new(int("8")),
        }
    );
}

#[test]
fn paren_not_mistaken_for_cast_when_name_is_unknown() {
    // `x` is not a known type name, so `(x)(y)` must read as a call whose
    // callee is a parenthesized expression, not a cast.
    assert_eq!(
        parse("(x)(y)"),
        Expr::Call {
            callee: Box::new(Expr::Paren(Box::new(ident("x")))),
            args: vec![ident("y")],
        }
    );
}

#[test]
fn function_pointer_cast_type() {
    // i_video.c:717 - `signal(SIGINT, (void (*)(int)) I_Quit);` - the
    // hardest disambiguation case, and needs no KnownTypeNames entries at
    // all since `void`/`int` are base keywords.
    assert_eq!(
        parse("(void (*)(int)) I_Quit"),
        Expr::Cast {
            ty: Type::FunctionPointer {
                ret: Box::new(Type::Named("void".to_string())),
                params: vec![Type::Named("int".to_string())],
            },
            expr: Box::new(ident("I_Quit")),
        }
    );
}

#[test]
fn struct_tag_cast() {
    assert_eq!(
        parse("(struct foo *)p"),
        Expr::Cast {
            ty: Type::Pointer(Box::new(Type::Named("struct foo".to_string()))),
            expr: Box::new(ident("p")),
        }
    );
}

#[test]
fn literals_and_paren_expr_round_trip_into_tree_shape() {
    assert_eq!(parse("123"), int("123"));
    assert_eq!(parse("1.5f"), Expr::FloatLit("1.5f".to_string()));
    assert_eq!(parse("\"hi\""), Expr::StrLit("\"hi\"".to_string()));
    assert_eq!(parse("'c'"), Expr::CharLit("'c'".to_string()));
    assert_eq!(parse("(x)"), Expr::Paren(Box::new(ident("x"))));
}

#[test]
fn ternary_from_real_corpus_shape() {
    // p_map.c-style: `dist = dx>dy ? dx : dy;` (expression only, no `;`).
    assert_eq!(
        parse("dx>dy ? dx : dy"),
        Expr::Ternary {
            cond: Box::new(Expr::Binary {
                op: BinaryOp::Gt,
                lhs: Box::new(ident("dx")),
                rhs: Box::new(ident("dy")),
            }),
            then_expr: Box::new(ident("dx")),
            else_expr: Box::new(ident("dy")),
        }
    );
}

#[test]
fn checked_parse_accepts_a_complete_expression() {
    assert_eq!(
        parse_checked("dx>dy ? dx : dy"),
        Some(parse("dx>dy ? dx : dy"))
    );
}

#[test]
fn checked_parse_rejects_a_partial_match() {
    // am_map.c's real local `enum { LEFT=1, RIGHT=2, ... };` - not a valid
    // expression at all, but `parse_expr` alone happily matches just the
    // leading `enum` as `Expr::Ident("enum")` and silently stops there,
    // leaving the brace-group tokens after it unconsumed. `parse_expr`
    // itself must still return that partial match unchanged (every other
    // call site feeds it a token range already known to be exactly one
    // expression) - only the checked variant must reject it.
    assert_eq!(parse("enum { LEFT = 1 }"), ident("enum"));
    assert_eq!(parse_checked("enum { LEFT = 1 }"), None);
}
