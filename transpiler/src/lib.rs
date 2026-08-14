pub mod parser;

use parser::ast::File;
use std::path::Path;

/// Reads and parses a single C source file into its AST.
pub fn parse_file(path: &Path) -> std::io::Result<File> {
    let src = std::fs::read_to_string(path)?;
    let tokens = parser::scan::scan(&src);
    let items = parser::record::build_items(tokens);
    Ok(File { path: path.to_path_buf(), items })
}
