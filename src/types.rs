use bincode::{Decode, Encode};

/// Used to represent unix timestamp in milliseconds
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct UnixTimestampMillis {
    pub timestamp: u64,
}

impl UnixTimestampMillis {
    pub fn new(timestamp: u64) -> Self {
        Self { timestamp }
    }

    pub fn to_duration(self, other: Self) -> DurationMillis {
        DurationMillis {
            duration: other.timestamp - self.timestamp,
        }
    }
}

/// A duration represented in milliseconds
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct DurationMillis {
    pub duration: u64,
}
