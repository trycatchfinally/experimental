use criterion::{Criterion, criterion_group, criterion_main};
use rpov::{matrices::Matrix4, tuples::Tuple4};

fn matrix_tuple_compare(c: &mut Criterion) {
    let a = Matrix4::from([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 8.0, 16.0],
        [3.0, 6.0, 9.0, 12.0],
        [4.0, 8.0, 16.0, 32.0],
    ]);
    let t = Tuple4::new(1.0, 1.0, 1.0, 1.0);
    let mut g = c.benchmark_group("multiply_tuple");
    g.bench_function("dot_product", |b| {
        b.iter(|| {
            // let _ = black_box(black_box(a).multiply_tuple(&black_box(  t)));
            std::hint::black_box(a.multiply_tuple_dot(&t))
        })
    });
    g.bench_function("accumulate", |b| {
        b.iter(|| {
            // let _ = black_box(black_box(a).multiply_tuple1(&black_box(t)));
            std::hint::black_box(a.multiply_tuple(&t))
        })
    });
}
criterion_group!(benches, matrix_tuple_compare);
criterion_main!(benches);
