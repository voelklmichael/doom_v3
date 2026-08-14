use super::*;

#[test]
fn round_trips_arbitrary_text() {
    let src = "// hi\n#define X 1\nint x = \"a{b}c\"; /* {not a brace} */\n";
    let toks = scan(src);
    let rebuilt: String = toks.iter().map(RawToken::text).collect();
    assert_eq!(rebuilt, src);
}

#[test]
fn classifies_kinds() {
    let src = "// c\n#define X 1\nint x;\n\"s\" 'c'\n/* b */\n";
    let toks = scan(src);
    let kinds: Vec<&str> = toks
        .iter()
        .map(|t| match t {
            RawToken::LineComment(_) => "line",
            RawToken::BlockComment(_) => "block",
            RawToken::StringLit(_) => "str",
            RawToken::CharLit(_) => "char",
            RawToken::PreprocLine(_) => "pp",
            RawToken::Code(_) => "code",
        })
        .collect();
    assert_eq!(kinds, vec!["line", "pp", "code", "str", "code", "char", "code", "block", "code"]);
}

#[test]
fn backslash_continued_macro_is_one_directive() {
    let src = "#define FOO(a) \\\n  ((a)+1)\nint x;\n";
    let toks = scan(src);
    assert!(matches!(&toks[0], RawToken::PreprocLine(s) if s.text.contains("((a)+1)")));
}
