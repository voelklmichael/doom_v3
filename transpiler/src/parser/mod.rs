//! A hand-rolled, staged front end for the linuxdoom-1.10 C sources.
//! Produces one AST per file, preserving both C declarations and
//! preprocessor macros; function bodies are kept as opaque, unparsed text.
//! See each submodule for the step it implements.

pub mod ast;
pub mod brace;
pub mod cond;
pub mod corpus;
pub mod decl;
pub mod preproc;
pub mod record;
pub mod scan;
pub mod stmt;
pub mod trivia;
