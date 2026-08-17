use super::*;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "transpiler-codegen-write-test-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn generate_lib_rs_includes_preamble_and_sorted_modules() {
    let out = generate_lib_rs(&["m_misc".to_string(), "doomtype".to_string()]);
    assert!(out.contains("#![allow(non_snake_case"));
    let doomtype_pos = out.find("pub mod doomtype;").unwrap();
    let m_misc_pos = out.find("pub mod m_misc;").unwrap();
    assert!(doomtype_pos < m_misc_pos, "modules should be sorted");
}

#[test]
fn write_all_writes_every_module_plus_lib_rs() {
    let dir = temp_dir("basic");
    let modules = vec![
        ("a".to_string(), "pub const A: i32 = 1;\n".to_string()),
        ("b".to_string(), "pub const B: i32 = 2;\n".to_string()),
    ];
    let written = write_all(&dir, &modules).unwrap();
    assert_eq!(written.len(), 3); // a.rs, b.rs, lib.rs

    let a_content = std::fs::read_to_string(dir.join("a.rs")).unwrap();
    assert_eq!(a_content, "pub const A: i32 = 1;\n");
    let lib_content = std::fs::read_to_string(dir.join("lib.rs")).unwrap();
    assert!(lib_content.contains("pub mod a;"));
    assert!(lib_content.contains("pub mod b;"));

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn write_all_clears_stale_files_from_a_previous_run() {
    let dir = temp_dir("stale");
    // Simulate a leftover module from before some module was renamed/removed.
    std::fs::write(dir.join("old_removed_module.rs"), "stale").unwrap();

    let modules = vec![("fresh".to_string(), "pub const X: i32 = 1;\n".to_string())];
    write_all(&dir, &modules).unwrap();

    assert!(!dir.join("old_removed_module.rs").exists());
    assert!(dir.join("fresh.rs").exists());

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn write_all_with_no_modules_still_writes_lib_rs() {
    let dir = temp_dir("empty");
    let written = write_all(&dir, &[]).unwrap();
    assert_eq!(written.len(), 1);
    assert!(dir.join("lib.rs").exists());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn run_rustfmt_on_empty_path_list_is_a_no_op() {
    run_rustfmt(&[]).unwrap();
}
