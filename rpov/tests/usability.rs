#[cfg(test)]
mod tests {
    use rpov::{make_tuple, make_vector, point};

    #[test]
    fn test_point_display() {
        let p = point(1.0, 2.0, 3.0);
        assert_eq!(format!("{}", p), "point(1, 2, 3)");
    }

    #[test]
    fn test_vector_display() {
        let v = make_vector(1.0, 2.0, 3.0);
        assert_eq!(format!("{}", v), "vector(1, 2, 3)");
    }

    #[test]
    fn test_tuple_display() {
        let t = make_tuple(1.0, 2.0, 3.0, 4.0);
        assert_eq!(format!("{}", t), "tuple(1, 2, 3, 4)");
        assert_eq!(
            format!("{:?}", t),
            "Tuple4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 }"
        );
    }

    #[test]
    fn test_color_display() {
        let c = rpov::colors::Color::new(-0.5, 0.4, 1.7);
        assert_eq!(format!("{}", c), "Color(r=-0.5, g=0.4, b=1.7)");
        assert_eq!(
            format!("{:?}", c),
            "Color { red: -0.5, green: 0.4, blue: 1.7 }"
        );
    }

    #[test]
    fn test_matrix_identity_display() {
        let m: rpov::matrices::Matrix<f32, 3> = rpov::matrices::Matrix3::identity();
        assert_eq!(
            format!("{:?}", m),
            "Matrix { data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] }"
        );
    }
}
