// Floating-point precision is configured here
#[cfg(feature = "f32")]
pub type Float = f32;
#[cfg(feature = "f32")]
pub use std::f32 as float_consts;

#[cfg(not(feature = "f32"))]
pub type Float = f64;

#[cfg(not(feature = "f32"))]
pub use std::f64 as float_consts;

#[cfg(test)]
mod tests {
    use super::Float;
    #[test]
    fn check_float_type() {
        if cfg!(feature = "f32") {
            assert_eq!(std::any::type_name::<Float>(), "f32");
        } else {
            assert_eq!(std::any::type_name::<Float>(), "f64");
        }
    }
}
