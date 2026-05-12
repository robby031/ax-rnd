use std::hint::black_box;
use std::time::{Duration, Instant};

use ax_rnd::{fill_bytes, fill_u64, rnd};
use fastrand::Rng as FastRng;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

const WARMUP_MS: u64 = 100;
const MEASURE_MS: u64 = 3000;

fn bench_next_u64() {
    let mut r = rnd(123);
    let name = "next_u64";

    // warmup
    let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
    while Instant::now() < warmup_end {
        black_box(r.next_u64());
    }

    // measure
    let target = Duration::from_millis(MEASURE_MS);
    let start = Instant::now();
    let mut iters: u64 = 0;
    loop {
        black_box(r.next_u64());
        iters += 1;
        if iters % 4096 == 0 && start.elapsed() >= target {
            break;
        }
    }
    let elapsed = start.elapsed();
    let ns = elapsed.as_secs_f64() * 1e9 / iters as f64;

    println!(
        "  {:20}  {:>12} iter  {:>8.2} ps/iter",
        name,
        iters,
        ns * 1000.0
    );
}

fn bench_next_u32() {
    let mut r = rnd(123);
    let name = "next_u32";

    let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
    while Instant::now() < warmup_end {
        black_box(r.next_u32());
    }

    let target = Duration::from_millis(MEASURE_MS);
    let start = Instant::now();
    let mut iters: u64 = 0;
    loop {
        black_box(r.next_u32());
        iters += 1;
        if iters % 4096 == 0 && start.elapsed() >= target {
            break;
        }
    }
    let elapsed = start.elapsed();
    let ns = elapsed.as_secs_f64() * 1e9 / iters as f64;

    println!(
        "  {:20}  {:>12} iter  {:>8.2} ps/iter",
        name,
        iters,
        ns * 1000.0
    );
}

fn bench_next_f64() {
    let mut r = rnd(123);
    let name = "next_f64";

    let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
    while Instant::now() < warmup_end {
        black_box(r.next_f64());
    }

    let target = Duration::from_millis(MEASURE_MS);
    let start = Instant::now();
    let mut iters: u64 = 0;
    loop {
        black_box(r.next_f64());
        iters += 1;
        if iters % 4096 == 0 && start.elapsed() >= target {
            break;
        }
    }
    let elapsed = start.elapsed();
    let ns = elapsed.as_secs_f64() * 1e9 / iters as f64;

    println!(
        "  {:20}  {:>12} iter  {:>8.2} ps/iter",
        name,
        iters,
        ns * 1000.0
    );
}

fn bench_fill_bytes(size: usize) {
    let mut r = rnd(123);
    let mut buf = vec![0u8; size];
    let name = format!("fill_bytes/{}", humansize(size));

    let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
    while Instant::now() < warmup_end {
        fill_bytes(&mut r, &mut buf);
        black_box(&buf);
    }

    let target = Duration::from_millis(MEASURE_MS);
    let start = Instant::now();
    let mut iters: u64 = 0;
    loop {
        fill_bytes(&mut r, &mut buf);
        black_box(&buf);
        iters += 1;
        if iters % 16 == 0 && start.elapsed() >= target {
            break;
        }
    }
    let elapsed = start.elapsed();
    let bytes_total = iters * size as u64;
    let gibs = bytes_total as f64 / (1024.0 * 1024.0 * 1024.0);
    let secs = elapsed.as_secs_f64();

    println!(
        "  {:20}  {:>12} iter  {:>8.2} GiB/s",
        name,
        iters,
        gibs / secs
    );
}

