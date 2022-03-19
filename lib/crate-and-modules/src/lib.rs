use std::ops::Range;

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
    for _ in 0..days {
        fern.grow();
    }
}

/// Return true if two ranges overlap.
/// 2つのRangeに重なる部分があればtrueを返す
/// ```
/// assert_eq!(crate_and_modules::overlap(0..7, 3..10), true);
/// assert_eq!(crate_and_modules::overlap(1..5, 101..105), false);
/// ```
///
/// If either range is empty, they dont' count as overlapping.
/// どちらかの範囲が空であれば、重なっていないことにする
/// ```
/// assert_eq!(crate_and_modules::overlap(0..0, 0..10), false);
/// ```
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}
