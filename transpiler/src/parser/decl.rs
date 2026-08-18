//! Step 4: variable declarations, both with an initializer, e.g.
//! `static const char rcsid[] = "...";` / `default_t defaults[] = { ... };`,
//! and without one, e.g. `extern int key_right;` / `static byte *wipe_scr;`.
//!
//! No operator grammar: a scalar initializer's own expression text
//! (`(30*TICRATE)`, `"a"`, `&foo`, ...) is always kept as raw text - no
//! attempt is made to parse casts/operators/precedence. A *braced*
//! initializer's top-level comma-separated shape is structured (see
//! `parse_braced_init`), which alone is enough to make `mobjinfo[]`/
//! `states[]`-style tables inspectable per-row/per-field instead of one
//! opaque string, because step 1 already found the matching `{`/`}`.

use super::ast::{
    ActiveBranch, Chunk, Init, InitCondBranch, InitCondGroup, RawToken, Storage, Type, TypedefDecl,
    VarDecl, render_tokens_no_comments, split_top_level,
};
use super::preproc::{self, Directive};

/// Parses a plain `;`-terminated statement with no top-level brace group,
/// e.g. `static const char rcsid[] = "...";` (initializer) or
/// `extern int key_right;` (no initializer - `initializer` comes back
/// `None`). Splits on any top-level comma into multiple `VarDecl`s sharing
/// one base type first (e.g. `static fixed_t m_x, m_y;` - `am_map.c`; C
/// forbids repeating the type word on a later declarator) - mirrors
/// `record.rs`'s identical `parse_field_group` fix for struct/union fields
/// and `stmt::decl::try_parse_decl_stmt`'s identical local-declaration
/// handling, via the same `parse_declarator_with_base`/
/// `parse_bare_declarator_suffix` pair. All-or-nothing: if any declarator in
/// the group fails to parse, the whole group is dropped rather than
/// guessing which subset is safe to keep - matches this function's
/// pre-existing single-declarator failure behavior (an unparseable
/// declaration was already silently dropped here, not changed by this fix).
/// Returns `None` only if the first declarator itself doesn't parse.
pub fn try_parse_var_flat(stmt: &str) -> Option<Vec<VarDecl>> {
    let s = stmt.trim();
    let s = s.strip_suffix(';').unwrap_or(s).trim();
    let mut pieces = split_top_level(s, ',').into_iter();
    let first = pieces.next()?;
    let (decl_part, init_part) = match split_top_level_eq(first.trim()) {
        Some((d, i)) => (d, Some(i.trim().to_string())),
        None => (first.trim(), None),
    };
    let (storage, base_ty, ty, name) = parse_declarator_with_base(decl_part.trim())?;
    let mut out = vec![VarDecl {
        storage: storage.clone(),
        ty,
        name,
        initializer: init_part.map(Init::Expr),
    }];
    for piece in pieces {
        let (decl_part, init_part) = match split_top_level_eq(piece.trim()) {
            Some((d, i)) => (d, Some(i.trim().to_string())),
            None => (piece.trim(), None),
        };
        let (ty, name) = parse_bare_declarator_suffix(decl_part.trim(), &base_ty)?;
        out.push(VarDecl {
            storage: storage.clone(),
            ty,
            name,
            initializer: init_part.map(Init::Expr),
        });
    }
    Some(out)
}

/// Parses the `TYPE NAME[dims] =` header preceding a brace-initializer
/// group, e.g. `mobjinfo_t mobjinfo[NUMMOBJTYPES] =` before `{ ... };`.
/// Caller (record.rs) has already confirmed `header` ends with `=`. `inner`
/// is the group's contents (excluding the `{`/`}` themselves). Always
/// produces `Some` initializer - a braced group with no leading `=` isn't
/// this function's shape at all (record.rs never calls it in that case).
pub fn try_parse_var_braced(header: &str, inner: &[RawToken]) -> Option<VarDecl> {
    let decl_part = header.trim().strip_suffix('=')?.trim();
    let (storage, ty, name) = parse_declarator(decl_part)?;
    Some(VarDecl {
        storage,
        ty,
        name,
        initializer: Some(Init::Braced(parse_braced_init(inner))),
    })
}

