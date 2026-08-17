//! `.c`+`.h` module merging: groups the corpus's 124 files into 75 Rust
//! modules (one per basename that has a `.c`, a `.h`, or both), merges each
//! pair's already-parsed items (header-then-source order) with a small
//! dedup pass so a header's forward declaration doesn't also get emitted
//! alongside the `.c` file's real definition, and resolves cross-module type
//! visibility into `use crate::{module}::*;` glob imports - mirroring C's
//! own `#include` mental model directly, since every generated item is
//! `pub` (see `codegen::items`). Consumes already-parsed `(Item, Trivia)`
//! lists; parsing itself (and building the `known_types`/`defines`
//! environment) stays a caller responsibility (`codegen::write`, PR D).

use crate::parser::ast::{ActiveBranch, EnumDecl, Item, ItemKind, RecordDecl, Trivia};
use crate::parser::preproc::Directive;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

/// One of the 75 Rust modules a merged `.c`+`.h` pair (or lone `.c`/`.h`)
/// becomes. `name` is the shared basename (e.g. `"m_misc"` for
/// `m_misc.c`+`m_misc.h`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleUnit {
    pub name: String,
    pub header: Option<PathBuf>,
    pub source: Option<PathBuf>,
}

/// Groups `paths` (every `.c`/`.h` in the corpus) into `ModuleUnit`s by
/// shared basename - 49 pairs + 13 `.c`-only + 13 `.h`-only = 75 modules for
/// the real `linuxdoom-1.10` corpus. Sorted by name for deterministic output.
pub fn group_into_modules(paths: &[PathBuf]) -> Vec<ModuleUnit> {
    let mut map: BTreeMap<String, ModuleUnit> = BTreeMap::new();
    for path in paths {
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let entry = map.entry(stem.to_string()).or_insert_with(|| ModuleUnit {
            name: stem.to_string(),
            header: None,
            source: None,
        });
        match path.extension().and_then(|e| e.to_str()) {
            Some("h") => entry.header = Some(path.clone()),
            Some("c") => entry.source = Some(path.clone()),
            _ => {}
        }
    }
    map.into_values().collect()
}

/// The Rust module a corpus filename belongs to: its basename, shared
/// between a `.c`/`.h` pair (e.g. `"m_misc.c"` and `"m_misc.h"` both map to
/// `"m_misc"`) - matches `group_into_modules`'s grouping exactly.
pub fn module_name_for_file(filename: &str) -> &str {
    filename
        .strip_suffix(".c")
        .or_else(|| filename.strip_suffix(".h"))
        .unwrap_or(filename)
}

/// Walks `items`, calling `f` on every item that would actually be emitted -
/// i.e. recursing into a `Conditional`'s already-resolved *active* branch
/// only (never a dead `None`/`Unknown` branch), matching
/// `codegen::items::emit_conditional`'s own policy exactly. Shared by both
/// the dedup pass (what declarations actually reach the output) and the
/// `#include` collection below (what's actually visible).
fn for_each_active_item<'a>(items: &'a [(Item, Trivia)], f: &mut impl FnMut(&'a Item)) {
    for (item, _) in items {
        match &item.kind {
            ItemKind::Conditional(group) => {
                let body = match group.active {
                    ActiveBranch::Branch(n) => Some(&group.branches[n].body),
                    ActiveBranch::Else => group.else_body.as_ref(),
                    ActiveBranch::None | ActiveBranch::Unknown => None,
                };
                if let Some(body) = body {
                    for_each_active_item(body, f);
                }
            }
            _ => f(item),
        }
    }
}

