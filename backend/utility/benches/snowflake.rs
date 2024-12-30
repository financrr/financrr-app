use criterion::{black_box, criterion_group, criterion_main, Criterion};

use utility::snowflake::generator::SnowflakeGenerator;

pub fn next_id_benchmark(c: &mut Criterion) {
    let generator = SnowflakeGenerator::new(1, 0).expect("Failed to create SnowflakeGenerator");
    c.bench_function("next_id", |b| b.iter(|| black_box(generator.next_id().unwrap())));
}

pub fn next_benchmark(c: &mut Criterion) {
    let generator = SnowflakeGenerator::new(1, 0).expect("Failed to create SnowflakeGenerator");
    c.bench_function("next", |b| b.iter(|| black_box(generator.next().unwrap())));
}

criterion_group!(benches, next_id_benchmark, next_benchmark);
criterion_main!(benches);
