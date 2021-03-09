use chrono::DateTime;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn rfc3339_seconds(c: &mut Criterion) {
    const STRING: &str = "2018-02-13T23:08:32Z";

    c.bench_function("parse_rfc3339_seconds_humantime", |b| {
        b.iter(|| humantime::parse_rfc3339(black_box(STRING)).unwrap())
    });
    c.bench_function("parse_rfc3339_seconds_chrono", |b| {
        b.iter(|| DateTime::parse_from_rfc3339(black_box(STRING)).unwrap())
    });
}

fn rfc3339_millis(c: &mut Criterion) {
    const STRING: &str = "2018-02-13T23:08:32.123Z";

    c.bench_function("parse_rfc3339_millis_humantime", |b| {
        b.iter(|| humantime::parse_rfc3339(black_box(STRING)).unwrap())
    });
    c.bench_function("parse_rfc3339_millis_chrono", |b| {
        b.iter(|| DateTime::parse_from_rfc3339(black_box(STRING)).unwrap())
    });
}

fn rfc3339_nanos(c: &mut Criterion) {
    const STRING: &str = "2018-02-13T23:08:32.123456983Z";

    c.bench_function("parse_rfc3339_nanos_humantime", |b| {
        b.iter(|| humantime::parse_rfc3339(black_box(STRING)).unwrap())
    });
    c.bench_function("parse_rfc3339_nanos_chrono", |b| {
        b.iter(|| DateTime::parse_from_rfc3339(black_box(STRING)).unwrap())
    });
}

criterion_group!(benches, rfc3339_seconds, rfc3339_millis, rfc3339_nanos);
criterion_main!(benches);
