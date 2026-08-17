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
//! PR C: `module` (`.c`+`.h` merge/dedup + cross-module `use` resolution).
//! PR D (this step): `write` (filesystem + `rustfmt` invocation) - see
//! `main.rs`'s `--emit-rust` flag for the full pipeline wiring.

pub mod expr;
pub mod ident;
pub mod items;
pub mod macros;
pub mod module;
pub mod types;
pub mod write;
