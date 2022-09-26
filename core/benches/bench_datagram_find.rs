use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jutsu_core::Find;



fn criterion_benchmark(c: &mut Criterion) {
    let buf = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 102, 0, 6, 113, 119, 101, 114, 116, 121, 105];

    // c.bench_function("Find::from_buf", |b| b.iter(|| Find::from_buf(black_box(&buf))));
    c.bench_function("Find::result_from_buf", |b| b.iter(|| Find::result_from_buf(black_box(&buf))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);