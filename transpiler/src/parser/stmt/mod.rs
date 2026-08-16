//! Step 7: statement/expression parsing for function bodies. Only the C
//! token lexer (`lex.rs`) exists so far - the statement/expression grammar
//! itself is a later step. Nothing here is wired into `record.rs`/`ast.rs`
//! yet; `ItemKind::FunctionDef`'s body stays fully opaque until then.

pub mod lex;
