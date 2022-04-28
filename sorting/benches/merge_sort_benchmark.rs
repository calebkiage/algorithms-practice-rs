use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::slice::ParallelSliceMut;
use sorting::merge_sort::{top_down_merge_sort, top_down_merge_sort_par, bottom_up_merge_sort};

pub fn top_down_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("top down merge sort");
    let mut input = (vec![1; 4_000]).into_boxed_slice();
    let mut work = (vec![4; 4_000]).into_boxed_slice();
    group.bench_function("seq", |cb| cb.iter(|| top_down_merge_sort(black_box(&mut input), black_box(&mut work))));
    group.bench_function("parallel", |cb| cb.iter(|| top_down_merge_sort_par(black_box(&mut input), black_box(&mut work))));
    group.finish();
}

pub fn bottom_up_benchmark(c: &mut Criterion) {
    let mut input = (vec![1; 4_000]).into_boxed_slice();
    let mut work = (vec![4; 4_000]).into_boxed_slice();
    c.bench_function("bottom up merge sort", |cb| cb.iter(|| bottom_up_merge_sort(black_box(&mut input), black_box(&mut work))));
}

pub fn native_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("native merge sort");
    let mut input = (vec![1; 4_000_000]).into_boxed_slice();
    group.bench_function("seq", |cb| cb.iter(|| black_box(&mut input).sort()));
    group.bench_function("parallel", |cb| cb.iter(|| black_box(&mut input).par_sort()));
    group.finish();
}

criterion_group!(benches, top_down_benchmark, bottom_up_benchmark, native_benchmark);
criterion_main!(benches);