/// Splits a `{ ... }` initializer's contents on top-level `,` into its
/// elements, and folds any `#if`/`#ifdef`/`#ifndef`...`#endif` run found
/// between elements into a nested `Init::Conditional` (e.g. `m_misc.c`'s
/// `defaults[]` table, whose `#ifdef NORMALUNIX`/`#ifdef SNDSERV`/`#ifdef
/// LINUX` blocks gate whole rows - the directive lines sit inside this
/// initializer's own opaque token stream, so they never become their own
/// top-level `Item` and are invisible to `cond::fold_conditionals`). Each
/// non-directive element is either a nested `Init::Braced` sub-list (a
/// literal `{` at this level, e.g. one row of the table) - recursively
/// parsed the same way, so arbitrarily nested tables work for free - or a
/// scalar `Init::Expr` kept as raw text. Comments are dropped, same
/// reasoning as `render_tokens_no_comments` everywhere else: a trailing
/// `// comment, with a comma` must not fracture the split.
fn parse_braced_init(inner: &[RawToken]) -> Vec<Init> {
    fold_init_conditionals(collect_raw_elements(inner))
}

/// A flat piece of a braced initializer's contents, before conditional
/// folding: either a real element, or a preprocessor directive line found
/// between elements. `raw_text` on the directive variant is the directive
/// line's own exact text - only used as a fallback (see
/// `fold_init_conditionals`) if it turns out not to be part of a balanced
/// `#if...#endif` run, which never happens in the real corpus but keeps
/// this degrading gracefully rather than losing the line's content.
enum RawElem {
    Value(Init),
    Directive { raw_text: String, parsed: Directive },
}

fn collect_raw_elements(inner: &[RawToken]) -> Vec<RawElem> {
    let chunks = super::brace::group_braces(inner.to_vec());
    let mut elements = Vec::new();
    let mut pending = String::new();

    for chunk in chunks {
        match chunk {
            Chunk::Flat(toks) => {
                for run in split_on_preproc_lines(&toks) {
                    match run {
                        FlatRun::Code(sub_toks) => {
                            let text = render_tokens_no_comments(&sub_toks);
                            let combined = format!("{pending}{text}");
                            let complete =
                                combined.trim_end().ends_with(',') || combined.trim().is_empty();
                            let mut parts = split_top_level(&combined, ',');
                            let leftover = if complete { None } else { parts.pop() };
                            for part in &parts {
                                push_expr_element(part, &mut elements);
                            }
                            pending = leftover.unwrap_or_default();
                        }
                        FlatRun::Directive(raw_text) => {
                            // A directive always sits on its own line, so
                            // `pending` right before it is normally just
                            // whitespace (or empty) - flush it as a
                            // best-effort element first in case it isn't.
                            push_expr_element(&pending, &mut elements);
                            pending.clear();
                            let parsed = preproc::parse_directive(&raw_text);
                            elements.push(RawElem::Directive { raw_text, parsed });
                        }
                    }
                }
            }
            Chunk::Group { inner: ginner, .. } => {
                // A brace group at the top level of an initializer list is
                // always itself one element (a nested braced sub-list), not
                // something to split further at *this* level. Whatever's
                // pending right before it is normally just whitespace (the
                // separating `,`), but if it's something else - a shape
                // this parser doesn't otherwise expect here - it's kept as
                // its own best-effort scalar element rather than dropped.
                push_expr_element(&pending, &mut elements);
                pending.clear();
                elements.push(RawElem::Value(Init::Braced(parse_braced_init(&ginner))));
            }
        }
    }
    push_expr_element(&pending, &mut elements);
    elements
}

