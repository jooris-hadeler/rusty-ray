#[macro_export]
/// Create a new Interval with the given start and end.
macro_rules! interval {
    ($start:expr, $end:expr) => {
        $crate::interval::Interval {
            start: $start,
            end: $end,
        }
    };
}

#[derive(Debug, Clone, Copy)]
/// An interval from a start to an end.
pub struct Interval {
    /// The start of the interval.
    pub start: f64,
    /// The end of the interval.
    pub end: f64,
}

impl Interval {
    #[inline]
    /// Checks if the interval contains a value.
    pub fn contains(&self, value: f64) -> bool {
        self.start <= value && value <= self.end
    }
}
