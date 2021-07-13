use crate::TimeBounds;

pub trait AsTimePoint {
    fn as_time_bound(self) -> u64;
}

pub struct MilliSecondEpochTime(pub u64);
pub struct SecondEpochTime(pub u64);

impl AsTimePoint for MilliSecondEpochTime {
    fn as_time_bound(self) -> u64 {
        self.0 / 1000
    }
}

impl AsTimePoint for SecondEpochTime {
    fn as_time_bound(self) -> u64 {
        self.0
    }
}

impl AsTimePoint for () {
    fn as_time_bound(self) -> u64 {
        0
    }
}

impl TimeBounds {
    pub fn from_time_points<T: AsTimePoint, S: AsTimePoint>(
        min_time: T,
        max_time: S,
    ) -> TimeBounds {
        TimeBounds {
            min_time: min_time.as_time_bound(),
            max_time: max_time.as_time_bound(),
        }
    }
}