enum FlatRun {
    Code(Vec<RawToken>),
    Directive(String),
}

/// Splits a flat run of tokens into alternating code/directive-line pieces,
/// so multiple adjacent directive lines (e.g. `m_misc.c`'s `#endif` /
/// `#endif` / `#ifdef LINUX` sitting back-to-back with only whitespace
/// between them) each become their own `Directive` piece instead of gluing
/// into one unparseable blob.
fn split_on_preproc_lines(toks: &[RawToken]) -> Vec<FlatRun> {
    let mut out = Vec::new();
    let mut cur = Vec::new();
    for tok in toks {
        if let RawToken::PreprocLine(span) = tok {
            if !cur.is_empty() {
                out.push(FlatRun::Code(std::mem::take(&mut cur)));
            }
            out.push(FlatRun::Directive(span.text.clone()));
        } else {
            cur.push(tok.clone());
        }
    }
    if !cur.is_empty() {
        out.push(FlatRun::Code(cur));
    }
    out
}

fn push_expr_element(text: &str, elements: &mut Vec<RawElem>) {
    let text = text.trim();
    if !text.is_empty() {
        elements.push(RawElem::Value(Init::Expr(text.to_string())));
    }
}

/// What an in-progress `InitCondBuilder` is currently accumulating a body
/// for - mirrors `cond::Cur`.
enum InitCur {
    Branch(Directive),
    Else,
}

/// Mirrors `cond::Builder`, but accumulates `Init` elements (no trivia/raw
/// bookkeeping needed - see `InitCondBranch`'s doc comment).
struct InitCondBuilder {
    branches: Vec<InitCondBranch>,
    cur: InitCur,
    cur_body: Vec<Init>,
    else_body: Option<Vec<Init>>,
}

impl InitCondBuilder {
    fn new(directive: Directive) -> Self {
        InitCondBuilder {
            branches: Vec::new(),
            cur: InitCur::Branch(directive),
            cur_body: Vec::new(),
            else_body: None,
        }
    }

    fn push_body(&mut self, init: Init) {
        self.cur_body.push(init);
    }

    fn advance(&mut self, next: InitCur) {
        let body = std::mem::take(&mut self.cur_body);
        match std::mem::replace(&mut self.cur, next) {
            InitCur::Branch(directive) => self.branches.push(InitCondBranch { directive, body }),
            InitCur::Else => self.else_body = Some(body),
        }
    }

    fn finish(mut self) -> InitCondGroup {
        let body = std::mem::take(&mut self.cur_body);
        match std::mem::replace(&mut self.cur, InitCur::Else) {
            InitCur::Branch(directive) => self.branches.push(InitCondBranch { directive, body }),
            InitCur::Else => self.else_body = Some(body),
        }
        InitCondGroup {
            branches: self.branches,
            else_body: self.else_body,
            active: ActiveBranch::Unknown,
        }
    }
}

/// Concatenates every element across all of a group's branches/else body,
/// dropping the conditional structure itself. Used only for an unterminated
/// `#if` at EOF (see `fold_init_conditionals`) - never exercised by the
/// real corpus, which is fully balanced, but keeps every real element
/// reachable rather than losing it under a dropped opener.
fn flatten_group(group: InitCondGroup) -> Vec<Init> {
    let mut out = Vec::new();
    for branch in group.branches {
        out.extend(branch.body);
    }
    if let Some(else_body) = group.else_body {
        out.extend(else_body);
    }
    out
}

