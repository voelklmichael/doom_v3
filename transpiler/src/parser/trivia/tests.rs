use super::*;

#[test]
fn detects_banner_comment() {
    let comments = vec![
        Comment::Line("//\n".to_string()),
        Comment::Line("// M_DrawText\n".to_string()),
        Comment::Line("//\n".to_string()),
    ];
    assert_eq!(banner_doc(&comments).as_deref(), Some("M_DrawText"));
}

#[test]
fn non_banner_returns_none() {
    let comments = vec![Comment::Line("// just one line\n".to_string())];
    assert_eq!(banner_doc(&comments), None);
}
