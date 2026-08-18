//! Corpus-wide known-type-name computation, used to resolve the C
//! cast-vs-parenthesized-expression ambiguity (see `stmt::expr`). Kept
//! separate from `stmt/` (this module touches the filesystem, reusing the
//! existing `parse_file` pipeline) so `stmt::expr` itself stays easy to
//! unit test with a hand-built `KnownTypeNames`.
//!
//! Each file gets its own precise set: its own declarations, plus whatever
//! its (transitive) local `#include`s make visible - not a blanket union of
//! every typedef in the corpus. A flat union would be unsound in principle:
//! an identifier that's a typedef in one header could coincidentally be an
//! ordinary variable/function name in an unrelated file that never includes
//! that header, and a global union would wrongly treat it as a cast target
//! there too.
//!
//! Computing this needs an order: a file's set can only be finalized once
//! every file it `#include`s has already been resolved. `#include`s form a
//! graph, not a tree - `r_data.h`/`r_state.h` mutually include each other in
//! this corpus - so a plain topological sort isn't always possible. Handled
//! by first collapsing strongly connected components (Tarjan's algorithm)
//! into single units: the one real cycle here just means those two files
//! share one merged type environment, rather than breaking the ordering.
//! System/angled includes (`<stdlib.h>`) and any quoted include that isn't
//! one of `paths` are ignored - they contribute no names (best-effort, not
//! a real preprocessor).

use super::ast::{CondGroup, File, FnSig, Item, ItemKind, RecordDecl, Trivia, Type};
use super::preproc::Directive;
use super::stmt::expr::KnownTypeNames;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// The shared machinery behind `compute_known_type_names`,
/// `compute_known_globals`, and `compute_known_functions`: parse every file
/// in `paths` once, build the local `#include` graph, then combine each
/// SCC's own harvested value (via `merge`) with everything transitively
/// visible through `#include` from outside that SCC - the one real cycle
/// in this corpus (`r_data.h`/`r_state.h`) collapses to a single shared
/// unit rather than breaking the dependency ordering (see `tarjan_scc`). A
/// file that fails to parse, or a `#include` target not present in
/// `paths`, is skipped rather than treated as an error - this is a
/// best-effort analysis aid, not a correctness-critical pass in its own
/// right.
fn compute_per_file_env<V: Clone + Default>(
    paths: &[PathBuf],
    harvest: impl Fn(&[(Item, Trivia)]) -> V,
    merge: impl Fn(&mut V, &V),
) -> HashMap<String, V> {
    let files: HashMap<String, File> = paths
        .iter()
        .filter_map(|p| {
            let file = crate::parse_file(p).ok()?;
            let name = p.file_name()?.to_str()?.to_string();
            Some((name, file))
        })
        .collect();

    let own: HashMap<String, V> = files
        .iter()
        .map(|(name, file)| (name.clone(), harvest(&file.items)))
        .collect();

    let local_includes: HashMap<String, Vec<String>> = files
        .iter()
        .map(|(name, file)| {
            let mut incs = Vec::new();
            collect_local_includes(&file.items, &files, &mut incs);
            (name.clone(), incs)
        })
        .collect();

    // Tarjan emits each SCC only once every SCC it points into (i.e. every
    // file it depends on) has already been emitted - exactly the order this
    // needs, no separate topological-sort pass required.
    let sccs = tarjan_scc(&local_includes);

    let mut visible: HashMap<String, V> = HashMap::new();
    for scc in sccs {
        let scc_set: HashSet<&str> = scc.iter().map(String::as_str).collect();

        let mut combined = V::default();
        for member in &scc {
            if let Some(o) = own.get(member) {
                merge(&mut combined, o);
            }
        }
        for member in &scc {
            for dep in local_includes.get(member).into_iter().flatten() {
                if scc_set.contains(dep.as_str()) {
                    continue; // already folded in via `combined` above
                }
                if let Some(dep_visible) = visible.get(dep) {
                    merge(&mut combined, dep_visible);
                }
            }
        }
        for member in scc {
            visible.insert(member, combined.clone());
        }
    }
    visible
}