/// Folds a flat `RawElem` sequence into `Init`s, matching `#if`/`#ifdef`/
/// `#ifndef`...`#elif`...`#else`...`#endif` runs into `Init::Conditional` -
/// the same single left-to-right stack-based algorithm as
/// `cond::fold_conditionals`, specialized for initializer elements (no
/// trivia/raw bytes to carry, since `Init` isn't part of the file's
/// round-trip contract). A stray `#elif`/`#else`/`#endif` with no open
/// group, or any other directive shape (e.g. a stray `#define`), degrades
/// to a scalar `Init::Expr` holding the directive's own raw text rather
/// than being dropped - matching this parser's usual "never lose data"
/// fallback stance, even though this path isn't exercised by the real
/// corpus (which is fully balanced).
fn fold_init_conditionals(elements: Vec<RawElem>) -> Vec<Init> {
    let mut stack: Vec<InitCondBuilder> = Vec::new();
    let mut top: Vec<Init> = Vec::new();

    for elem in elements {
        match elem {
            RawElem::Value(v) => push_current(&mut stack, &mut top, v),
            RawElem::Directive { raw_text, parsed } => match parsed {
                Directive::If { .. } | Directive::IfDef { .. } => {
                    stack.push(InitCondBuilder::new(parsed));
                }
                Directive::Elif { .. } if !stack.is_empty() => {
                    stack.last_mut().unwrap().advance(InitCur::Branch(parsed));
                }
                Directive::Else if !stack.is_empty() => {
                    stack.last_mut().unwrap().advance(InitCur::Else);
                }
                Directive::Endif if !stack.is_empty() => {
                    let group = stack.pop().unwrap().finish();
                    push_current(&mut stack, &mut top, Init::Conditional(group));
                }
                _ => push_current(&mut stack, &mut top, Init::Expr(raw_text)),
            },
        }
    }

    // Unterminated `#if` at EOF: flatten rather than lose the elements
    // accumulated inside it (see `flatten_group`).
    while let Some(builder) = stack.pop() {
        for v in flatten_group(builder.finish()) {
            push_current(&mut stack, &mut top, v);
        }
    }

    top
}

fn push_current(stack: &mut [InitCondBuilder], top: &mut Vec<Init>, v: Init) {
    match stack.last_mut() {
        Some(b) => b.push_body(v),
        None => top.push(v),
    }
}

/// Parses a plain, brace-free `typedef TYPE NAME;`, e.g.
/// `typedef bool boolean;` or `typedef unsigned char byte;`.
/// `typedef struct/union/enum { ... } NAME;` goes through record.rs instead
/// since it involves a brace group.
pub fn try_parse_typedef_flat(stmt: &str) -> Option<TypedefDecl> {
    let s = stmt.trim().strip_suffix(';')?.trim();
    let rest = s.strip_prefix("typedef")?;
    if rest.starts_with(|c: char| c.is_alphanumeric() || c == '_') {
        return None; // e.g. "typedefFoo" is not the "typedef" keyword
    }
    // Storage-class keywords on a typedef (e.g. `typedef const int foo;`)
    // are discarded here, same as before `Type` existed - `TypedefDecl` has
    // nowhere to put them and no real typedef in this corpus uses one.
    let (_, underlying, name) = parse_declarator(rest.trim())?;
    Some(TypedefDecl { underlying, name })
}

/// Finds the first top-level `=` in a brace-free expression, skipping `==`,
/// `!=`, `<=`, `>=` and compound assignment operators (`+=`, `-=`, ...).
fn split_top_level_eq(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'=' {
            let prev = if i > 0 { Some(bytes[i - 1]) } else { None };
            let next = bytes.get(i + 1).copied();
            let compound_prev = matches!(
                prev,
                Some(
                    b'=' | b'!'
                        | b'<'
                        | b'>'
                        | b'+'
                        | b'-'
                        | b'*'
                        | b'/'
                        | b'%'
                        | b'&'
                        | b'|'
                        | b'^'
                )
            );
            if next == Some(b'=') || compound_prev {
                i += 1;
                continue;
            }
            return Some((&s[..i], &s[i + 1..]));
        }
        i += 1;
    }
    None
}

