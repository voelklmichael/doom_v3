use super::*;
use crate::parser::ast::render_chunks;
use crate::parser::scan::scan;

fn round_trip(src: &str) {
    let toks = scan(src);
    let chunks = group_braces(toks);
    assert_eq!(render_chunks(&chunks), src);
}

#[test]
fn flat_and_one_group() {
    round_trip("int x;\nstruct foo { int a; int b; };\nint y;\n");
}

#[test]
fn nested_braces_stay_opaque_but_matched() {
    let src = "typedef struct { int a; struct { int b; } nested; } foo_t;\n";
    let toks = scan(src);
    let chunks = group_braces(toks);
    assert_eq!(render_chunks(&chunks), src);
    // exactly one top-level group (the nested one is swallowed into `inner`)
    let groups = chunks.iter().filter(|c| matches!(c, Chunk::Group { .. })).count();
    assert_eq!(groups, 1);
}

#[test]
fn brace_inside_block_comment_does_not_confuse_grouping() {
    // mirrors the r_draw.c / s_sound.c pattern: commented-out code with braces
    let src = "void f() {\n    /* if (x) {\n       y();\n    } */\n    real();\n}\n";
    let toks = scan(src);
    let chunks = group_braces(toks);
    assert_eq!(render_chunks(&chunks), src);
    let groups: Vec<&Chunk> = chunks.iter().filter(|c| matches!(c, Chunk::Group { .. })).collect();
    assert_eq!(groups.len(), 1);
}

#[test]
fn ifdef_inside_initializer_list_round_trips() {
    // mirrors m_misc.c's defaults[] table
    let src = "default_t defaults[] = {\n    {\"a\", &a, 1},\n#ifdef NORMALUNIX\n    {\"b\", &b, 2},\n#endif\n};\n";
    round_trip(src);
}
