pub struct BloomStat {
    pub num_bits: f64,
    pub num_hash: f64,
    pub num_ones: f64,
}

impl BloomStat {
    pub fn estimate_count(&self) -> f64 {
        let b: f64 = self.num_bits;
        let h: f64 = self.num_hash;
        let o: f64 = self.num_ones;
        -b * (1.0 / h) * (1.0 - o / b).ln()
    }
}