/// Computes each file's own precise `KnownTypeNames`, keyed by file name
/// (`path.file_name()` - matches how a `#include "foo.h"` directive spells
/// its target). Each file gets its own precise set: its own declarations,
/// plus whatever its (transitive) local `#include`s make visible - not a
/// blanket union of every typedef in the corpus, which would be unsound
/// (an identifier that's a typedef in one header could coincidentally be
/// an ordinary variable/function name in an unrelated file that never
/// includes that header).
pub fn compute_known_type_names(paths: &[PathBuf]) -> HashMap<String, KnownTypeNames> {
    compute_per_file_env(paths, own_type_names, KnownTypeNames::extend)
}

fn merge_map<V: Clone>(into: &mut HashMap<String, V>, from: &HashMap<String, V>) {
    into.extend(from.iter().map(|(k, v)| (k.clone(), v.clone())));
}

/// Computes each file's own precise global-variable environment
/// (`global var name -> Type`), using the exact same per-file
/// `#include`-visibility machinery as `compute_known_type_names` - just
/// harvesting `ItemKind::Var` instead of typedef/tag names. Needed so
/// `stmt::scope::Scope` can resolve a bare global identifier (not a
/// parameter or local) to its declared `Type`.
pub fn compute_known_globals(paths: &[PathBuf]) -> HashMap<String, HashMap<String, Type>> {
    compute_per_file_env(paths, own_global_types, merge_map)
}

/// Computes each file's own precise function-signature environment
/// (`function name -> FnSig`), same shape again - harvesting
/// `ItemKind::FunctionDecl`/`FunctionDef`'s signature instead. Needed to
/// look up a call site's callee (must be visible via the *caller's* own
/// `#include`s to be a real, resolvable call).
pub fn compute_known_functions(paths: &[PathBuf]) -> HashMap<String, HashMap<String, FnSig>> {
    compute_per_file_env(paths, own_function_sigs, merge_map)
}

/// Computes each file's own precise `#define` environment (`macro name ->
/// value text`), same `#include`-graph machinery again. Unlike
/// `own_type_names`/`own_global_types`/`own_function_sigs`, this
/// deliberately does **not** recurse into `ItemKind::Conditional` bodies -
/// only genuinely unconditional, top-level `#define`s count as "this file
/// defines this macro" (a `#define` inside an `#if`/`#ifdef` branch is only
/// real if that branch's own condition holds, which is exactly the
/// chicken-and-egg case `preproc::eval_if_expr`/`eval_ifdef` exist to
/// resolve - not attempted here). Within one file, a top-level `#undef`
/// removes the name from that file's own contribution before merging -
/// real corpus example: `am_map.c` defines and immediately `#undef`s a
/// scratch macro named `R` four times over (each pair bracketing one
/// `mline_t` array literal); by the time this harvest finishes walking the
/// file, `R` has been inserted and removed four times and correctly ends
/// up absent, with no special-casing needed - it just falls out of
/// processing items in their real file order.
pub fn compute_known_defines(paths: &[PathBuf]) -> HashMap<String, HashMap<String, String>> {
    compute_per_file_env(paths, own_defines, merge_map)
}

/// Computes each file's own precise typedef environment (typedef name ->
/// its underlying `Type`), same `#include`-graph machinery again. Needed
/// alongside `compute_known_records`: a struct/union field's own type is
/// often a typedef *name*, not the underlying shape directly (real corpus
/// case: `info.h`'s `state_t.action` field is `Named("actionf_t")` -
/// itself a union of further typedef'd function-pointer types,
/// `actionf_p1`/`actionf_v`/`actionf_p2` - resolving one level through a
/// typedef is what lets array/braced-initializer codegen recognize a
/// function-pointer-typed *value* position even when it's spelled via an
/// alias rather than a literal `Type::FunctionPointer`).
pub fn compute_known_typedefs(paths: &[PathBuf]) -> HashMap<String, HashMap<String, Type>> {
    compute_per_file_env(paths, own_typedefs, merge_map)
}

