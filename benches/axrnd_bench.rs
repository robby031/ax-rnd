use ax_rnd::{fill_bytes, fill_u64, rnd};
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

fn bench_single_core(c: &mut Criterion) {
    let mut group = c.benchmark_group("axrnd-core");
    group.bench_function("next_u64", |b| {
        let mut rnd = rnd(123);
        b.iter(|| {
            black_box(rnd.next_u64());
        });
    });

    group.bench_function("next_u32", |b| {
        let mut rnd = rnd(123);
        b.iter(|| {
            black_box(rnd.next_u32());
        });
    });

    group.bench_function("next_f64", |b| {
        let mut rnd = rnd(123);
        b.iter(|| {
            black_box(rnd.next_f64());
        });
    });

    group.finish();
}

fn bench_fill_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("axrnd-fill-bytes");
    let sizes = [64usize, 256, 1024, 4096, 64 * 1024, 1024 * 1024];
    for size in sizes {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut rnd = rnd(123);
            let mut buf = vec![0u8; size];
            b.iter(|| {
                fill_bytes(&mut rnd, &mut buf);
                black_box(&buf);
            });
        });
    }

    group.finish();
}

fn bench_fill_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("axrnd-fill-u64");
    let sizes = [64usize, 256, 1024, 4096, 65536];
    for size in sizes {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut rnd = rnd(123);
            let mut data = vec![0u64; size];
            b.iter(|| {
                fill_u64(&mut rnd, &mut data);
                black_box(&data);
            });
        });
    }

    group.finish();
}

criterion_group!(
    axrnd_internal,
    bench_single_core,
    bench_fill_bytes,
    bench_fill_u64,
);

criterion_main!(axrnd_internal);
