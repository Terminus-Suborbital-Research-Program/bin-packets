use bincode::{Decode, Encode};

use crate::data::IcarusStatus;
use crate::{data::EjectorStatus, phases::EjectorPhase};

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq)]
pub struct ScientificPacket {
    pub packets: u32,
    pub temperature: f32,
}

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq)]
pub enum CommandPacket {
    SyncTime(u32),
    Ping,
    ConnectionTest(ConnectionTest),
    EjectorPhaseSet(EjectorPhase),
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
    Command(CommandPacket),
    IcarusState(IcarusStatus),
    EjectorStatus(EjectorStatus),
}
