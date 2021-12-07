use criterion::{criterion_group, criterion_main, Criterion};
mod day_07;
mod day_06;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day7 part 2", |b| b.iter(|| day_07::part_2()));
    c.bench_function("day6 part 2", |b| b.iter(|| day_06::part_2()));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