fn own_typedefs(items: &[(Item, Trivia)]) -> HashMap<String, Type> {
    let mut typedefs = HashMap::new();
    collect_typedefs(items, &mut typedefs);
    typedefs
}

fn collect_typedefs(items: &[(Item, Trivia)], typedefs: &mut HashMap<String, Type>) {
    for (item, _) in items {
        match &item.kind {
            ItemKind::Typedef(td) => {
                typedefs.insert(td.name.clone(), td.underlying.clone());
            }
            ItemKind::Conditional(cg) => {
                for branch in &cg.branches {
                    collect_typedefs(&branch.body, typedefs);
                }
                if let Some(else_body) = &cg.else_body {
                    collect_typedefs(else_body, typedefs);
                }
            }
            _ => {}
        }
    }
}

/// Computes each file's own precise struct/union-layout environment (type
/// name -> `RecordDecl`), same `#include`-graph machinery again - keyed by
/// *both* a record's `tag` and its `typedef_name` when both exist (mirrors
/// `codegen::types::map_type`'s own "resolve via either spelling" rule, e.g.
/// a self-referential field declared via the tag before its own typedef is
/// complete). Needed so array/braced-initializer codegen can look up a
/// struct/union-typed table row's field order (`states[]`/`mobjinfo[]`/...),
/// something nothing before this needed - only that a name referred to
/// *some* type (`KnownTypeNames`).
pub fn compute_known_records(paths: &[PathBuf]) -> HashMap<String, HashMap<String, RecordDecl>> {
    compute_per_file_env(paths, own_records, merge_map)
}

fn own_records(items: &[(Item, Trivia)]) -> HashMap<String, RecordDecl> {
    let mut records = HashMap::new();
    collect_records(items, &mut records);
    records
}

fn collect_records(items: &[(Item, Trivia)], records: &mut HashMap<String, RecordDecl>) {
    for (item, _) in items {
        match &item.kind {
            ItemKind::Record(rd) => {
                if let Some(tag) = &rd.tag {
                    records.insert(tag.clone(), rd.clone());
                }
                if let Some(name) = &rd.typedef_name {
                    records.insert(name.clone(), rd.clone());
                }
            }
            ItemKind::Conditional(cg) => {
                for branch in &cg.branches {
                    collect_records(&branch.body, records);
                }
                if let Some(else_body) = &cg.else_body {
                    collect_records(else_body, records);
                }
            }
            _ => {}
        }
    }
}

fn own_defines(items: &[(Item, Trivia)]) -> HashMap<String, String> {
    let mut defines = HashMap::new();
    for (item, _) in unwrap_include_guard(items) {
        match &item.kind {
            ItemKind::Preproc(Directive::DefineObject { name, value }) => {
                defines.insert(name.clone(), value.clone());
            }
            ItemKind::Preproc(Directive::DefineFunction { name, .. }) => {
                defines.insert(name.clone(), String::new());
            }
            ItemKind::Preproc(Directive::Undef { name }) => {
                defines.remove(name);
            }
            _ => {}
        }
    }
    defines
}

