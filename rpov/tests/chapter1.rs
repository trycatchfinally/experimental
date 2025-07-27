use std::collections::HashMap;

use assert_float_eq::assert_float_absolute_eq as assert_eq_float;
use cucumber::{World as _, given, then};

use rpov::{Float, Plus, PointOrVector, Tuple, make_point, make_vector};

#[derive(Debug, Default, cucumber::World)]
struct TheWorld {
    vars: HashMap<String, Tuple>,
}

#[given(expr = r"{word} ← {tuple}")]
fn given_expr(world: &mut TheWorld, var: String, t: Tuple) {
    world.vars.insert(var, t);
}

#[then(expr = "{word}.{word} = {float}")]
fn check_x_value(world: &mut TheWorld, var: String, field: String, val: f32) {
    let found = world.vars.get(&var).unwrap();
    let actual = match field.as_str() {
        "x" => found.x,
        "y" => found.y,
        "z" => found.z,
        "w" => found.w,
        _ => panic!("not found: {}", field),
    };
    assert_eq_float!(actual, val);
}

#[then(expr = "{word} is a {word}")]
fn a_is_a_point_or_vector(world: &mut TheWorld, var: String, p_or_v: String) {
    let found = world.vars.get(&var).unwrap();
    match p_or_v.as_str() {
        "point" => assert!(found.is_point()),
        "vector" => assert!(found.is_vector()),
        _ => panic!("not found: {}", p_or_v),
    }
}

#[then(expr = "{word} is not a {word}")]
fn a_is_not_a_point_or_vector(world: &mut TheWorld, var: String, p_or_v: String) {
    let found = world.vars.get(&var).unwrap();
    match p_or_v.as_str() {
        "point" => assert!(!found.is_point()),
        "vector" => assert!(!found.is_vector()),
        _ => panic!("not found: {}", p_or_v),
    }
}

#[tokio::main]
async fn main() {
    TheWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/01-tuples.feature")
        .await;
}

#[given(expr = r"{word} ← point\({float}, {float}, {float}\)")]
fn p_assign_point(world: &mut TheWorld, var: String, x: Float, y: Float, z: Float) {
    let p = make_point(x, y, z);
    assert!(p.is_point());
    world.vars.insert(var, p);
}
#[given(expr = r"{word} ← vector\({float}, {float}, {float}\)")]
fn p_assign_vector(world: &mut TheWorld, var: String, x: Float, y: Float, z: Float) {
    let v = make_vector(x, y, z);
    assert!(v.is_vector());
    world.vars.insert(var, v);
    // Write code here that turns the phrase above into concrete actions
}

#[then(expr = r"{word} = {tuple}")]
fn var_eq_tuple(world: &mut TheWorld, var: String, given: Tuple) {
    let found = world.vars.get(&var).unwrap();
    assert_eq_float!(found.x, given.x);
    assert_eq_float!(found.y, given.y);
    assert_eq_float!(found.z, given.z);
    assert_eq_float!(found.w, given.w);
}

#[then(expr = r"{word} + {word} = {tuple}")]
fn add_tuples(world: &mut TheWorld, v1: String, v2: String, expected: Tuple) {
    let found1 = world.vars.get(&v1).unwrap();
    let found2 = world.vars.get(&v2).unwrap();
    let actual = found1.plus(*found2);
    assert_eq_float!(actual.x, expected.x);
    assert_eq_float!(actual.y, expected.y);
    assert_eq_float!(actual.z, expected.z);
    assert_eq_float!(actual.w, expected.w);
}
