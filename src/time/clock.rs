// per Pan Docs: A “dot” = one 2^22 Hz (≅ 4.194 MHz) time unit.

pub struct Clock {
    name: String,
    period_dots: u32,
    count_dots: u32,

    pub debug_total_ticks: u64,
}

impl Clock {
    pub fn new(name: impl Into<String>, period_dots: u32) -> Self {
        Self {
            name: name.into(),
            period_dots,
            count_dots: 0,
            debug_total_ticks: 0,
        }
    }

    pub fn set_period_dots(&mut self, period_dots: u32) {
        self.period_dots = period_dots;
    }

    pub fn update_and_check(&mut self) -> bool {
        self.count_dots += 1;

        if self.count_dots >= self.period_dots {
            self.count_dots = 0;
            self.debug_total_ticks += 1;
            true
        } else {
            false
        }
    }

    pub fn print(&self) {
        println!("Simple clock {}", self.name);
        println!("  period: {} dots", self.period_dots);
        println!("  count:  {} dots", self.count_dots);
        println!("  total ticks: {}", self.debug_total_ticks);
    }
}
