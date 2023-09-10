use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fiff::parser::FifParser;

fn read_tags(file: String) {
    let tags = FifParser::read_tags(file.into())
        .expect("Should have been able to read tags from test file");

    assert_eq!(tags.len(), 671);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("read tags", |b| {
        b.iter(|| read_tags(black_box("data/file_0.fif".to_string())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
