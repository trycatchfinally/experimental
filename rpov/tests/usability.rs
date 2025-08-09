#[cfg(test)]
mod tests {
    use rpov::{make_vector, point};

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
}
