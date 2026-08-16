//! Step 6: folds flat, sibling `#if`/`#ifdef`/`#ifndef`/`#elif`/`#else`/
//! `#endif` items (as produced by `record::build_items`) into a nested
//! `ItemKind::Conditional` tree.
//!
//! A single left-to-right pass over a stack of in-progress groups builds the
//! whole tree - nesting falls out naturally because a group that finishes
//! gets appended straight into whatever group is now on top of the stack (or
//! the top-level output, if the stack is empty), so no separate recursive
//! call is needed.
//!
//! Only directives that already appear as their own sibling `Item` are seen
//! here. A `#ifdef` sitting mid-array-initializer (e.g. `m_misc.c`'s
//! `defaults[]`) never becomes a sibling `Item` in the first place - it's
//! embedded, opaque, inside that item's `raw` - so it's invisible to this
//! pass, same as before this step existed.
//!
//! Malformed input (a stray `#elif`/`#else`/`#endif` with no open group, or
//! an unterminated `#if` at EOF) degrades gracefully: the offending
//! directives are left as plain flat items rather than folded, matching the
//! rest of the parser's "never lose bytes" fallback behavior. The real
//! corpus (all 124 files) is fully balanced, so this path is a safety net,
//! not something exercised by real input.

use super::ast::{ActiveBranch, CondBranch, CondGroup, File, Item, ItemKind, Trivia, render_items};
use super::preproc::{self, Directive, Tri};
use std::collections::HashMap;

/// What a `Builder` is currently accumulating a body for.
enum Cur {
    /// An `#if`/`#ifdef`/`#ifndef`/`#elif` branch, with its opening directive.
    Branch(Directive),
    /// The `#else` body.
    Else,
}

struct Builder {
    branches: Vec<CondBranch>,
    cur: Cur,
    cur_body: Vec<(Item, Trivia)>,
    else_body: Option<Vec<(Item, Trivia)>>,
    /// Every item consumed by this group so far, in original order -
    /// rebuilds the group's `raw` text, and flattens it back to plain items
    /// if the group turns out to be unterminated.
    raw_items: Vec<(Item, Trivia)>,
}

impl Builder {
    fn new(directive: Directive, opener: (Item, Trivia)) -> Self {
        Builder {
            branches: Vec::new(),
            cur: Cur::Branch(directive),
            cur_body: Vec::new(),
            else_body: None,
            raw_items: vec![opener],
        }
    }

    /// A normal content item: goes into the body currently being filled.
    fn push_body(&mut self, item: Item, trivia: Trivia) {
        self.raw_items.push((item.clone(), trivia.clone()));
        self.cur_body.push((item, trivia));
    }

    /// `#elif`/`#else`: close the branch/else body being filled and open the
    /// next one. `marker` is the elif/else item itself, recorded only for
    /// `raw` reconstruction - it's never part of any body.
    fn advance(&mut self, next: Cur, marker: (Item, Trivia)) {
        let body = std::mem::take(&mut self.cur_body);
        match std::mem::replace(&mut self.cur, next) {
            Cur::Branch(directive) => self.branches.push(CondBranch { directive, body }),
            Cur::Else => self.else_body = Some(body),
        }
        self.raw_items.push(marker);
    }

    /// `#endif`: close out the group, returning its structured form plus the
    /// flat, ordered item list it was built from (for `raw`).
    fn finish(mut self, marker: (Item, Trivia)) -> (CondGroup, Vec<(Item, Trivia)>) {
        let body = std::mem::take(&mut self.cur_body);
        match std::mem::replace(&mut self.cur, Cur::Else) {
            Cur::Branch(directive) => self.branches.push(CondBranch { directive, body }),
            Cur::Else => self.else_body = Some(body),
        }
        self.raw_items.push(marker);
        (
            CondGroup {
                branches: self.branches,
                else_body: self.else_body,
                active: ActiveBranch::Unknown,
            },
            self.raw_items,
        )
    }
}

pub fn fold_conditionals(items: Vec<(Item, Trivia)>) -> Vec<(Item, Trivia)> {
    let mut stack: Vec<Builder> = Vec::new();
    let mut top: Vec<(Item, Trivia)> = Vec::new();

    for (item, trivia) in items {
        match &item.kind {
            ItemKind::Preproc(Directive::If { .. })
            | ItemKind::Preproc(Directive::IfDef { .. }) => {
                let directive = directive_of(&item.kind).clone();
                stack.push(Builder::new(directive, (item, trivia)));
            }
            ItemKind::Preproc(Directive::Elif { .. }) if !stack.is_empty() => {
                let directive = directive_of(&item.kind).clone();
                stack
                    .last_mut()
                    .unwrap()
                    .advance(Cur::Branch(directive), (item, trivia));
            }
            ItemKind::Preproc(Directive::Else) if !stack.is_empty() => {
                stack.last_mut().unwrap().advance(Cur::Else, (item, trivia));
            }
            ItemKind::Preproc(Directive::Endif) if !stack.is_empty() => {
                let builder = stack.pop().unwrap();
                let (group, raw_items) = builder.finish((item, trivia));
                // The opener's own leading trivia (e.g. a comment right
                // before `#ifdef`) becomes this synthesized item's trivia,
                // so it must NOT also be rendered as part of `raw` -
                // otherwise it'd be emitted twice on round-trip.
                let group_trivia = raw_items[0].1.clone();
                let mut raw = raw_items[0].0.raw.clone();
                raw.push_str(&render_items(&raw_items[1..]));
                let group_item = Item {
                    kind: ItemKind::Conditional(group),
                    raw,
                };
                push_current(&mut stack, &mut top, (group_item, group_trivia));
            }
            _ => push_current(&mut stack, &mut top, (item, trivia)),
        }
    }

    // Unterminated conditionals at EOF: flatten defensively rather than
    // losing them. Each pop's `raw_items` is already flat and in order, so
    // re-feeding it through `push_current` un-nests it one level; repeating
    // until the stack drains handles arbitrarily deep unterminated nesting.
    while let Some(builder) = stack.pop() {
        for pair in builder.raw_items {
            push_current(&mut stack, &mut top, pair);
        }
    }

    top
}

