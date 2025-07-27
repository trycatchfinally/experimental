use num_traits::AsPrimitive;
pub type Int = i32;
pub type Float = f32;
pub type Tuple4 = (Float, Float, Float, Float);

pub const W_POINT: Float = 1.0;
pub const W_VECTOR: Float = 0.0;

pub fn make_tuple<T: AsPrimitive<Float>>(x: T, y: T, z: T, w: T) -> Tuple4 {
    (x.as_(), y.as_(), z.as_(), w.as_())
}

pub fn make_point(x: Float, y: Float, z: Float) -> Tuple4 {
    (x, y, z, W_POINT)
}
pub fn make_vector(x: Float, y: Float, z: Float) -> Tuple4 {
    (x, y, z, W_VECTOR)
}

pub trait Plus {
    fn plus(self, other: Self) -> Self;
}
impl Plus for Tuple4 {
    // type Output = Self;

    fn plus(self, other: Self) -> Self {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

pub trait PointOrVector {
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
}

impl PointOrVector for Tuple4 {
    fn is_point(&self) -> bool {
        self.3 as Int == W_POINT as Int
    }

    fn is_vector(&self) -> bool {
        self.3 as Int == W_VECTOR as Int
    }
}

pub fn parse_tuple(s: &str) -> Result<Tuple4, &str> {
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
    use crate::Plus;
    use crate::make_tuple;
    use crate::parse_tuple;
    #[test]
    fn test_add() {
        let a1 = make_tuple(3, -2, 5, 1);
        let a2 = make_tuple(-2, 3, 1, 0);
        assert!(a1.plus(a2) == make_tuple(1, 1, 6, 1));
    }

    #[test]
    fn test_parse_tuple() {
        let s = "tuple1(4.3, -4.2, 3.1, 1.0)";
        let result = parse_tuple(s).unwrap();
        assert_eq!(result, (4.3, -4.2, 3.1, 1.0));
    }
}