fn bench_fill_u64(size: usize) {
    let mut r = rnd(123);
    let mut buf = vec![0u64; size];
    let name = format!("fill_u64/{}", humansize(size * 8));

    let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
    while Instant::now() < warmup_end {
        fill_u64(&mut r, &mut buf);
        black_box(&buf);
    }

    let target = Duration::from_millis(MEASURE_MS);
    let start = Instant::now();
    let mut iters: u64 = 0;
    loop {
        fill_u64(&mut r, &mut buf);
        black_box(&buf);
        iters += 1;
        if iters % 16 == 0 && start.elapsed() >= target {
            break;
        }
    }
    let elapsed = start.elapsed();
    let bytes_total = iters * (size * 8) as u64;
    let gibs = bytes_total as f64 / (1024.0 * 1024.0 * 1024.0);
    let secs = elapsed.as_secs_f64();

    println!(
        "  {:20}  {:>12} iter  {:>8.2} GiB/s",
        name,
        iters,
        gibs / secs
    );
}

fn bench_head_to_head_next_u64() {
    println!("\n  --- head-to-head next_u64 ---");

    // axrng
    {
        let mut ax = rnd(123);
        let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
        while Instant::now() < warmup_end {
            black_box(ax.next_u64());
        }
        let target = Duration::from_millis(MEASURE_MS);
        let start = Instant::now();
        let mut iters: u64 = 0;
        loop {
            black_box(ax.next_u64());
            iters += 1;
            if iters % 4096 == 0 && start.elapsed() >= target {
                break;
            }
        }
        let ps = start.elapsed().as_secs_f64() * 1e12 / iters as f64;
        println!(
            "    {:12}  {:>12} iter  {:>8.2} ps/iter",
            "axrng", iters, ps
        );
    }

    // fastrand
    {
        let mut fast = FastRng::with_seed(123);
        let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
        while Instant::now() < warmup_end {
            black_box(fast.u64(..));
        }
        let target = Duration::from_millis(MEASURE_MS);
        let start = Instant::now();
        let mut iters: u64 = 0;
        loop {
            black_box(fast.u64(..));
            iters += 1;
            if iters % 4096 == 0 && start.elapsed() >= target {
                break;
            }
        }
        let ps = start.elapsed().as_secs_f64() * 1e12 / iters as f64;
        println!(
            "    {:12}  {:>12} iter  {:>8.2} ps/iter",
            "fastrand", iters, ps
        );
    }

    // rand SmallRng
    {
        let mut small = SmallRng::seed_from_u64(123);
        let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
        while Instant::now() < warmup_end {
            black_box(small.next_u64());
        }
        let target = Duration::from_millis(MEASURE_MS);
        let start = Instant::now();
        let mut iters: u64 = 0;
        loop {
            black_box(small.next_u64());
            iters += 1;
            if iters % 4096 == 0 && start.elapsed() >= target {
                break;
            }
        }
        let ps = start.elapsed().as_secs_f64() * 1e12 / iters as f64;
        println!(
            "    {:12}  {:>12} iter  {:>8.2} ps/iter",
            "rand_small", iters, ps
        );
    }
}

