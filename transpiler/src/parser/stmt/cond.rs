//! Step 7f: folds flat, sibling `#if`/`#ifdef`/`#ifndef`/`#elif`/`#else`/
//! `#endif` *statements* (as produced by `stmt::parse::parse_block`) into a
//! nested `StmtKind::Conditional` tree - the statement-level counterpart to
//! `cond::fold_conditionals`, one layer down. Same single left-to-right
//! pass over a stack of in-progress groups; nesting falls out the same way,
//! no separate recursive call needed *for a single flat list*.
//!
//! Unlike the top-level version, a `Block`'s own statement list isn't the
//! only place a mid-body `#ifdef` can appear - an `if`/`while`/`for`/
//! `switch` body, or an explicit nested `{...}` block, is its own separate
//! `Block`, not part of the same flat stream. So `fold_conditionals` here
//! also recurses into every nested `Block` reachable from a `Stmt`
//! (including the branch bodies of a `Conditional` this same pass just
//! built), which the top-level version never needed to do.
//!
//! Malformed input (a stray `#elif`/`#else`/`#endif` with no open group, or
//! an unterminated `#if` at end-of-body) degrades gracefully, same as
//! `cond::fold_conditionals`: the offending statements are left flat rather
//! than folded. The real corpus is fully balanced (verified via a
//! corpus-wide census before implementing this), so this is a safety net,
//! not something exercised by real input.

use super::super::ast::{ActiveBranch, Trivia};
use super::super::preproc::{self, Directive, Tri};
use super::ast::{Block, Stmt, StmtCondBranch, StmtCondGroup, StmtKind};
use std::collections::HashMap;

enum Cur {
    Branch(Directive),
    Else,
}

struct Builder {
    branches: Vec<StmtCondBranch>,
    cur: Cur,
    cur_body: Vec<(Stmt, Trivia)>,
    else_body: Option<Vec<(Stmt, Trivia)>>,
    /// Every statement consumed by this group so far, in original order -
    /// rebuilds the group's `raw` text, and flattens it back to plain
    /// statements if the group turns out to be unterminated.
    raw_stmts: Vec<(Stmt, Trivia)>,
}

impl Builder {
    fn new(directive: Directive, opener: (Stmt, Trivia)) -> Self {
        Builder {
            branches: Vec::new(),
            cur: Cur::Branch(directive),
            cur_body: Vec::new(),
            else_body: None,
            raw_stmts: vec![opener],
        }
    }

    fn push_body(&mut self, stmt: Stmt, trivia: Trivia) {
        self.raw_stmts.push((stmt.clone(), trivia.clone()));
        self.cur_body.push((stmt, trivia));
    }

    /// `#elif`/`#else`: close the branch/else body being filled and open the
    /// next one. `marker` is the elif/else statement itself, recorded only
    /// for `raw` reconstruction - it's never part of any body.
    fn advance(&mut self, next: Cur, marker: (Stmt, Trivia)) {
        let body = std::mem::take(&mut self.cur_body);
        match std::mem::replace(&mut self.cur, next) {
            Cur::Branch(directive) => self.branches.push(StmtCondBranch {
                directive,
                body: Block { stmts: body },
            }),
            Cur::Else => self.else_body = Some(body),
        }
        self.raw_stmts.push(marker);
    }

    /// `#endif`: close out the group, returning its structured form plus the
    /// flat, ordered statement list it was built from (for `raw`).
    fn finish(mut self, marker: (Stmt, Trivia)) -> (StmtCondGroup, Vec<(Stmt, Trivia)>) {
        let body = std::mem::take(&mut self.cur_body);
        match std::mem::replace(&mut self.cur, Cur::Else) {
            Cur::Branch(directive) => self.branches.push(StmtCondBranch {
                directive,
                body: Block { stmts: body },
            }),
            Cur::Else => self.else_body = Some(body),
        }
        self.raw_stmts.push(marker);
        (
            StmtCondGroup {
                branches: self.branches,
                else_body: self.else_body.map(|stmts| Block { stmts }),
                active: ActiveBranch::Unknown,
            },
            self.raw_stmts,
        )
    }
}

/// Folds `block`'s own statement list, then recurses into every nested
/// `Block` reachable from the result.
pub fn fold_conditionals(block: Block) -> Block {
    let flat = fold_flat(block.stmts);
    Block {
        stmts: flat.into_iter().map(|(s, t)| (fold_nested(s), t)).collect(),
    }
}