/// This file's own direct (one-hop) local `#include` targets, by filename
/// (e.g. `"d_think.h"`) - only `#include "..."` (`angled: false`) targets
/// count, matching `parser::corpus`'s own local-vs-system distinction.
/// Recurses into resolved-active `Conditional` branches, since some
/// `#include`s in this corpus sit inside `#ifdef` guards.
pub fn direct_local_includes(items: &[(Item, Trivia)]) -> Vec<String> {
    let mut out = Vec::new();
    for_each_active_item(items, &mut |item| {
        if let ItemKind::Preproc(Directive::Include {
            path,
            angled: false,
        }) = &item.kind
        {
            out.push(path.clone());
        }
    });
    out
}

/// The transitive closure of `direct_local_includes` edges, starting from
/// `start` (not included in the result unless reachable via a cycle back to
/// itself). A plain visited-set-guarded BFS over `graph` (file name -> its
/// own direct local includes) - correctly handles cycles (e.g. this
/// corpus's `r_data.h`<->`r_state.h` pair) for free, and deliberately does
/// *not* need the Tarjan-SCC/dependency-ordering machinery
/// `parser::corpus`'s per-file *environment* computations require: a flat
/// reachable-file-set has no "a dependency's value must be finished first"
/// ordering constraint the way those do.
pub fn transitively_visible_files(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
) -> HashSet<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(start.to_string());
    while let Some(name) = queue.pop_front() {
        let Some(edges) = graph.get(&name) else {
            continue;
        };
        for next in edges {
            if visited.insert(next.clone()) {
                queue.push_back(next.clone());
            }
        }
    }
    visited
}

/// Builds the `use crate::{module}::*;` glob-import lines a merged module
/// needs: every corpus file transitively locally-`#include`-visible from
/// any of `constituent_files` (its own `.h`/`.c` pair), mapped to module
/// names, deduped, excluding the module's own name, sorted for
/// deterministic output. Every generated item is `pub` (see
/// `codegen::items`), so a glob import makes everything that file's own
/// `#include` graph would see in C directly nameable here too - the same
/// mental model, no per-name ownership map needed.
///
/// Known accepted limitation: two unrelated glob-imported modules
/// defining the same name is possible and unhandled (only becomes a real
/// Rust compile error if the colliding name is actually referenced
/// unqualified) - acceptable given this whole phase's "expect real compile
/// errors from deferred features" posture.
pub fn use_statements_for_module(
    graph: &HashMap<String, Vec<String>>,
    own_module: &str,
    constituent_files: &[String],
) -> Vec<String> {
    let mut visible_modules: BTreeSet<String> = BTreeSet::new();
    for file in constituent_files {
        for visible_file in transitively_visible_files(graph, file) {
            visible_modules.insert(module_name_for_file(&visible_file).to_string());
        }
    }
    visible_modules.remove(own_module);
    visible_modules
        .into_iter()
        .map(|m| format!("use crate::{m}::*;\n"))
        .collect()
}

/// The name a `Record`/`Enum` declares itself under, for dedup purposes -
/// same preference (`typedef_name` else `tag`) `codegen::items::emit_record`
/// uses to pick the type's own Rust name.
fn record_name(rd: &RecordDecl) -> Option<&str> {
    rd.typedef_name.as_deref().or(rd.tag.as_deref())
}
fn enum_name(ed: &EnumDecl) -> Option<&str> {
    ed.typedef_name.as_deref().or(ed.tag.as_deref())
}

/// Which names have a "stronger" version elsewhere in `items` - a real
/// `FunctionDef` for some `FunctionDecl`'s name, or a with-initializer `Var`
/// for some without-initializer `Var`'s name. Only scans what would
/// actually be emitted (`for_each_active_item`), so a definition sitting in
/// a dead conditional branch never causes a real declaration elsewhere to
/// be wrongly dropped.
fn collect_stronger_names(items: &[(Item, Trivia)]) -> (HashSet<String>, HashSet<String>) {
    let mut has_def = HashSet::new();
    let mut has_init_var = HashSet::new();
    for_each_active_item(items, &mut |item| match &item.kind {
        ItemKind::FunctionDef(sig, _) => {
            has_def.insert(sig.name.clone());
        }
        ItemKind::Var(vd) if vd.initializer.is_some() => {
            has_init_var.insert(vd.name.clone());
        }
        _ => {}
    });
    (has_def, has_init_var)
}

