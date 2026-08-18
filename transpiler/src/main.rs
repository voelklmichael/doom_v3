use std::collections::HashMap;
use std::path::{Path, PathBuf};
use transpiler::codegen::{
    items as codegen_items, module as codegen_module, write as codegen_write,
};
use transpiler::parse_file_with_types;
use transpiler::parser::corpus::{
    compute_known_defines, compute_known_records, compute_known_type_names, compute_known_typedefs,
};
use transpiler::parser::evidence::{collect_evidence, summarize};
use transpiler::parser::stmt::expr::KnownTypeNames;
use transpiler::parser::{ast, cond, preproc, trivia};

/// The externally pre-defined macro list (compiler `-D`-style flags) used
/// to resolve `#if`/`#ifdef` alongside whatever the source itself
/// `#define`s (see `parser::corpus::compute_known_defines`). Matches this
/// corpus's own name (`linuxdoom-1.10`) and its default Unix build -
/// confirmed with the user via a corpus-wide census before picking this:
/// `LINUX`/`NORMALUNIX` are the only macros that need to come from
/// *outside* the source (everything else interesting - `RANGECHECK`,
/// `SNDSERV` - is already `#define`'d unconditionally in `doomdef.h`).
/// Everything else (C++ mode, other platforms, debug asserts, the
/// alternate `SNDINTR` sound driver, French) starts undefined.
const PREDEFINED_MACROS: &[&str] = &["LINUX", "NORMALUNIX"];

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if take_flag(&mut args, "--array-evidence") {
        print_array_evidence();
        return;
    }
    if take_flag(&mut args, "--emit-rust") {
        run_codegen();
        return;
    }
    let strip_guards = take_flag(&mut args, "--strip-guards");
    let paths: Vec<PathBuf> = if args.is_empty() {
        default_target_files()
    } else if args == ["--all"] {
        all_files()
    } else {
        args.into_iter().map(PathBuf::from).collect()
    };

    // Cast-vs-parenthesized-expression disambiguation, and #if/#ifdef
    // resolution, both need to know about things a file's `#include`s make
    // visible, which can live outside `paths` itself (e.g. a single target
    // `.c` file's own headers) - so both always scan the *whole* corpus,
    // not just `paths`, regardless of which files are the actual output
    // targets.
    let known_types = compute_known_type_names(&all_files());
    let known_defines = compute_known_defines(&all_files());
    let predefined: HashMap<String, String> = PREDEFINED_MACROS
        .iter()
        .map(|m| (m.to_string(), String::new()))
        .collect();

    for path in paths {
        let name = path.file_name().and_then(|n| n.to_str());
        let known = name
            .and_then(|n| known_types.get(n))
            .cloned()
            .unwrap_or_default();
        let mut defines = predefined.clone();
        if let Some(file_defines) = name.and_then(|n| known_defines.get(n)) {
            defines.extend(file_defines.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
        match parse_file_with_types(&path, &known, &defines) {
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

/// `--array-evidence`: runs the call-site evidence pass (see
/// `parser::evidence`) over the whole corpus and prints the aggregated
/// per-`(function, param)` summary, sorted by array-evidence strength.
/// Best-effort/heuristic - see `EvidenceKind`'s own doc comment for what
/// this can and can't tell you.
fn print_array_evidence() {
    let paths = all_files();
    let evidence = collect_evidence(&paths);
    let summary = summarize(&evidence);
    println!(
        "{} call-site evidence hits across {} (function, param) pairs\n",
        evidence.len(),
        summary.len()
    );
    for s in &summary {
        let verdict = match (s.array_hits, s.single_object_hits) {
            (0, _) => continue,
            (_, 0) => "array",
            _ => "MIXED",
        };
        println!(
            "{:<28} {:<16} array={:<3} single={:<3} {verdict}",
            s.function, s.param_name, s.array_hits, s.single_object_hits
        );
    }
}

/// `--emit-rust`: runs the full codegen pipeline over the whole corpus and
/// writes `doom_rs/src/*.rs`. Always operates on all 124 files (module
/// merging inherently needs both halves of every `.c`+`.h` pair) - not
/// driven by the positional-args subset the default JSON-dump mode uses.
/// Skeleton phase only (see `codegen` module docs / the approved plan at
/// `/home/michael/.claude/plans/eager-marinating-axolotl.md`): types,
/// struct/union/enum definitions, and function signatures are real;
/// function bodies and variable initializers are stubs, and macros are out
/// of scope entirely - `cargo build -p doom_rs` is expected to have real,
/// already-understood errors from those deferred features, not a green
/// build.
fn run_codegen() {
    let all = all_files();
    let known_types = compute_known_type_names(&all);
    let known_defines = compute_known_defines(&all);
    let known_records = compute_known_records(&all);
    let known_typedefs = compute_known_typedefs(&all);
    let predefined: HashMap<String, String> = PREDEFINED_MACROS
        .iter()
        .map(|m| (m.to_string(), String::new()))
        .collect();

    // Parse every file once, keyed by filename - codegen::module's merge
    // driver needs both halves of a `.c`+`.h` pair, and the include-graph
    // computation below needs every file's own items too.
    let mut parsed: HashMap<String, Vec<(ast::Item, ast::Trivia)>> = HashMap::new();
    for path in &all {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let known = known_types.get(name).cloned().unwrap_or_default();
        let mut defines = predefined.clone();
        if let Some(file_defines) = known_defines.get(name) {
            defines.extend(file_defines.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
        match parse_file_with_types(path, &known, &defines) {
            Ok(file) => {
                parsed.insert(name.to_string(), file.items);
            }
            Err(e) => eprintln!("failed to parse {}: {e}", path.display()),
        }
    }

    let include_graph: HashMap<String, Vec<String>> = parsed
        .iter()
        .map(|(name, items)| (name.clone(), codegen_module::direct_local_includes(items)))
        .collect();

    let units = codegen_module::group_into_modules(&all);
    let mut generated: Vec<(String, String)> = Vec::with_capacity(units.len());
    let mut total_items = 0usize;
    for unit in &units {
        fn file_name(p: &Path) -> Option<&str> {
            p.file_name().and_then(|n| n.to_str())
        }
        let header_items = unit
            .header
            .as_deref()
            .and_then(file_name)
            .and_then(|n| parsed.get(n));
        let source_items = unit
            .source
            .as_deref()
            .and_then(file_name)
            .and_then(|n| parsed.get(n));
        let merged = codegen_module::merge_items(
            header_items.map(Vec::as_slice),
            source_items.map(Vec::as_slice),
        );
        total_items += merged.len();

        let constituent_files = codegen_module::constituent_file_names(unit);
        let uses = codegen_module::use_statements_for_module(
            &include_graph,
            &unit.name,
            &constituent_files,
        );
        // A macro can appear in either half of the merged `.c`+`.h` pair, so
        // its own cast disambiguation (see codegen::macros) needs the union
        // of both files' known-type environments, not just one.
        let mut module_known = KnownTypeNames::new();
        // Same reasoning for struct/union layouts: a merged module's own
        // struct-typed table (e.g. `info.c`'s `states[]`) may reference a
        // record defined in either half of the pair, or transitively via
        // either half's own `#include`s.
        let mut module_records: HashMap<String, transpiler::parser::ast::RecordDecl> =
            HashMap::new();
        // Same reasoning again for typedefs: resolving a function-pointer
        // value spelled via a typedef alias (e.g. `info.h`'s `actionf_t`
        // union, whose own fields are typedef'd function pointers) needs to
        // see typedefs from either half of the merged pair too.
        let mut module_typedefs: HashMap<String, transpiler::parser::ast::Type> = HashMap::new();
        for name in &constituent_files {
            if let Some(k) = known_types.get(name) {
                module_known.extend(k);
            }
            if let Some(r) = known_records.get(name) {
                module_records.extend(r.iter().map(|(k, v)| (k.clone(), v.clone())));
            }
            if let Some(t) = known_typedefs.get(name) {
                module_typedefs.extend(t.iter().map(|(k, v)| (k.clone(), v.clone())));
            }
        }
        let body =
            codegen_items::emit_items(&merged, &module_known, &module_records, &module_typedefs);

        let mut text = uses.concat();
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(&body);
        generated.push((unit.name.clone(), text));
    }

    let src_dir = Path::new("doom_rs/src");
    match codegen_write::write_all(src_dir, &generated) {
        Ok(written) => {
            println!(
                "wrote {} modules ({total_items} total merged items) to {}",
                generated.len(),
                src_dir.display()
            );
            if let Err(e) = codegen_write::run_rustfmt(&written) {
                eprintln!("failed to run rustfmt: {e}");
            } else {
                println!("ran rustfmt over {} files", written.len());
            }
        }
        Err(e) => eprintln!("failed to write {}: {e}", src_dir.display()),
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
        Var(_) => "var".to_string(),
        FunctionDecl(_) => "fn-decl".to_string(),
        FunctionDef(..) => "fn-def".to_string(),
        Conditional(g) => format!(
            "conditional:{}:{}",
            directive_label(&g.branches[0].directive),
            active_label(g.active)
        ),
        Raw => "raw".to_string(),
    }
}

fn active_label(active: ast::ActiveBranch) -> String {
    match active {
        ast::ActiveBranch::Branch(i) => format!("branch{i}"),
        ast::ActiveBranch::Else => "else".to_string(),
        ast::ActiveBranch::None => "none".to_string(),
        ast::ActiveBranch::Unknown => "unknown".to_string(),
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
