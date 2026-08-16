use super::*;
use crate::parser::scan::scan;

fn lex(src: &str) -> Vec<CTok> {
    lex_ctoks(&scan(src))
}

#[test]
fn round_trips_arbitrary_expression_text() {
    let src = "x = a->b[i] + (foo_t)*p /* c */ // line\n\"str\" 'c' ? 1 : 2;\n";
    let toks = lex(src);
    assert_eq!(render_ctoks(&toks), src);
}

#[test]
fn round_trips_real_corpus_function_body() {
    // p_enemy.c's A_Look: goto/label, nested if/switch/case/default,
    // pointer/member expressions, comments, tabs - a realistic mix, not
    // just synthetic snippets. Braces are lexed as plain punctuators here
    // (no group_braces pass), which this lexer supports defensively even
    // though the real pipeline never calls it that way - see CTok::Group's
    // doc comment.
    let src = "void A_Look (mobj_t* actor)\n{\n    mobj_t*\ttarg;\n\t\n    actor->threshold = 0;\t// any shot will wake up\n    targ = actor->subsector->sector->soundtarget;\n\n    if (targ\n\t&& (targ->flags & MF_SHOOTABLE) )\n    {\n\tactor->target = targ;\n\n\tif ( actor->flags & MF_AMBUSH )\n\t{\n\t    if (P_CheckSight (actor, actor->target))\n\t\tgoto seeyou;\n\t}\n\telse\n\t    goto seeyou;\n    }\n\t\n\t\n    if (!P_LookForPlayers (actor, false) )\n\treturn;\n\t\t\n    // go into chase state\n  seeyou:\n    if (actor->info->seesound)\n    {\n\tint\t\tsound;\n\t\t\n\tswitch (actor->info->seesound)\n\t{\n\t  case sfx_posit1:\n\t  case sfx_posit2:\n\t  case sfx_posit3:\n\t    sound = sfx_posit1+P_Random()%3;\n\t    break;\n\n\t  case sfx_bgsit1:\n\t  case sfx_bgsit2:\n\t    sound = sfx_bgsit1+P_Random()%2;\n\t    break;\n\n\t  default:\n\t    sound = actor->info->seesound;\n\t    break;\n\t}\n\n\tif (actor->type==MT_SPIDER\n\t    || actor->type == MT_CYBORG)\n\t{\n\t    // full volume\n\t    S_StartSound (NULL, sound);\n\t}\n\telse\n\t    S_StartSound (actor, sound);\n    }\n";
    let toks = lex(src);
    assert_eq!(render_ctoks(&toks), src);
    assert!(!toks.iter().any(|t| matches!(t, CTok::Unknown(_))));
}

#[test]
fn round_trips_via_tokenize_chunks_with_nested_group() {
    // A `{...}` inside the token stream (e.g. a compound statement body)
    // must survive tokenize_chunks -> render_ctoks as an opaque, byte-exact
    // Group, same as elsewhere in this codebase.
    let src = "if (x) { y = 1; } z = 2;";
    let toks = scan(src);
    let chunks = crate::parser::brace::group_braces(toks);
    let ctoks = tokenize_chunks(chunks);
    assert_eq!(render_ctoks(&ctoks), src);
    assert!(ctoks.iter().any(|t| matches!(t, CTok::Group { .. })));
}

