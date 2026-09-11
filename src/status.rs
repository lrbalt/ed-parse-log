use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::Display;

use crate::{EDString, common_types::Credits};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct StatusFlags: u64 {
        const DOCKED = 1 << 0;
        const LANDED = 1 << 1;
        const LANDING_GEAR_DOWN = 1 << 2;
        const SHIELDS_UP = 1 << 3;
        const SUPERCRUISE = 1 << 4;
        const FLIGHT_ASSIST_OFF = 1 << 5;
        const HARDPOINTS_DEPLOYED = 1 << 6;
        const IN_WING = 1 << 7;
        const LIGHTS_ON = 1 << 8;
        const CARGO_SCOOP_DEPLOYED = 1 << 9;
        const SILENT_RUNNING = 1 << 10;
        const SCOOPING_FUEL = 1 << 11;
        const SRV_HANDBRAKE = 1 << 12;
        const SRV_USING_TURRET_VIEW = 1 << 13;
        const SRV_TURRET_RETRACTED = 1 << 14;
        const SRV_DRIVE_ASSIST = 1 << 15;
        const FSD_MASS_LOCKED = 1 << 16;
        const FSD_CHARGING = 1 << 17;
        const FSD_COOLDOWN = 1 << 18;
        const LOW_FUEL = 1 << 19;
        const OVER_HEATING = 1 << 20;
        const HAS_LAT_LONG = 1 << 21;
        const IS_IN_DANGER = 1 << 22;
        const BEING_INTERDICTED = 1 << 23;
        const IN_MAIN_SHIP = 1 << 24;
        const IN_FIGHTER = 1 << 25;
        const IN_SRV = 1 << 26;
        const HUD_IN_ANALYSIS_MODE = 1 << 27;
        const NIGHT_VISION = 1 << 28;
        const ALTITUDE_FROM_AVERAGE_RADIUS = 1 << 29;
        const FSD_JUMP = 1 << 30;
        const SRV_HIGH_BEAM = 1 << 31;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct StatusFlags2: u64 {
        const ON_FOOT = 1 << 0;
        const IN_TAXI = 1 << 1;
        const IN_MULTICREW = 1 << 2;
        const ON_FOOT_IN_STATION = 1 << 3;
        const ON_FOOT_ON_PLANET = 1 << 4;
        const AIM_DOWN_SIGHT = 1 << 5;
        const LOW_OXYGEN = 1 << 6;
        const LOW_HEALTH = 1 << 7;
        const COLD = 1 << 8;
        const HOT = 1 << 9;
        const VERY_COLD = 1 << 10;
        const VERY_HOT = 1 << 11;
        const GLIDE_MODE = 1 << 12;
        const ON_FOOT_IN_HANGAR = 1 << 13;
        const ON_FOOT_SOCIAL_SPACE = 1 << 14;
        const ON_FOOT_EXTERIOR = 1 << 15;
        const BREATHABLE_ATMOSPHERE = 1 << 16;
        const TELEPRESENCE_MULTICREW = 1 << 17;
        const PHYSICAL_MULTICREW = 1 << 18;
        const FSD_HYPERDRIVE_CHARGING = 1 << 19;
        const SUPERCRUISE_OVERCHARGE = 1 << 20;
        const SUPERCRUISE_ASSIST = 1 << 21;
        const NPC_CREW_ACTIVE = 1 << 22;
    }
}

impl Serialize for StatusFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for StatusFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