/// Heuristic C declarator parser: `[storage...] TYPE [*]NAME (['[' dim ']'])*`,
/// or the function-pointer shape `[storage...] TYPE (*NAME) (PARAMS)`. Not a
/// full C grammar - good enough to pull apart the shapes actually used in
/// the target files without parsing expressions. Array dims and pointer
/// stars are folded into the returned `Type` (`Array`/`Pointer` wrapping a
/// base `Named`) rather than returned as separate fields.
pub(crate) fn parse_declarator(s: &str) -> Option<(Vec<Storage>, Type, String)> {
    let (storage, _base_ty, ty, name) = parse_declarator_with_base(s)?;
    Some((storage, ty, name))
}

/// Same as `parse_declarator`, but additionally returns the declarator's
/// base type (storage/type words only, *before* this declarator's own
/// `*`/`[]` decoration is applied). Added so `stmt::decl`'s multi-declarator
/// local parsing (`int *a, b[4];` - C forbids repeating the type word for
/// later declarators) can reuse one shared base type across sibling
/// declarators via `parse_bare_declarator_suffix`, instead of re-deriving it
/// per declarator. For the function-pointer shape, there's no meaningful
/// "undecorated base" separate from the whole return type, so `base_ty`
/// there is just a clone of `ty` itself.
pub(crate) fn parse_declarator_with_base(s: &str) -> Option<(Vec<Storage>, Type, Type, String)> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let mut base = s;
    let mut dims: Vec<Option<String>> = Vec::new();
    while base.ends_with(']') {
        let open = base.rfind('[')?;
        let dim = base[open + 1..base.len() - 1].trim();
        dims.push(if dim.is_empty() {
            None
        } else {
            Some(dim.to_string())
        });
        base = base[..open].trim_end();
    }
    dims.reverse();

    // A declarator never legitimately ends in `)` other than the
    // function-pointer shape (`TYPE (*NAME)(PARAMS)`, or its array variant
    // `TYPE (*NAME[N])(PARAMS)`) - falling through to the plain
    // whitespace-token parsing below for such text would misread e.g.
    // `boolean (*traverser_t) (intercept_t *in)` as a garbage `*`-prefixed
    // name (`in)`) instead of correctly failing or extracting `traverser_t`.
    // So this shape is tried first, and if it doesn't match, parsing fails
    // outright rather than falling through. The `while base.ends_with(']')`
    // loop above never fires for this shape (a fn-pointer declarator ends in
    // `)`, not `]`), so `dims` is always empty here - the array dims, if
    // any, live *inside* the name parens instead and are already wrapped
    // into the `Type` `parse_fnptr_declarator` returns.
    if base.ends_with(')') {
        let (storage, ty, name) = parse_fnptr_declarator(base)?;
        return Some((storage, ty.clone(), ty, name));
    }

    let tokens: Vec<&str> = base.split_whitespace().collect();
    let last = *tokens.last()?;
    let star_count = last.len() - last.trim_start_matches('*').len();
    let name = last.trim_start_matches('*').to_string();
    let first_char = name.chars().next()?;
    if !(first_char.is_alphabetic() || first_char == '_') {
        return None;
    }

    let mut storage = Vec::new();
    let mut ty_parts: Vec<&str> = Vec::new();
    for t in &tokens[..tokens.len() - 1] {
        match Storage::from_keyword(t) {
            Some(kw) if ty_parts.is_empty() => storage.push(kw),
            _ => ty_parts.push(t),
        }
    }
    let mut ty_text = ty_parts.join(" ");
    if ty_text.is_empty() {
        // Real corpus case: am_map.c's `register outcode1 = 0;`/
        // `register outcode2 = 0;` - pre-ANSI-C's "implicit int" rule (a
        // declaration with a storage-class specifier but no type specifier
        // defaults to `int`). Only apply this when a real storage keyword
        // was actually found (`storage` non-empty) - otherwise this input
        // never looked like a declaration attempt in the first place (e.g.
        // a bare identifier `foo;` on its own), and failing here as before
        // is still correct.
        if storage.is_empty() {
            return None;
        }
        ty_text = "int".to_string();
    }
    // A bare `struct`/`union`/`enum` keyword with nothing else as the type
    // text means the "name" token we just grabbed (`last`) is actually the
    // record's tag, not a separate declarator name - i.e. this whole input
    // is a tagless-variable-free forward declaration like `struct line_s;`
    // (r_defs.h), not a variable of type "struct" named after its own tag.
    // Must fail here rather than accept it, or `line_s` gets misclassified
    // as `ItemKind::Var` with type `Named("struct")` (renders as the
    // keyword-escaped `struct_`, a nonexistent type) instead of falling
    // through classification to `ItemKind::Raw` as intended.
    if matches!(ty_text.as_str(), "struct" | "union" | "enum") {
        return None;
    }
    let base_ty = parse_type_text(&ty_text);
    let mut ty = base_ty.clone();
    for _ in 0..star_count {
        ty = Type::Pointer(Box::new(ty));
    }
    let ty = wrap_array_dims(ty, &dims);
    Some((storage, base_ty, ty, name))
}

