use criterion::{criterion_group, criterion_main, Criterion};
use jutsu_core::MemInfo;

fn criterion_benchmark(c: &mut Criterion) {
    // let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();

    c.bench_function("MemInfo::new", |b| b.iter(|| MemInfo::new()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);