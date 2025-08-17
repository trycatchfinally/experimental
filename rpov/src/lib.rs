pub mod camera;
pub mod canvas;
pub mod colors;
pub mod floats;
pub mod intersections;
pub mod lighting;
pub mod materials;
pub mod matrices;
pub mod normals;
pub mod patterns;
pub mod planes;
pub mod rays;
pub mod shapes;
pub mod spheres;
pub mod transformations;
pub mod tuples;
pub mod world;

#[macro_export]
macro_rules! assert_same_object {
    ($a:expr, $b:expr) => {
        let a_ptr = ($a) as *const _ as *const ();
        let b_ptr = ($b) as *const _ as *const ();
        assert_eq!(a_ptr, b_ptr, "Objects do not have the same memory address");
    };
}
