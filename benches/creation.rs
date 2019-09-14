#[macro_use]
extern crate criterion;
use criterion::black_box;
use criterion::Criterion;
use crossbeam_utils::thread;
use std::sync::Arc;

use ustring::*;

fn create_ustrings(blns: &String, num: usize) {
    for s in blns.split_whitespace().cycle().take(num) {
        black_box(u!(s));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let path = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("data")
        .join("blns.txt");
    let blns = Arc::new(std::fs::read_to_string(path).unwrap());

    // there are 2146 tokens in blns.txt, so this will find an already-existing
    // string ~4 times for every string created
    // First pass with a HashMap gives ~88ns per creation
    let s = blns.clone();
    c.bench_function("create 10k", move |b| {
        let s = s.clone();
        b.iter(|| {
            _clear_cache();
            create_ustrings(&s, 10_000);
        });
    });

    // test lookups.
    // First pass gives ~1ns for the lookup
    let ustrings: Vec<UString> = blns.split_whitespace().map(|s| u!(s)).collect();
    c.bench_function("lookup", move |b| {
        let us = &ustrings;
        b.iter(|| {
            for u in us {
                black_box({
                    u.as_str();
                })
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);