fn fold_flat(stmts: Vec<(Stmt, Trivia)>) -> Vec<(Stmt, Trivia)> {
    let mut stack: Vec<Builder> = Vec::new();
    let mut top: Vec<(Stmt, Trivia)> = Vec::new();

    for (stmt, trivia) in stmts {
        match &stmt.kind {
            StmtKind::Preproc(Directive::If { .. })
            | StmtKind::Preproc(Directive::IfDef { .. }) => {
                let directive = directive_of(&stmt.kind).clone();
                stack.push(Builder::new(directive, (stmt, trivia)));
            }
            StmtKind::Preproc(Directive::Elif { .. }) if !stack.is_empty() => {
                let directive = directive_of(&stmt.kind).clone();
                stack
                    .last_mut()
                    .unwrap()
                    .advance(Cur::Branch(directive), (stmt, trivia));
            }
            StmtKind::Preproc(Directive::Else) if !stack.is_empty() => {
                stack.last_mut().unwrap().advance(Cur::Else, (stmt, trivia));
            }
            StmtKind::Preproc(Directive::Endif) if !stack.is_empty() => {
                let builder = stack.pop().unwrap();
                let (group, raw_stmts) = builder.finish((stmt, trivia));
                // The opener's own leading trivia/labels become this
                // synthesized statement's trivia/labels, so they must NOT
                // also be rendered as part of `raw` - otherwise they'd be
                // emitted twice on round-trip.
                let group_trivia = raw_stmts[0].1.clone();
                let labels = raw_stmts[0].0.labels.clone();
                let mut raw = raw_stmts[0].0.raw.clone();
                raw.push_str(&render_body(&raw_stmts[1..]));
                let group_stmt = Stmt {
                    kind: StmtKind::Conditional(group),
                    labels,
                    raw,
                };
                push_current(&mut stack, &mut top, (group_stmt, group_trivia));
            }
            _ => push_current(&mut stack, &mut top, (stmt, trivia)),
        }
    }

    // Unterminated conditionals at end-of-body: flatten defensively rather
    // than losing them, same as `cond::fold_conditionals`.
    while let Some(builder) = stack.pop() {
        for pair in builder.raw_stmts {
            push_current(&mut stack, &mut top, pair);
        }
    }

    top
}

fn directive_of(kind: &StmtKind) -> &Directive {
    match kind {
        StmtKind::Preproc(d) => d,
        _ => unreachable!("directive_of called on a non-Preproc statement"),
    }
}

fn push_current(stack: &mut [Builder], top: &mut Vec<(Stmt, Trivia)>, pair: (Stmt, Trivia)) {
    match stack.last_mut() {
        Some(b) => b.push_body(pair.0, pair.1),
        None => top.push(pair),
    }
}

fn render_body(stmts: &[(Stmt, Trivia)]) -> String {
    let mut out = String::new();
    for (s, t) in stmts {
        for c in &t.leading {
            out.push_str(c.text());
        }
        out.push_str(&s.raw);
    }
    out
}

/// Recurses into every nested `Block` reachable from `stmt`: the bodies of
/// `if`/`while`/`do`-`while`/`for`/`switch`, an explicit nested block, and
/// (since `fold_flat` just built it) a `Conditional`'s own branch bodies.
fn fold_nested(mut stmt: Stmt) -> Stmt {
    stmt.kind = match stmt.kind {
        StmtKind::Block(b) => StmtKind::Block(fold_conditionals(b)),
        StmtKind::If {
            cond,
            then_branch,
            else_branch,
        } => StmtKind::If {
            cond,
            then_branch: Box::new(fold_nested(*then_branch)),
            else_branch: else_branch.map(|e| Box::new(fold_nested(*e))),
        },
        StmtKind::While { cond, body } => StmtKind::While {
            cond,
            body: Box::new(fold_nested(*body)),
        },
        StmtKind::DoWhile { body, cond } => StmtKind::DoWhile {
            body: Box::new(fold_nested(*body)),
            cond,
        },
        StmtKind::For {
            init,
            cond,
            step,
            body,
        } => StmtKind::For {
            init,
            cond,
            step,
            body: Box::new(fold_nested(*body)),
        },
        StmtKind::Switch { scrutinee, body } => StmtKind::Switch {
            scrutinee,
            body: Box::new(fold_nested(*body)),
        },
        StmtKind::Conditional(cg) => StmtKind::Conditional(StmtCondGroup {
            branches: cg
                .branches
                .into_iter()
                .map(|b| StmtCondBranch {
                    directive: b.directive,
                    body: fold_conditionals(b.body),
                })
                .collect(),
            else_body: cg.else_body.map(fold_conditionals),
            active: cg.active,
        }),
        other => other,
    };
    stmt
}

/// Fills in every `StmtCondGroup.active` reachable from `block`, given a
/// `#define` environment - the statement-level counterpart to
/// `cond::resolve_conditionals`. Same evaluation rules
/// (`preproc::eval_ifdef`/`eval_if_expr`), and the same extra requirement
/// `fold_conditionals` already has one layer down: recursing into every
/// nested `Block` reachable from a `Stmt`, not just this one's own flat
/// list, since an `if`/`while`/`for`/`switch` body or an explicit nested
/// `{...}` block is its own separate `Block`.
pub fn resolve_conditionals(block: &mut Block, defines: &HashMap<String, String>) {
    for (stmt, _) in &mut block.stmts {
        resolve_stmt(stmt, defines);
    }
}

fn resolve_stmt(stmt: &mut Stmt, defines: &HashMap<String, String>) {
    match &mut stmt.kind {
        StmtKind::Block(b) => resolve_conditionals(b, defines),
        StmtKind::If {
            then_branch,
            else_branch,
            ..
        } => {
            resolve_stmt(then_branch, defines);
            if let Some(e) = else_branch {
                resolve_stmt(e, defines);
            }
        }
        StmtKind::While { body, .. }
        | StmtKind::DoWhile { body, .. }
        | StmtKind::For { body, .. }
        | StmtKind::Switch { body, .. } => resolve_stmt(body, defines),
        StmtKind::Conditional(group) => resolve_group(group, defines),
        _ => {}
    }
}

fn resolve_group(group: &mut StmtCondGroup, defines: &HashMap<String, String>) {
    let mut resolved = None;
    for (i, branch) in group.branches.iter().enumerate() {
        match eval_directive(&branch.directive, defines) {
            Tri::True => {
                resolved = Some(ActiveBranch::Branch(i));
                break;
            }
            Tri::False => continue,
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
    // own branch containing them is itself active - same reasoning as
    // `cond::resolve_group`.
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
