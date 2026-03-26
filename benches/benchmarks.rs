use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use temprs::util::utils::*;

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn bench_tmp_dir() -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "temprs_bench_{}_{}",
        std::process::id(),
        id
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn bench_transform_idx(c: &mut Criterion) {
    c.bench_function("transform_idx_positive", |b| {
        b.iter(|| util_transform_idx(black_box(3), black_box(10)))
    });

    c.bench_function("transform_idx_negative", |b| {
        b.iter(|| util_transform_idx(black_box(-2), black_box(10)))
    });
}

fn bench_path_to_string(c: &mut Criterion) {
    let path = PathBuf::from("/tmp/temprs/tempfile123456");
    c.bench_function("path_to_string", |b| {
        b.iter(|| util_path_to_string(black_box(&path)))
    });
}

fn bench_time_ms(c: &mut Criterion) {
    c.bench_function("time_ms", |b| {
        b.iter(|| util_time_ms())
    });
}

fn bench_file_to_paths(c: &mut Criterion) {
    let dir = bench_tmp_dir();
    let file = dir.join("paths.txt");
    let content: String = (0..100)
        .map(|i| format!("/tmp/temprs/tempfile{}", i))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    fs::write(&file, &content).unwrap();

    c.bench_function("file_to_paths_100", |b| {
        b.iter(|| util_file_to_paths(black_box(&file)))
    });

    let _ = fs::remove_dir_all(&dir);
}

fn bench_paths_to_file(c: &mut Criterion) {
    let dir = bench_tmp_dir();
    let out = dir.join("out.txt");
    let paths: Vec<PathBuf> = (0..100)
        .map(|i| PathBuf::from(format!("/tmp/temprs/tempfile{}", i)))
        .collect();

    c.bench_function("paths_to_file_100", |b| {
        b.iter(|| util_paths_to_file(black_box(paths.clone()), black_box(&out)))
    });

    let _ = fs::remove_dir_all(&dir);
}

fn bench_overwrite_file(c: &mut Criterion) {
    let dir = bench_tmp_dir();
    let file = dir.join("overwrite.txt");
    let content = "hello world\n".repeat(100);

    c.bench_function("overwrite_file_100_lines", |b| {
        b.iter(|| util_overwrite_file(black_box(&file), black_box(&content)))
    });

    let _ = fs::remove_dir_all(&dir);
}

fn bench_append_file(c: &mut Criterion) {
    let dir = bench_tmp_dir();
    let file = dir.join("append.txt");

    c.bench_function("append_file", |b| {
        b.iter_custom(|iters| {
            fs::write(&file, "").unwrap();
            let start = std::time::Instant::now();
            for _ in 0..iters {
                util_append_file(black_box(&file), black_box("a line\n"));
            }
            start.elapsed()
        })
    });

    let _ = fs::remove_dir_all(&dir);
}

fn bench_file_contents_to_string(c: &mut Criterion) {
    let dir = bench_tmp_dir();
    let file = dir.join("read.txt");
    let content = "line of text\n".repeat(100);
    fs::write(&file, &content).unwrap();

    c.bench_function("file_contents_to_string_100_lines", |b| {
        b.iter(|| util_file_contents_to_string(black_box(&file)))
    });

    let _ = fs::remove_dir_all(&dir);
}

fn bench_lines_to_file(c: &mut Criterion) {
    let dir = bench_tmp_dir();
    let out = dir.join("lines_out.txt");
    let lines: Vec<String> = (0..100).map(|i| format!("line {}", i)).collect();

    c.bench_function("lines_to_file_100", |b| {
        b.iter_custom(|iters| {
            let start = std::time::Instant::now();
            for _ in 0..iters {
                let _ = fs::remove_file(&out);
                util_lines_to_file(black_box(&out), black_box(lines.clone()));
            }
            start.elapsed()
        })
    });

    let _ = fs::remove_dir_all(&dir);
}

criterion_group!(
    benches,
    bench_transform_idx,
    bench_path_to_string,
    bench_time_ms,
    bench_file_to_paths,
    bench_paths_to_file,
    bench_overwrite_file,
    bench_append_file,
    bench_file_contents_to_string,
    bench_lines_to_file,
);
criterion_main!(benches);
