//! Step 2 of the array-vs-pointer plan (`plans/humming-knitting-simon.md`):
//! studies real call sites to recover information a C signature alone
//! can't carry - a pointer-typed parameter like `int *arr` is
//! syntactically identical whether it's conceptually an array or a single
//! object's address, because C's array-to-pointer decay erases the
//! distinction at the signature. This walks every call site in the corpus
//! and records, per `(function, param_index)`, whatever evidence the
//! *argument actually passed* provides - never a rewrite of `ast::Type`
//! itself (see this module's own doc comment on `EvidenceKind` for why).
//!
//! Built entirely on read-only analysis over the existing AST -
//! `parser::corpus::compute_known_globals`/`compute_known_functions` for
//! corpus-wide visibility, `stmt::scope::Scope` for resolving a bare
//! identifier's declared type at the point it's used. Touches no parsing
//! or round-trip logic.

use super::ast::{FnSig, Item, ItemKind, Trivia, Type};
use super::corpus::{compute_known_functions, compute_known_globals, compute_known_type_names};
use super::stmt::ast::{Block, ForInit, Label, Stmt, StmtKind};
use super::stmt::decl::LocalInit;
use super::stmt::expr::{Expr, UnaryOp};
use super::stmt::scope::Scope;
use std::collections::HashMap;
use std::path::PathBuf;

/// What a single argument expression tells us about the parameter it's
/// passed to. Intentionally just two directly-observable shapes - anything
/// else (pointer arithmetic, another call's return value, a `->`/`.`
/// member access, a cast, `NULL`, ...) is too ambiguous to call either way
/// and contributes no evidence at all, matching this parser's consistent
/// "don't guess past what's actually decidable" stance. This is *never*
/// merged back into `ast::Type` - it's an advisory signal about usage, and
/// the same parameter can legitimately receive both kinds of argument at
/// different call sites (a "buffer of length N" function is routinely
/// called with both a real array and `&single_value, 1`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKind {
    /// The argument is a bare identifier declared as an array somewhere in
    /// its resolvable scope.
    Array,
    /// The argument is `&something` - address-of a single value is never
    /// an array.
    SingleObject,
}

#[derive(Debug, Clone)]
pub struct CallSiteEvidence {
    pub function: String,
    pub param_index: usize,
    pub param_name: String,
    pub caller_file: String,
    pub caller_function: String,
    pub kind: EvidenceKind,
}

/// Walks every function body in `paths`, recording `CallSiteEvidence` for
/// every pointer-typed parameter position where the argument passed is
/// either a known-array identifier or an explicit `&value`. Return-value
/// evidence (does a pointer-returning function's result get indexed by its
/// callers?) is out of scope here - see the plan for why.
pub fn collect_evidence(paths: &[PathBuf]) -> Vec<CallSiteEvidence> {
    let known_types = compute_known_type_names(paths);
    let globals = compute_known_globals(paths);
    let functions = compute_known_functions(paths);
    let mut evidence = Vec::new();

    for path in paths {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let known = known_types.get(name).cloned().unwrap_or_default();
        let Ok(file) = crate::parse_file_with_types(path, &known) else {
            continue;
        };
        let file_globals = globals.get(name).cloned().unwrap_or_default();
        let file_functions = functions.get(name).cloned().unwrap_or_default();
        walk_items(
            &file.items,
            name,
            &file_globals,
            &file_functions,
            &mut evidence,
        );
    }

    evidence
}

fn walk_items(
    items: &[(Item, Trivia)],
    caller_file: &str,
    globals: &HashMap<String, Type>,
    functions: &HashMap<String, FnSig>,
    out: &mut Vec<CallSiteEvidence>,
) {
    for (item, _) in items {
        match &item.kind {
            ItemKind::FunctionDef(sig, body) => {
                let mut scope = Scope::new(globals);
                for p in &sig.params {
                    scope.declare(&p.name, p.ty.clone());
                }
                let mut w = Walker {
                    functions,
                    caller_file,
                    caller_function: &sig.name,
                    out,
                };
                w.walk_block(&body.block, &mut scope);
            }
            ItemKind::Conditional(cg) => {
                for branch in &cg.branches {
                    walk_items(&branch.body, caller_file, globals, functions, out);
                }
                if let Some(else_body) = &cg.else_body {
                    walk_items(else_body, caller_file, globals, functions, out);
                }
            }
            _ => {}
        }
    }
}

