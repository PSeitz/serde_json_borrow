use std::fs::File;
use std::hint::black_box;
use std::io::{BufRead, BufReader};

use binggan::plugins::{BPUTrasher, CacheTrasher};
use binggan::{BenchRunner, PeakMemAlloc, INSTRUMENTED_SYSTEM};
use serde_json_borrow::OwnedValue;

#[global_allocator]
pub static GLOBAL: &PeakMemAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

fn lines_for_file(file: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| line.unwrap())
}

fn main() {
    parse_bench();
    access_bench();
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

    let mut runner: BenchRunner = BenchRunner::new();
    runner
        .add_plugin(CacheTrasher::default())
        .add_plugin(BPUTrasher::default());
    runner.set_name("parse");

    for (name, (input_gen, size)) in named_data {
        let mut runner = runner.new_group();
        runner.set_input_size(size as usize);
        runner.set_name(name);

        let access = get_access_for_input_name(name);
        runner.register("serde_json", move |_data| {
            let mut val = None;
            for line in input_gen() {
                let json: serde_json::Value = serde_json::from_str(&line).unwrap();
                val = Some(json);
            }
            black_box(val);
        });

        runner.register("serde_json + access by key", move |_data| {
            let mut total_size = 0;
            for line in input_gen() {
                let json: serde_json::Value = serde_json::from_str(&line).unwrap();
                total_size += access_json(&json, access);
            }
            black_box(total_size);
        });
        runner.register("serde_json_borrow::OwnedValue", move |_data| {
            let mut val = None;
            for line in input_gen() {
                let json: OwnedValue = OwnedValue::parse_from(line).unwrap();
                val = Some(json);
            }
            black_box(val);
        });

        runner.register(
            "serde_json_borrow::OwnedValue + access by key",
            move |_data| {
                let mut total_size = 0;
                for line in input_gen() {
                    let json: OwnedValue = OwnedValue::parse_from(line).unwrap();
                    total_size += access_json_borrowed(&json, access);
                }
                black_box(total_size);
            },
        );

        runner.register("SIMD_json_borrow", move |_data| {
            for line in input_gen() {
                let mut data: Vec<u8> = line.into();
                let v: simd_json::BorrowedValue = simd_json::to_borrowed_value(&mut data).unwrap();
                black_box(v);
            }
        });
        runner.run();
    }
}

fn get_access_for_input_name(name: &str) -> &[&[&'static str]] {
    match name {
        "hdfs" => &[&["severity_text", "timestamp", "body"]],
        "simple_json" => &[&["last_name"]],
        "gh-archive" => &[
            &["id"],
            &["type"],
            &["actor", "avatar_url"],
            &["actor", "url"],
            &["actor", "id"],
            &["actor", "login"],
            &["actor", "url"],
            &["actor", "avatar_url"],
            &["type"],
            &["actor", "id"],
            &["publuc"],
            &["created_at"],
        ],
        "wiki" => &[&["body", "url"]],
        _ => &[],
    }
}

fn access_bench() {
    let mut runner: BenchRunner = BenchRunner::new();
    runner
        .add_plugin(CacheTrasher::default())
        .add_plugin(BPUTrasher::default());
    runner.set_name("access");

    let file_name_path_and_access = vec![
        ("simple_json", "./benches/simple-parse-bench.json"),
        ("gh-archive", "./benches/gh.json"),
    ];

    for (name, path) in &file_name_path_and_access {
        let access = get_access_for_input_name(name);
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
                // walk the access keys until the end. return 0 if value does not exist
                total_size += access_json(el, access);
            }
            total_size
        });
        group.register_with_input(
            "serde_json_borrow access",
            &serde_json_borrows,
            move |data| {
                let mut total_size = 0;
                for el in data.iter() {
                    total_size += access_json_borrowed(el, access);
                }
                total_size
            },
        );
        group.run();
    }
}

fn access_json(el: &serde_json::Value, access: &[&[&str]]) -> usize {
    let mut total_size = 0;
    // walk the access keys until the end. return 0 if value does not exist
    for access in access {
        let mut val = Some(el);
        for key in *access {
            val = val.and_then(|v| v.get(key));
        }
        if let Some(v) = val {
            total_size += v.as_str().map(|s| s.len()).unwrap_or(0);
        }
    }
    total_size
}

fn access_json_borrowed(el: &OwnedValue, access: &[&[&str]]) -> usize {
    let mut total_size = 0;
    for access in access {
        // walk the access keys until the end. return 0 if value does not exist
        let mut val = Some(el.get_value());
        for key in *access {
            val = val.and_then(|v| v.get(key));
        }
        if let Some(val) = val && let Some(v) = val.as_str() {
            total_size += v.len();
        }
    }
    total_size
}
