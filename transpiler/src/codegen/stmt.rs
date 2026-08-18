//! `parser::stmt::ast` -> Rust statement text - the codegen counterpart to
//! `parser::stmt`, and the last unstarted phase of this backend (every
//! function body was a `todo!()` stub before this). See the approved plan
//! at `/home/michael/.claude/plans/enchanted-moseying-pebble.md` for the
//! full design.
//!
//! Same "never guess, degrade loudly and locally" discipline as the rest of
//! this codegen backend: an unrecognized or not-yet-translated *statement*
//! degrades to a flagged comment plus a `todo!()` call, in its own place in
//! the output - never bailing the whole containing function. A real corpus
//! census (721 functions, 17150 statements) found 13.2% of statements are
//! `StmtKind::Raw` alone; at that rate, failing the whole function on any
//! unrecognized statement would stub out the vast majority of functions
//! instead of translating what's actually understood.
//!
//! **Scope of this module today**: straight-line statements only
//! (`Expr`/`Decl`/`Block`/`Return`/`Empty`/`Preproc`/`Conditional`/`Raw`/
//! `Goto`/named labels). Control flow (`If`/`While`/`DoWhile`/`For`/
//! `Break`/`Continue`) and `Switch` each degrade their whole statement for
//! now - later phases of the same plan build those out; a control-flow-
//! heavy function still gets its straight-line statements translated in
//! the meantime, only the control-flow ones stub out individually.
//!
//! Every expression (conditions aside - see `codegen::expr::
//! render_condition`) goes through the already-mature `codegen::expr::
//! render_expr` unchanged - this module only ever deals in statement-level
//! shape, never re-solves an expression-rendering problem `codegen::expr`
//! already has an answer for.

use super::expr::render_expr;
use super::ident::ident;
use super::init::render_scalar_init_expr;
use super::types::{map_type, type_is_malformed};
use crate::parser::ast::{ActiveBranch, FnSig, RecordDecl, Storage, Type};
use crate::parser::stmt::ast::{Block, FnBody, Label, Stmt, StmtCondGroup, StmtKind};
use crate::parser::stmt::decl::{DeclStmt, LocalInit};
use crate::parser::stmt::expr::KnownTypeNames;
use std::collections::HashMap;

/// Every corpus-derived environment a function body needs to translate -
/// the same six `known_*` maps `codegen::items::emit_item` already carries
/// for every other `ItemKind` arm, bundled into one struct rather than
/// threaded as more positional arguments (`emit_items`'s own call chain
/// already carries `#[allow(clippy::too_many_arguments)]` in three places).
pub struct BodyCtx<'a> {
    pub known: &'a KnownTypeNames,
    pub known_records: &'a HashMap<String, RecordDecl>,
    pub known_typedefs: &'a HashMap<String, Type>,
    pub known_functions: &'a HashMap<String, FnSig>,
    pub known_globals: &'a HashMap<String, Type>,
    pub known_defines: &'a HashMap<String, String>,
}

/// Renders the *inside* of one function body - no surrounding braces, no
/// `unsafe` wrapper (`codegen::items::emit_function_def` owns those, since
/// they're a property of the whole function, not of any one statement).
/// Never fails, never panics on the transpiler's own side: every
/// unrecognized shape degrades to a flagged comment plus a `todo!()`
/// statement in the *generated* Rust, per-statement.
pub fn render_function_body(body: &FnBody, ctx: &BodyCtx<'_>) -> String {
    render_block(&body.block, ctx)
}

fn render_block(block: &Block, ctx: &BodyCtx<'_>) -> String {
    block
        .stmts
        .iter()
        .map(|(stmt, _)| render_stmt(stmt, ctx))
        .collect()
}

fn render_stmt(stmt: &Stmt, ctx: &BodyCtx<'_>) -> String {
    let mut prefix = String::new();
    for label in &stmt.labels {
        match label {
            // Only the jump itself has no translation - control that
            // reaches this statement by ordinary fall-through still does,
            // so the statement is translated normally below, not degraded.
            Label::Named(name) => {
                prefix.push_str(&format!(
                    "// C label {name}: (goto targets are not translated)\n"
                ));
            }
            // `Label::Case`/`Default` are only ever attached to statements
            // inside a `switch` body - unreachable here today, since
            // `StmtKind::Switch` degrades its whole statement without ever
            // recursing into individual case-labeled statements. Defensive
            // fallback, not a real corpus path.
            Label::Case(_) | Label::Default => {
                return format!(
                    "{prefix}{}",
                    degrade("case/default label outside switch translation", &stmt.raw)
                );
            }
        }
    }
    format!("{prefix}{}", render_stmt_kind(stmt, ctx))
}

