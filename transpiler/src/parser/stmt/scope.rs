//! Step 7g: scoped identifier resolution over a function body - resolves a
//! bare `Ident` back to its declared `Type`, needed before any call-site
//! analysis (see the array-vs-pointer evidence plan) can be sound. Without
//! real scoping, a name match conflates unrelated same-named variables in
//! different functions/blocks - this exists specifically to avoid that.
//!
//! Deliberately just the resolution primitive, not a walker: a caller
//! (`stmt::evidence`, once it exists) drives `push`/`declare`/`pop` itself
//! while it walks a `Block`/`Stmt` tree for its own purposes, in exactly
//! the traversal order a real C compiler's own scope rules require - see
//! each method's doc comment for the ordering contract. This mirrors how
//! `KnownTypeNames` (`expr.rs`) is a plain lookup structure that
//! `parser::corpus` populates and `stmt::expr` only ever queries, never a
//! self-driving visitor.

use super::super::ast::Type;
use std::collections::HashMap;

/// A stack of block-scoped name->`Type` frames, falling back to a shared
/// corpus-wide global-variable environment (see
/// `parser::corpus::compute_known_globals`) for anything not found locally.
pub struct Scope<'a> {
    globals: &'a HashMap<String, Type>,
    /// One frame per active block, innermost last. Starts with exactly one
    /// frame (the function's own parameter scope) - `new` pushes it so a
    /// caller can `declare` parameters immediately without a separate
    /// `push()` call first.
    stack: Vec<HashMap<String, Type>>,
}

impl<'a> Scope<'a> {
    /// Starts a new scope for one function body, with `globals` as the
    /// fallback environment and one empty frame ready for the function's
    /// own parameters to be `declare`d into.
    pub fn new(globals: &'a HashMap<String, Type>) -> Self {
        Scope {
            globals,
            stack: vec![HashMap::new()],
        }
    }

    /// Enters a nested scope (a `StmtKind::Block`, or an `if`/`while`/`for`/
    /// `switch` body that's a `Block`) - call before walking its
    /// statements, `pop` after. A `for` loop's own declared init variable
    /// is scoped to the *whole* statement (init/cond/step/body), not just
    /// the body block, so a caller walking a `For` should `push` before
    /// processing `init` and `pop` only after the whole statement, not
    /// around `body` alone.
    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    /// Leaves the innermost scope - every name `declare`d since the
    /// matching `push` (or, for the outermost frame, since `new`) stops
    /// resolving once this returns.
    pub fn pop(&mut self) {
        self.stack.pop();
        debug_assert!(
            !self.stack.is_empty(),
            "popped the function's own outermost (parameter) scope"
        );
    }

    /// Registers `name` in the *innermost* active frame, shadowing any
    /// outer declaration of the same name until that frame is `pop`ped.
    /// Call exactly when a function parameter or a `DeclStmt`'s declarator
    /// is encountered while walking left-to-right - declaring at the
    /// walk's actual position, not upfront, is what gives C's "only visible
    /// after its own declaration point" rule for free.
    pub fn declare(&mut self, name: &str, ty: Type) {
        self.stack
            .last_mut()
            .expect("Scope always has at least one frame")
            .insert(name.to_string(), ty);
    }

    /// Resolves `name` to its declared `Type`: innermost frame outward,
    /// falling back to the corpus-wide global environment. `None` means
    /// genuinely unresolvable (an undeclared name, a name declared in a
    /// file this one doesn't `#include`, or a macro-expanded identifier
    /// this parser never tracks types for at all) - never a paper over an
    /// ambiguous case, matching this parser's usual "don't guess past what
    /// isn't actually decidable" stance.
    pub fn resolve(&self, name: &str) -> Option<&Type> {
        for frame in self.stack.iter().rev() {
            if let Some(ty) = frame.get(name) {
                return Some(ty);
            }
        }
        self.globals.get(name)
    }
}

#[cfg(test)]
mod tests;
