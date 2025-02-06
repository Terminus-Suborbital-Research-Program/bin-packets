use bincode::{Decode, Encode};

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq)]
pub struct ScientificPacket {
    pub packets: u32,
    pub temperature: f32,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, Hash)]
pub enum CommandPacket {
    SyncTime(u32),
    MoveServoDegrees(i32),
    EchoMessage(u8),
    ConnectionTest(ConnectionTest),
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub enum InfoPacket {
    Heartbeat(u64),
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, Hash)]
pub enum ConnectionTest {
    Start,
    Sequence(u8),
    End,
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct TelemetryPacket {
    pub gyro: (f32, f32, f32),
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub enum ApplicationPacket {
    Scientific(ScientificPacket),
    Command(CommandPacket),
    Telemetry(TelemetryPacket),
    Info(InfoPacket),
}