/// Carries the per-function context (which corpus function signatures are
/// visible, which file/function this evidence is attributed to) through
/// the recursive walk, so `walk_block`/`walk_stmt`/`walk_expr` don't need
/// an ever-growing parameter list.
struct Walker<'a> {
    functions: &'a HashMap<String, FnSig>,
    caller_file: &'a str,
    caller_function: &'a str,
    out: &'a mut Vec<CallSiteEvidence>,
}

impl Walker<'_> {
    fn walk_block(&mut self, block: &Block, scope: &mut Scope) {
        scope.push();
        for (stmt, _) in &block.stmts {
            self.walk_stmt(stmt, scope);
        }
        scope.pop();
    }

    fn walk_stmt(&mut self, stmt: &Stmt, scope: &mut Scope) {
        for label in &stmt.labels {
            if let Label::Case(e) = label {
                self.walk_expr(e, scope);
            }
        }
        match &stmt.kind {
            StmtKind::Expr(e) => self.walk_expr(e, scope),
            StmtKind::Decl(d) => {
                for decl in &d.declarators {
                    if let Some(init) = &decl.initializer {
                        self.walk_local_init(init, scope);
                    }
                    scope.declare(&decl.name, decl.ty.clone());
                }
            }
            StmtKind::Block(b) => self.walk_block(b, scope),
            StmtKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.walk_expr(cond, scope);
                self.walk_stmt(then_branch, scope);
                if let Some(e) = else_branch {
                    self.walk_stmt(e, scope);
                }
            }
            StmtKind::While { cond, body } | StmtKind::DoWhile { body, cond } => {
                self.walk_expr(cond, scope);
                self.walk_stmt(body, scope);
            }
            StmtKind::For {
                init,
                cond,
                step,
                body,
            } => {
                // A `for` loop's own declared init variable is scoped to
                // the whole statement, not just `body` - push before
                // `init`, pop only after `body` (see `stmt::scope::Scope`'s
                // own doc comment on this exact ordering contract).
                scope.push();
                match init {
                    Some(ForInit::Decl(d)) => {
                        for decl in &d.declarators {
                            if let Some(i) = &decl.initializer {
                                self.walk_local_init(i, scope);
                            }
                            scope.declare(&decl.name, decl.ty.clone());
                        }
                    }
                    Some(ForInit::Expr(e)) => self.walk_expr(e, scope),
                    None => {}
                }
                if let Some(c) = cond {
                    self.walk_expr(c, scope);
                }
                if let Some(s) = step {
                    self.walk_expr(s, scope);
                }
                self.walk_stmt(body, scope);
                scope.pop();
            }
            StmtKind::Switch { scrutinee, body } => {
                self.walk_expr(scrutinee, scope);
                self.walk_stmt(body, scope);
            }
            StmtKind::Return(Some(e)) => self.walk_expr(e, scope),
            StmtKind::Conditional(cg) => {
                // A `#ifdef`/`#endif` run is *not* a C scope - unlike an
                // explicit `{...}` block, whatever it wraps is at the same
                // scope level as its surroundings, so its branch bodies are
                // walked directly against the current `scope` with no
                // `push`/`pop` of their own.
                for branch in &cg.branches {
                    for (s, _) in &branch.body.stmts {
                        self.walk_stmt(s, scope);
                    }
                }
                if let Some(else_body) = &cg.else_body {
                    for (s, _) in &else_body.stmts {
                        self.walk_stmt(s, scope);
                    }
                }
            }
            _ => {}
        }
    }

    fn walk_local_init(&mut self, init: &LocalInit, scope: &mut Scope) {
        match init {
            LocalInit::Expr(e) => self.walk_expr(e, scope),
            LocalInit::Braced(items) => {
                for i in items {
                    self.walk_local_init(i, scope);
                }
            }
        }
    }

    fn walk_expr(&mut self, expr: &Expr, scope: &Scope) {
        if let Expr::Call { callee, args } = expr {
            if let Expr::Ident(fname) = &**callee {
                self.check_call(fname, args, scope);
            }
            self.walk_expr(callee, scope);
            for a in args {
                self.walk_expr(a, scope);
            }
            return;
        }
        match expr {
            Expr::Paren(a)
            | Expr::Unary { expr: a, .. }
            | Expr::Postfix { expr: a, .. }
            | Expr::Cast { expr: a, .. } => self.walk_expr(a, scope),
            Expr::Binary { lhs, rhs, .. }
            | Expr::Assign { lhs, rhs, .. }
            | Expr::Index {
                base: lhs,
                index: rhs,
            } => {
                self.walk_expr(lhs, scope);
                self.walk_expr(rhs, scope);
            }
            Expr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                self.walk_expr(cond, scope);
                self.walk_expr(then_expr, scope);
                self.walk_expr(else_expr, scope);
            }
            Expr::Comma(items) => items.iter().for_each(|x| self.walk_expr(x, scope)),
            Expr::Member { base, .. } | Expr::Arrow { base, .. } => self.walk_expr(base, scope),
            Expr::Sizeof(super::stmt::expr::SizeofArg::Expr(e)) => self.walk_expr(e, scope),
            _ => {}
        }
    }

    fn check_call(&mut self, fname: &str, args: &[Expr], scope: &Scope) {
        let Some(sig) = self.functions.get(fname) else {
            return;
        };
        for (i, arg) in args.iter().enumerate() {
            let Some(param) = sig.params.get(i) else {
                continue;
            };
            if !matches!(param.ty, Type::Pointer(_)) {
                continue;
            }
            let kind = match arg {
                Expr::Ident(argname) => match scope.resolve(argname) {
                    Some(Type::Array(..)) => Some(EvidenceKind::Array),
                    _ => None,
                },
                Expr::Unary {
                    op: UnaryOp::AddrOf,
                    ..
                } => Some(EvidenceKind::SingleObject),
                _ => None,
            };
            if let Some(kind) = kind {
                self.out.push(CallSiteEvidence {
                    function: fname.to_string(),
                    param_index: i,
                    param_name: param.name.clone(),
                    caller_file: self.caller_file.to_string(),
                    caller_function: self.caller_function.to_string(),
                    kind,
                });
            }
        }
    }
}

