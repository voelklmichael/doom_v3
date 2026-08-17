//! Rust code-generation backend, consuming the C front end's AST
//! (`parser::ast`). Unsafe transliteration, not idiomatic Rust: raw pointers
//! stay raw pointers, unions stay unions, global mutable state stays
//! `static mut`, functions are `unsafe extern "C" fn`. See
//! `/home/michael/.claude/plans/eager-marinating-axolotl.md` for the full
//! phased design.
//!
//! PR A: `types` (the `Type` -> Rust mapper) and `ident` (keyword escaping +
//! nested-record name synthesis).
//! PR B: `items` (per-`ItemKind` emission), consuming a single already-
//! parsed `ast::File`'s items directly.
//! PR C (this step): `module` (`.c`+`.h` merge/dedup + cross-module `use`
//! resolution) - no file I/O yet.

pub mod ident;
pub mod items;
pub mod module;
pub mod types;