/// Parses a later comma-separated declarator in a multi-declarator local
/// (`int *a, b[4];` - this handles the `b[4]` piece, given `base_ty` already
/// derived from `a`'s own `parse_declarator_with_base` call), applying its
/// own `*`/`[]` decoration on top of the shared base type. Doesn't handle
/// the function-pointer-declarator shape (`(*NAME)(PARAMS)`) for a later
/// piece - unconfirmed in the corpus for locals, so returns `None` (caller
/// degrades to a `Raw` statement) rather than guessing.
pub(crate) fn parse_bare_declarator_suffix(s: &str, base_ty: &Type) -> Option<(Type, String)> {
    let mut base = s.trim();
    if base.is_empty() || base.ends_with(')') {
        return None;
    }
    let mut dims: Vec<Option<String>> = Vec::new();
    while base.ends_with(']') {
        let open = base.rfind('[')?;
        let dim = base[open + 1..base.len() - 1].trim();
        dims.push(if dim.is_empty() {
            None
        } else {
            Some(dim.to_string())
        });
        base = base[..open].trim_end();
    }
    dims.reverse();

    let star_count = base.len() - base.trim_start_matches('*').len();
    let name = base.trim_start_matches('*').to_string();
    let first_char = name.chars().next()?;
    if !(first_char.is_alphabetic() || first_char == '_') {
        return None;
    }

    let mut ty = base_ty.clone();
    for _ in 0..star_count {
        ty = Type::Pointer(Box::new(ty));
    }
    let ty = wrap_array_dims(ty, &dims);
    Some((ty, name))
}