impl Serialize for StatusFlags2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for StatusFlags2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, Display)]
#[repr(u8)]
pub enum GuiFocus {
    #[strum(to_string = "No Focus")]
    NoFocus = 0,
    #[strum(to_string = "Internal Panel")]
    InternalPanel = 1,
    #[strum(to_string = "External Panel")]
    ExternalPanel = 2,
    #[strum(to_string = "Communication Panel")]
    CommsPanel = 3,
    #[strum(to_string = "Role Panel")]
    RolePanel = 4,
    #[strum(to_string = "Station Services")]
    StationServices = 5,
    #[strum(to_string = "Galaxy Map")]
    GalaxyMap = 6,
    #[strum(to_string = "System Map")]
    SystemMap = 7,
    Orrery = 8,
    #[strum(to_string = "FSS Mode")]
    FSSMode = 9,
    #[strum(to_string = "SAA Mode")]
    SAAMode = 10,
    Codex = 11,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FuelStatus {
    fuel_main: f64,
    fuel_reservoir: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct OnFootStatus {
    pub oxygen: f64,
    pub health: f64,
    pub temperature: f64,
    pub selected_weapon: EDString,
    #[serde(rename = "SelectedWeapon_Localised")]
    pub selected_weapon_localised: EDString,
    pub gravity: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Destination {
    system: u64,
    body: u64,
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LegalState {
    Allied,
    Clean,
    IllegalCargo,
    Speeding,
    Wanted,
    Hostile,
    PassengerWanted,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-12-07T10:31:37Z", "event":"Status", "Flags":16842765, "Pips":[2,8,2], "FireGroup":0, 
    "Fuel":{ "FuelMain":15.146626, "FuelReservoir":0.382796 }, "GuiFocus":5 })]
#[testcase({ "timestamp":"2017-12-07T12:03:14Z", "event":"Status", "Flags":18874376, "Pips":[4,8,0], "FireGroup":0,
    "Fuel":{ "FuelMain":15.146626, "FuelReservoir":0.382796 }, "GuiFocus":0, "Latitude":-28.584963,
    "Longitude":6.826313, "Heading":109, "Altitude": 404 })]
#[testcase({ "timestamp":"2026-05-15T12:51:43Z", "event":"Status", "Flags":0 })]
#[testcase({ "timestamp":"2026-06-06T13:01:47Z", "event":"Status", "Flags":151060485, "Flags2":0, "Pips":[2,8,2], 
    "FireGroup":0, "GuiFocus":0, "Fuel":{ "FuelMain":128.000000, "FuelReservoir":1.110000 }, "Cargo":1324.000000, 
    "LegalState":"Hostile", "Balance":28304956592, "Destination":{ "System":2869441275273, "Body":33, 
    "Name":"Val-rasha Starport" } })]
#[testcase({ "timestamp":"2026-06-17T17:36:28Z", "event":"Status", "Flags":5, "Flags2":90121, "Oxygen":1.000000, 
    "Health":1.000000, "Temperature":293.000000, "SelectedWeapon":"", "LegalState":"Clean", "BodyName":"Borisenko Dock", 
    "Balance":28340984858 })]
#[testcase({ "timestamp":"2026-06-17T17:45:24Z", "event":"Status", "Flags":6291456, "Flags2":33041, "Oxygen":1.000000, 
    "Health":1.000000, "Temperature":127.730576, "SelectedWeapon":"$humanoid_fists_name;", 
    "SelectedWeapon_Localised":"Unarmed", "Gravity":0.219317, "LegalState":"Clean", "Latitude":-41.343941, 
    "Longitude":-60.463566, "Heading":-70, "BodyName":"16 Cygni B 6 a", "PlanetRadius":3507661.250000, "Balance":28340984858 })]
#[testcase({ "timestamp":"2026-09-07T13:08:33Z", "event":"Status", "Flags":419430488, "Flags2":0, "Pips":[4,8,0], "FireGroup":2, 
    "GuiFocus":0, "Fuel":{ "FuelMain":20.956671, "FuelReservoir":0.465250 }, "Cargo":0.000000, "LegalState":"Allied", 
    "Balance":31603096519, "Destination":{ "System":5367098657608, "Body":0, "Name":"Teegarden's star" } })]
pub struct EDLogStatus {
    pub flags: StatusFlags,
    pub flags2: Option<StatusFlags2>,
    pub pips: Option<[u8; 3]>,
    pub fire_group: Option<u64>,
    pub gui_focus: Option<GuiFocus>,
    pub fuel: Option<FuelStatus>,
    pub cargo: Option<f64>,
    pub legal_state: Option<LegalState>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub heading: Option<i64>,
    pub body_name: Option<EDString>,
    pub planet_radius: Option<f64>,
    pub balance: Option<Credits>,
    pub destination: Option<Destination>,
    #[serde(flatten)]
    pub on_foot: Option<OnFootStatus>,
}
