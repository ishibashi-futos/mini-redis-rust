pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    #[cfg(target_os = "macos")]
    pub fn grow(&mut self) {
        self.size *= 1.0 * self.growth_rate;
    }

    #[cfg(target_os = "windows")]
    pub fn grow(&mut self) {
        self.size *= 1.05 * self.growth_rate;
    }
}

pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0 .. days {
        fern.grow();
    }
}
