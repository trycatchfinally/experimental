pub mod canvas;
pub mod colors;
pub mod matrices;
pub mod rays;
pub mod transformations;
pub mod tuples;

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg},
};
pub const W_POINT: f32 = 1.0;
pub const W_VECTOR: f32 = 0.0;

pub fn make_tuple<T: TupleElement>(x: T, y: T, z: T, w: T) -> Tuple4<T> {
    Tuple4 { x, y, z, w }
}

pub fn point<T: TupleElement>(x: T, y: T, z: T) -> Tuple4<T> {
    let w: T = T::from(W_POINT).unwrap_or_else(|| {
        panic!(
            "Failed to convert W_POINT to type {}",
            std::any::type_name::<T>()
        )
    });
    make_tuple(x, y, z, w)
}
pub fn make_vector<T: TupleElement>(x: T, y: T, z: T) -> Tuple4<T> {
    let w: T = T::from(W_VECTOR).unwrap_or_else(|| {
        panic!(
            "Failed to convert W_VECTOR to type {}",
            std::any::type_name::<T>()
        )
    });
    make_tuple(x, y, z, w)
}
pub fn vector<T: TupleElement>(x: T, y: T, z: T) -> Tuple4<T> {
    let w: T = T::from(W_VECTOR).unwrap_or_else(|| {
        panic!(
            "Failed to convert W_VECTOR to type {}",
            std::any::type_name::<T>()
        )
    });
    make_tuple(x, y, z, w)
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Tuple4<T: TupleElement> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub fn check_tuple<T: TupleElement>(actual: Tuple4<T>, expected: Tuple4<T>) {
    let eps = T::from(0.00001).expect("Failed to convert 0.00001 to type T");
    assert!(
        (actual.x - expected.x).abs() <= eps,
        "X value check failed: got {}, expected {}",
        actual.x,
        expected.x
    );
    assert!(
        (actual.y - expected.y).abs() <= eps,
        "Y value check failed: got {}, expected {}",
        actual.y,
        expected.y
    );
    assert!(
        (actual.z - expected.z).abs() <= eps,
        "Z value check failed: got {}, expected {}",
        actual.z,
        expected.z
    );
    assert!(
        (actual.w - expected.w).abs() <= eps,
        "W value check failed: got {}, expected {}",
        actual.w,
        expected.w
    );

    assert!(
        actual.is_point() == expected.is_point(),
        "Point check failed: got {}, expected {}",
        actual.is_point(),
        expected.is_point()
    );
}

impl<T: TupleElement> Display for Tuple4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl<T: TupleElement> Tuple4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Tuple4 { x, y, z, w }
    }
}

pub trait TupleElement:
    Mul<Output = Self>
    + Add<Output = Self>
    + num_traits::Float
    + Copy
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + Default
    + Display
    + std::fmt::Debug
{
}

impl TupleElement for f32 {}
impl TupleElement for f64 {}

impl<T: TupleElement> Tuple4<T> {
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Tuple4<T> {
        let mag = self.magnitude();
        if mag == T::zero() {
            panic!("Cannot normalize a zero vector");
        }
        Tuple4 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: Tuple4<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Tuple4<T>) -> Tuple4<T> {
        assert!(
            self.is_vector() && other.is_vector(),
            "Cross product is only defined for vectors"
        );
        make_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: Tuple4<T>) -> Tuple4<T> {
        assert!(
            self.is_vector() && normal.is_vector(),
            "Reflection is only defined for vectors"
        );
        let dot_product = self.dot(normal);
        let two = T::from(2.0).unwrap_or_else(|| {
            panic!(
                "Failed to convert 2.0 to type {}",
                std::any::type_name::<T>()
            )
        });
        make_vector(
            self.x - two * dot_product * normal.x,
            self.y - two * dot_product * normal.y,
            self.z - two * dot_product * normal.z,
        )
    }
}

impl<T: TupleElement> std::ops::Add<Tuple4<T>> for Tuple4<T> {
    type Output = Tuple4<T>;
    fn add(self, other: Self) -> Self {
        Tuple4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl<T: TupleElement> std::ops::Sub<Tuple4<T>> for Tuple4<T> {
    type Output = Tuple4<T>;
    fn sub(self, other: Self) -> Self {
        Tuple4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: TupleElement> std::ops::Mul<T> for Tuple4<T> {
    type Output = Tuple4<T>;

    fn mul(self, other: T) -> Tuple4<T> {
        Tuple4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl<T: TupleElement> std::ops::Div<T> for Tuple4<T> {
    type Output = Tuple4<T>;

    fn div(self, other: T) -> Tuple4<T> {
        Tuple4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl<T: TupleElement> std::ops::Neg for Tuple4<T> {
    type Output = Tuple4<T>;

    fn neg(self) -> Tuple4<T> {
        Tuple4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

pub trait PointOrVector {
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
}

impl<T: TupleElement> PointOrVector for Tuple4<T> {
    fn is_point(&self) -> bool {
        self.w.trunc().to_i32().expect("expected w") == 1
        // T::to_i32(W_POINT).expect("expected W_POINT")
    }

    fn is_vector(&self) -> bool {
        self.w.trunc().to_i32().expect("expected w") == 0
    }
}

#[cfg(test)]
mod test {
    use crate::Tuple4;
    use crate::make_tuple;

    #[test]
    fn test_add() {
        let a1: Tuple4<f32> = make_tuple(3_f32, -2_f32, 5_f32, 1_f32);
        let a2: Tuple4<f32> = make_tuple(-2_f32, 3_f32, 1_f32, 0_f32);
        let c = a1 + a2;
        assert!(c == make_tuple(1_f32, 1_f32, 6_f32, 1_f32));
    }

    #[test]
    fn color_components_are_red_green_blue() {
        let c = crate::colors::Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }
}
