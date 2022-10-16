pub struct BloomFilter {
    data: Vec<u8>,
    hash_count: u8,
    false_positive_probability: f64,
}

impl BloomFilter {

    pub fn build(n: usize, m: usize, k: u8) -> Self {
        BloomFilter {
            data: vec![0; n],
            hash_count: k,
            false_positive_probability: (1.0
                - f64::exp(-1.0 * k as f64 * m as f64 / (8.0 * n as f64)))
            .powf(k as f64),
        }
    }

    pub fn insert(&mut self, key: &str) {
        for idx in 0..self.hash_count {
            let hash = self.hash(key, idx);
            let mask = 128 >> (hash % 8);
            self.data[hash / 8] |= mask
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        for idx in 0..self.hash_count {
            let hash = self.hash( key, idx);
            let mask = 128 >> (hash % 8);
            if self.data[hash / 8] & mask != mask {
                return false;
            }
        }
        true
    }

    fn hash(&self, s: &str, idx: u8) -> usize {
        let mut hash = 5381_usize;
        for x in s.as_bytes() {
            hash = hash
                .wrapping_shl(5)
                .wrapping_add(hash)
                .wrapping_add(*x as usize);
        }
        hash = hash
                .wrapping_shl(5)
                .wrapping_add(hash)
                .wrapping_add(idx as usize);
        hash % (8 * self.data.len())
    }

    pub fn false_positive_probability(&self) -> f64 {
        self.false_positive_probability
    }
}
