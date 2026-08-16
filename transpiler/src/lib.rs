pub mod parser;

use parser::ast::File;
use parser::stmt::expr::KnownTypeNames;
use std::collections::HashMap;
use std::path::Path;

/// Reads and parses a single C source file into its AST, with no corpus
/// type-name context - function bodies still get fully parsed, just with
/// less precise cast-vs-parenthesized-expression disambiguation inside them
/// (see `parser::record::build_items`'s doc comment), and no `#if`/`#ifdef`
/// resolution (every `CondGroup`/`StmtCondGroup.active` stays `Unknown`).
/// Real per-file output should use `parse_file_with_types` instead, fed a
/// `KnownTypeNames` from `parser::corpus::compute_known_type_names` and a
/// `#define` environment from `parser::corpus::compute_known_defines`.
pub fn parse_file(path: &Path) -> std::io::Result<File> {
    parse_file_with_types(path, &KnownTypeNames::new(), &HashMap::new())
}

/// Same as `parse_file`, but threads `known` through to every function
/// body's cast-vs-parenthesized-expression disambiguation, and `defines`
/// through to resolving which branch of every `#if`/`#ifdef` (at both the
/// top level and inside function bodies) would actually compile - see
/// `parser::cond::resolve_conditionals`. If the file begins with the id
/// Software license/RCS banner comment, that banner is dropped before
/// parsing - see `parser::trivia::strip_leading_banner`.
pub fn parse_file_with_types(
    path: &Path,
    known: &KnownTypeNames,
    defines: &HashMap<String, String>,
) -> std::io::Result<File> {
    let src = std::fs::read_to_string(path)?;
    let src = parser::trivia::strip_leading_banner(&src);
    let tokens = parser::scan::scan(src);
    let items = parser::record::build_items_with_types(tokens, known);
    let mut items = parser::cond::fold_conditionals(items);
    parser::cond::resolve_conditionals(&mut items, defines);
    Ok(File {
        path: path.to_path_buf(),
        items,
    })
}
