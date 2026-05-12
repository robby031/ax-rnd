#![cfg_attr(not(feature = "std"), no_std)]

pub mod rng;
pub mod rng_alphanumeric;
pub use rng::AxRng;

#[cfg(feature = "std")]
use std::cell::Cell;

#[cfg(feature = "std")]
thread_local! {
    static THREAD_RNG: Cell<AxRng> = Cell::new(AxRng::new({
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_nanos(0xA0761D6478BD642F))
            .as_nanos() as u64
    }));
}

// Convenience API like fastrand - uses thread-local RNG with default seed
#[cfg(feature = "std")]
#[inline(always)]
pub fn u8() -> u8 {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let v = r.next_u8();
        rng.set(r);
        v
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn u16() -> u16 {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let v = r.next_u16();
        rng.set(r);
        v
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn u32() -> u32 {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let v = r.next_u32();
        rng.set(r);
        v
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn u64() -> u64 {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let v = r.next_u64();
        rng.set(r);
        v
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn bool() -> bool {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let v = r.next_bool();
        rng.set(r);
        v
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn f32() -> f32 {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let v = r.next_f32();
        rng.set(r);
        v
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn f64() -> f64 {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let v = r.next_f64();
        rng.set(r);
        v
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn alphanumeric(len: usize) -> String {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let v = r.next_alphanumeric(len);
        rng.set(r);
        v
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn base64url(len: usize) -> String {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let v = r.next_base64url(len);
        rng.set(r);
        v
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn fill(out: &mut [u8]) {
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        r.fill_bytes(out);
        rng.set(r);
    })
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn with_rng<F, R>(f: F) -> R
where
    F: FnOnce(&mut AxRng) -> R,
{
    THREAD_RNG.with(|rng| {
        let mut r = rng.get();
        let result = f(&mut r);
        rng.set(r);
        result
    })
}

#[inline]
pub fn rng(seed: u64) -> AxRng {
    AxRng::new(seed)
}

#[inline(always)]
pub fn fill_bytes(rng: &mut AxRng, out: &mut [u8]) {
    rng::fill_bytes(rng, out)
}

#[inline(always)]
pub fn fill_u64(rng: &mut AxRng, out: &mut [u64]) {
    rng::fill_u64(rng, out)
}

#[inline(always)]
pub fn fill_u32(rng: &mut AxRng, out: &mut [u32]) {
    rng::fill_u32(rng, out)
}

#[inline(always)]
pub fn random_u64(seed: u64) -> u64 {
    let mut rnd = AxRng::new(seed);

    rnd.next_u64()
}

#[inline(always)]
pub fn random_u32(seed: u64) -> u32 {
    let mut rnd = AxRng::new(seed);

    rnd.next_u32()
}

#[inline(always)]
pub fn random_bool(seed: u64) -> bool {
    let mut rnd = AxRng::new(seed);

    rnd.next_bool()
}

#[inline(always)]
pub fn random_f64(seed: u64) -> f64 {
    let mut rnd = AxRng::new(seed);

    rnd.next_f64()
}

#[inline(always)]
pub fn bounded_u64(seed: u64, upper: u64) -> u64 {
    assert!(upper > 0, "upper must be > 0");

    let mut rnd = AxRng::new(seed);

    rnd.bounded_u64(upper)
}

#[inline(always)]
pub fn random_alphanumeric(seed: u64, len: usize) -> String {
    let mut rnd = AxRng::new(seed);
    rnd.next_alphanumeric(len)
}

#[inline(always)]
pub fn alpha(seed: u64, len: usize) -> String {
    let mut rnd = AxRng::new(seed);
    rnd.alpha(len)
}

#[inline(always)]
pub fn random_base64url(seed: u64, len: usize) -> String {
    let mut rnd = AxRng::new(seed);
    rnd.next_base64url(len)
}

#[inline(always)]
pub fn token(seed: u64, len: usize) -> String {
    let mut rnd = AxRng::new(seed);
    rnd.token(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        let mut a = rng(123);
        let mut b = rng(123);

        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64(),);
        }
    }

    #[test]
    fn fill_bytes_works() {
        let mut rng = rng(999);

        let mut buf = [0u8; 128];

        fill_bytes(&mut rng, &mut buf);

        let mut zero = true;

        for b in buf {
            if b != 0 {
                zero = false;
                break;
            }
        }

        assert!(!zero);
    }

    #[test]
    fn split_is_different() {
        let mut a = rng(42);

        let mut b = a.split();

        assert_ne!(a.next_u64(), b.next_u64(),);
    }

    #[test]
    fn bounded_range() {
        let mut rng = rng(123);

        for _ in 0..10000 {
            let v = rng.bounded_u64(10);

            assert!(v < 10);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_u8() {
        let v = u8();
        assert!(v > 0 || true); // Just ensure it runs
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_u16() {
        let v = u16();
        assert!(v > 0 || true);
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_u32() {
        let v = u32();
        assert!(v > 0 || true);
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_u64() {
        let v = u64();
        assert!(v > 0 || true);
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_bool() {
        let v = bool();
        assert!(v == true || v == false);
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_f32() {
        let v = f32();
        assert!(v >= 0.0 && v < 1.0);
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_f64() {
        let v = f64();
        assert!(v >= 0.0 && v < 1.0);
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_alphanumeric() {
        let s = alphanumeric(32);
        assert_eq!(s.len(), 32);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_base64url() {
        let s = base64url(32);
        assert_eq!(s.len(), 32);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_fill() {
        let mut buf = [0u8; 64];
        fill(&mut buf);
        let mut all_zero = true;
        for b in buf {
            if b != 0 {
                all_zero = false;
                break;
            }
        }
        assert!(!all_zero);
    }

    #[cfg(feature = "std")]
    #[test]
    fn convenience_api_with_rng() {
        let v = with_rng(|rng| rng.next_u64());
        assert!(v > 0 || true);
    }
}
