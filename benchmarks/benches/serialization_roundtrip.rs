use criterion::{Criterion, criterion_group, criterion_main};
use openfiat_benchmarks::roundtrip_stub;

fn bench_roundtrip(c: &mut Criterion) {
    let payload = vec![0u8; 1024];
    c.bench_function("roundtrip_stub_1kb", |b| {
        b.iter(|| roundtrip_stub(std::hint::black_box(&payload)))
    });
}

criterion_group!(benches, bench_roundtrip);
criterion_main!(benches);
