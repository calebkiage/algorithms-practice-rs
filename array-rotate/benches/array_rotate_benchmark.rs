use array_rotate::{rotate, rotate_efficient};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn size() -> i32 {
    40_000
}

pub fn rotate_bench(c: &mut Criterion) {
    let size = size();
    let mut input = vec![1; size as usize];
    for i in 0..size {
        input[i as usize] = i;
    }
    c.bench_function("rotate", |cb| {
        cb.iter(|| rotate(black_box(&mut input), black_box(100)))
    });
}

pub fn rotate_efficient_bench(c: &mut Criterion) {
    let size = size();
    let mut input = vec![1; size as usize];
    for i in 0..size {
        input[i as usize] = i;
    }
    c.bench_function("rotate_efficient", |cb| {
        cb.iter(|| rotate_efficient(black_box(&mut input), black_box(100)))
    });
}

criterion_group!(benches, rotate_bench, rotate_efficient_bench);
criterion_main!(benches);
