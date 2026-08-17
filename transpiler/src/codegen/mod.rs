//! Rust code-generation backend, consuming the C front end's AST
//! (`parser::ast`). Unsafe transliteration, not idiomatic Rust: raw pointers
//! stay raw pointers, unions stay unions, global mutable state stays
//! `static mut`, functions are `unsafe extern "C" fn`. See
//! `/home/michael/.claude/plans/eager-marinating-axolotl.md` for the full
//! phased design.
//!
//! PR A (this step): `types` (the `Type` -> Rust mapper) and `ident`
//! (keyword escaping + nested-record name synthesis) only - no `ItemKind`
//! dispatch, no file I/O yet.

pub mod ident;
pub mod types;
