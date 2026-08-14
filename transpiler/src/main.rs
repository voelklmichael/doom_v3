use std::path::{Path, PathBuf};
use transpiler::parse_file;
use transpiler::parser::{ast, preproc, trivia};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let paths: Vec<PathBuf> = if args.is_empty() {
        default_target_files()
    } else {
        args.into_iter().map(PathBuf::from).collect()
    };

    for path in paths {
        match parse_file(&path) {
            Ok(file) => {
                let rebuilt = file.render();
                let original = std::fs::read_to_string(&path).unwrap_or_default();
                let round_trips = rebuilt == original;
                println!("==== {} ====", path.display());
                println!(
                    "items: {}, round-trip exact: {}",
                    file.items.len(),
                    round_trips
                );
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
