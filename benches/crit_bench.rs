use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use serde_json_borrow::OwnedValue;

const JSON_TEST_DATA_SIMPLE: &str = include_str!("simple-parse-bench.json");
const JSON_TEST_DATA_HDFS_LOG: &str = include_str!("hdfs.json");

//pub fn bench_for_lines(c: &mut Criterion, lines: Vec<&str>, group_name: &str) {
pub fn bench_for_lines<'a, F, I>(c: &mut Criterion, iter_gen: F, group_name: &str)
where
    F: Fn() -> I,
    I: Iterator<Item = &'a str>,
{
    let mut group = c.benchmark_group(group_name);
    group.throughput(Throughput::Bytes(JSON_TEST_DATA_SIMPLE.len() as u64));
    group.bench_function("serde-json-owned", |b| {
        b.iter(|| {
            let mut val = None;
            for line in iter_gen() {
                let json: serde_json::Value = serde_json::from_str(line).unwrap();
                val = Some(json);
            }
            val
        })
    });
    group.bench_function("serde-json-borrowed", |b| {
        b.iter(|| {
            let mut val = None;
            for line in iter_gen() {
                let json: serde_json_borrow::Value = serde_json::from_str(line).unwrap();
                val = Some(json);
            }
            val
        })
    });

    group.bench_function("serde-json-borrowed-owned", |b| {
        b.iter(|| {
            let mut val = None;
            for line in iter_gen() {
                let json: OwnedValue = OwnedValue::parse_from(line.to_string()).unwrap();
                val = Some(json);
            }
            val
        })
    });
}

pub fn simple_json_to_doc_benchmark(c: &mut Criterion) {
    let lines: Vec<&str> = JSON_TEST_DATA_SIMPLE
        .lines()
        .map(|line| line.trim())
        .collect();

    bench_for_lines(c, || lines.iter().cloned(), "simple_json");

    let lines: Vec<&str> = JSON_TEST_DATA_HDFS_LOG
        .lines()
        .map(|line| line.trim())
        .collect();

    bench_for_lines(c, || lines.iter().cloned(), "hdfs_json");
}

criterion_group!(benches, simple_json_to_doc_benchmark);
criterion_main!(benches);
