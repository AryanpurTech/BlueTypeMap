use blue_typemap::{Data, DataMut, DataMutStorage, TypeMap};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn normal_functions(x: u32) {
    let mut _a = x;
    _a *= 2;
}

fn non_mutable_functions(x: Data<u32>) {
    let mut _a = *x.get();
    _a *= 2;
}

fn mutable_functions(x: DataMut<u32>) {
    let mut _a = *x;
    _a *= 2;
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = black_box(100_000u32);

    c.bench_function("normal functions", |b| b.iter(|| normal_functions(input)));

    c.bench_function("non-mutable functions (TypeMap)", |b| {
        b.iter_batched(
            || {
                let mut container = TypeMap::default();
                container.bind(Data::new(input));
                container
            },
            |container| container.call(non_mutable_functions),
            criterion::BatchSize::SmallInput,
        );
    });

    c.bench_function("mutable functions (TypeMap)", |b| {
        b.iter_batched(
            || {
                let mut container = TypeMap::default();
                container.bind(DataMutStorage::new(input));
                container
            },
            |container| container.call(mutable_functions),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
