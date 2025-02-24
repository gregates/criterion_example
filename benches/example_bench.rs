use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use criterion_example::add;

fn setup_small_slice() -> impl FnOnce() {
    let v = vec![0, 1];

    move || {
        let _ = add(&v);
    }
}

fn setup_big_slice() -> impl FnOnce() {
    let v = (0..10_000_000).collect::<Vec<_>>();

    move || {
        let _ = add(&v);
    }
}

fn setup_big_slice_returned() -> impl FnOnce() -> Vec<usize> {
    let v = (0..10_000_000).collect::<Vec<_>>();

    move || {
        let _ = add(&v);
        v
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
    c.bench_function("big slice returned", |b| {
        b.iter_batched(
            || setup_big_slice_returned(),
            |call| call(),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("no closure", |b| {
        b.iter_batched(
            || (0..10_000_000).collect::<Vec<_>>(),
            |v| add(&v),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("no closure & return input", |b| {
        b.iter_batched(
            || (0..10_000_000).collect::<Vec<_>>(),
            |v| { add(&v); v },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("black box", |b| {
        b.iter_batched(
            || setup_big_slice(),
            |call| black_box(call),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("black box no closure", |b| {
        b.iter_batched(
            || (0..10_000_000).collect::<Vec<_>>(),
            |v| black_box(move || add(&v)),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, example_bench);

criterion_main!(benches);
