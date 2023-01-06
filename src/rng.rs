pub struct Rng {
    seed: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng { seed }
    }

    pub fn gen(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 16;
        self.seed
    }

    pub fn gen_f32(&mut self) -> f32 {
        self.gen() as f32 / u64::MAX as f32
    }

    pub fn gen_f32_range(&mut self, low: f32, high: f32) -> f32 {
        self.gen_f32() * (high - low) + low
    }
}