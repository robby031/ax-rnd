use ax_rnd::{fill_bytes as ax_fill_bytes, rnd as ax_rnd};
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use fastrand::Rng as FastRng;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

fn bench_next_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("next_u64_head_to_head");
    group.bench_function("axrng", |b| {
        let mut rng = ax_rnd(123);
        b.iter(|| {
            black_box(rng.next_u64());
        });
    });

    group.bench_function("rand_smallrng", |b| {
        let mut rng = SmallRng::seed_from_u64(123);
        b.iter(|| {
            black_box(rng.next_u64());
        });
    });

    group.bench_function("fastrand", |b| {
        let mut rng = FastRng::with_seed(123);
        b.iter(|| {
            black_box(rng.u64(..));
        });
    });

    group.finish();
}

fn bench_fill_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("fill_bytes_head_to_head");
    let sizes = [1024usize, 4096, 65536, 1024 * 1024];

    for size in sizes {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("axrng", size), &size, |b, &size| {
            let mut rng = ax_rnd(123);
            let mut buf = vec![0u8; size];
            b.iter(|| {
                ax_fill_bytes(&mut rng, &mut buf);
                black_box(&buf);
            });
        });

        group.bench_with_input(
            BenchmarkId::new("rand_smallrng", size),
            &size,
            |b, &size| {
                let mut rng = SmallRng::seed_from_u64(123);
                let mut buf = vec![0u8; size];
                b.iter(|| {
                    rng.fill_bytes(&mut buf);
                    black_box(&buf);
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("fastrand", size), &size, |b, &size| {
            let mut rng = FastRng::with_seed(123);
            let mut buf = vec![0u8; size];
            b.iter(|| {
                rng.fill(&mut buf);
                black_box(&buf);
            });
        });
    }
    group.finish();
}

criterion_group!(competitors, bench_next_u64, bench_fill_bytes,);
criterion_main!(competitors);
