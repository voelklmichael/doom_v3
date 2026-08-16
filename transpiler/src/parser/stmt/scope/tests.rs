use super::*;
use std::collections::HashMap;

fn int_ty() -> Type {
    Type::Named("int".to_string())
}

fn ptr(ty: Type) -> Type {
    Type::Pointer(Box::new(ty))
}

fn array(ty: Type) -> Type {
    Type::Array(Box::new(ty), None)
}

#[test]
fn resolves_a_declared_parameter() {
    let globals = HashMap::new();
    let mut scope = Scope::new(&globals);
    scope.declare("x", int_ty());
    assert_eq!(scope.resolve("x"), Some(&int_ty()));
}

#[test]
fn falls_back_to_globals() {
    let mut globals = HashMap::new();
    globals.insert("g_state".to_string(), ptr(int_ty()));
    let scope = Scope::new(&globals);
    assert_eq!(scope.resolve("g_state"), Some(&ptr(int_ty())));
}

#[test]
fn unresolvable_name_is_none() {
    let globals = HashMap::new();
    let scope = Scope::new(&globals);
    assert_eq!(scope.resolve("never_declared"), None);
}

#[test]
fn nested_block_shadows_outer_declaration() {
    let globals = HashMap::new();
    let mut scope = Scope::new(&globals);
    scope.declare("x", int_ty());
    scope.push();
    scope.declare("x", array(int_ty()));
    // The inner declaration shadows the outer one while its frame is active.
    assert_eq!(scope.resolve("x"), Some(&array(int_ty())));
    scope.pop();
    // Popping the inner frame restores visibility of the outer declaration.
    assert_eq!(scope.resolve("x"), Some(&int_ty()));
}

#[test]
fn sibling_blocks_do_not_leak_into_each_other() {
    let globals = HashMap::new();
    let mut scope = Scope::new(&globals);

    scope.push();
    scope.declare("only_in_first", int_ty());
    scope.pop();

    scope.push();
    // A name declared in an earlier *sibling* block must not still resolve
    // here - each `push`/`pop` pair is its own independent frame.
    assert_eq!(scope.resolve("only_in_first"), None);
    scope.pop();
}

#[test]
fn for_loop_init_variable_stays_visible_across_a_nested_body_block() {
    // for (int i = 0; i < n; i++) { ... } - `i` is scoped to the whole
    // statement (push before `init`, pop after the whole `For`), while the
    // body itself is a separate nested `Block` that pushes its own frame.
    let globals = HashMap::new();
    let mut scope = Scope::new(&globals);

    scope.push(); // the for-statement's own scope
    scope.declare("i", int_ty());

    scope.push(); // the body block's own nested scope
    scope.declare("body_local", int_ty());
    // `i` (from the enclosing for-scope) is still visible inside the body.
    assert_eq!(scope.resolve("i"), Some(&int_ty()));
    assert_eq!(scope.resolve("body_local"), Some(&int_ty()));
    scope.pop(); // leave the body block

    // The body's own local no longer resolves, but `i` still does - the
    // for-statement's own scope is still active.
    assert_eq!(scope.resolve("body_local"), None);
    assert_eq!(scope.resolve("i"), Some(&int_ty()));

    scope.pop(); // leave the for-statement's own scope
    assert_eq!(scope.resolve("i"), None);
}

#[test]
fn local_shadows_a_global_of_the_same_name() {
    let mut globals = HashMap::new();
    globals.insert("count".to_string(), ptr(int_ty()));
    let mut scope = Scope::new(&globals);
    // A local `int count;` shadowing a global `int *count;`.
    scope.declare("count", int_ty());
    assert_eq!(scope.resolve("count"), Some(&int_ty()));
}

#[test]
fn parameters_stay_visible_through_the_whole_function() {
    let globals = HashMap::new();
    let mut scope = Scope::new(&globals);
    scope.declare("arg", array(int_ty()));

    scope.push();
    scope.push();
    assert_eq!(scope.resolve("arg"), Some(&array(int_ty())));
    scope.pop();
    scope.pop();
}
