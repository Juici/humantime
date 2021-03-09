use std::io::Write;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

const SECS: i64 = 1_483_228_799;
const NANOS: u32 = 0;

const BUF_CAPACITY: usize = 50;

fn rfc3339_humantime(c: &mut Criterion) {
    use std::time::{Duration, UNIX_EPOCH};

    c.bench_function("format_rfc3339_seconds_humantime", |b| {
        let time = UNIX_EPOCH + Duration::new(SECS as u64, NANOS);

        b.iter_batched_ref(
            || Vec::with_capacity(BUF_CAPACITY),
            |buf| {
                write!(
                    buf,
                    "{}",
                    humantime::format_rfc3339_seconds(black_box(time))
                )
                .unwrap()
            },
            BatchSize::PerIteration,
        );
    });
}

fn rfc3339_chrono(c: &mut Criterion) {
    use chrono::format::Fixed::*;
    use chrono::format::Item;
    use chrono::format::Item::*;
    use chrono::format::Numeric::*;
    use chrono::format::Pad::*;
    use chrono::{DateTime, NaiveDateTime, Utc};

    c.bench_function("format_rfc3339_seconds_chrono", |b| {
        let time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(SECS, NANOS), Utc);

        // formatting code from env_logger
        const ITEMS: &[Item<'static>] = {
            &[
                Numeric(Year, Zero),
                Literal("-"),
                Numeric(Month, Zero),
                Literal("-"),
                Numeric(Day, Zero),
                Literal("T"),
                Numeric(Hour, Zero),
                Literal(":"),
                Numeric(Minute, Zero),
                Literal(":"),
                Numeric(Second, Zero),
                Fixed(TimezoneOffsetZ),
            ]
        };

        b.iter_batched_ref(
            || Vec::with_capacity(BUF_CAPACITY),
            |buf| write!(buf, "{}", black_box(time).format_with_items(ITEMS.iter())).unwrap(),
            BatchSize::PerIteration,
        );
    });
}

criterion_group!(benches, rfc3339_humantime, rfc3339_chrono);
criterion_main!(benches);
