use super::split_top_level;

#[test]
fn splits_on_plain_top_level_commas() {
    assert_eq!(split_top_level("a, b, c", ','), vec!["a", " b", " c"]);
}

#[test]
fn does_not_split_inside_parens_brackets_braces() {
    assert_eq!(
        split_top_level("f(a, b), c[1, 2], {3, 4}", ','),
        vec!["f(a, b)", " c[1, 2]", " {3, 4}"]
    );
}

#[test]
fn char_literal_containing_the_separator_is_not_split() {
    // hu_stuff.c's real frenchKeyMap[] shape: `'L',',','N'` - the middle
    // element is a char literal whose own content is a comma.
    assert_eq!(
        split_top_level("'L',',','N'", ','),
        vec!["'L'", "','", "'N'"]
    );
}

#[test]
fn char_literal_containing_a_bracket_does_not_open_a_false_depth() {
    // hu_stuff.c's real frenchKeyMap[] shape: `'(',')'` - two separate
    // scalar elements, not one depth-nested blob.
    assert_eq!(split_top_level("'(',')'", ','), vec!["'('", "')'"]);
}

#[test]
fn string_literal_containing_separator_and_brackets_is_not_split() {
    assert_eq!(
        split_top_level(r#""a,(b)", c"#, ','),
        vec![r#""a,(b)""#, " c"]
    );
}

#[test]
fn escaped_quote_inside_char_literal_does_not_end_it_early() {
    // `'\''` - an escaped single-quote char literal; the escaped quote must
    // not be mistaken for the literal's own closing quote.
    assert_eq!(split_top_level(r"'\'', 'x'", ','), vec![r"'\''", " 'x'"]);
}

#[test]
fn escaped_backslash_before_closing_quote_still_closes_the_literal() {
    assert_eq!(split_top_level(r"'\\', 'x'", ','), vec![r"'\\'", " 'x'"]);
}
