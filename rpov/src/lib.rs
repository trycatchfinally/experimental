pub mod canvas;
pub mod colors;
pub mod matrices;

use std::{
    ops::{Div, Mul, Neg},
    str::FromStr,
};

use cucumber::{Parameter, parser::Error};
use num_traits::AsPrimitive;
pub type Int = i32;
pub type Float = f32;
pub type RawTuple4 = (Float, Float, Float, Float);

pub const W_POINT: Float = 1.0;
pub const W_VECTOR: Float = 0.0;

pub fn make_tuple<T: AsPrimitive<Float>>(x: T, y: T, z: T, w: T) -> Tuple {
    Tuple {
        x: x.as_(),
        y: y.as_(),
        z: z.as_(),
        w: w.as_(),
    }
}

pub fn make_point(x: Float, y: Float, z: Float) -> Tuple {
    make_tuple(x, y, z, W_POINT)
}
pub fn make_vector(x: Float, y: Float, z: Float) -> Tuple {
    make_tuple(x, y, z, W_VECTOR)
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Tuple4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Tuple4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Tuple4 { x, y, z, w }
    }
}

#[derive(Debug, Default, Parameter, Clone, Copy, PartialEq)]
#[param(
    name = "tuple",
    regex = r"tuple\([+-]?([0-9]*[.])?[0-9]+, [+-]?([0-9]*[.])?[0-9]+, [+-]?([0-9]*[.])?[0-9]+, [+-]?([0-9]*[.])?[0-9]+\)"
)]
pub struct Tuple {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}

impl Tuple {
    pub fn new(x: Float, y: Float, z: Float, w: Float) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn magnitude(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        if mag == 0.0 {
            panic!("Cannot normalize a zero vector");
        }
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: Tuple) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Tuple) -> Tuple {
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

    pub fn reflect(&self, normal: Tuple) -> Tuple {
        assert!(
            self.is_vector() && normal.is_vector(),
            "Reflection is only defined for vectors"
        );
        let dot_product = self.dot(normal);
        make_vector(
            self.x - 2.0 * dot_product * normal.x,
            self.y - 2.0 * dot_product * normal.y,
            self.z - 2.0 * dot_product * normal.z,
        )
    }
}

impl FromStr for Tuple {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = parse_tuple4(s).unwrap();
        Ok(Tuple {
            x: t.0,
            y: t.1,
            z: t.2,
            w: t.3,
        })
    }
}

pub trait PlusMinus: Neg + Mul<Float, Output = Self> + Div<f32, Output = Self> {
    fn plus(self, other: Self) -> Self;
    fn minus(self, other: Self) -> Self;
}

impl PlusMinus for Tuple {
    fn plus(self, other: Self) -> Self {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    fn minus(self, other: Self) -> Self {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::Mul<Float> for Tuple {
    type Output = Tuple;

    fn mul(self, other: Float) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl std::ops::Div<Float> for Tuple {
    type Output = Tuple;

    fn div(self, other: Float) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
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

impl PointOrVector for RawTuple4 {
    fn is_point(&self) -> bool {
        self.3 as Int == W_POINT as Int
    }

    fn is_vector(&self) -> bool {
        self.3 as Int == W_VECTOR as Int
    }
}

impl PointOrVector for Tuple {
    fn is_point(&self) -> bool {
        self.w as Int == W_POINT as Int
    }

    fn is_vector(&self) -> bool {
        self.w as Int == W_VECTOR as Int
    }
}

pub fn parse_tuple4(s: &str) -> Result<RawTuple4, &str> {
    let s = s.trim();
    let start = s.find('(');
    let end = s.find(')');

    if start.is_none() || end.is_none() {
        return Err("Invalid tuple string: missing parentheses");
    }

    let start = start.unwrap();
    let end = end.unwrap();

    if start >= end {
        return Err("Invalid tuple string: invalid parentheses");
    }

    let numbers_str = &s[start + 1..end];
    let parts: Vec<&str> = numbers_str.split(',').collect();

    if parts.len() != 4 {
        return Err("Invalid tuple string: wrong number of components");
    }

    let mut values = [0.0; 4];
    for (i, part) in parts.iter().enumerate() {
        match part.trim().parse::<Float>() {
            Ok(num) => values[i] = num,
            Err(_) => return Err("Invalid number in tuple string"),
        }
    }

    Ok((values[0], values[1], values[2], values[3]))
}

#[cfg(test)]
mod test {
    use crate::PlusMinus;
    use crate::make_tuple;
    use crate::parse_tuple4;

    #[test]
    fn test_add() {
        let a1 = make_tuple(3, -2, 5, 1);
        let a2 = make_tuple(-2, 3, 1, 0);
        assert!(a1.plus(a2) == make_tuple(1, 1, 6, 1));
    }

    #[test]
    fn test_parse_tuple() {
        let s = "tuple1(4.3, -4.2, 3.1, 1.0)";
        let result = parse_tuple4(s).unwrap();
        assert_eq!(result, (4.3, -4.2, 3.1, 1.0));
    }

    #[test]
    fn color_components_are_red_green_blue() {
        let c = crate::colors::Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }
}
