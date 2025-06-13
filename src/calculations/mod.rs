//! Financial calculation modules

pub mod interest;
pub mod investment;
pub mod loan;
pub mod statistics;
pub mod ratios;

// Re-export commonly used functions
pub use interest::*;
pub use investment::*;
pub use loan::*;
pub use statistics::*;
pub use ratios::*;