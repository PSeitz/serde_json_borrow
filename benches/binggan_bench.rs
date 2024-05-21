use std::fs::File;
use std::hint::black_box;
use std::io::{BufRead, BufReader};

use binggan::InputGroup;
use serde_json_borrow::OwnedValue;

fn main() {
    let lines_for_file = |file| {
        BufReader::new(File::open(file).unwrap())
            .lines()
            .map(|line| line.unwrap())
    };

    let mut named_data = Vec::new();

    let mut add = |name, path| {
        named_data.push((
            name,
            (
                move || lines_for_file(path),
                File::open(path).unwrap().metadata().unwrap().len(),
            ),
        ));
    };

    add("simple_json", "./benches/simple-parse-bench.json");
    add("hdfs", "./benches/hdfs.json");
    add("hdfs_with_array", "./benches/hdfs_with_array.json");
    add("wiki", "./benches/wiki.json");
    add("gh-archive", "./benches/gh.json");

    bench_for_lines(InputGroup::new_with_inputs(named_data));
}

fn bench_for_lines<F, I>(mut runner: InputGroup<(F, u64)>)
where
    F: Fn() -> I + 'static,
    I: Iterator<Item = String>,
{
    runner.throughput(|data| data.1 as usize);
    runner.register("serde_json", move |data| {
        let mut val = None;
        let iter = data.0();
        for line in iter {
            let json: serde_json::Value = serde_json::from_str(&line).unwrap();
            val = Some(json);
        }
        black_box(val);
    });
    runner.register("serde_json_borrow", move |data| {
        let mut val = None;
        let iter = data.0();
        for line in iter {
            let json: OwnedValue = OwnedValue::parse_from(line).unwrap();
            val = Some(json);
        }
        black_box(val);
    });
    runner.register("simd_serde_json_borrow", move |data| {
        let mut val = None;
        let iter = data.0();
        for line in iter {
            let json: OwnedValue = OwnedValue::from_string_simd(line).unwrap();
            val = Some(json);
        }
        black_box(val);
    });
    runner.register("simd_serde_json_borrow_value_builder", move |data| {
        let mut val = None;
        let iter = data.0();
        for line in iter {
            let json: OwnedValue = OwnedValue::from_string_simd2(line).unwrap();
            val = Some(json);
        }
        black_box(val);
    });
    runner.register("simd_json_BorrowedValue", move |data| {
        let iter = data.0();
        for line in iter {
            let mut data: Vec<u8> = line.into();
            let v: simd_json::BorrowedValue = simd_json::to_borrowed_value(&mut data).unwrap();
            black_box(v);
        }
    });

    runner.run();
}
