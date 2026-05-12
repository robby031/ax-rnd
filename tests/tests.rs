use ax_rnd::{AxRng, bounded_u64, fill_bytes, fill_u32, fill_u64, rng};

#[test]
fn deterministic_sequence() {
    let mut a = rng(123456);
    let mut b = rng(123456);

    for _ in 0..100_000 {
        assert_eq!(a.next_u64(), b.next_u64(),);
    }
}

#[test]
fn different_seed_produces_different_stream() {
    let mut a = rng(1);
    let mut b = rng(2);

    let mut diff = false;

    for _ in 0..1024 {
        if a.next_u64() != b.next_u64() {
            diff = true;
            break;
        }
    }

    assert!(diff);
}

#[test]
fn fill_bytes_not_zero() {
    let mut rnd = rng(777);

    let mut buf = [0u8; 4096];

    fill_bytes(&mut rnd, &mut buf);

    let mut all_zero = true;

    for b in buf {
        if b != 0 {
            all_zero = false;
            break;
        }
    }

    assert!(!all_zero);
}

#[test]
fn fill_u64_changes_memory() {
    let mut rnd = rng(123);

    let mut data = [0u64; 1024];

    fill_u64(&mut rnd, &mut data);

    let mut all_zero = true;

    for v in data {
        if v != 0 {
            all_zero = false;
            break;
        }
    }

    assert!(!all_zero);
}

#[test]
fn fill_u32_changes_memory() {
    let mut rnd = rng(999);

    let mut data = [0u32; 2048];

    fill_u32(&mut rnd, &mut data);

    let mut all_zero = true;

    for v in data {
        if v != 0 {
            all_zero = false;
            break;
        }
    }

    assert!(!all_zero);
}

#[test]
fn bounded_u64_stays_in_range() {
    let mut rnd = rng(555);

    for _ in 0..100_000 {
        let value = rnd.bounded_u64(37);

        assert!(value < 37);
    }
}

#[test]
fn top_level_bounded_works() {
    for _ in 0..1000 {
        let value = bounded_u64(123, 16);

        assert!(value < 16);
    }
}

#[test]
fn next_bool_works() {
    let mut rnd = rng(1);

    let mut true_count = 0;
    let mut false_count = 0;

    for _ in 0..10000 {
        if rnd.next_bool() {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }

    assert!(true_count > 0);
    assert!(false_count > 0);
}

#[test]
fn next_f32_range() {
    let mut rnd = rng(321);

    for _ in 0..100_000 {
        let v = rnd.next_f32();

        assert!(v >= 0.0);
        assert!(v < 1.0);
    }
}

#[test]
fn next_f64_range() {
    let mut rnd = rng(321);

    for _ in 0..100_000 {
        let v = rnd.next_f64();

        assert!(v >= 0.0);
        assert!(v < 1.0);
    }
}

#[test]
fn split_produces_new_stream() {
    let mut rnd = rng(42);

    let mut split = rnd.split();

    let mut diff = false;

    for _ in 0..2048 {
        if rnd.next_u64() != split.next_u64() {
            diff = true;
            break;
        }
    }

    assert!(diff);
}

#[test]
fn large_fill_stability() {
    let mut rnd = rng(888);

    let mut data = vec![0u8; 1024 * 1024];

    fill_bytes(&mut rnd, &mut data);

    let mut checksum = 0u64;

    for b in data {
        checksum = checksum.wrapping_add(b as u64);
    }

    assert_ne!(checksum, 0);
}

#[test]
fn state_roundtrip() {
    let mut rnd = rng(123);

    let _ = rnd.next_u64();

    let state = rnd.state();

    let mut restored = AxRng::from_raw(state[0]);

    for _ in 0..1000 {
        assert_eq!(rnd.next_u64(), restored.next_u64(),);
    }
}

#[test]
fn next_alphanumeric_length() {
    let mut rnd = rng(42);

    for len in [0, 1, 10, 32, 100, 1000] {
        let s = rnd.next_alphanumeric(len);
        assert_eq!(s.len(), len, "length mismatch for {}", len);
    }
}

#[test]
fn next_alphanumeric_charset() {
    let mut rnd = rng(999);
    let s = rnd.next_alphanumeric(1000);

    for c in s.chars() {
        assert!(c.is_ascii_alphanumeric(), "invalid character: {}", c);
    }
}

#[test]
fn next_alphanumeric_deterministic() {
    let mut a = rng(12345);
    let mut b = rng(12345);

    for _ in 0..100 {
        assert_eq!(a.next_alphanumeric(50), b.next_alphanumeric(50));
    }
}

#[test]
fn next_base64url_charset() {
    let mut rnd = rng(999);
    let s = rnd.next_base64url(1000);

    for c in s.chars() {
        assert!(
            c.is_ascii_alphanumeric() || c == '-' || c == '_',
            "invalid character in base64url: {}",
            c
        );
    }
}

#[test]
fn next_base64url_deterministic() {
    let mut a = rng(12345);
    let mut b = rng(12345);

    for _ in 0..100 {
        assert_eq!(a.next_base64url(50), b.next_base64url(50));
    }
}

#[test]
fn alpha_vs_token_different() {
    let mut rnd = rng(42);
    let alpha = rnd.alpha(32);
    let token = rnd.token(32);
    assert_ne!(alpha, token);
}
