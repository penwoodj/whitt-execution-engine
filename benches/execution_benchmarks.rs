use criterion::{criterion_group, criterion_main};

fn bench_placeholder(_criterion: &mut criterion::Criterion) {
    // Benchmarks will be added per-phase as features are implemented.
    // Phase 03 (Quality Loops) will add step execution latency benchmarks.
    // Phase 04 (Memory Search) will add search query latency benchmarks.
}

criterion_group!(benches, bench_placeholder);
criterion_main!(benches);
