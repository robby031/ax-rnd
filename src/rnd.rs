#[derive(Clone, Copy, Debug)]
pub struct Axrnd {
    state: u64,
}

const GR: u64 = 0x9E3779B97F4A7C15;
const SPLIT_INC: u64 = 0x9E3779B97F4A7C15;

impl Axrnd {
    #[inline]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline]
    pub const fn from_raw(state: u64) -> Self {
        Self { state }
    }

    #[inline]
    pub fn state(&self) -> [u64; 1] {
        [self.state]
    }

    #[inline(always)]
    fn step(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GR);
        let xored = self.state ^ GR;
        let math = (self.state as u128).wrapping_mul(xored as u128);
        (math as u64) ^ ((math >> 64) as u64)
    }

    #[inline(always)]
    pub fn next_u32(&mut self) -> u32 {
        self.step() as u32
    }

    #[inline(always)]
    pub fn next_u64(&mut self) -> u64 {
        self.step()
    }

    #[inline(always)]
    pub fn next_u16(&mut self) -> u16 {
        self.next_u32() as u16
    }

    #[inline(always)]
    pub fn next_u8(&mut self) -> u8 {
        self.next_u32() as u8
    }

    #[inline(always)]
    pub fn next_bool(&mut self) -> bool {
        (self.next_u32() & 1) != 0
    }

    #[inline(always)]
    pub fn next_f64(&mut self) -> f64 {
        const NORM: f64 = 1.0 / ((1u64 << 52) as f64);
        ((self.next_u64() >> 12) as f64) * NORM
    }

    #[inline(always)]
    pub fn next_f32(&mut self) -> f32 {
        const NORM: f32 = 1.0 / ((1u32 << 24) as f32);
        ((self.next_u32() >> 8) as f32) * NORM
    }

    #[inline(always)]
    pub fn next_u64x4(&mut self) -> [u64; 4] {
        [
            self.next_u64(),
            self.next_u64(),
            self.next_u64(),
            self.next_u64(),
        ]
    }

    #[inline(always)]
    pub fn next_u32x4(&mut self) -> [u32; 4] {
        [
            self.next_u32(),
            self.next_u32(),
            self.next_u32(),
            self.next_u32(),
        ]
    }

    #[inline(always)]
    pub fn fill_u64(&mut self, out: &mut [u64]) {
        let len = out.len();
        let mut i = 0;
        while i + 8 <= len {
            out[i] = self.next_u64();
            out[i + 1] = self.next_u64();
            out[i + 2] = self.next_u64();
            out[i + 3] = self.next_u64();
            out[i + 4] = self.next_u64();
            out[i + 5] = self.next_u64();
            out[i + 6] = self.next_u64();
            out[i + 7] = self.next_u64();
            i += 8;
        }
        while i < len {
            out[i] = self.next_u64();
            i += 1;
        }
    }

    #[inline(always)]
    pub fn fill_bytes(&mut self, out: &mut [u8]) {
        let len = out.len();
        let mut i = 0;
        while i + 8 <= len {
            let v = self.next_u64();
            unsafe {
                core::ptr::write_unaligned(out.as_mut_ptr().add(i) as *mut u64, v);
            }
            i += 8;
        }
        if i < len {
            let bytes = self.next_u64().to_le_bytes();
            let remain = len - i;
            out[i..i + remain].copy_from_slice(&bytes[..remain]);
        }
    }

    #[inline(always)]
    pub fn bounded_u64(&mut self, upper: u64) -> u64 {
        assert!(upper > 0, "upper must be > 0");
        let mut x = self.next_u64();
        let mut m = (x as u128).wrapping_mul(upper as u128);
        let mut l = m as u64;
        if l < upper {
            let t = upper.wrapping_neg() % upper;
            while l < t {
                x = self.next_u64();
                m = (x as u128).wrapping_mul(upper as u128);
                l = m as u64;
            }
        }
        (m >> 64) as u64
    }

    #[inline(always)]
    pub fn bounded_u32(&mut self, upper: u32) -> u32 {
        assert!(upper > 0, "upper must be > 0");
        self.bounded_u64(upper as u64) as u32
    }

    #[inline(always)]
    pub fn split(&mut self) -> Self {
        let mut cloned = *self;
        cloned.state = cloned.state.wrapping_add(SPLIT_INC);

        let math = (cloned.state as u128).wrapping_mul((cloned.state ^ GR) as u128);

        cloned.state = (math as u64) ^ ((math >> 64) as u64);
        cloned
    }

    #[inline(always)]
    pub fn reseed(&mut self, seed: u64) {
        *self = Self::new(seed);
    }
}

impl Default for Axrnd {
    #[inline]
    fn default() -> Self {
        const { Self::new(0xA0761D6478BD642F) }
    }
}

#[inline(always)]
pub fn fill_bytes(rnd: &mut Axrnd, out: &mut [u8]) {
    rnd.fill_bytes(out)
}

#[inline(always)]
pub fn fill_u64(rnd: &mut Axrnd, out: &mut [u64]) {
    rnd.fill_u64(out)
}

#[inline(always)]
pub fn fill_u32(rnd: &mut Axrnd, out: &mut [u32]) {
    let len = out.len();
    let mut i = 0;
    while i + 8 <= len {
        let a = rnd.next_u64();
        let b = rnd.next_u64();
        let c = rnd.next_u64();
        let d = rnd.next_u64();
        out[i] = a as u32;
        out[i + 1] = (a >> 32) as u32;
        out[i + 2] = b as u32;
        out[i + 3] = (b >> 32) as u32;
        out[i + 4] = c as u32;
        out[i + 5] = (c >> 32) as u32;
        out[i + 6] = d as u32;
        out[i + 7] = (d >> 32) as u32;
        i += 8;
    }
    while i < len {
        out[i] = rnd.next_u32();
        i += 1;
    }
}
