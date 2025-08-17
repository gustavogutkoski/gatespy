use criterion::{criterion_group, criterion_main, Criterion};
use gatespy::scan_host;

fn bench_scan(c: &mut Criterion) {
    c.bench_function("scan localhost", |b| {
        b.iter(|| scan_host("127.0.0.1"))
    });
}

criterion_group!(benches, bench_scan);
criterion_main!(benches);
