use std::path::{Path, PathBuf};
use transpiler::parse_file;
use transpiler::parser::{ast, cond, preproc, trivia};

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let strip_guards = take_flag(&mut args, "--strip-guards");
    let paths: Vec<PathBuf> = if args.is_empty() {
        default_target_files()
    } else if args == ["--all"] {
        all_files()
    } else {
        args.into_iter().map(PathBuf::from).collect()
    };

    for path in paths {
        match parse_file(&path) {
            Ok(mut file) => {
                let original = std::fs::read_to_string(&path).unwrap_or_default();
                let original = trivia::strip_leading_banner(&original).to_string();
                // Compare round-trip against the pre-strip AST: stripping
                // guards is a real, intentional content removal, so a
                // stripped file is expected to no longer round-trip exact.
                let round_trips = file.render() == original;

                let stripped_guard = if strip_guards {
                    cond::strip_include_guard(&mut file)
                } else {
                    None
                };

                println!("==== {} ====", path.display());
                println!(
                    "items: {}, round-trip exact: {}",
                    file.items.len(),
                    round_trips
                );
                if let Some(name) = &stripped_guard {
                    println!("stripped include guard: {name}");
                }
                for (item, trivia) in &file.items {
                    let kind = kind_label(&item.kind);
                    let doc = trivia::banner_doc(&trivia.leading);
                    match doc {
                        Some(d) => println!("  {kind:<14} {d}"),
                        None => println!("  {kind:<14}"),
                    }
                }
                match write_json(&file) {
                    Ok(out_path) => println!("wrote {}\n", out_path.display()),
                    Err(e) => eprintln!("failed to write JSON for {}: {e}\n", path.display()),
                }
            }
            Err(e) => eprintln!("failed to parse {}: {e}", path.display()),
        }
    }
}

/// Removes the first occurrence of `flag` from `args` (if present) and
/// reports whether it was found.
fn take_flag(args: &mut Vec<String>, flag: &str) -> bool {
    match args.iter().position(|a| a == flag) {
        Some(i) => {
            args.remove(i);
            true
        }
        None => false,
    }
}

fn kind_label(kind: &ast::ItemKind) -> String {
    use ast::ItemKind::*;
    match kind {
        Preproc(d) => format!("preproc:{}", directive_label(d)),
        Record(r) => format!("record:{:?}", r.kind),
        Enum(_) => "enum".to_string(),
        Typedef(_) => "typedef".to_string(),
        Const(_) => "const".to_string(),
        FunctionDecl(_) => "fn-decl".to_string(),
        FunctionDef(..) => "fn-def".to_string(),
        Conditional(g) => format!("conditional:{}", directive_label(&g.branches[0].directive)),
        Raw => "raw".to_string(),
    }
}

fn directive_label(d: &preproc::Directive) -> &'static str {
    use preproc::Directive::*;
    match d {
        Include { .. } => "include",
        DefineObject { .. } => "define",
        DefineFunction { .. } => "define-fn",
        Undef { .. } => "undef",
        IfDef { .. } => "ifdef",
        If { .. } => "if",
        Elif { .. } => "elif",
        Else => "else",
        Endif => "endif",
        Pragma(_) => "pragma",
        Error(_) => "error",
        Other(_) => "other",
    }
}

/// Serializes `file`'s AST to `target/parsed/<file_name>.json` (relative to
/// the current directory) and returns the path written.
fn write_json(file: &ast::File) -> std::io::Result<PathBuf> {
    let out_dir = Path::new("target/parsed");
    std::fs::create_dir_all(out_dir)?;
    let file_name = file.path.file_name().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "path has no file name")
    })?;
    let out_path = out_dir.join(format!("{}.json", file_name.to_string_lossy()));
    let json = serde_json::to_string_pretty(file)?;
    std::fs::write(&out_path, json)?;
    Ok(out_path)
}

fn default_target_files() -> Vec<PathBuf> {
    let base = Path::new("linuxdoom-1.10");
    ["doomtype.h", "doomdef.h", "m_misc.c", "info.h", "info.c"]
        .into_iter()
        .map(|f| base.join(f))
        .collect()
}

/// Every `.c`/`.h` file in `linuxdoom-1.10/`, sorted for deterministic
/// output. Used by `--all`.
fn all_files() -> Vec<PathBuf> {
    let base = Path::new("linuxdoom-1.10");
    let entries = std::fs::read_dir(base)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", base.display()));
    let mut files: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| {
            matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("c") | Some("h")
            )
        })
        .collect();
    files.sort();
    files
}
