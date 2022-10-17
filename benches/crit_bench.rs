use criterion::{criterion_group, criterion_main, Criterion, Throughput};

const JSON_TEST_DATA: &str = include_str!("simple-parse-bench.json");

pub fn simple_json_to_doc_benchmark(c: &mut Criterion) {
    let lines: Vec<&str> = JSON_TEST_DATA.lines().map(|line| line.trim()).collect();

    let mut group = c.benchmark_group("flat-json-to-doc");
    group.throughput(Throughput::Bytes(JSON_TEST_DATA.len() as u64));
    group.bench_function("serde-json-owned", |b| {
        b.iter(|| {
            let mut val = None;
            for line in &lines {
                let json: serde_json::Value = serde_json::from_str(line).unwrap();
                val = Some(json);
            }
            val
        })
    });
    group.bench_function("serde-json-borrowed", |b| {
        b.iter(|| {
            let mut val = None;
            for line in &lines {
                let json: serde_json_borrow::Value = serde_json::from_str(line).unwrap();
                val = Some(json);
            }
            val
        })
    });
}

criterion_group!(benches, simple_json_to_doc_benchmark);
criterion_main!(benches);
