use std::{collections::HashMap, str::FromStr};

use assert_float_eq::assert_float_absolute_eq as assert_eq_float;
use cucumber::{
    Parameter, World as _,
    codegen::anyhow::{Error, Ok},
    given, then,
};

use rpov::{make_point, make_vector, parse_tuple, Float, PointOrVector, Tuple4, Plus};

#[derive(Debug, Default, cucumber::World)]
struct TheWorld {
    vars: HashMap<String, Tuple4>,
}

#[derive(Debug, Default, Parameter)]
#[param(
    name = "tuple1",
    regex = r"tuple1\([+-]?([0-9]*[.])?[0-9]+, [+-]?([0-9]*[.])?[0-9]+, [+-]?([0-9]*[.])?[0-9]+, [+-]?([0-9]*[.])?[0-9]+\)"
)]
struct Tuple1 {
    x: Float,
    y: Float,
    z: Float,
    w: Float,
}

impl FromStr for Tuple1 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = parse_tuple(s).unwrap();
        Ok(Tuple1 {
            x: t.0,
            y: t.1,
            z: t.2,
            w: t.3,
        })
    }
}

#[given(expr = r"{word} <- tuple\({float}, {float}, {float}, {float}\)")]
fn given_expr(world: &mut TheWorld, var: String, x: Float, y: Float, z: Float, w: Float) {
    world.vars.insert(var, (x, y, z, w));
}

#[then(expr = "{word}.{word} = {float}")]
fn check_x_value(world: &mut TheWorld, var: String, field: String, val: f32) {
    let found = world.vars.get(&var).unwrap();
    let actual = match field.as_str() {
        "x" => found.0,
        "y" => found.1,
        "z" => found.2,
        "w" => found.3,
        _ => panic!("not found: {}", field),
    };
    assert_eq!(actual, val);
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

#[given(expr = r"{word} <- point\({float}, {float}, {float}\)")]
fn p_assign_point(world: &mut TheWorld, var: String, x: Float, y: Float, z: Float) {
    let p = make_point(x, y, z);
    assert!(p.is_point());
    world.vars.insert(var, p);
}
#[given(expr = r"{word} <- vector\({float}, {float}, {float}\)")]
fn p_assign_vector(world: &mut TheWorld, var: String, x: Float, y: Float, z: Float) {
    let v = make_vector(x, y, z);
    assert!(v.is_vector());
    world.vars.insert(var, v);
    // Write code here that turns the phrase above into concrete actions
}

#[then(expr = r"{word} = tuple\({float}, {float}, {float}, {float})")]
fn var_eq_tuple(world: &mut TheWorld, var: String, x: Float, y: Float, z: Float, w: Float) {
    let found = world.vars.get(&var).unwrap();
    assert_eq_float!(found.0, x);
    assert_eq_float!(found.1, y);
    assert_eq_float!(found.2, z);
    assert_eq_float!(found.3, w);
}

#[then(expr = r"{word} + {word} = tuple\({float}, {float}, {float}, {float}\)")]
fn add_tuples(
    world: &mut TheWorld,
    v1: String,
    v2: String,
    x: Float,
    y: Float,
    z: Float,
    w: Float,
) {
    let found1 = world.vars.get(&v1).unwrap();
    let found2 = world.vars.get(&v2).unwrap();
    let actual = found1.plus(*found2);
    assert_eq_float!(actual.0, x);
    assert_eq_float!(actual.1, y);
    assert_eq_float!(actual.2, z);
    assert_eq_float!(actual.3, w);
}

#[given(expr = "{word} <- {tuple1}")]
fn b_lt_tuple(_world: &mut TheWorld, var: String, t1: Tuple1) {
    // Write code here that turns the phrase above into concrete actions
    panic!("xyz: {} {:?}", var, t1)
}