fn bench_head_to_head_fill_1mb() {
    println!("\n  --- head-to-head fill_bytes/1MB ---");

    const SIZE: usize = 1024 * 1024;
    let mut ax_r = rnd(123);
    let mut fast_r = FastRng::with_seed(123);
    let mut small_r = SmallRng::seed_from_u64(123);
    let mut ax_buf = vec![0u8; SIZE];
    let mut fast_buf = vec![0u8; SIZE];
    let mut small_buf = vec![0u8; SIZE];

    let target = Duration::from_millis(MEASURE_MS);

    // axrng
    {
        let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
        while Instant::now() < warmup_end {
            fill_bytes(&mut ax_r, &mut ax_buf);
            black_box(&ax_buf);
        }
        let start = Instant::now();
        let mut iters: u64 = 0;
        loop {
            fill_bytes(&mut ax_r, &mut ax_buf);
            black_box(&ax_buf);
            iters += 1;
            if iters % 4 == 0 && start.elapsed() >= target {
                break;
            }
        }
        let elapsed = start.elapsed();
        let gibs =
            (iters * SIZE as u64) as f64 / (1024.0 * 1024.0 * 1024.0) / elapsed.as_secs_f64();
        println!(
            "    {:12}  {:>12} iter  {:>8.2} GiB/s",
            "axrng", iters, gibs
        );
    }

    // fastrand
    {
        let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
        while Instant::now() < warmup_end {
            fast_r.fill(&mut fast_buf);
            black_box(&fast_buf);
        }
        let start = Instant::now();
        let mut iters: u64 = 0;
        loop {
            fast_r.fill(&mut fast_buf);
            black_box(&fast_buf);
            iters += 1;
            if iters % 4 == 0 && start.elapsed() >= target {
                break;
            }
        }
        let elapsed = start.elapsed();
        let gibs =
            (iters * SIZE as u64) as f64 / (1024.0 * 1024.0 * 1024.0) / elapsed.as_secs_f64();
        println!(
            "    {:12}  {:>12} iter  {:>8.2} GiB/s",
            "fastrand", iters, gibs
        );
    }

    // rand SmallRng
    {
        let warmup_end = Instant::now() + Duration::from_millis(WARMUP_MS);
        while Instant::now() < warmup_end {
            small_r.fill_bytes(&mut small_buf);
            black_box(&small_buf);
        }
        let start = Instant::now();
        let mut iters: u64 = 0;
        loop {
            small_r.fill_bytes(&mut small_buf);
            black_box(&small_buf);
            iters += 1;
            if iters % 4 == 0 && start.elapsed() >= target {
                break;
            }
        }
        let elapsed = start.elapsed();
        let gibs =
            (iters * SIZE as u64) as f64 / (1024.0 * 1024.0 * 1024.0) / elapsed.as_secs_f64();
        println!(
            "    {:12}  {:>12} iter  {:>8.2} GiB/s",
            "rand_small", iters, gibs
        );
    }
}

fn bench_latency_distribution() {
    println!("\n=== latency distribution (next_u64, batch of 1_000) ===");
    println!("  note: single-call latency (~400 ps) is below timer resolution.");
    println!("        distribution is measured over batches and divided.");

    let mut r = rnd(123);
    const BATCH: usize = 1_000;
    const SAMPLES: usize = 20_000;
    let mut samples = Vec::with_capacity(SAMPLES);

    for _ in 0..SAMPLES {
        let start = Instant::now();
        for _ in 0..BATCH {
            black_box(r.next_u64());
        }
        let per_call_ns = start.elapsed().as_secs_f64() * 1e9 / BATCH as f64;
        samples.push(per_call_ns);
    }

    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = samples[0];
    let p50 = samples[SAMPLES / 2];
    let p99 = samples[SAMPLES * 99 / 100];
    let max = samples[SAMPLES - 1];
    let avg: f64 = samples.iter().sum::<f64>() / SAMPLES as f64;

    println!("  min: {:>8.2} ps", min * 1000.0);
    println!("  avg: {:>8.2} ps", avg * 1000.0);
    println!("  p50: {:>8.2} ps", p50 * 1000.0);
    println!("  p99: {:>8.2} ps", p99 * 1000.0);
    println!("  max: {:>8.2} ps", max * 1000.0);
}

fn bench_startup_latency() {
    println!("\n=== startup latency (AxRng::new) ===");

    let target = Duration::from_millis(MEASURE_MS);
    let start = Instant::now();
    let mut iters: u64 = 0;
    loop {
        black_box(ax_rnd::AxRng::new(123));
        iters += 1;
        if iters % 4096 == 0 && start.elapsed() >= target {
            break;
        }
    }
    let ps = start.elapsed().as_secs_f64() * 1e12 / iters as f64;
    println!("  {:>12} iter  {:>8.2} ps/iter", iters, ps);
}

fn bench_split_latency() {
    println!("\n=== split latency ===");

    let mut r = rnd(123);
    let target = Duration::from_millis(MEASURE_MS);
    let start = Instant::now();
    let mut iters: u64 = 0;
    loop {
        black_box(r.split());
        iters += 1;
        if iters % 4096 == 0 && start.elapsed() >= target {
            break;
        }
    }
    let ps = start.elapsed().as_secs_f64() * 1e12 / iters as f64;
    println!("  {:>12} iter  {:>8.2} ps/iter", iters, ps);
}

