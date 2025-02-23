use bincode::{Decode, Encode};
use defmt::Format;

/// Used to represent unix timestamp in milliseconds
#[derive(Debug, Clone, Copy, Encode, Decode, Format)]
pub struct UnixTimestampMillis {
    pub timestamp: u64,
}

#[allow(dead_code)]
impl UnixTimestampMillis {
    pub fn new(timestamp: u64) -> Self {
        Self { timestamp }
    }
}

// Impliment add + sub for UnixTimestampMillis and DurationMillis
impl core::ops::Add<DurationMillis> for UnixTimestampMillis {
    type Output = UnixTimestampMillis;

    fn add(self, rhs: DurationMillis) -> Self::Output {
        UnixTimestampMillis {
            timestamp: self.timestamp + rhs.duration,
        }
    }
}

impl core::ops::Sub<DurationMillis> for UnixTimestampMillis {
    type Output = UnixTimestampMillis;

    fn sub(self, rhs: DurationMillis) -> Self::Output {
        UnixTimestampMillis {
            timestamp: self.timestamp - rhs.duration,
        }
    }
}

/// Subtracting two UnixTimestampMillis results in a DurationMillis
/// We abs this so that we can always get a positive duration
impl core::ops::Sub<UnixTimestampMillis> for UnixTimestampMillis {
    type Output = DurationMillis;

    fn sub(self, rhs: UnixTimestampMillis) -> Self::Output {
        DurationMillis {
            duration: (self.timestamp as i64 - rhs.timestamp as i64).abs() as u64,
        }
    }
}

/// A duration represented in milliseconds
#[derive(Debug, Clone, Copy, Encode, Decode, Format)]
pub struct DurationMillis {
    pub duration: u64,
}

impl DurationMillis {
    pub fn new(duration: u64) -> Self {
        Self { duration }
    }

    pub fn millis(self) -> u64 {
        self.duration
    }
}
