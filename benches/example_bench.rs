use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use criterion_example::add;

fn setup_small_slice() -> impl FnOnce() {
    let v = vec![0; 2];

    move || {
        let _ = add(&v);
    }
}

fn setup_big_slice() -> impl FnOnce() {
    let v = vec![0; 10_000_000];

    move || {
        let _ = add(&v);
    }
}

fn example_bench(c: &mut Criterion)  {
    c.bench_function("small slice", |b| {
        b.iter_batched(
            || setup_small_slice(),
            |call| call(),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("big slice", |b| {
        b.iter_batched(
            || setup_big_slice(),
            |call| call(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, example_bench);

criterion_main!(benches);
