use super::*;

fn corpus_path(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../linuxdoom-1.10")
        .join(file)
}

fn known_for(map: &HashMap<String, KnownTypeNames>, file: &str) -> KnownTypeNames {
    map.get(file).cloned().unwrap_or_default()
}

#[test]
fn harvests_plain_typedef_names() {
    // doomtype.h's `boolean`/`byte` - both real typedefs, and both sit
    // inside an #ifdef/#else conditional branch (the classic __BYTEBOOL__
    // guard), which is exactly why this must recurse into CondGroup
    // branches rather than only scanning flat top-level items.
    let map = compute_known_type_names(&[corpus_path("doomtype.h")]);
    let known = known_for(&map, "doomtype.h");
    assert!(known.contains("boolean"));
    assert!(known.contains("byte"));
}

#[test]
fn harvests_typedef_struct_name() {
    // p_mobj.h: `typedef struct mobj_s { ... } mobj_t;` - both the tag
    // (mobj_s) and the typedef name (mobj_t) should be recognized as types.
    let map = compute_known_type_names(&[corpus_path("p_mobj.h")]);
    let known = known_for(&map, "p_mobj.h");
    assert!(known.contains("mobj_t"));
    assert!(known.contains("mobj_s"));
}

#[test]
fn unrelated_identifiers_are_not_known_types() {
    let map = compute_known_type_names(&[corpus_path("doomtype.h")]);
    let known = known_for(&map, "doomtype.h");
    assert!(!known.contains("some_totally_unrelated_name"));
}

#[test]
fn skips_unreadable_paths_without_failing() {
    let map = compute_known_type_names(&[PathBuf::from("/nonexistent/path/does_not_exist.h")]);
    assert!(map.is_empty());
}

#[test]
fn sees_types_transitively_included() {
    // p_mobj.h includes info.h, which (transitively, through further
    // includes) is how a real function body sees names it never declares
    // or includes directly itself - this is the whole point of computing
    // per-file visibility instead of a flat corpus-wide union.
    let map = compute_known_type_names(&[
        corpus_path("p_mobj.h"),
        corpus_path("info.h"),
        corpus_path("doomtype.h"),
        corpus_path("d_think.h"),
        corpus_path("tables.h"),
        corpus_path("m_fixed.h"),
        corpus_path("doomdata.h"),
        corpus_path("doomdef.h"),
    ]);
    let known = known_for(&map, "p_mobj.h");
    // info.h's own typedefs, reached only via p_mobj.h's `#include "info.h"`.
    assert!(known.contains("state_t"));
    // d_think.h's own typedefs, reached only via p_mobj.h's
    // `#include "d_think.h"`.
    assert!(known.contains("think_t"));
}

#[test]
fn unrelated_files_do_not_see_each_others_types() {
    // doomtype.h and d_think.h are both real corpus leaves (zero local
    // #includes each) that never include one another, directly or
    // transitively - a flat corpus-wide union would incorrectly make each
    // one "see" the other's typedefs; per-file visibility must not.
    let map = compute_known_type_names(&[corpus_path("doomtype.h"), corpus_path("d_think.h")]);
    let doomtype = known_for(&map, "doomtype.h");
    let d_think = known_for(&map, "d_think.h");
    assert!(!doomtype.contains("actionf_p1"));
    assert!(!d_think.contains("boolean"));
}

#[test]
fn mutually_including_files_share_their_types() {
    // r_data.h and r_state.h #include each other (the corpus's one real
    // #include cycle) - both should end up seeing both files' own types,
    // rather than the cycle breaking the dependency ordering.
    let map = compute_known_type_names(&[
        corpus_path("r_data.h"),
        corpus_path("r_state.h"),
        corpus_path("r_defs.h"),
        corpus_path("d_player.h"),
        corpus_path("doomtype.h"),
        corpus_path("doomdef.h"),
        corpus_path("d_think.h"),
        corpus_path("tables.h"),
        corpus_path("m_fixed.h"),
        corpus_path("doomdata.h"),
        corpus_path("info.h"),
        corpus_path("p_mobj.h"),
        corpus_path("d_items.h"),
        corpus_path("p_pspr.h"),
        corpus_path("d_ticcmd.h"),
    ]);
    let r_data = known_for(&map, "r_data.h");
    let r_state = known_for(&map, "r_state.h");
    // sector_t is only reachable via r_data.h's own `#include "r_defs.h"`;
    // player_t is only reachable via r_state.h's own `#include "d_player.h"`.
    // Neither file includes the other's dependency directly - only the
    // r_data.h <-> r_state.h cycle joins the two branches together, so both
    // names should be visible from both files.
    assert!(r_data.contains("sector_t"));
    assert!(r_state.contains("sector_t"));
    assert!(r_data.contains("player_t"));
    assert!(r_state.contains("player_t"));
}