fn render_stmt_kind(stmt: &Stmt, ctx: &BodyCtx<'_>) -> String {
    match &stmt.kind {
        StmtKind::Expr(e) => match render_expr(e, ctx.known_globals) {
            Some(rendered) => format!("{rendered};\n"),
            None => degrade("expression not yet translated", &stmt.raw),
        },
        StmtKind::Decl(d) => render_decl(d, ctx),
        StmtKind::Block(b) => format!("{{\n{}}}\n", render_block(b, ctx)),
        StmtKind::Return(None) => "return;\n".to_string(),
        StmtKind::Return(Some(e)) => match render_expr(e, ctx.known_globals) {
            Some(rendered) => format!("return {rendered};\n"),
            None => degrade("return expression not yet translated", &stmt.raw),
        },
        StmtKind::Empty => String::new(),
        // A directive isn't executable code - nothing is lost by dropping
        // straight to a comment, unlike every other degrade case here.
        StmtKind::Preproc(_) => format!(
            "// C preprocessor directive at statement position (not executable, nothing lost):\n{}",
            comment_lines(&stmt.raw)
        ),
        StmtKind::Conditional(g) => render_stmt_conditional(stmt, g, ctx),
        StmtKind::Raw => degrade("statement not yet translated", &stmt.raw),
        // No general translation exists for an arbitrary `goto` without CFG
        // reconstruction (see this module's own doc comment / the approved
        // plan) - always degrades, deliberately, not a gap to close later
        // without a much bigger design.
        StmtKind::Goto(label) => degrade(&format!("goto {label} not translated"), &stmt.raw),
        // Control flow and switch: out of scope for this module today, see
        // the doc comment at the top - a later phase of the same plan
        // builds these out.
        StmtKind::If { .. } => degrade("if statement not yet translated", &stmt.raw),
        StmtKind::While { .. } => degrade("while statement not yet translated", &stmt.raw),
        StmtKind::DoWhile { .. } => degrade("do-while statement not yet translated", &stmt.raw),
        StmtKind::For { .. } => degrade("for statement not yet translated", &stmt.raw),
        StmtKind::Switch { .. } => degrade("switch statement not yet translated", &stmt.raw),
        StmtKind::Break => degrade("break statement not yet translated", &stmt.raw),
        StmtKind::Continue => degrade("continue statement not yet translated", &stmt.raw),
    }
}

fn render_stmt_conditional(stmt: &Stmt, group: &StmtCondGroup, ctx: &BodyCtx<'_>) -> String {
    match group.active {
        ActiveBranch::Branch(n) => render_block(&group.branches[n].body, ctx),
        ActiveBranch::Else => group
            .else_body
            .as_ref()
            .map(|b| render_block(b, ctx))
            .unwrap_or_default(),
        ActiveBranch::None => String::new(),
        // Dropping *statements* silently would make the function quietly
        // do less than the real C - unlike the equivalent case one level up
        // (`codegen::items::emit_conditional`), where an unresolved
        // top-level `#if` just fails to define something, loudly, at
        // compile time. The `todo!()` keeps this one just as loud.
        ActiveBranch::Unknown => {
            degrade("unresolved #if condition, needs manual review", &stmt.raw)
        }
    }
}

fn render_decl(d: &DeclStmt, ctx: &BodyCtx<'_>) -> String {
    let is_static = d.storage.contains(&Storage::Static);
    let mut out = String::new();
    for decl in &d.declarators {
        let name = ident(&decl.name);
        if type_is_malformed(&decl.ty) {
            // Mirrors `codegen::items::format_params`'s identical precedent
            // for a malformed *param* type: keep the binding so the
            // failure is one loud type error, not an E0425 cascade over
            // every later use of this name.
            out.push_str(&format!(
                "let mut {name}: () = (); // TODO: unparsed local type, needs manual translation\n"
            ));
            continue;
        }
        let ty = map_type(&decl.ty);
        let init = render_local_init(decl.initializer.as_ref(), &decl.ty, ctx);
        // `static`, not `let mut`: several real corpus locals (am_map.c's
        // `cheatstate`/`bigstate`, f_finale.c's `laststage`) depend on
        // persistence across calls, which a fresh `let` on every call
        // would silently break.
        let kw = if is_static { "static mut" } else { "let mut" };
        out.push_str(&format!("{kw} {name}: {ty} = unsafe {{ {init} }};\n"));
    }
    out
}

/// Renders one declarator's initializer, or the same flagged `zeroed()`
/// stub `codegen::items::emit_var` already uses at module scope for the
/// same reason: a missing/unrenderable initializer, or (real corpus shape:
/// exclusively on `static` locals, e.g. `am_map.c:461`) a `LocalInit::
/// Braced` - translating a local braced initializer needs the same
/// array/struct-row machinery `codegen::init`'s `render_array_init`/
/// `render_struct_init` provide at module scope, which operate on the
/// module-level `Init` type, not `LocalInit` - out of scope for this first
/// pass given how rare the real corpus shape is (5 sites, all `static`).
/// **Always initializes** - a C automatic local already holds garbage
/// before its own initializer runs, so zero-init is not a semantic
/// regression, and leaving a binding uninitialized would hand rustc a
/// flow-sensitivity problem it rejects (E0381) across most of the corpus.
fn render_local_init(init: Option<&LocalInit>, ty: &Type, ctx: &BodyCtx<'_>) -> String {
    if let Some(LocalInit::Expr(e)) = init
        && let Some(rendered) = render_scalar_init_expr(
            e,
            ty,
            ctx.known_typedefs,
            ctx.known_functions,
            ctx.known_globals,
        )
    {
        return rendered;
    }
    "std::mem::zeroed() /* TODO: initializer not yet translated */".to_string()
}

/// The universal degrade: `raw`'s exact original text as `//`-prefixed
/// comment lines (never inside the `todo!()` string itself - C text
/// contains `{`/`}`/`"`/`\`/newlines that would produce broken Rust or
/// bogus format placeholders if embedded in a format-string literal),
/// followed by a `todo!()` call whose argument is always one of this
/// module's own constant reason strings (never raw C text), so no runtime
/// escaping concern exists there either.
fn degrade(reason: &str, raw: &str) -> String {
    format!(
        "// TODO: {reason}:\n{}todo!({reason:?});\n",
        comment_lines(raw)
    )
}

fn comment_lines(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        out.push_str("// ");
        out.push_str(line);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests;