fn directive_of(kind: &ItemKind) -> &Directive {
    match kind {
        ItemKind::Preproc(d) => d,
        _ => unreachable!("directive_of called on a non-Preproc item"),
    }
}

fn push_current(stack: &mut [Builder], top: &mut Vec<(Item, Trivia)>, pair: (Item, Trivia)) {
    match stack.last_mut() {
        Some(b) => b.push_body(pair.0, pair.1),
        None => top.push(pair),
    }
}

/// If `file` starts with the classic C header include-guard idiom -
/// `#ifndef GUARD` / `#define GUARD` wrapping the rest of the file, with no
/// `#elif`/`#else` - returns the guard macro name. Verified against all 62
/// `linuxdoom-1.10` headers: every one has exactly this shape as `items[0]`.
pub fn include_guard_name(file: &File) -> Option<String> {
    let (item, _) = file.items.first()?;
    let group = match &item.kind {
        ItemKind::Conditional(g) => g,
        _ => return None,
    };
    if group.else_body.is_some() || group.branches.len() != 1 {
        return None;
    }
    let branch = &group.branches[0];
    let name = match &branch.directive {
        Directive::IfDef { name, negate: true } => name,
        _ => return None,
    };
    let (first_body_item, _) = branch.body.first()?;
    match &first_body_item.kind {
        ItemKind::Preproc(Directive::DefineObject { name: def_name, .. }) if def_name == name => {
            Some(name.clone())
        }
        _ => None,
    }
}

/// Removes the include-guard wrapper found by `include_guard_name`, splicing
/// its body (minus the `#define GUARD` line) directly into `file.items` in
/// place. No-op (returns `None`) if the file doesn't start with that exact
/// shape.
///
/// Unlike parsing, this is a real, byte-losing transform - it does not
/// preserve `File::render` round-tripping, and is only ever applied when
/// explicitly requested (see `--strip-guards` in `main.rs`).
pub fn strip_include_guard(file: &mut File) -> Option<String> {
    let name = include_guard_name(file)?;
    let (item, _) = file.items.remove(0);
    let mut branches = match item.kind {
        ItemKind::Conditional(g) => g.branches,
        _ => unreachable!("include_guard_name already checked this"),
    };
    let body = branches.remove(0).body;
    let rest: Vec<_> = body.into_iter().skip(1).collect();
    file.items.splice(0..0, rest);
    Some(name)
}

/// Fills in every `CondGroup.active` reachable from `items`, including
/// inside every `FunctionDef` body (via `stmt::cond::resolve_conditionals`),
/// given a `#define` environment (typically `main.rs`'s pre-defined list
/// merged with a file's own `corpus::compute_known_defines` entry). One
/// call resolves the whole tree at both AST levels, since it's simpler than
/// threading `defines` down through `record::build_items_with_types`/
/// `stmt::parse_function_body` at parse time. Mutates in place, the same
/// established precedent as `strip_include_guard`, but unlike that
/// function, this never removes or reorders bytes; it only ever writes
/// `active` fields, so it's always safe to call regardless of whether the
/// caller wants byte-exact round-trip afterward (`File::render()` never
/// reads `active` at all).
pub fn resolve_conditionals(items: &mut [(Item, Trivia)], defines: &HashMap<String, String>) {
    for (item, _) in items.iter_mut() {
        match &mut item.kind {
            ItemKind::Conditional(group) => resolve_group(group, defines),
            ItemKind::FunctionDef(_, body) => {
                super::stmt::cond::resolve_conditionals(&mut body.block, defines)
            }
            _ => {}
        }
    }
}

fn resolve_group(group: &mut CondGroup, defines: &HashMap<String, String>) {
    let mut resolved = None;
    for (i, branch) in group.branches.iter().enumerate() {
        match eval_directive(&branch.directive, defines) {
            Tri::True => {
                resolved = Some(ActiveBranch::Branch(i));
                break;
            }
            Tri::False => continue,
            // Can't safely skip past an undecidable branch to check the
            // ones after it - stop here, leaving `resolved` unset.
            Tri::Unknown => {
                resolved = Some(ActiveBranch::Unknown);
                break;
            }
        }
    }
    group.active = resolved.unwrap_or(if group.else_body.is_some() {
        ActiveBranch::Else
    } else {
        ActiveBranch::None
    });

    // Nested conditionals resolve independently of whether this group's
    // own branch containing them is itself active - still useful
    // information, and cheap to compute uniformly rather than special-
    // casing "skip resolving inside a branch we already know is dead".
    for branch in &mut group.branches {
        resolve_conditionals(&mut branch.body, defines);
    }
    if let Some(else_body) = &mut group.else_body {
        resolve_conditionals(else_body, defines);
    }
}

fn eval_directive(directive: &Directive, defines: &HashMap<String, String>) -> Tri {
    match directive {
        Directive::IfDef { name, negate } => preproc::eval_ifdef(name, *negate, defines),
        Directive::If { expr } | Directive::Elif { expr } => preproc::eval_if_expr(expr, defines),
        _ => Tri::Unknown,
    }
}

#[cfg(test)]
mod tests;
