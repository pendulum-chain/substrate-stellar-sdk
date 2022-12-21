use crate::TimeBounds;

pub trait IntoTimePoint {
    fn into_time_point(self) -> u64;
}

pub struct MilliSecondEpochTime(pub u64);
pub struct SecondEpochTime(pub u64);

impl IntoTimePoint for MilliSecondEpochTime {
    fn into_time_point(self) -> u64 {
        self.0 / 1000
    }
}

impl IntoTimePoint for SecondEpochTime {
    fn into_time_point(self) -> u64 {
        self.0
    }
}

impl IntoTimePoint for () {
    fn into_time_point(self) -> u64 {
        0
    }
}

impl TimeBounds {
    pub fn from_time_points<T: IntoTimePoint, S: IntoTimePoint>(min_time: T, max_time: S) -> TimeBounds {
        TimeBounds { min_time: min_time.into_time_point(), max_time: max_time.into_time_point() }
    }
}
