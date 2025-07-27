use std::collections::HashMap;

use assert_float_eq::assert_float_absolute_eq as assert_eq_float;
use cucumber::{World as _, given, then, when};

use rpov::{Float, PlusMinus, PointOrVector, Tuple, make_point, make_vector};

#[derive(Debug, Default, cucumber::World)]
struct TheWorld {
    vars: HashMap<String, Tuple>,
}

impl TheWorld {
    fn get(&self, var: &str) -> Tuple {
        // If the variable starts with a '-', negate the value
        if var.starts_with('-') {
            let var = var.trim_start_matches('-');
            -*self
                .vars
                .get(var)
                .unwrap_or_else(|| panic!("Variable not found: {}", var))
        } else {
            *self
                .vars
                .get(var)
                .unwrap_or_else(|| panic!("Variable not found: {}", var))
        }
    }

    fn insert(&mut self, var: String, value: Tuple) {
        assert!(
            !self.vars.contains_key(&var),
            "Variable already exists: {}",
            var
        );
        self.vars.insert(var, value);
    }
}

#[given(expr = r"{word} ← {tuple}")]
fn given_expr(world: &mut TheWorld, var: String, t: Tuple) {
    world.insert(var, t);
}

#[then(expr = "{word}.{word} = {float}")]
fn check_x_value(world: &mut TheWorld, var: String, field: String, val: f32) {
    let found = world.get(&var);
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
    let found = world.get(&var);
    match p_or_v.as_str() {
        "point" => assert!(found.is_point()),
        "vector" => assert!(found.is_vector()),
        _ => panic!("not found: {}", p_or_v),
    }
}

#[then(expr = "{word} is not a {word}")]
fn a_is_not_a_point_or_vector(world: &mut TheWorld, var: String, p_or_v: String) {
    let found = world.get(&var);
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
    world.insert(var, p);
}
#[given(expr = r"{word} ← vector\({float}, {float}, {float}\)")]
fn p_assign_vector(world: &mut TheWorld, var: String, x: Float, y: Float, z: Float) {
    let v = make_vector(x, y, z);
    assert!(v.is_vector());
    world.insert(var, v);
}

#[then(expr = r"{word} - {word} = vector\({float}, {float}, {float}\)")]
fn p_x_minus_y_equals_vector(
    world: &mut TheWorld,
    a: String,
    b: String,
    x: Float,
    y: Float,
    z: Float,
) {
    let found_a = world.get(&a);
    let found_b = world.get(&b);
    let actual = found_a.minus(found_b);
    assert!(actual.is_vector());
    assert_eq_float!(actual.x, x);
    assert_eq_float!(actual.y, y);
    assert_eq_float!(actual.z, z);
}
#[then(expr = r"{word} - {word} = point\({float}, {float}, {float}\)")]
fn p_x_minus_y_equals_point(
    world: &mut TheWorld,
    a: String,
    b: String,
    x: Float,
    y: Float,
    z: Float,
) {
    let found_a = world.get(&a);
    let found_b = world.get(&b);
    let actual = found_a.minus(found_b);
    assert!(actual.is_point());
    assert_eq_float!(actual.x, x);
    assert_eq_float!(actual.y, y);
    assert_eq_float!(actual.z, z);
}

#[then(expr = r"{word} = {tuple}")]
fn var_eq_tuple(world: &mut TheWorld, var: String, given: Tuple) {
    let found = world.get(&var);
    assert_eq_float!(found.x, given.x);
    assert_eq_float!(found.y, given.y);
    assert_eq_float!(found.z, given.z);
    assert_eq_float!(found.w, given.w);
}

#[then(expr = r"{word} + {word} = {tuple}")]
fn add_tuples(world: &mut TheWorld, v1: String, v2: String, expected: Tuple) {
    let found1 = world.get(&v1);
    let found2 = world.get(&v2);
    let actual = found1.plus(found2);
    assert_eq_float!(actual.x, expected.x);
    assert_eq_float!(actual.y, expected.y);
    assert_eq_float!(actual.z, expected.z);
    assert_eq_float!(actual.w, expected.w);
}

