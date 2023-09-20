//! Benchmark for the CIC filter.  
//!
//! # Example
//!
//! ```
//! cargo criterion
//! ```
//!

use cic_fixed::CicDecimationFilter;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn cic_decimation_benchmark(c: &mut Criterion) {
    let mut filter = CicDecimationFilter::<64, 5>::new();
    c.bench_function("cic", |b| {
        b.iter(|| {
            if let Some(_x) = filter.filter(black_box(&1)) {
                // do nothing
            }
            if let Some(_x) = filter.filter(black_box(&-1)) {
                // do nothing
            }
        })
    });
}

criterion_group!(benches, cic_decimation_benchmark);
criterion_main!(benches);