/// Real corpus headers wrap almost their entire content in the classic
/// `#ifndef GUARD`/`#define GUARD`...`#endif` include-guard idiom (see
/// `cond::include_guard_name`, which confirms this exact shape holds for
/// all 62 headers) - meaning a header's "real" top-level `#define`s (e.g.
/// `doomdef.h`'s `RANGECHECK`/`SNDSERV`) are technically nested one level
/// inside that `CondGroup`'s single branch, not directly top-level items.
/// Unwrapping it here for `own_defines`'s purposes is safe (unlike
/// `cond::strip_include_guard`, this is read-only - it never mutates or
/// loses bytes, just chooses which slice to scan): the guard's condition
/// is, by construction, always true the first time a header's content is
/// genuinely reached - there'd be nothing to see otherwise. Only this one
/// specific, always-true wrapper is unwrapped - an arbitrary other
/// `#ifdef` still correctly stays out of `own_defines`'s reach.
///
/// The returned slice deliberately **excludes** the guard's own opening
/// `#define GUARD` line (verified via a corpus-wide check: no guard name
/// is ever referenced anywhere outside its own defining file, so nothing
/// is lost by never counting it as "defined" at all). Keeping it would be
/// actively wrong, not just superfluous: `cond::resolve_conditionals`
/// evaluates a file's *own* `#ifndef GUARD` against this same environment,
/// and if `GUARD` showed up as already "defined" (because this file itself
/// defines it, one line later), the guard's own opening condition would
/// self-defeat - always resolving as if the header had already been
/// included once before, which is never true for the single-file-at-a-time
/// model this parser uses.
fn unwrap_include_guard(items: &[(Item, Trivia)]) -> &[(Item, Trivia)] {
    let Some((first, _)) = items.split_first() else {
        return items;
    };
    let ItemKind::Conditional(group) = &first.0.kind else {
        return items;
    };
    if group.else_body.is_some() || group.branches.len() != 1 {
        return items;
    }
    let branch = &group.branches[0];
    let Directive::IfDef { name, negate: true } = &branch.directive else {
        return items;
    };
    match branch.body.first() {
        Some((body_item, _))
            if matches!(
                &body_item.kind,
                ItemKind::Preproc(Directive::DefineObject { name: def_name, .. })
                    if def_name == name
            ) =>
        {
            &branch.body[1..]
        }
        _ => items,
    }
}

fn own_global_types(items: &[(Item, Trivia)]) -> HashMap<String, Type> {
    let mut globals = HashMap::new();
    collect_global_types(items, &mut globals);
    globals
}

fn collect_global_types(items: &[(Item, Trivia)], globals: &mut HashMap<String, Type>) {
    for (item, _) in items {
        match &item.kind {
            ItemKind::Var(vds) => {
                for vd in vds {
                    globals.insert(vd.name.clone(), vd.ty.clone());
                }
            }
            ItemKind::Conditional(cg) => {
                for branch in &cg.branches {
                    collect_global_types(&branch.body, globals);
                }
                if let Some(else_body) = &cg.else_body {
                    collect_global_types(else_body, globals);
                }
            }
            _ => {}
        }
    }
}

fn own_function_sigs(items: &[(Item, Trivia)]) -> HashMap<String, FnSig> {
    let mut sigs = HashMap::new();
    collect_function_sigs(items, &mut sigs);
    sigs
}

fn collect_function_sigs(items: &[(Item, Trivia)], sigs: &mut HashMap<String, FnSig>) {
    for (item, _) in items {
        match &item.kind {
            ItemKind::FunctionDecl(sig) => {
                sigs.insert(sig.name.clone(), sig.clone());
            }
            ItemKind::FunctionDef(sig, _) => {
                sigs.insert(sig.name.clone(), sig.clone());
            }
            ItemKind::Conditional(cg) => {
                for branch in &cg.branches {
                    collect_function_sigs(&branch.body, sigs);
                }
                if let Some(else_body) = &cg.else_body {
                    collect_function_sigs(else_body, sigs);
                }
            }
            _ => {}
        }
    }
}

fn own_type_names(items: &[(Item, Trivia)]) -> KnownTypeNames {
    let mut known = KnownTypeNames::new();
    collect_from_items(items, &mut known);
    known
}

fn collect_from_items(items: &[(Item, Trivia)], known: &mut KnownTypeNames) {
    for (item, _) in items {
        match &item.kind {
            ItemKind::Typedef(td) => known.insert(td.name.clone()),
            ItemKind::Record(rd) => {
                if let Some(tag) = &rd.tag {
                    known.insert(tag.clone());
                }
                if let Some(name) = &rd.typedef_name {
                    known.insert(name.clone());
                }
            }
            ItemKind::Enum(ed) => {
                if let Some(tag) = &ed.tag {
                    known.insert(tag.clone());
                }
                if let Some(name) = &ed.typedef_name {
                    known.insert(name.clone());
                }
            }
            ItemKind::Conditional(cg) => collect_from_conditional(cg, known),
            _ => {}
        }
    }
}

