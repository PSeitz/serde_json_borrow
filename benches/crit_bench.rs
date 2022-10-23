use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use serde_json_borrow::OwnedValue;

pub fn bench_for_lines<'a, F, I>(
    c: &mut Criterion,
    iter_gen: F,
    group_name: &str,
    payload_size: u64,
) where
    F: Fn() -> I,
    I: Iterator<Item = String>,
{
    let mut group = c.benchmark_group(group_name);
    group.throughput(Throughput::Bytes(payload_size));
    group.bench_function("serde-json-owned", |b| {
        b.iter(|| {
            let mut val = None;
            for line in iter_gen() {
                let json: serde_json::Value = serde_json::from_str(&line).unwrap();
                val = Some(json);
            }
            val
        })
    });
    group.bench_function("serde-json-borrowed", |b| {
        b.iter(|| {
            let mut is_bool = false;
            for line in iter_gen() {
                let json: serde_json_borrow::Value = serde_json::from_str(&line).unwrap();
                is_bool = json.is_bool();
            }
            is_bool
        })
    });

    group.bench_function("serde-json-borrowed-owned", |b| {
        b.iter(|| {
            let mut val = None;
            for line in iter_gen() {
                let json: OwnedValue = OwnedValue::parse_from(line).unwrap();
                val = Some(json);
            }
            val
        })
    });
}

pub fn simple_json_to_doc_benchmark(c: &mut Criterion) {
    let lines_for_file = |file| {
        BufReader::new(File::open(file).unwrap())
            .lines()
            .map(|line| line.unwrap())
    };

    let file = "./benches/simple-parse-bench.json";
    bench_for_lines(
        c,
        || lines_for_file(file),
        "simple_json",
        File::open(file).unwrap().metadata().unwrap().len(),
    );

    let file = "./benches/hdfs.json";
    bench_for_lines(
        c,
        || lines_for_file(file),
        "hdfs_json",
        File::open(file).unwrap().metadata().unwrap().len(),
    );
}

criterion_group!(benches, simple_json_to_doc_benchmark);
criterion_main!(benches);
