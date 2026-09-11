pub mod codex;
pub mod combat;
pub mod commander;
pub mod common_types;
pub mod engineers;
pub mod exploration;
pub mod exploration_old;
pub mod fleet_carrier;
pub mod log_line;
pub mod market;
pub mod market_item_type;
pub mod material;
pub mod mission;
pub mod modules;
pub mod navigation;
pub mod odyssey;
pub mod other;
pub mod powerplay;
pub mod ship;
pub mod ship_module;
pub mod ship_type;
pub mod shipyard;
pub mod squadron;
pub mod startup;
pub mod station_services;
pub mod statistics;
pub mod status;
pub mod suits;
pub mod trade;
pub mod travel;
pub(crate) mod utils;

pub use utils::to_human_readable_string;

#[cfg(feature = "interning")]
use symbol_table::GlobalSymbol;

#[cfg(feature = "interning")]
pub type EDString = GlobalSymbol;

#[cfg(not(feature = "interning"))]
pub type EDString = String;