/// Recognizes the function-pointer declarator shape `RETTYPE (*NAME) (PARAMS)`,
/// e.g. `void (*actionf_v)()` or `boolean (*traverser_t) (intercept_t *in)`,
/// as well as its array-of-function-pointers variant
/// `RETTYPE (*NAME[N])(PARAMS)`, e.g. `int (*wipes[])(int, int, int)`
/// (`f_wipe.c`). Returns `(storage, ty, name)` with `ty` a
/// `Type::FunctionPointer` (wrapped in `Type::Array` for the `(*NAME[N])`
/// variant) so it composes with the plain `ty`/`name` fields
/// `VarDecl`/`TypedefDecl`/`Field` already use. Each parameter in
/// `PARAMS` is itself parsed via `parse_declarator` (falling back to a bare
/// `parse_type_text` read if it has no name, e.g. `traverser_t`'s
/// `intercept_t *in` *does* have a name "in" that gets discarded here -
/// `Type::FunctionPointer::params` is types only, see its doc comment).
fn parse_fnptr_declarator(s: &str) -> Option<(Vec<Storage>, Type, String)> {
    let params_open = matching_open_paren(s)?;
    let params_text = s[params_open + 1..s.len() - 1].trim();
    let before_params = s[..params_open].trim();
    if !before_params.ends_with(')') {
        return None;
    }
    let name_open = matching_open_paren(before_params)?;
    let name_group = before_params[name_open + 1..before_params.len() - 1].trim();
    let mut name_base = name_group.strip_prefix('*')?.trim();
    let mut dims: Vec<Option<String>> = Vec::new();
    while name_base.ends_with(']') {
        let open = name_base.rfind('[')?;
        let dim = name_base[open + 1..name_base.len() - 1].trim();
        dims.push(if dim.is_empty() {
            None
        } else {
            Some(dim.to_string())
        });
        name_base = name_base[..open].trim_end();
    }
    dims.reverse();
    let name = name_base.to_string();
    let first_char = name.chars().next()?;
    if !(first_char.is_alphabetic() || first_char == '_')
        || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
    {
        return None;
    }

    let ret_raw = before_params[..name_open].trim();
    let tokens: Vec<&str> = ret_raw.split_whitespace().collect();
    let mut storage = Vec::new();
    let mut ty_parts = Vec::new();
    for t in &tokens {
        match Storage::from_keyword(t) {
            Some(kw) if ty_parts.is_empty() => storage.push(kw),
            _ => ty_parts.push(*t),
        }
    }
    let ret_text = ty_parts.join(" ");
    if ret_text.is_empty() {
        return None;
    }
    let ret = parse_type_text(&ret_text);
    let params = parse_fnptr_params(params_text);
    let ty = wrap_array_dims(
        Type::FunctionPointer {
            ret: Box::new(ret),
            params,
        },
        &dims,
    );
    Some((storage, ty, name))
}

/// Splits a function-pointer type's own parameter-list text (e.g. `"int,
/// int, int"` or `"intercept_t *in"`) on top-level `,` into `Type`s. `()`
/// and `(void)` both mean no parameters. Each piece is tried as a named
/// declarator first (discarding the name - see `Type::FunctionPointer`'s
/// doc comment) and falls back to a bare-type read if it has none.
fn parse_fnptr_params(params_text: &str) -> Vec<Type> {
    let trimmed = params_text.trim();
    if trimmed.is_empty() || trimmed == "void" {
        return Vec::new();
    }
    split_top_level(trimmed, ',')
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| match parse_declarator(p) {
            Some((_, ty, _)) => ty,
            None => parse_type_text(p),
        })
        .collect()
}

/// Reads a bare type-text with no declarator name attached (e.g. a
/// function-pointer's return type, or an anonymous parameter's type) into a
/// `Type`, by counting any trailing `*` characters as pointer depth. Works
/// whether the star is glued to the base (`"char*"`) or separated by
/// whitespace (`"char *"`, `"char * *"`) - both this codebase's styles.
pub(crate) fn parse_type_text(text: &str) -> Type {
    let text = text.trim();
    let base = text.trim_end_matches(['*', ' ']);
    let star_count = text[base.len()..].matches('*').count();
    let mut ty = Type::Named(base.to_string());
    for _ in 0..star_count {
        ty = Type::Pointer(Box::new(ty));
    }
    ty
}

/// Wraps `base` in one `Type::Array` per entry of `dims`, outermost bracket
/// first - see `Type::Array`'s doc comment for why this means iterating
/// `dims` in *reverse*.
pub(crate) fn wrap_array_dims(base: Type, dims: &[Option<String>]) -> Type {
    let mut ty = base;
    for dim in dims.iter().rev() {
        ty = Type::Array(Box::new(ty), dim.clone());
    }
    ty
}

/// Finds the index of the `(` matching the final `)` of `s`, or `None` if
/// `s` doesn't end in `)`.
fn matching_open_paren(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    if bytes.last() != Some(&b')') {
        return None;
    }
    let mut depth = 0i32;
    for (i, &b) in bytes.iter().enumerate().rev() {
        match b {
            b')' => depth += 1,
            b'(' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests;
