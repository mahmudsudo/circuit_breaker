use criterion::{black_box, criterion_group, criterion_main, Criterion};
use circuit_breaker::CircuitBreaker;
use std::time::Duration;

fn circuit_breaker_benchmark(c: &mut Criterion) {
    let cb = CircuitBreaker::new(3, Duration::from_secs(60));

    c.bench_function("execute successful operation", |b| {
        b.iter(|| {
            cb.execute(|| Ok::<_, std::io::Error>(black_box("success")))
        })
    });

    // Add more benchmarks as needed
}

criterion_group!(benches, circuit_breaker_benchmark);
criterion_main!(benches);
