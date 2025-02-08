use bincode::{Decode, Encode};

use crate::{
    phases::EjectorPhase,
    types::{DurationMillis, UnixTimestampMillis},
};

// Data Packet types

/// Status packet for Ejector
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct EjectorStatus {
    pub phase: EjectorPhase,
    pub time_in_phase: DurationMillis,
    pub timestamp: UnixTimestampMillis,
    pub packet_number: u16,
}

/// Status packet for ICARUS
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct IcarusStatus {
    pub time_in_phase: DurationMillis,
    pub timestamp: UnixTimestampMillis,
    pub packet_number: u16,
}

/// Status packet for JUPITER
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct JupiterStatus {
    pub time_in_phase: DurationMillis,
    pub timestamp: UnixTimestampMillis,
    pub packet_number: u16,
}

/// Status packet for Relay
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct RelayStatus {
    pub timestamp: UnixTimestampMillis,
    pub packet_number: u16,
}

/// Data packet for GUARD Geiger counter
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct GeigerData {
    pub counts: u32,
    pub over: DurationMillis,
    pub timestamp: UnixTimestampMillis,
    pub packet_number: u16,
}

/// Data packet for Peltier power generation
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct PeltierData {
    pub power: f32,
    pub temp_cold_c: f32,
    pub timestamp: UnixTimestampMillis,
    pub packet_number: u16,
}

/// Data packet for Solar panel power generation
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct SolarData {
    pub power: f32,
    pub timestamp: UnixTimestampMillis,
    pub packet_number: u16,
}

/// Telemetry packet for JUPITER
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct JupiterTelemetry {
    pub battery_voltage: f32,
    pub timestamp: UnixTimestampMillis,
    pub packet_number: u16,
    pub high_g_accel: f32, // Placeholder
    pub low_g_accel: f32,  // Placeholder
    pub gyro: f32,         // Placeholder
    pub temp_c: f32,       // Placeholder
    pub pressure_bar: f32, // Placeholder
    pub humidity: f32,     // Placeholder
}

/// Telemetry packet for ICARUS
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct IcarusTelemetry {
    pub battery_voltage: f32,
    pub timestamp: UnixTimestampMillis,
    pub packet_number: u16,
    pub accel: f32,  // Placeholder
    pub gyro: f32,   // Placeholder
    pub mag: f32,    // Placeholder
    pub temp_c: f32, // Placeholder
}
