use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nqueens::NQueens;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("nqueens 10", |b| {
        b.iter(|| {
            let mut nqueen = NQueens::new(black_box(10)).unwrap().into_random_state();
            while nqueen.step() != 0 {}
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
