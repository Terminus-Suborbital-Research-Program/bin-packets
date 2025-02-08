use bincode::{Decode, Encode};

use crate::commands::CommandPacket;

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub enum ApplicationPacket {
    Command(CommandPacket),
}
