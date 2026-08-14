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

#[test]
fn strips_leading_license_banner() {
    let src = "// Emacs style mode select   -*- C++ -*- \n\
               //-----------------------------------------------------------------------------\n\
               //\n\
               // DESCRIPTION:\n\
               //\tSomething.\n\
               //-----------------------------------------------------------------------------\n\
               \n\
               \n\
               #ifndef __DOOMTYPE__\n";
    assert_eq!(strip_leading_banner(src), "#ifndef __DOOMTYPE__\n");
}

#[test]
fn leaves_files_without_the_banner_unchanged() {
    let src = "// just some other comment\nint x;\n";
    assert_eq!(strip_leading_banner(src), src);
}

#[test]
fn leaves_files_with_no_leading_comment_unchanged() {
    let src = "int x;\n";
    assert_eq!(strip_leading_banner(src), src);
}
