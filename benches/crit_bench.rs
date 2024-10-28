use std::fs::File;
use std::hint::black_box;
use std::io::{BufRead, BufReader};

use binggan::plugins::{BPUTrasher, CacheTrasher};
use binggan::{BenchRunner, InputGroup};
use serde_json_borrow::OwnedValue;

fn lines_for_file(file: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| line.unwrap())
}

fn main() {
    access_bench();
    parse_bench();
}

fn bench_for_lines<F, I>(mut runner: InputGroup<(F, u64)>)
where
    F: Fn() -> I + 'static,
    I: Iterator<Item = String>,
{
    runner
        .add_plugin(CacheTrasher::default())
        .add_plugin(BPUTrasher::default());
    runner.set_name("parse");
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
    runner.register("SIMD_json_borrow", move |data| {
        let iter = data.0();
        for line in iter {
            let mut data: Vec<u8> = line.into();
            let v: simd_json::BorrowedValue = simd_json::to_borrowed_value(&mut data).unwrap();
            black_box(v);
        }
    });

    runner.run();
}

fn parse_bench() {
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
fn access_bench() {
    let mut runner: BenchRunner = BenchRunner::new();
    runner
        .add_plugin(CacheTrasher::default())
        .add_plugin(BPUTrasher::default());
    runner.set_name("access");

    let file_name_path_and_access = vec![
        (
            "simple_json",
            "./benches/simple-parse-bench.json",
            vec!["last_name"],
        ),
        (
            "gh-archive",
            "./benches/gh.json",
            vec!["actor", "avatar_url"],
        ),
    ];

    for (name, path, access) in &file_name_path_and_access {
        let file_size = File::open(path).unwrap().metadata().unwrap().len();
        let serde_jsons: Vec<serde_json::Value> = lines_for_file(path)
            .map(|line| serde_json::from_str(&line).unwrap())
            .collect();
        let serde_json_borrows: Vec<OwnedValue> = lines_for_file(path)
            .map(|line| OwnedValue::parse_from(line).unwrap())
            .collect();

        let mut group = runner.new_group();
        group.set_name(name);
        group.set_input_size(file_size as usize);
        group.register_with_input("serde_json access", &serde_jsons, move |data| {
            let mut total_size = 0;
            for el in data.iter() {
                // walk the access keys until the end. return 0 if value does no exist
                let mut val = Some(el);
                for key in access {
                    val = val.and_then(|v| v.get(key));
                }
                if let Some(v) = val {
                    total_size += v.as_str().map(|s| s.len()).unwrap_or(0);
                }
            }
            total_size
        });
        group.register_with_input(
            "serde_json_borrow access",
            &serde_json_borrows,
            move |data| {
                let mut total_size = 0;
                for el in data.iter() {
                    // walk the access keys until the end. return 0 if value does no exist
                    let mut val = el.get_value();
                    for key in access {
                        val = val.get(*key);
                    }
                    if let Some(v) = val.as_str() {
                        total_size += v.len();
                    }
                }
                total_size
            },
        );
        group.run();
    }
}
