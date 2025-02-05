use bincode::{Decode, Encode};

/// Used to represent unix timestamp in milliseconds
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct UnixTimestampMillis {
    pub timestamp: u64,
}

///
#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq)]
pub struct ScientificPacket {
    pub packets: u32,
    pub temperature: f32,
}

/// A variety of easy commands to send between different devices
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub enum CommandPacket {
    SyncTime(UnixTimestampMillis),
    ConnectionTest(ConnectionTest),
    Ping,
    Eject,
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub enum InfoPacket {
    Heartbeat(u64),
}

/// Range test command, just iterates up to 255
#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, Hash)]
pub enum ConnectionTest {
    Start,
    Sequence(u8),
    End,
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct TelemetryPacket {
    pub gyro_acelleration: [f32; 3],  // degrees/s^2
    pub linear_accleration: [f32; 3], // m/s^2
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub enum ApplicationPacket {
    Scientific(ScientificPacket),
    Command(CommandPacket),
    Telemetry(TelemetryPacket),
    Info(InfoPacket),
}