#[test]
fn identifiers_and_keywords_are_all_ident() {
    let toks = lex("if foo_Bar123 _leading");
    let idents: Vec<&str> = toks
        .iter()
        .filter(|t| matches!(t, CTok::Ident(_)))
        .map(|t| match t {
            CTok::Ident(s) => s.text.as_str(),
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(idents, vec!["if", "foo_Bar123", "_leading"]);
}

#[test]
fn decimal_int_and_float_literals() {
    let cases: &[(&str, bool)] = &[
        ("123", false),
        ("123UL", false),
        ("0", false),
        ("123.456", true),
        (".5", true),
        ("123.", true),
        ("1e10", true),
        ("1.5e-10f", true),
        ("2E+3L", true),
    ];
    for (text, expect_float) in cases {
        let toks = lex(text);
        assert_eq!(
            toks.len(),
            1,
            "unexpected token count for {text:?}: {toks:?}"
        );
        match &toks[0] {
            CTok::IntLit(s) => {
                assert!(!expect_float, "{text:?} classified as int, expected float");
                assert_eq!(s.text, *text);
            }
            CTok::FloatLit(s) => {
                assert!(expect_float, "{text:?} classified as float, expected int");
                assert_eq!(s.text, *text);
            }
            other => panic!("expected a numeric literal for {text:?}, got {other:?}"),
        }
    }
}

#[test]
fn hex_int_literal_with_suffix_and_hex_e_digit() {
    // `e`/`E` inside a hex literal is a hex digit, not an exponent marker -
    // 0xE and 0x1E must stay integers, not be misread as floats.
    for text in ["0x1A", "0X1a", "0xFFUL", "0xE", "0x1E"] {
        let toks = lex(text);
        assert_eq!(toks.len(), 1);
        match &toks[0] {
            CTok::IntLit(s) => assert_eq!(s.text, text),
            other => panic!("expected IntLit for {text:?}, got {other:?}"),
        }
    }
}

#[test]
fn maximal_munch_punctuators() {
    let cases: &[(&str, Punct)] = &[
        (">>=", Punct::ShrEq),
        ("<<=", Punct::ShlEq),
        ("...", Punct::Ellipsis),
        ("->", Punct::Arrow),
        ("++", Punct::PlusPlus),
        ("--", Punct::MinusMinus),
        ("<<", Punct::Shl),
        (">>", Punct::Shr),
        ("<=", Punct::Le),
        (">=", Punct::Ge),
        ("==", Punct::EqEq),
        ("!=", Punct::NotEq),
        ("&&", Punct::AmpAmp),
        ("||", Punct::PipePipe),
        ("+=", Punct::PlusEq),
        ("-=", Punct::MinusEq),
        ("*=", Punct::StarEq),
        ("/=", Punct::SlashEq),
        ("%=", Punct::PercentEq),
        ("&=", Punct::AmpEq),
        ("|=", Punct::PipeEq),
        ("^=", Punct::CaretEq),
    ];
    for (text, expected) in cases {
        let toks = lex(text);
        assert_eq!(toks.len(), 1, "unexpected split for {text:?}: {toks:?}");
        match &toks[0] {
            CTok::Punct(p, s) => {
                assert_eq!(p, expected, "wrong punct for {text:?}");
                assert_eq!(s.text, *text);
            }
            other => panic!("expected Punct for {text:?}, got {other:?}"),
        }
    }
}

#[test]
fn single_char_punctuators_not_over_matched() {
    // `-` alone (not `->`/`--`/`-=`) followed by a digit must not fuse into
    // the punctuator match.
    let toks = lex("a - b");
    let puncts: Vec<&Punct> = toks
        .iter()
        .filter_map(|t| match t {
            CTok::Punct(p, _) => Some(p),
            _ => None,
        })
        .collect();
    assert_eq!(puncts, vec![&Punct::Minus]);
}

#[test]
fn dot_alone_is_not_ellipsis() {
    let toks = lex("a.b");
    let puncts: Vec<&Punct> = toks
        .iter()
        .filter_map(|t| match t {
            CTok::Punct(p, _) => Some(p),
            _ => None,
        })
        .collect();
    assert_eq!(puncts, vec![&Punct::Dot]);
}

#[test]
fn string_char_comment_preproc_pass_through_unchanged() {
    let src = "\"hi\" 'c' // line\n/* block */\n#define X 1\n";
    let toks = lex(src);
    let kinds: Vec<&str> = toks
        .iter()
        .map(|t| match t {
            CTok::Str(_) => "str",
            CTok::Char(_) => "char",
            CTok::LineComment(_) => "line",
            CTok::BlockComment(_) => "block",
            CTok::PreprocLine(_) => "pp",
            CTok::Trivia(_) => "trivia",
            other => panic!("unexpected token kind: {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        vec![
            "str", "trivia", "char", "trivia", "line", "block", "trivia", "pp"
        ]
    );
}

#[test]
fn whitespace_is_kept_as_trivia_not_dropped() {
    let toks = lex("a   b");
    assert!(matches!(&toks[1], CTok::Trivia(s) if s.text == "   "));
    assert_eq!(render_ctoks(&toks), "a   b");
}

#[test]
fn is_trivial_skips_whitespace_and_comments_but_not_preproc() {
    let toks = lex("a /* c */\n#define X 1\nb");
    let non_trivial_kinds: Vec<&str> = toks
        .iter()
        .filter(|t| !t.is_trivial())
        .map(|t| match t {
            CTok::Ident(_) => "ident",
            CTok::PreprocLine(_) => "pp",
            other => panic!("unexpected non-trivial token: {other:?}"),
        })
        .collect();
    assert_eq!(non_trivial_kinds, vec!["ident", "pp", "ident"]);
}