fn bench_bounded_u64_latency() {
    println!("\n=== bounded_u64 latency (upper=100) ===");

    let mut r = rnd(123);
    let target = Duration::from_millis(MEASURE_MS);
    let start = Instant::now();
    let mut iters: u64 = 0;
    loop {
        black_box(r.bounded_u64(100));
        iters += 1;
        if iters % 4096 == 0 && start.elapsed() >= target {
            break;
        }
    }
    let ps = start.elapsed().as_secs_f64() * 1e12 / iters as f64;
    println!("  {:>12} iter  {:>8.2} ps/iter", iters, ps);
}

fn bench_seed_quality() {
    println!("\n=== seed quality (collision / divergence) ===");

    const STREAMS: usize = 256;
    const SAMPLES: usize = 1024;

    // Test 1: different seeds must not collide in first 1024 values
    let mut first_vals = std::collections::HashSet::new();
    let mut collisions = 0usize;
    for seed in 0..STREAMS {
        let mut r = rnd(seed as u64);
        let v = r.next_u64();
        if !first_vals.insert(v) {
            collisions += 1;
        }
    }
    println!(
        "  first-value collisions: {}/{}  {}",
        collisions,
        STREAMS,
        if collisions == 0 { "PASS" } else { "FAIL" }
    );

    // Test 2: seed+1 stream must diverge from seed stream within 16 values
    let mut max_diverge = 0usize;
    for seed in 0..STREAMS {
        let mut a = rnd(seed as u64);
        let mut b = rnd(seed as u64 + 1);
        let mut diverged_at = SAMPLES;
        for i in 0..SAMPLES {
            if a.next_u64() != b.next_u64() {
                diverged_at = i;
                break;
            }
        }
        max_diverge = max_diverge.max(diverged_at);
    }
    println!(
        "  max divergence position: {}  {}",
        max_diverge,
        if max_diverge < 2 { "PASS" } else { "WARN" }
    );

    // Test 3: split streams must diverge immediately
    let mut a = rnd(42);
    let mut b = a.split();
    let mut split_diverge = SAMPLES;
    for i in 0..SAMPLES {
        if a.next_u64() != b.next_u64() {
            split_diverge = i;
            break;
        }
    }
    println!(
        "  split divergence position: {}  {}",
        split_diverge,
        if split_diverge < 2 { "PASS" } else { "FAIL" }
    );
}

fn print_memory_footprint() {
    println!("\n=== memory footprint ===");
    println!(
        "  AxRnd struct size: {} bytes",
        std::mem::size_of::<ax_rnd::AxRng>()
    );
}

fn humansize(bytes: usize) -> String {
    if bytes >= 1024 * 1024 {
        format!("{}MB", bytes / (1024 * 1024))
    } else if bytes >= 1024 {
        format!("{}KB", bytes / 1024)
    } else {
        format!("{}B", bytes)
    }
}

fn main() {
    println!(
        "AxRng manual benchmark ({}ms warmup + {}ms measure)\n",
        WARMUP_MS, MEASURE_MS
    );

    println!("=== axrng core ===");
    bench_next_u64();
    bench_next_u32();
    bench_next_f64();

    println!("\n=== axrng fill ===");
    bench_fill_bytes(64);
    bench_fill_bytes(256);
    bench_fill_bytes(1024);
    bench_fill_bytes(4096);
    bench_fill_bytes(64 * 1024);
    bench_fill_bytes(1024 * 1024);

    println!("\n=== axrng fill_u64 ===");
    bench_fill_u64(128);
    bench_fill_u64(1024);
    bench_fill_u64(64 * 1024);

    bench_head_to_head_next_u64();
    bench_head_to_head_fill_1mb();

    bench_latency_distribution();
    bench_startup_latency();
    bench_split_latency();
    bench_bounded_u64_latency();
    bench_seed_quality();
    print_memory_footprint();

    println!("\nDone.");
}
