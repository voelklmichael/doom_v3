use super::*;

fn corpus_path(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../linuxdoom-1.10")
        .join(file)
}

#[test]
fn array_evidence_from_a_real_call_site() {
    // am_map.c: AM_drawLineCharacter's first parameter (`mline_t *lineguy`)
    // is called with player_arrow/cheat_player_arrow/thintriangle_guy -
    // three real file-scope arrays (`mline_t player_arrow[] = {...};`)
    // defined directly in am_map.c itself.
    let evidence = collect_evidence(&[corpus_path("am_map.c")]);
    let hits: Vec<_> = evidence
        .iter()
        .filter(|e| e.function == "AM_drawLineCharacter" && e.param_index == 0)
        .collect();
    assert!(
        hits.iter().all(|e| e.kind == EvidenceKind::Array),
        "expected only Array evidence for AM_drawLineCharacter's lineguy param, got {hits:?}"
    );
    assert!(
        hits.len() >= 3,
        "expected at least 3 array-evidence call sites, got {}",
        hits.len()
    );
    assert!(hits.iter().all(|e| e.param_name == "lineguy"));
    assert!(hits.iter().all(|e| e.caller_file == "am_map.c"));
}

#[test]
fn single_object_evidence_from_address_of() {
    // am_map.c: AM_rotate(fixed_t *x, fixed_t *y, angle_t a) is always
    // called as AM_rotate(&l.a.x, &l.a.y, angle) / AM_rotate(&l.b.x,
    // &l.b.y, angle) - address-of a struct member, never an array.
    let evidence = collect_evidence(&[corpus_path("am_map.c")]);
    let param0: Vec<_> = evidence
        .iter()
        .filter(|e| e.function == "AM_rotate" && e.param_index == 0)
        .collect();
    let param1: Vec<_> = evidence
        .iter()
        .filter(|e| e.function == "AM_rotate" && e.param_index == 1)
        .collect();
    assert!(!param0.is_empty());
    assert!(!param1.is_empty());
    assert!(param0.iter().all(|e| e.kind == EvidenceKind::SingleObject));
    assert!(param1.iter().all(|e| e.kind == EvidenceKind::SingleObject));
}

#[test]
fn summarize_aggregates_per_function_and_param() {
    let evidence = collect_evidence(&[corpus_path("am_map.c")]);
    let summary = summarize(&evidence);
    let lineguy = summary
        .iter()
        .find(|s| s.function == "AM_drawLineCharacter" && s.param_index == 0)
        .expect("expected a summary entry for AM_drawLineCharacter's lineguy param");
    assert!(lineguy.array_hits >= 3);
    assert_eq!(lineguy.single_object_hits, 0);

    let rotate_x = summary
        .iter()
        .find(|s| s.function == "AM_rotate" && s.param_index == 0)
        .expect("expected a summary entry for AM_rotate's x param");
    assert_eq!(rotate_x.array_hits, 0);
    assert!(rotate_x.single_object_hits > 0);
}

#[test]
fn non_pointer_parameters_get_no_evidence() {
    // AM_drawLineCharacter's other params (int/fixed_t/angle_t, all
    // non-pointer) should never generate evidence at all - only
    // Type::Pointer parameters are inspected.
    let evidence = collect_evidence(&[corpus_path("am_map.c")]);
    assert!(
        !evidence
            .iter()
            .any(|e| e.function == "AM_drawLineCharacter" && e.param_index != 0),
        "only param_index 0 (lineguy) is pointer-typed on AM_drawLineCharacter"
    );
}
