//! Identifier escaping and name synthesis for the Rust codegen backend.
//!
//! C identifiers are used verbatim as Rust identifiers wherever possible
//! (this is a transliteration, not an idiomatic rewrite - no forced
//! snake_case/PascalCase), but some real C identifiers in this corpus
//! collide with Rust reserved words (`type`, `box`, `move`, `where`, ...).
//! `ident()` is a single, always-applied, total function handling this: a
//! uniform suffix-rename (`type` -> `type_`) for every collision, rather
//! than raw-identifier syntax (`r#type`) - `r#` can't cover `self`/`Self`/
//! `super`/`crate`/`extern` at all (they're path-resolution keywords, not
//! ordinary reserved words), so a uniform rule avoids a second, partial
//! escaping mechanism existing alongside it. Same convention `bindgen` uses
//! for this exact C-to-Rust identifier problem.

/// Every strict, reserved, and edition-gated Rust keyword, plus the
/// `self`/`Self`/`super`/`crate`/`extern` path-resolution keywords that
/// `r#` cannot escape. Deliberately over-inclusive (e.g. `gen`/`try` are
/// only reserved in newer editions) - a total, always-applied function
/// doesn't need to track edition boundaries precisely.
const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield", "gen",
];

/// Escapes `name` for use as a Rust identifier: appends `_` if it collides
/// with a Rust keyword, otherwise returns it unchanged. Total - always safe
/// to call, even on a name that doesn't collide.
pub fn ident(name: &str) -> String {
    if KEYWORDS.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

/// Synthesizes a name for an anonymous nested struct/union field (C has no
/// named type for it - e.g. `p_local.h`'s `intercept_t`'s union field `d`
/// becomes `intercept_t_d`). Applied to the *parts*, not the whole - callers
/// should still run the result through `ident()` if it could itself collide
/// (vanishingly unlikely for a synthesized compound name, but `ident` is
/// cheap and total, so there's no reason not to).
pub fn synthesize_nested_name(parent: &str, field: &str) -> String {
    format!("{parent}_{field}")
}

#[cfg(test)]
mod tests;