/// Recursively drops items superseded by a stronger version elsewhere in
/// the same merged module (a `FunctionDecl` when a same-name `FunctionDef`
/// exists, or a no-initializer `Var` when a same-name with-initializer `Var`
/// exists), and drops every `Typedef`/`Record`/`Enum` beyond the first
/// occurrence of its own name (a genuine ODR violation if this ever fires
/// for real in a well-formed corpus - not expected, but degrading to "keep
/// the first, drop the rest" is safer than emitting a guaranteed-duplicate-
/// definition compile error). Only ever recurses into a `Conditional`'s
/// already-resolved *active* branch, leaving dead branches (and the
/// `Conditional` wrapper itself) untouched - matches `for_each_active_item`
/// and keeps `codegen::items::emit_conditional`'s own `Unknown`/`None`
/// handling working unchanged on the merged tree.
fn dedup_active(
    items: &mut Vec<(Item, Trivia)>,
    has_def: &HashSet<String>,
    has_init_var: &HashSet<String>,
    seen_types: &mut HashSet<String>,
) {
    items.retain_mut(|(item, _)| match &mut item.kind {
        ItemKind::Conditional(group) => {
            match &group.active {
                ActiveBranch::Branch(n) => {
                    let n = *n;
                    dedup_active(
                        &mut group.branches[n].body,
                        has_def,
                        has_init_var,
                        seen_types,
                    );
                }
                ActiveBranch::Else => {
                    if let Some(body) = &mut group.else_body {
                        dedup_active(body, has_def, has_init_var, seen_types);
                    }
                }
                ActiveBranch::None | ActiveBranch::Unknown => {}
            }
            true
        }
        ItemKind::FunctionDecl(sig) => !has_def.contains(&sig.name),
        ItemKind::Var(vd) if vd.initializer.is_none() => !has_init_var.contains(&vd.name),
        ItemKind::Typedef(td) => seen_types.insert(td.name.clone()),
        ItemKind::Record(rd) => match record_name(rd) {
            Some(name) => seen_types.insert(name.to_string()),
            None => true,
        },
        ItemKind::Enum(ed) => match enum_name(ed) {
            Some(name) => seen_types.insert(name.to_string()),
            None => true,
        },
        _ => true,
    });
}

/// Merges a module's header and source items (header-then-source order -
/// the natural C reading order, and also what a hand-written merged Rust
/// module would look like: types/decls first, then definitions), then
/// drops declarations superseded by a real definition elsewhere in the
/// same module. `None` for either input just contributes nothing (a
/// `.c`-only or `.h`-only module).
pub fn merge_items(
    header: Option<&[(Item, Trivia)]>,
    source: Option<&[(Item, Trivia)]>,
) -> Vec<(Item, Trivia)> {
    let mut combined: Vec<(Item, Trivia)> = Vec::new();
    if let Some(h) = header {
        combined.extend(h.iter().cloned());
    }
    if let Some(s) = source {
        combined.extend(s.iter().cloned());
    }
    let (has_def, has_init_var) = collect_stronger_names(&combined);
    let mut seen_types = HashSet::new();
    dedup_active(&mut combined, &has_def, &has_init_var, &mut seen_types);
    combined
}

/// A `ModuleUnit`'s constituent filenames (whichever of header/source
/// exist), for `use_statements_for_module`'s `constituent_files` argument.
pub fn constituent_file_names(unit: &ModuleUnit) -> Vec<String> {
    [&unit.header, &unit.source]
        .into_iter()
        .flatten()
        .filter_map(|p: &PathBuf| file_name_string(p))
        .collect()
}

fn file_name_string(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(str::to_string)
}

#[cfg(test)]
mod tests;
