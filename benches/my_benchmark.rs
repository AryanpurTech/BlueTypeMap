use criterion::{black_box, criterion_group, criterion_main, Criterion};
use blue_typemap::{TypeMap, Data, DataMutStorage, DataMut};

fn normal_functions(value: u32){
    fn normal(x: u32) {
        let mut _a = x.clone();
        _a *= 2;
    }

    normal(value);
}

fn non_mutable_functions(value: u32){
    fn non_mutable(x: Data<u32>) {
        let mut _a = x.get().clone();
        _a *= 2;
    }

    let mut container = TypeMap::default();
    container.bind(Data::new(value));
    container.call(non_mutable);
}

fn mutable_functions(value: u32){
    fn mutable(x: DataMut<u32>) {
        let mut _a = x.clone();
        _a *= 2;
    }

    let mut container = TypeMap::default();
    container.bind(DataMutStorage::new(value));
    container.call(mutable);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("normal functions 20", |b| b.iter(|| normal_functions(black_box(20))));
    c.bench_function("non mutable functions 20", |b| b.iter(|| non_mutable_functions(black_box(20))));
    c.bench_function("mutable functions 20", |b| b.iter(|| mutable_functions(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
