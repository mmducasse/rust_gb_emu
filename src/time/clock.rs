#[derive(Clone)]
pub struct Clock {
    name: String,
    frequency_hz: f64,
    time_since_last_tick_s: f64,

    debug_total_ticks: u64,
}

impl Clock {
    pub fn new(name: impl Into<String>, frequency_hz: f64) -> Self {
        Self {
            name: name.into(),
            frequency_hz,
            time_since_last_tick_s: 0.0,

            debug_total_ticks: 0,
        }
    }

    #[inline]
    pub fn period(&self) -> f64 {
        return 1.0 / self.frequency_hz;
    }

    pub fn set_frequency_hz(&mut self, frequency_hz: f64) {
        self.frequency_hz = frequency_hz;
    }

    pub fn update(&mut self, elapsed_s: f64) -> u32 {
        let time_since_last_tick_s = self.time_since_last_tick_s + elapsed_s;
        let ticks = (time_since_last_tick_s / self.period()) as u32;
        self.time_since_last_tick_s = time_since_last_tick_s - ((ticks as f64) * self.period());

        self.debug_total_ticks += ticks as u64;

        return ticks;
    }

    pub fn print(&self) {
        println!(
            "Clock \"{}\": freq = {} Hz, total ticks = {}",
            self.name, self.frequency_hz, self.debug_total_ticks
        );
    }
}
