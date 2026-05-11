#![cfg_attr(not(feature = "std"), no_std)]

pub mod rnd;
pub mod rnd_alphanumeric;
pub use rnd::Axrnd;

#[inline]
pub fn rnd(seed: u64) -> Axrnd {
    Axrnd::new(seed)
}

#[inline(always)]
pub fn fill_bytes(rnd: &mut Axrnd, out: &mut [u8]) {
    rnd::fill_bytes(rnd, out)
}

#[inline(always)]
pub fn fill_u64(rnd: &mut Axrnd, out: &mut [u64]) {
    rnd::fill_u64(rnd, out)
}

#[inline(always)]
pub fn fill_u32(rnd: &mut Axrnd, out: &mut [u32]) {
    rnd::fill_u32(rnd, out)
}

#[inline(always)]
pub fn random_u64(seed: u64) -> u64 {
    let mut rnd = Axrnd::new(seed);

    rnd.next_u64()
}

#[inline(always)]
pub fn random_u32(seed: u64) -> u32 {
    let mut rnd = Axrnd::new(seed);

    rnd.next_u32()
}

#[inline(always)]
pub fn random_bool(seed: u64) -> bool {
    let mut rnd = Axrnd::new(seed);

    rnd.next_bool()
}

#[inline(always)]
pub fn random_f64(seed: u64) -> f64 {
    let mut rnd = Axrnd::new(seed);

    rnd.next_f64()
}

#[inline(always)]
pub fn bounded_u64(seed: u64, upper: u64) -> u64 {
    assert!(upper > 0, "upper must be > 0");

    let mut rnd = Axrnd::new(seed);

    rnd.bounded_u64(upper)
}

#[inline(always)]
pub fn random_alphanumeric(seed: u64, len: usize) -> String {
    let mut rnd = Axrnd::new(seed);
    rnd.next_alphanumeric(len)
}

#[inline(always)]
pub fn alpha(seed: u64, len: usize) -> String {
    let mut rnd = Axrnd::new(seed);
    rnd.alpha(len)
}

#[inline(always)]
pub fn random_base64url(seed: u64, len: usize) -> String {
    let mut rnd = Axrnd::new(seed);
    rnd.next_base64url(len)
}

#[inline(always)]
pub fn token(seed: u64, len: usize) -> String {
    let mut rnd = Axrnd::new(seed);
    rnd.token(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        let mut a = rnd(123);
        let mut b = rnd(123);

        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64(),);
        }
    }

    #[test]
    fn fill_bytes_works() {
        let mut rnd = rnd(999);

        let mut buf = [0u8; 128];

        fill_bytes(&mut rnd, &mut buf);

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
        let mut a = rnd(42);

        let mut b = a.split();

        assert_ne!(a.next_u64(), b.next_u64(),);
    }

    #[test]
    fn bounded_range() {
        let mut rnd = rnd(123);

        for _ in 0..10000 {
            let v = rnd.bounded_u64(10);

            assert!(v < 10);
        }
    }
}
