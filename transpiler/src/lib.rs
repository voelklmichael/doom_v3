pub mod parser;

use parser::ast::File;
use std::path::Path;

/// Reads and parses a single C source file into its AST. If the file begins
/// with the id Software license/RCS banner comment, that banner is dropped
/// before parsing - see `parser::trivia::strip_leading_banner`.
pub fn parse_file(path: &Path) -> std::io::Result<File> {
    let src = std::fs::read_to_string(path)?;
    let src = parser::trivia::strip_leading_banner(&src);
    let tokens = parser::scan::scan(src);
    let items = parser::record::build_items(tokens);
    let items = parser::cond::fold_conditionals(items);
    Ok(File {
        path: path.to_path_buf(),
        items,
    })
}
