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

fn parse_define(rest: &str) -> Directive {
    let rest = rest.trim_start();
    let name_end = rest
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(rest.len());
    let name = &rest[..name_end];
    let after_name = &rest[name_end..];
    // No space between name and '(' is what makes this function-like, per C rules.
    if let Some(paren_rest) = after_name.strip_prefix('(') {
        if let Some(close) = paren_rest.find(')') {
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
    }
    Directive::DefineObject {
        name: name.to_string(),
        value: after_name.trim().to_string(),
    }
}

#[cfg(test)]
mod tests;
