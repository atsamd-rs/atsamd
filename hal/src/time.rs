//! Time units

// Frequency based

/// Hertz
pub type Hertz = fugit::HertzU32;

/// KiloHertz
pub type KiloHertz = fugit::KilohertzU32;

/// MegaHertz
pub type MegaHertz = fugit::MegahertzU32;

// Period based

/// Seconds
pub type Seconds = fugit::SecsDurationU64;

/// Milliseconds
pub type Milliseconds = fugit::MillisDurationU64;

/// Microseconds
pub type Microseconds = fugit::MicrosDurationU64;

/// Nanoseconds
pub type Nanoseconds = fugit::NanosDurationU64;