fn globals_for(map: &HashMap<String, HashMap<String, Type>>, file: &str) -> HashMap<String, Type> {
    map.get(file).cloned().unwrap_or_default()
}

#[test]
fn harvests_plain_global() {
    let map = compute_known_globals(&[corpus_path("doomstat.h")]);
    let globals = globals_for(&map, "doomstat.h");
    assert_eq!(
        globals.get("nomonsters"),
        Some(&Type::Named("boolean".to_string()))
    );
}

#[test]
fn harvests_array_global() {
    // `extern player_t players[MAXPLAYERS];` - array-ness must survive into
    // the harvested `Type`, not just the scalar base type.
    let map = compute_known_globals(&[corpus_path("doomstat.h")]);
    let globals = globals_for(&map, "doomstat.h");
    assert_eq!(
        globals.get("players"),
        Some(&Type::Array(
            Box::new(Type::Named("player_t".to_string())),
            Some("MAXPLAYERS".to_string())
        ))
    );
}

#[test]
fn unrelated_identifiers_are_not_known_globals() {
    let map = compute_known_globals(&[corpus_path("doomstat.h")]);
    let globals = globals_for(&map, "doomstat.h");
    assert!(!globals.contains_key("some_totally_unrelated_name"));
}

#[test]
fn skips_unreadable_paths_without_failing_globals() {
    let map = compute_known_globals(&[PathBuf::from("/nonexistent/path/does_not_exist.h")]);
    assert!(map.is_empty());
}

#[test]
fn sees_globals_transitively_included() {
    // i_sound.h directly `#include`s doomstat.h - its own globals should be
    // visible from i_sound.h without i_sound.h declaring or including them
    // itself, mirroring `sees_types_transitively_included` one layer up.
    let map = compute_known_globals(&[corpus_path("i_sound.h"), corpus_path("doomstat.h")]);
    let globals = globals_for(&map, "i_sound.h");
    assert!(globals.contains_key("nomonsters"));
}

#[test]
fn unrelated_files_do_not_see_each_others_globals() {
    // d_think.h is a real corpus leaf (zero local #includes, zero own
    // globals) that never includes doomstat.h - a flat corpus-wide union
    // would incorrectly make it "see" doomstat.h's globals; per-file
    // visibility must not.
    let map = compute_known_globals(&[corpus_path("d_think.h"), corpus_path("doomstat.h")]);
    let d_think = globals_for(&map, "d_think.h");
    assert!(!d_think.contains_key("nomonsters"));
}

#[test]
fn mutually_including_files_share_their_globals() {
    // r_data.h and r_state.h #include each other (the corpus's one real
    // #include cycle) - r_state.h's own globals (r_data.h declares none of
    // its own) should still be visible from r_data.h via the cycle.
    let map = compute_known_globals(&[corpus_path("r_data.h"), corpus_path("r_state.h")]);
    let r_data = globals_for(&map, "r_data.h");
    assert!(r_data.contains_key("colormaps"));
}

fn functions_for(
    map: &HashMap<String, HashMap<String, FnSig>>,
    file: &str,
) -> HashMap<String, FnSig> {
    map.get(file).cloned().unwrap_or_default()
}

#[test]
fn harvests_function_signature() {
    let map = compute_known_functions(&[corpus_path("i_system.h")]);
    let sigs = functions_for(&map, "i_system.h");
    let sig = sigs.get("I_Tactile").expect("expected I_Tactile");
    assert_eq!(sig.params.len(), 3);
    assert_eq!(sig.ret_ty, Type::Named("void".to_string()));
}