#[then(expr = r"{word} * {float} = {tuple}")]
fn var_times_float_tuple(world: &mut TheWorld, var: String, f: Float, expected: Tuple) {
    let found = world.get(&var);
    let actual = found * f;
    assert_eq_float!(actual.x, expected.x);
    assert_eq_float!(actual.y, expected.y);
    assert_eq_float!(actual.z, expected.z);
    assert_eq_float!(actual.w, expected.w);
}

#[then(expr = r"{word} \/ {float} = {tuple}")]
fn var_div_float_tuple(world: &mut TheWorld, var: String, f: Float, expected: Tuple) {
    let found = world.get(&var);
    let actual = found / f;
    assert_eq_float!(actual.x, expected.x);
    assert_eq_float!(actual.y, expected.y);
    assert_eq_float!(actual.z, expected.z);
    assert_eq_float!(actual.w, expected.w);
}

#[then(expr = r"magnitude\({word}\) = {float}")]
fn magnitude_var_float(world: &mut TheWorld, var: String, expected: Float) {
    let found = world.get(&var);
    let actual = found.magnitude();
    assert_eq_float!(actual, expected);
}

#[then(expr = r"magnitude\({word}\) = √{float}")]
fn magnitude_var_sqrt_float(world: &mut TheWorld, var: String, expected: Float) {
    magnitude_var_float(world, var, expected.sqrt());
}

#[then(expr = r"normalize\({word}\) =( approximately) vector\({float}, {float}, {float}\)")]
fn normalize_eq_vector(world: &mut TheWorld, var: String, x: Float, y: Float, z: Float) {
    let found = world.get(&var);
    let actual = found.normalize();
    assert!(actual.is_vector());
    assert_eq_float!(actual.x, x, 1e-4);
    assert_eq_float!(actual.y, y, 1e-4);
    assert_eq_float!(actual.z, z, 1e-4);
}

#[when(expr = r"{word} ← normalize\({word}\)")]
fn norm_normalize_v(world: &mut TheWorld, var: String, input: String) {
    let found = world.get(&input);
    let normalized = found.normalize();
    world.insert(var, normalized);
}

#[then(expr = r"dot\({word}, {word}\) = {float}")]
fn dot_a_b_equals_f(world: &mut TheWorld, a: String, b: String, expected: Float) {
    let found_a = world.get(&a);
    let found_b = world.get(&b);
    let actual = found_a.dot(found_b);
    assert_eq_float!(actual, expected);
}

#[then(expr = r"cross\({word}, {word}\) = vector\({float}, {float}, {float}\)")]
fn cross_a_b_equals_vector(
    world: &mut TheWorld,
    a: String,
    b: String,
    x: Float,
    y: Float,
    z: Float,
) {
    let expected = make_vector(x, y, z);
    let found_a = world.get(&a);
    let found_b = world.get(&b);
    let actual = found_a.cross(found_b);
    assert!(actual.is_vector());
    assert_eq_float!(actual.x, expected.x);
    assert_eq_float!(actual.y, expected.y);
    assert_eq_float!(actual.z, expected.z);
}

#[when(expr = r"{word} ← reflect\({word}, {word}\)")]
fn r_reflect_v_n(world: &mut TheWorld, dest: String, v: String, n: String) {
    let found_v = world.get(&v);
    let found_n = world.get(&n);
    let reflected = found_v.reflect(found_n);
    world.insert(dest, reflected);
}

#[then(expr = r"value\({word}\) = vector\({float}, {float}, {float}\)")]
fn r_eq_vector(world: &mut TheWorld, var: String, x: Float, y: Float, z: Float) {
    let found = world.get(&var);
    let expected = make_vector(x, y, z);
    assert!(found.is_vector());
    assert_eq_float!(found.x, expected.x);
    assert_eq_float!(found.y, expected.y);
    assert_eq_float!(found.z, expected.z);
}
