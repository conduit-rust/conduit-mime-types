extern crate conduit_mime_types;
extern crate criterion;

use conduit_mime_types::Types;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Types::new()", |b| b.iter(|| Types::new()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