fn collect_from_conditional(cg: &CondGroup, known: &mut KnownTypeNames) {
    for branch in &cg.branches {
        collect_from_items(&branch.body, known);
    }
    if let Some(else_body) = &cg.else_body {
        collect_from_items(else_body, known);
    }
}

/// Collects the file names of every local (quoted, `angled: false`)
/// `#include` this file's items reference - recursing into `Conditional`
/// branches, since e.g. `doomtype.h`'s own typedefs sit inside an `#ifdef`
/// guard and the same is true of some `#include`s elsewhere in the corpus.
/// A target not present in `files` (a system header, or a typo/missing
/// file) is silently dropped - it contributes no names either way.
fn collect_local_includes(
    items: &[(Item, Trivia)],
    files: &HashMap<String, File>,
    out: &mut Vec<String>,
) {
    for (item, _) in items {
        match &item.kind {
            ItemKind::Preproc(Directive::Include {
                path,
                angled: false,
            }) => {
                if files.contains_key(path) && !out.contains(path) {
                    out.push(path.clone());
                }
            }
            ItemKind::Conditional(cg) => {
                for branch in &cg.branches {
                    collect_local_includes(&branch.body, files, out);
                }
                if let Some(else_body) = &cg.else_body {
                    collect_local_includes(else_body, files, out);
                }
            }
            _ => {}
        }
    }
}

/// Tarjan's strongly-connected-components algorithm over the `#include`
/// graph (`edges[file]` = the files it directly includes). Returns SCCs in
/// the order they're completed, which - since completing a node requires
/// first completing every node reachable from it - is always dependencies
/// before dependents. Most SCCs here are singletons; a cycle (like
/// `r_data.h`/`r_state.h`) comes back as one multi-file group.
fn tarjan_scc(edges: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    struct State<'a> {
        edges: &'a HashMap<String, Vec<String>>,
        counter: usize,
        stack: Vec<String>,
        on_stack: HashSet<String>,
        index: HashMap<String, usize>,
        lowlink: HashMap<String, usize>,
        result: Vec<Vec<String>>,
    }

    fn strongconnect(v: &str, st: &mut State) {
        st.index.insert(v.to_string(), st.counter);
        st.lowlink.insert(v.to_string(), st.counter);
        st.counter += 1;
        st.stack.push(v.to_string());
        st.on_stack.insert(v.to_string());

        if let Some(neighbors) = st.edges.get(v) {
            for w in neighbors.clone() {
                if !st.index.contains_key(&w) {
                    strongconnect(&w, st);
                    let wl = st.lowlink[&w];
                    let vl = st.lowlink[v];
                    st.lowlink.insert(v.to_string(), vl.min(wl));
                } else if st.on_stack.contains(&w) {
                    let wi = st.index[&w];
                    let vl = st.lowlink[v];
                    st.lowlink.insert(v.to_string(), vl.min(wi));
                }
            }
        }

        if st.lowlink[v] == st.index[v] {
            let mut component = Vec::new();
            loop {
                let w = st.stack.pop().expect("v's own SCC root is still on stack");
                st.on_stack.remove(&w);
                let done = w == v;
                component.push(w);
                if done {
                    break;
                }
            }
            st.result.push(component);
        }
    }

    let mut st = State {
        edges,
        counter: 0,
        stack: Vec::new(),
        on_stack: HashSet::new(),
        index: HashMap::new(),
        lowlink: HashMap::new(),
        result: Vec::new(),
    };

    // Sorted for determinism - iteration order over `edges` (a HashMap)
    // would otherwise make SCC-internal member order (and which SCCs tie
    // for "no dependency between them") vary run to run.
    let mut nodes: Vec<&String> = edges.keys().collect();
    nodes.sort();
    for v in nodes {
        if !st.index.contains_key(v) {
            strongconnect(v, &mut st);
        }
    }
    st.result
}

#[cfg(test)]
mod tests;
