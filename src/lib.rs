pub mod codex;
pub mod commander;
pub mod common_types;
pub mod community_goal;
pub mod docking;
pub mod drone;
pub mod engineers;
pub mod exploration;
pub mod fleet_carrier;
pub mod loadout;
pub mod location;
pub mod locker;
pub mod log_line;
pub mod market;
pub mod materials;
pub mod mission;
pub mod modules;
pub mod navigation;
pub mod powerplay;
pub mod ship;
pub mod shipyard;
pub mod statistics;
pub mod suits;
pub mod supercruise;
pub mod transport;
pub(crate) mod utils;
pub mod wing;

pub use utils::to_human_readable_string;

#[cfg(feature = "interning")]
use symbol_table::GlobalSymbol;

#[cfg(feature = "interning")]
pub type EDString = GlobalSymbol;

#[cfg(not(feature = "interning"))]
pub type EDString = String;
