pub mod codex;
pub mod combat;
pub mod common_types;
pub mod exploration;
pub mod fleet_carrier;
pub mod log_line;
pub mod market_item;
pub mod material;
pub mod modules;
pub mod odyssey;
pub mod other;
pub mod powerplay;
pub mod ship_module;
pub mod ship_type;
pub mod squadron;
pub mod startup;
pub mod station_services;
pub mod statistics;
pub mod status;
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