#[test]
fn harvests_variadic_function_signature() {
    let map = compute_known_functions(&[corpus_path("i_system.h")]);
    let sigs = functions_for(&map, "i_system.h");
    let sig = sigs.get("I_Error").expect("expected I_Error");
    assert!(sig.variadic);
}

#[test]
fn unrelated_files_do_not_see_each_others_functions() {
    let map = compute_known_functions(&[corpus_path("d_think.h"), corpus_path("i_system.h")]);
    let d_think = functions_for(&map, "d_think.h");
    assert!(!d_think.contains_key("I_Tactile"));
}

#[test]
fn sees_functions_transitively_included() {
    // g_game.c directly `#include`s i_system.h - its function signatures
    // should be visible from g_game.c without g_game.c declaring them
    // itself, mirroring the type/global transitive-visibility tests.
    let map = compute_known_functions(&[corpus_path("g_game.c"), corpus_path("i_system.h")]);
    let sigs = functions_for(&map, "g_game.c");
    assert!(sigs.contains_key("I_GetTime"));
}

fn defines_for(
    map: &HashMap<String, HashMap<String, String>>,
    file: &str,
) -> HashMap<String, String> {
    map.get(file).cloned().unwrap_or_default()
}

#[test]
fn harvests_plain_define() {
    let map = compute_known_defines(&[corpus_path("doomdef.h")]);
    let defines = defines_for(&map, "doomdef.h");
    assert_eq!(defines.get("SNDSERV"), Some(&"1".to_string()));
}

#[test]
fn own_include_guard_name_is_not_harvested() {
    // doomdef.h's own `#ifndef __DOOMDEF__`/`#define __DOOMDEF__` wrapper -
    // if __DOOMDEF__ leaked through as "defined" here, doomdef.h's own
    // opening #ifndef check would self-defeat when later fed through
    // cond::resolve_conditionals (always resolving as if the header had
    // already been included once before, which is never true for the
    // single-file-at-a-time model this parser uses).
    let map = compute_known_defines(&[corpus_path("doomdef.h")]);
    let defines = defines_for(&map, "doomdef.h");
    assert!(!defines.contains_key("__DOOMDEF__"));
}

#[test]
fn harvests_valueless_define() {
    // `#define RANGECHECK` (no value) - real corpus text, doomdef.h.
    let map = compute_known_defines(&[corpus_path("doomdef.h")]);
    let defines = defines_for(&map, "doomdef.h");
    assert_eq!(defines.get("RANGECHECK"), Some(&String::new()));
}

#[test]
fn sequential_define_undef_pairs_leave_nothing_behind() {
    // am_map.c: `#define R (...)` / `#undef R`, four times over, bracketing
    // four different mline_t array literals - by end of file `R` must be
    // absent, not "defined as whatever the last #define said" (which would
    // happen if #undef weren't applied in file order).
    let map = compute_known_defines(&[corpus_path("am_map.c")]);
    let defines = defines_for(&map, "am_map.c");
    assert!(!defines.contains_key("R"));
}

#[test]
fn sees_defines_transitively_included() {
    // i_system.c directly `#include`s doomdef.h - RANGECHECK/SNDSERV
    // (defined unconditionally in doomdef.h) should be visible from
    // i_system.c without it declaring them itself. This is the concrete
    // case that makes the whole #if/#ifdef-resolution feature need the
    // #include graph, not just an externally pre-defined list.
    let map = compute_known_defines(&[corpus_path("i_system.c"), corpus_path("doomdef.h")]);
    let defines = defines_for(&map, "i_system.c");
    assert!(defines.contains_key("RANGECHECK"));
    assert_eq!(defines.get("SNDSERV"), Some(&"1".to_string()));
}

#[test]
fn unrelated_files_do_not_see_each_others_defines() {
    let map = compute_known_defines(&[corpus_path("d_think.h"), corpus_path("doomdef.h")]);
    let d_think = defines_for(&map, "d_think.h");
    assert!(!d_think.contains_key("RANGECHECK"));
}

#[test]
fn define_inside_a_conditional_branch_is_not_harvested() {
    // Only genuinely unconditional, top-level #defines count - one inside
    // an #ifdef branch is only real if that branch's own condition holds,
    // which this harvest doesn't attempt to resolve (that's
    // preproc::eval_if_expr/eval_ifdef's job, one layer up).
    let src = "#ifdef SOMETHING\n#define INSIDE_BRANCH 1\n#endif\n";
    let items = crate::parser::cond::fold_conditionals(crate::parser::record::build_items(
        crate::parser::scan::scan(src),
    ));
    let defines = own_defines(&items);
    assert!(!defines.contains_key("INSIDE_BRANCH"));
}

