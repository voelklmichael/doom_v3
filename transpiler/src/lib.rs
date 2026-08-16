pub mod parser;

use parser::ast::File;
use parser::stmt::expr::KnownTypeNames;
use std::path::Path;

/// Reads and parses a single C source file into its AST, with no corpus
/// type-name context - function bodies still get fully parsed, just with
/// less precise cast-vs-parenthesized-expression disambiguation inside them
/// (see `parser::record::build_items`'s doc comment). Real per-file output
/// should use `parse_file_with_types` instead, fed a `KnownTypeNames` from
/// `parser::corpus::compute_known_type_names`.
pub fn parse_file(path: &Path) -> std::io::Result<File> {
    parse_file_with_types(path, &KnownTypeNames::new())
}

/// Same as `parse_file`, but threads `known` through to every function
/// body's cast-vs-parenthesized-expression disambiguation. If the file
/// begins with the id Software license/RCS banner comment, that banner is
/// dropped before parsing - see `parser::trivia::strip_leading_banner`.
pub fn parse_file_with_types(path: &Path, known: &KnownTypeNames) -> std::io::Result<File> {
    let src = std::fs::read_to_string(path)?;
    let src = parser::trivia::strip_leading_banner(&src);
    let tokens = parser::scan::scan(src);
    let items = parser::record::build_items_with_types(tokens, known);
    let items = parser::cond::fold_conditionals(items);
    Ok(File {
        path: path.to_path_buf(),
        items,
    })
}
