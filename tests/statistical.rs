use ax_rnd::{Axrnd, rnd};

const SAMPLES: usize = 200_000;

#[test]
fn bit_histogram_uniform() {
    let mut rnd = rnd(42);
    let mut counts = [0u64; 64];

    for _ in 0..SAMPLES {
        let v = rnd.next_u64();
        for i in 0..64 {
            counts[i] += (v >> i) & 1;
        }
    }

    let expected = SAMPLES as f64 / 2.0;
    let tolerance = expected * 0.015;

    for i in 0..64 {
        let diff = (counts[i] as f64 - expected).abs();
        assert!(
            diff < tolerance,
            "bit {} count {} deviates too far from expected {}",
            i,
            counts[i],
            expected
        );
    }
}

#[test]
fn avalanche_effect() {
    let mut rnd = rnd(0xAABBCCDDEEFF0011);

    let mut total_flips = 0u64;
    let trials = 50_000;

    for _ in 0..trials {
        let base = rnd.next_u64();
        let flipped = base ^ 1;

        let out_base = {
            let mut r = Axrnd::new(base);
            r.next_u64()
        };
        let out_flipped = {
            let mut r = Axrnd::new(flipped);
            r.next_u64()
        };

        total_flips += (out_base ^ out_flipped).count_ones() as u64;
    }

    let avg = total_flips as f64 / trials as f64;
    let expected = 32.0;
    let diff = (avg - expected).abs();

    assert!(
        diff < 2.0,
        "avalanche average {:.2} deviates too far from expected {} (diff={:.2})",
        avg,
        expected,
        diff
    );
}

#[test]
fn stream_correlation_near_zero() {
    let mut a = rnd(123456);
    let mut b = rnd(654321);

    let n = 100_000usize;
    let mut greater = 0usize;
    let mut equal = 0usize;

    for _ in 0..n {
        let av = a.next_u64();
        let bv = b.next_u64();

        if av > bv {
            greater += 1;
        } else if av == bv {
            equal += 1;
        }
    }

    let ratio = greater as f64 / n as f64;

    assert!(
        (ratio - 0.5).abs() < 0.01,
        "stream win ratio {:.4} deviates from 0.5 (equal collisions={})",
        ratio,
        equal
    );
}

#[test]
fn low_byte_uniformity() {
    let mut rnd = rnd(777);
    let mut low_counts = [0u64; 256];
    let mut high_counts = [0u64; 256];

    let samples = 256_000usize;

    for _ in 0..samples {
        let v = rnd.next_u64();
        low_counts[(v & 0xFF) as usize] += 1;
        high_counts[((v >> 56) & 0xFF) as usize] += 1;
    }

    let expected = samples as f64 / 256.0;
    let tolerance = expected * 0.15;

    for i in 0..256 {
        let low_diff = (low_counts[i] as f64 - expected).abs();
        let high_diff = (high_counts[i] as f64 - expected).abs();

        assert!(
            low_diff < tolerance,
            "low byte {} count {} deviates from expected {}",
            i,
            low_counts[i],
            expected
        );
        assert!(
            high_diff < tolerance,
            "high byte {} count {} deviates from expected {}",
            i,
            high_counts[i],
            expected
        );
    }
}

#[test]
#[cfg(feature = "std")]
fn practrand_stream_quality() {
    use std::fs::File;
    use std::io::Write;

    let mut rnd = rnd(0xDEADBEEFCAFEBABE);
    let bytes = 8 * 1024 * 1024usize;

    let mut buf = vec![0u8; bytes];
    rnd.fill_bytes(&mut buf);

    let path = std::env::temp_dir().join("axrnd_practrand_8mb.bin");
    let mut file = File::create(&path).expect("create temp file");
    file.write_all(&buf).expect("write temp file");

    assert!(
        path.metadata().unwrap().len() as usize == bytes,
        "PractRand prep file size mismatch"
    );

    let ones = buf.iter().map(|b| b.count_ones() as u64).sum::<u64>();
    let total_bits = (bytes * 8) as u64;
    let ratio = ones as f64 / total_bits as f64;

    assert!(
        (ratio - 0.5).abs() < 0.001,
        "overall bit balance {:.4} too far from 0.5",
        ratio
    );

    eprintln!(
        "PractRand 8MB sample written to: {} (bit balance={:.4})",
        path.display(),
        ratio
    );
}
