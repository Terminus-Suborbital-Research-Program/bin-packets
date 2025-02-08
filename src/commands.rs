use bincode::{Decode, Encode};

use crate::{
    phases::{EjectorPhase, IcarusPhase, JupiterPhase},
    types::UnixTimestampMillis,
};

/// Commands a transition into a phase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum PhaseCommand {
    Jupiter(JupiterPhase),
    Icarus(IcarusPhase),
    Ejector(EjectorPhase),
}

/// Holds command variants
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub enum CommandPacket {
    SetTimeTo(UnixTimestampMillis),
    SetPhase(PhaseCommand),
    Ping,
}
