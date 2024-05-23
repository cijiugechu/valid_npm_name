use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hand-written", |b| {
        use valid_npm_name::ValidName;

        let input = "hello.js";

        b.iter(|| {
            let _ = ValidName::parse(black_box(input)).unwrap();
        })
    });

}

criterion_group!(bench, criterion_benchmark);
criterion_main!(bench);