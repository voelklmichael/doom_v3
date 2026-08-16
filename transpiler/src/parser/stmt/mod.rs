//! Step 7: statement/expression parsing for function bodies. The C token
//! lexer (`lex.rs`) and a full expression grammar (`expr.rs`) exist so far;
//! statement-level parsing (if/while/for/switch/...) is a later step.
//! Nothing here is wired into `record.rs`/`ast.rs` yet, so
//! `ItemKind::FunctionDef`'s body stays fully opaque until then.

pub mod expr;
pub mod lex;
