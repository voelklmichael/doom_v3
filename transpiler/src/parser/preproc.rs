//! Step 2: turns a `PreprocLine` token's raw text into a structured
//! `Directive`. Directives are kept flat (not nested into an `#if`-branch
//! tree) - `m_misc.c`'s `defaults[]` table has `#ifdef` blocks sitting mid
//! initializer-list, so branch-scoping can't assume `#if`/`#endif` cleanly
//! bracket whole declarations. That's left for a later step.

#[derive(Debug, Clone, serde::Serialize)]
pub enum Directive {
    Include {
        path: String,
        angled: bool,
    },
    DefineObject {
        name: String,
        value: String,
    },
    DefineFunction {
        name: String,
        params: Vec<String>,
        body: String,
    },
    Undef {
        name: String,
    },
    /// `#ifdef` (negate = false) or `#ifndef` (negate = true).
    IfDef {
        name: String,
        negate: bool,
    },
    If {
        expr: String,
    },
    Elif {
        expr: String,
    },
    Else,
    Endif,
    Pragma(String),
    Error(String),
    /// Anything not recognized above, kept as raw directive text.
    Other(String),
}

pub fn parse_directive(raw: &str) -> Directive {
    let trimmed = raw.trim_end_matches(['\n', '\r']);
    let after_hash = trimmed
        .trim_start()
        .strip_prefix('#')
        .unwrap_or(trimmed)
        .trim_start();
    let (kw, rest) = split_keyword(after_hash);
    let rest = rest.trim();
    match kw {
        "include" => parse_include(rest, trimmed),
        "define" => parse_define(rest),
        "undef" => Directive::Undef {
            name: rest.to_string(),
        },
        "ifdef" => Directive::IfDef {
            name: rest.to_string(),
            negate: false,
        },
        "ifndef" => Directive::IfDef {
            name: rest.to_string(),
            negate: true,
        },
        "if" => Directive::If {
            expr: rest.to_string(),
        },
        "elif" => Directive::Elif {
            expr: rest.to_string(),
        },
        "else" => Directive::Else,
        "endif" => Directive::Endif,
        "pragma" => Directive::Pragma(rest.to_string()),
        "error" => Directive::Error(rest.to_string()),
        _ => Directive::Other(trimmed.to_string()),
    }
}

fn split_keyword(s: &str) -> (&str, &str) {
    let s = s.trim_start();
    let end = s
        .find(|c: char| c.is_whitespace() || c == '(')
        .unwrap_or(s.len());
    (&s[..end], &s[end..])
}

fn parse_include(rest: &str, whole: &str) -> Directive {
    if let Some(p) = rest
        .strip_prefix('<')
        .and_then(|s| s.rsplit_once('>').map(|(p, _)| p))
    {
        Directive::Include {
            path: p.to_string(),
            angled: true,
        }
    } else if let Some(p) = rest
        .strip_prefix('"')
        .and_then(|s| s.rsplit_once('"').map(|(p, _)| p))
    {
        Directive::Include {
            path: p.to_string(),
            angled: false,
        }
    } else {
        Directive::Other(whole.to_string())
    }
}

/// The result of evaluating an `#ifdef`/`#ifndef`/`#if`/`#elif` condition
/// against a known set of `#define`s. `Unknown` is a real, first-class
/// outcome (not an error) - this evaluator only attempts the shapes
/// actually seen in the target corpus (bare `0`/`1` literals, and a bare
/// identifier looked up in `defines`), never a general C/preprocessor
/// expression grammar. Anything else degrades to `Unknown` rather than
/// guessing, matching this parser's usual stance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tri {
    True,
    False,
    Unknown,
}

/// Evaluates an `#ifdef NAME` (`negate = false`) or `#ifndef NAME`
/// (`negate = true`) against `defines` - always decidable (`defines`
/// either has an entry for `name` or it doesn't), never `Unknown`.
pub fn eval_ifdef(
    name: &str,
    negate: bool,
    defines: &std::collections::HashMap<String, String>,
) -> Tri {
    let is_defined = defines.contains_key(name.trim());
    if is_defined != negate {
        Tri::True
    } else {
        Tri::False
    }
}

/// Evaluates an `#if`/`#elif` expression's raw text against `defines`.
/// Handles exactly the shapes seen in the real corpus: a bare integer
/// literal (`"0"` -> `False`, any other bare integer -> `True`), or a bare
/// C identifier, looked up in `defines` and recursed on its value if
/// defined (e.g. `SNDSERV` -> `"1"` -> `True`) - an *undefined* identifier
/// evaluates to `False`, matching real C: a name with no macro definition
/// left over after macro expansion in an `#if` expression is replaced with
/// `0`. This is what makes `#elif SNDINTR` (real corpus text, `i_sound.c`)
/// resolve correctly: `SNDINTR` is never `#define`'d anywhere in this
/// corpus, so the bare identifier is `False`. Anything else (a real
/// expression with operators, `defined(...)`, comparisons, ...) - none of
/// which occur anywhere in this corpus - is `Unknown`, not guessed at.
pub fn eval_if_expr(expr: &str, defines: &std::collections::HashMap<String, String>) -> Tri {
    let e = strip_trailing_line_comment(expr).trim();
    if e.is_empty() {
        return Tri::Unknown;
    }
    if e.chars().all(|c| c.is_ascii_digit()) {
        return if e == "0" { Tri::False } else { Tri::True };
    }
    if is_c_identifier(e) {
        return match defines.get(e) {
            None => Tri::False,
            Some(value) => eval_if_expr(value, defines),
        };
    }
    Tri::Unknown
}

/// Strips a trailing `//...` line comment (and any whitespace before it) -
/// the raw `Directive::If`/`Elif` expression text isn't comment-stripped by
/// `parse_directive` itself (e.g. `"0 // UNUSED - debug?"`, tab included in
/// the real corpus text), since that's a concern specific to evaluation,
/// not directive parsing.
fn strip_trailing_line_comment(s: &str) -> &str {
    match s.find("//") {
        Some(i) => s[..i].trim_end(),
        None => s,
    }
}

fn is_c_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_define(rest: &str) -> Directive {
    let rest = rest.trim_start();
    let name_end = rest
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(rest.len());
    let name = &rest[..name_end];
    let after_name = &rest[name_end..];
    // No space between name and '(' is what makes this function-like, per C rules.
    if let Some(paren_rest) = after_name.strip_prefix('(')
        && let Some(close) = paren_rest.find(')')
    {
        let params_raw = &paren_rest[..close];
        let params: Vec<String> = if params_raw.trim().is_empty() {
            Vec::new()
        } else {
            params_raw
                .split(',')
                .map(|p| p.trim().to_string())
                .collect()
        };
        let body = paren_rest[close + 1..].trim().to_string();
        return Directive::DefineFunction {
            name: name.to_string(),
            params,
            body,
        };
    }
    Directive::DefineObject {
        name: name.to_string(),
        value: after_name.trim().to_string(),
    }
}

#[cfg(test)]
mod tests;
