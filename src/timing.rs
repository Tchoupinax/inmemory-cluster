pub struct TimingStats {
    total_time_micros: u128,
    count: u64,
}

impl TimingStats {
    pub fn new() -> Self {
        Self {
            total_time_micros: 0,
            count: 0,
        }
    }

    pub fn add_sample(&mut self, elapsed_micros: u128) {
        self.total_time_micros += elapsed_micros;
        self.count += 1;
    }

    pub fn average_micros(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.total_time_micros as f64 / self.count as f64)
        }
    }
}
