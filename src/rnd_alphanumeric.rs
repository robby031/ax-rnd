use crate::rnd::AxRng;

impl AxRng {
    pub fn next_alphanumeric(&mut self, len: usize) -> String {
        const CHARSET: &[u8; 62] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut result = String::with_capacity(len);

        let chars_per_batch = 5;
        let batches = len / chars_per_batch;
        let remainder = len % chars_per_batch;

        for _ in 0..batches {
            let v = self.next_u64();
            let mut x = v;
            for _ in 0..chars_per_batch {
                let idx = (x % 62) as usize;
                result.push(CHARSET[idx] as char);
                x /= 62;
            }
        }

        if remainder > 0 {
            let v = self.next_u64();
            let mut x = v;
            for _ in 0..remainder {
                let idx = (x % 62) as usize;
                result.push(CHARSET[idx] as char);
                x /= 62;
            }
        }

        result
    }

    pub fn next_base64url(&mut self, len: usize) -> String {
        const CHARSET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
        let mut result = String::with_capacity(len);

        let chars_per_batch = 5;
        let batches = len / chars_per_batch;
        let remainder = len % chars_per_batch;

        for _ in 0..batches {
            let v = self.next_u64();
            let mut x = v;
            for _ in 0..chars_per_batch {
                let idx = (x % 64) as usize;
                result.push(CHARSET[idx] as char);
                x /= 64;
            }
        }

        if remainder > 0 {
            let v = self.next_u64();
            let mut x = v;
            for _ in 0..remainder {
                let idx = (x % 64) as usize;
                result.push(CHARSET[idx] as char);
                x /= 64;
            }
        }

        result
    }

    pub fn alpha(&mut self, len: usize) -> String {
        self.next_alphanumeric(len)
    }

    pub fn token(&mut self, len: usize) -> String {
        self.next_base64url(len)
    }
}