fn typedefs_for(map: &HashMap<String, HashMap<String, Type>>, file: &str) -> HashMap<String, Type> {
    map.get(file).cloned().unwrap_or_default()
}

#[test]
fn harvests_function_pointer_typedef() {
    // d_think.h's real `typedef void (*actionf_v)();`.
    let map = compute_known_typedefs(&[corpus_path("d_think.h")]);
    let typedefs = typedefs_for(&map, "d_think.h");
    assert_eq!(
        typedefs.get("actionf_v"),
        Some(&Type::FunctionPointer {
            ret: Box::new(Type::Named("void".to_string())),
            params: vec![],
        })
    );
}

#[test]
fn unrelated_identifiers_are_not_known_typedefs() {
    let map = compute_known_typedefs(&[corpus_path("d_think.h")]);
    let typedefs = typedefs_for(&map, "d_think.h");
    assert!(!typedefs.contains_key("some_totally_unrelated_name"));
}

#[test]
fn skips_unreadable_paths_without_failing_typedefs() {
    let map = compute_known_typedefs(&[PathBuf::from("/nonexistent/path/does_not_exist.h")]);
    assert!(map.is_empty());
}

fn records_for(
    map: &HashMap<String, HashMap<String, RecordDecl>>,
    file: &str,
) -> HashMap<String, RecordDecl> {
    map.get(file).cloned().unwrap_or_default()
}

#[test]
fn harvests_anonymous_typedef_record_by_typedef_name() {
    // info.h's `state_t` is `typedef struct { ... } state_t;` - no tag, so
    // only reachable via its typedef name.
    let map = compute_known_records(&[corpus_path("info.h")]);
    let records = records_for(&map, "info.h");
    let rd = records.get("state_t").expect("expected state_t");
    assert_eq!(
        rd.fields
            .iter()
            .map(|f| f.name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "sprite",
            "frame",
            "tics",
            "action",
            "nextstate",
            "misc1",
            "misc2"
        ]
    );
}

#[test]
fn harvests_tagged_typedef_record_by_both_spellings() {
    // p_mobj.h's `typedef struct mobj_s { ... } mobj_t;` - a real
    // self-referential tag/typedef pair (the struct's own `snext` field is
    // declared via the tag, since the typedef isn't complete yet at that
    // point in the struct's own body) - both spellings must resolve to the
    // same fields, mirroring `codegen::types::map_type`'s own "resolve via
    // either spelling" rule.
    let map = compute_known_records(&[corpus_path("p_mobj.h")]);
    let records = records_for(&map, "p_mobj.h");
    let by_typedef = records.get("mobj_t").expect("expected mobj_t");
    let by_tag = records.get("mobj_s").expect("expected mobj_s");
    assert_eq!(
        by_typedef
            .fields
            .iter()
            .map(|f| &f.name)
            .collect::<Vec<_>>(),
        by_tag.fields.iter().map(|f| &f.name).collect::<Vec<_>>()
    );
}

#[test]
fn unrelated_identifiers_are_not_known_records() {
    let map = compute_known_records(&[corpus_path("info.h")]);
    let records = records_for(&map, "info.h");
    assert!(!records.contains_key("some_totally_unrelated_name"));
}

#[test]
fn sees_records_transitively_included() {
    // p_mobj.h directly #includes info.h - state_t (defined there) should
    // be visible from p_mobj.h without p_mobj.h declaring it itself,
    // mirroring `sees_types_transitively_included`/`sees_globals_...` one
    // layer up.
    let map = compute_known_records(&[corpus_path("p_mobj.h"), corpus_path("info.h")]);
    let p_mobj_records = records_for(&map, "p_mobj.h");
    assert!(p_mobj_records.contains_key("state_t"));
}

#[test]
fn skips_unreadable_paths_without_failing_records() {
    let map = compute_known_records(&[PathBuf::from("/nonexistent/path/does_not_exist.h")]);
    assert!(map.is_empty());
}