/// Per-`(function, param_index)` aggregate of `collect_evidence`'s raw
/// hits - the practically useful shape to read: `array_hits > 0 &&
/// single_object_hits == 0` is strong evidence the parameter is
/// conceptually an array; both non-zero is genuinely mixed usage, worth
/// surfacing as such rather than silently picking a side.
#[derive(Debug, Clone)]
pub struct EvidenceSummary {
    pub function: String,
    pub param_index: usize,
    pub param_name: String,
    pub array_hits: usize,
    pub single_object_hits: usize,
}

pub fn summarize(evidence: &[CallSiteEvidence]) -> Vec<EvidenceSummary> {
    let mut by_param: HashMap<(String, usize), EvidenceSummary> = HashMap::new();
    for e in evidence {
        let key = (e.function.clone(), e.param_index);
        let entry = by_param.entry(key).or_insert_with(|| EvidenceSummary {
            function: e.function.clone(),
            param_index: e.param_index,
            param_name: e.param_name.clone(),
            array_hits: 0,
            single_object_hits: 0,
        });
        match e.kind {
            EvidenceKind::Array => entry.array_hits += 1,
            EvidenceKind::SingleObject => entry.single_object_hits += 1,
        }
    }
    let mut out: Vec<EvidenceSummary> = by_param.into_values().collect();
    out.sort_by(|a, b| {
        b.array_hits
            .cmp(&a.array_hits)
            .then_with(|| a.function.cmp(&b.function))
            .then_with(|| a.param_index.cmp(&b.param_index))
    });
    out
}

#[cfg(test)]
mod tests;
