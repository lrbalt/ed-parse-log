use crate::{
    EDString,
    codex::{CodexCategory, CodexNames, CodexRegion, CodexSubCategory, GenusType},
    common_types::{
        AtmosphereType, BodySignalType, Credits, FSSSignalType, LuminosityType, MaterialCategory,
        PlanetClass, ReserveLevel, RingClass, ScanType, SignalType, StarClass, TerraformState,
        VulcanismType,
    },
    material::{EncodedMaterialName, ManufacturedMaterialName, RawMaterialName},
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-08-26T19:09:04Z", "event":"CodexEntry", "EntryID":1100502, "Name":"$Codex_Ent_G_TypeGiant_Name;", 
    "Name_Localised":"G Type Giant", "SubCategory":"$Codex_SubCategory_Stars;", "SubCategory_Localised":"Stars", 
    "Category":"$Codex_Category_StellarBodies;", "Category_Localised":"Astronomical Bodies", "Region":"$Codex_RegionName_18;", 
    "Region_Localised":"Inner Orion Spur", "System":"HIP 759", "SystemAddress":10460563811, "IsNewEntry":true })]
#[testcase({ "timestamp":"2026-01-26T15:45:23Z", "event":"CodexEntry", "EntryID":1101001, "Name":"$Codex_Ent_TTS_Type_Name;", 
    "Name_Localised":"T Tauri Star", "SubCategory":"$Codex_SubCategory_Stars;", "SubCategory_Localised":"Stars", 
    "Category":"$Codex_Category_StellarBodies;", "Category_Localised":"Astronomical Bodies", "Region":"$Codex_RegionName_10;", 
    "Region_Localised":"Norma Expanse", "System":"Skauduae QM-T c17-4", "SystemAddress":1185175214746, "BodyID":0, "IsNewEntry":true })]
#[testcase({ "timestamp":"2026-07-28T18:33:32Z", "event":"CodexEntry", "EntryID":2320605, "Name":"$Codex_Ent_Bacterial_06_G_Name;", 
    "Name_Localised":"Bacterium Alcyoneum - Emerald", "SubCategory":"$Codex_SubCategory_Organic_Structures;", 
    "SubCategory_Localised":"Organic structures", "Category":"$Codex_Category_Biology;", 
    "Category_Localised":"Biological and Geological", "Region":"$Codex_RegionName_4;", "Region_Localised":"Odin's Hold", 
    "System":"Dryooe Prou GG-Y f961", "SystemAddress":516065576045, "BodyID":21, "NearestDestination":"", "Latitude":16.642124, 
    "Longitude":-108.395966, "IsNewEntry":true })]
pub struct EDLogCodexEntry {
    #[serde(rename = "EntryID")]
    pub entry_id: u64,
    pub name: CodexNames,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub sub_category: CodexSubCategory,
    #[serde(rename = "SubCategory_Localised")]
    pub sub_category_localised: EDString,
    pub category: CodexCategory,
    #[serde(rename = "Category_Localised")]
    pub category_localised: EDString,
    pub region: CodexRegion,
    #[serde(rename = "Region_Localised")]
    pub region_localised: EDString,
    pub system: EDString,
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: Option<u64>,
    pub nearest_destination: Option<EDString>,
    #[serde(rename = "NearestDestination_Localised")]
    pub nearest_destination_localised: Option<EDString>,
    pub traits: Option<Vec<EDString>>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_new_entry: Option<bool>,
    pub new_traits_discovered: Option<bool>,
    pub voucher_amount: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-30T15:04:15Z", "event":"DiscoveryScan", "SystemAddress":11312985987147, "Bodies":1 })]
pub struct EDLogDiscoveryScan {
    pub system_address: u64,
    pub bodies: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Null":0 })]
#[testcase_struct({ "Star":4 })]
#[testcase_struct({ "Planet":1 })]
pub struct BodyParent {
    ring: Option<u64>,
    star: Option<u64>,
    null: Option<u64>,
    planet: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"Hydrogen", "Percent":73.118820 })]
pub struct MaterialOnBody {
    pub name: RawMaterialName,
    pub percent: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Ice":0.880023, "Rock":0.103365, "Metal":0.016612 })]
pub struct BodyComposition {
    ice: f64,
    rock: f64,
    metal: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Ring {
    name: EDString,
    ring_class: RingClass,
    #[serde(rename = "MassMT")]
    mass_mt: f64, // megatons
    inner_rad: f64,
    outer_rad: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AtmosphereComposition {
    name: AtmosphereType,
    percent: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "StarSystem":"Crookeou PZ-D d13-0", "SystemAddress":8279789171, "StarType":"MS", 
    "Subclass":4, "StellarMass":1.046875, "Radius":23464126.000000, "AbsoluteMagnitude":0.467133, 
    "Age_MY":10286, "Luminosity":"IIIb"})]
pub struct ScannedStarDetails {
    pub star_system: EDString,
    pub system_address: u64,
    pub star_type: StarClass,
    pub subclass: u8,      // 0-9
    pub stellar_mass: f64, //mass as multiple of Sol’s mass
    pub radius: f64,
    pub absolute_magnitude: f64,
    pub luminosity: LuminosityType,

    #[serde(rename = "Age_MY")]
    pub age_my: u64, // age in millions of years
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"TidalLock":true, "TerraformState":"", "PlanetClass":"Sudarsky class I gas giant", 
    "Atmosphere":"", "AtmosphereComposition":[ 
        { "Name":"Hydrogen", "Percent":73.118820 }, { "Name":"Helium", "Percent":26.881176 } 
    ], "Volcanism":"", "SurfaceGravity":5.869167, "SurfacePressure":0.000000, "Landable":false,
    "Composition":{ "Ice":0.880023, "Rock":0.103365, "Metal":0.016612 },"MassEM":8.107292,
"Materials":[ { "Name":"sulphur", "Percent":27.665514 }, { "Name":"carbon", "Percent":23.263832 } ]})]
pub struct ScannedBodyDetails {
    pub tidal_lock: bool,
    pub terraform_state: TerraformState,
    pub planet_class: PlanetClass,
    pub atmosphere: EDString,
    pub atmosphere_type: Option<AtmosphereType>,
    pub atmosphere_composition: Vec<AtmosphereComposition>,
    pub volcanism: VulcanismType,
    pub surface_gravity: f64,
    pub surface_pressure: f64,
    pub landable: bool,
    pub materials: Option<Vec<MaterialOnBody>>,
    pub composition: Option<BodyComposition>,
    #[serde(rename = "MassEM")]
    pub mass_em: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ScannedCommonDetails {
    // orbital parameters
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub orbital_inclination: f64,
    pub periapsis: f64,
    pub orbital_period: f64,

    pub ascending_node: f64,
    pub mean_anomaly: f64,
    pub surface_temperature: f64,

    // if rotating
    pub rotation_period: f64, // seconds
    pub axial_tilt: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Rings":[ 
    { "Name":"HIP 36601 C 5 A Ring", "RingClass":"eRingClass_Rocky", "MassMT":1.1261e+10, "InnerRad":6.0239e+07, "OuterRad":6.3172e+07 }, 
    { "Name":"HIP 36601 C 5 B Ring", "RingClass":"eRingClass_Icy", "MassMT":8.4636e+10, "InnerRad":6.3272e+07, "OuterRad":8.1918e+07 } ], 
    "ReserveLevel":"PristineResources"})]
pub struct ScannedRingDetails {
    pub rings: Vec<Ring>,
    pub reserve_level: ReserveLevel,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T03:05:10Z", "event":"Scan", "BodyName":"Wolf 865 A B Belt Cluster 1", "DistanceFromArrivalLS":237.092957 })]
#[testcase({ "timestamp":"2026-01-25T16:27:37Z", "event":"Scan", "ScanType":"AutoScan", "BodyName":"Wepe ZE-A g415", "BodyID":0, 
    "StarSystem":"Wepe ZE-A g415", "SystemAddress":27871541846, "DistanceFromArrivalLS":0.000000, "StarType":"O", "Subclass":0, 
    "StellarMass":18.582031, "Radius":4406219776.000000, "AbsoluteMagnitude":-6.894470, "Age_MY":8, "SurfaceTemperature":34124.000000, 
    "Luminosity":"Vz", "RotationPeriod":185024.686578, "AxialTilt":0.000000, "WasDiscovered":false, "WasMapped":false, "WasFootfalled":false })]
#[testcase({ "timestamp":"2026-03-30T07:21:02Z", "event":"Scan", "ScanType":"AutoScan", "BodyName":"Crookeou PZ-D d13-0 A", "BodyID":1, 
    "Parents":[ {"Null":0} ], "StarSystem":"Crookeou PZ-D d13-0", "SystemAddress":8279789171, "DistanceFromArrivalLS":0.000000, "StarType":"MS", 
    "Subclass":4, "StellarMass":1.046875, "Radius":21262647296.000000, "AbsoluteMagnitude":0.467133, "Age_MY":10286, "SurfaceTemperature":2853.000000, 
    "Luminosity":"IIIb", "SemiMajorAxis":4346273362636.566406, "Eccentricity":0.116731, "OrbitalInclination":-4.121931, "Periapsis":311.650187, 
    "OrbitalPeriod":19800743460.655212, "AscendingNode":-91.362499, "MeanAnomaly":10.587886, "RotationPeriod":10780308.587214, "AxialTilt":0.000000, 
    "WasDiscovered":true, "WasMapped":false, "WasFootfalled":false } )]
#[testcase({ "timestamp":"2022-11-08T20:32:31Z", "event":"Scan", "ScanType":"Detailed", "BodyName":"HIP 36601 C 5", "BodyID":57, 
    "Parents":[ {"Null":50}, {"Star":4}, {"Null":0} ], 
    "StarSystem":"HIP 36601", "SystemAddress":84456968626, "DistanceFromArrivalLS":153786.479702, "TidalLock":true, "TerraformState":"", 
    "PlanetClass":"Sudarsky class I gas giant", "Atmosphere":"", "AtmosphereComposition":[ { "Name":"Hydrogen", "Percent":73.118820 }, 
    { "Name":"Helium", "Percent":26.881176 } ], "Volcanism":"", "MassEM":8.107292, "Radius":23464126.000000, "SurfaceGravity":5.869167, 
    "SurfaceTemperature":121.758347, "SurfacePressure":0.000000, "Landable":false, "SemiMajorAxis":7426225364.208221, "Eccentricity":0.013424, 
    "OrbitalInclination":-9.891754, "Periapsis":80.738829, "OrbitalPeriod":32086901.664734, "RotationPeriod":34475477.363613, 
    "AxialTilt":-0.024055, "Rings":[ { "Name":"HIP 36601 C 5 A Ring", "RingClass":"eRingClass_Rocky", "MassMT":1.1261e+10, "InnerRad":6.0239e+07, 
    "OuterRad":6.3172e+07 }, { "Name":"HIP 36601 C 5 B Ring", "RingClass":"eRingClass_Icy", "MassMT":8.4636e+10, "InnerRad":6.3272e+07, 
    "OuterRad":8.1918e+07 } ], "ReserveLevel":"PristineResources", "WasDiscovered":true, "WasMapped":true })]
#[testcase({ "timestamp":"2026-08-31T19:22:27Z", "event":"Scan", "ScanType":"Detailed", "BodyName":"Praei Flee ID-J b0 B 1 a", 
    "BodyID":14, "Parents":[ {"Planet":13}, {"Star":2}, {"Null":0} ], "StarSystem":"Praei Flee ID-J b0", "SystemAddress":505332916737, 
    "DistanceFromArrivalLS":155045.431259, "TidalLock":true, "TerraformState":"", "PlanetClass":"Icy body", "Atmosphere":"", 
    "AtmosphereType":"None", "Volcanism":"", "MassEM":0.021615, "Radius":2457094.500000, "SurfaceGravity":1.427004, 
    "SurfaceTemperature":33.834370, "SurfacePressure":0.000000, "Landable":true, 
    "Materials":[ 
        { "Name":"sulphur", "Percent":27.665514 }, { "Name":"carbon", "Percent":23.263832 }, 
        { "Name":"phosphorus", "Percent":14.893905 }, { "Name":"iron", "Percent":11.943175 }, 
        { "Name":"nickel", "Percent":9.033318 }, { "Name":"manganese", "Percent":4.932409 }, 
        { "Name":"germanium", "Percent":3.150120 }, { "Name":"vanadium", "Percent":2.932828 }, 
        { "Name":"molybdenum", "Percent":0.779882 }, { "Name":"tin", "Percent":0.738850 }, 
        { "Name":"antimony", "Percent":0.666169 } ], 
    "Composition":{ "Ice":0.880023, "Rock":0.103365, "Metal":0.016612 }, "SemiMajorAxis":341321974.992752, "Eccentricity":0.000000, 
    "OrbitalInclination":-52.472669, "Periapsis":249.229361, "OrbitalPeriod":3105190.813541, "AscendingNode":51.948216, 
    "MeanAnomaly":185.472937, "RotationPeriod":3190762.416817, "AxialTilt":-0.163115, "WasDiscovered":false, "WasMapped":false, 
    "WasFootfalled":false })]
pub struct EDLogScan {
    pub scan_type: Option<ScanType>,
    pub body_name: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: Option<u64>,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f64,

    #[serde(flatten)]
    pub star_details: Option<ScannedStarDetails>,
    #[serde(flatten)]
    pub body_details: Option<ScannedBodyDetails>,

    // for both stars and planets
    #[serde(flatten)]
    pub ring_details: Option<ScannedRingDetails>,
    pub parents: Option<Vec<BodyParent>>,
    #[serde(flatten)]
    pub common_details: Option<ScannedCommonDetails>,

    pub was_discovered: Option<bool>,
    pub was_mapped: Option<bool>,
    pub was_footfalled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-07T19:57:12Z", "event":"FSSAllBodiesFound", 
    "SystemName":"Hyades Sector MX-T b3-2", "SystemAddress":5068732245337, "Count":1 })]
pub struct EDLogFSSAllBodiesFound {
    pub system_name: EDString,
    pub system_address: u64,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Type":"$SAA_SignalType_Geological;", "Type_Localised":"Geological", "Count":2 })]
#[testcase_struct({ "Type":"$PlanetaryMiningLocation_Name;", "Type_Localised":"Planetary Mining Location", "Count":11 } )]
pub struct BodySignal {
    #[serde(rename = "Type")]
    pub body_signal_type: BodySignalType,
    #[serde(rename = "Type_Localised")]
    pub body_signal_type_localised: Option<EDString>,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-31T19:06:37Z", "event":"FSSBodySignals", "BodyName":"Praei Flee FH-T d4-2 A 1 a", 
    "BodyID":19, "SystemAddress":76630120619, "Signals":[ 
        { "Type":"$SAA_SignalType_Geological;", "Type_Localised":"Geological", "Count":3 },
        { "Type":"$PlanetaryMiningLocation_Name;", "Type_Localised":"Planetary Mining Location", "Count":11 } 
] })]
pub struct EDLogFSSBodySignals {
    pub body_name: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub system_address: u64,
    pub signals: Vec<BodySignal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-12-17T10:30:50Z", "event":"FSSDiscoveryScan", "Progress":1.000000, 
    "BodyCount":2, "NonBodyCount":23, "SystemName":"HIP 12355", "SystemAddress":138741286052 })]
pub struct EDLogFSSDiscoveryScan {
    pub progress: f64, // value 0-1
    pub body_count: u64,
    pub non_body_count: u64,
    pub system_name: EDString,
    pub system_address: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SpawningInfo {
    spawning_state: EDString, //the BGS state that triggered this event
    #[serde(rename = "SpawningState_Localised")]
    spawning_state_localised: EDString,
    spawning_faction: EDString,
    #[serde(rename = "SpawningFaction_Localised")]
    spawning_faction_localised: EDString,
    spawning_power: Option<EDString>,
    opposing_power: Option<EDString>,
    threat_level: Option<u32>,
    time_remaining: f32, // remaining lifetime in seconds
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-21T14:55:03Z", "event":"FSSSignalDiscovered", "SystemAddress":908486218450, 
    "SignalName":"U | HOFI", "SignalType":"SquadronCarrier", "IsStation":true })]
#[testcase({"timestamp":"2022-09-05T15:35:31Z","event":"FSSSignalDiscovered",
    "SystemAddress":633474192082,"SignalName":"Wnuk-Lipinski Installation"})]
#[testcase({ "timestamp":"2022-06-25T13:37:06Z", "event":"FSSSignalDiscovered", "SystemAddress":2870782731769, 
    "SignalName":"$USS_DegradedEmissions;", "SignalName_Localised":"Unidentified signal source", 
    "USSType":"$USS_Type_Salvage;", "USSType_Localised":"Degraded emissions", "SpawningState":"$FactionState_None;", 
    "SpawningState_Localised":"None", "SpawningFaction":"$faction_none;", "SpawningFaction_Localised":"None", 
    "ThreatLevel":0, "TimeRemaining":701.706543 })]
pub struct EDLogFSSSignalDiscovered {
    pub system_address: u64,
    pub signal_name: EDString,
    #[serde(rename = "SignalName_Localised")]
    pub signal_name_localised: Option<EDString>,
    pub signal_type: Option<FSSSignalType>,
    #[serde(rename = "USSType")]
    pub uss_type: Option<EDString>,
    #[serde(rename = "USSType_Localised")]
    pub uss_type_localised: Option<EDString>,
    #[serde(flatten)]
    pub spawning_info: Option<SpawningInfo>,
    pub is_station: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CollectedEncodedMaterial {
    pub name: EncodedMaterialName,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CollectedManufacturedMaterial {
    pub name: ManufacturedMaterialName,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CollectedRawMaterial {
    pub name: RawMaterialName,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "Category", deny_unknown_fields)]
#[testcase_struct({"Category":"Encoded", "Name":"shieldcyclerecordings", "Name_Localised":"Distorted Shield Cycle Recordings"})]
#[testcase_struct({"Category":"Raw", "Name":"sulphur" })]
pub enum CollectedMaterialCategory {
    Encoded(CollectedEncodedMaterial),
    Manufactured(CollectedManufacturedMaterial),
    Raw(CollectedRawMaterial),
}

/// Convert from CollectedMaterialCategory to MaterialCategory.
/// Also used as a check that all variants of both enums are in sync.
impl From<CollectedMaterialCategory> for MaterialCategory {
    fn from(category: CollectedMaterialCategory) -> Self {
        match category {
            CollectedMaterialCategory::Encoded(_) => MaterialCategory::Encoded,
            CollectedMaterialCategory::Manufactured(_) => MaterialCategory::Manufactured,
            CollectedMaterialCategory::Raw(_) => MaterialCategory::Raw,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase")]
#[testcase({ "timestamp":"2026-07-17T17:28:05Z", "event":"MaterialCollected", "Category":"Encoded", "Name":"shieldcyclerecordings", "Name_Localised":"Distorted Shield Cycle Recordings", "Count":3 })]
#[testcase({ "timestamp":"2026-04-22T18:42:43Z", "event":"MaterialCollected", "Category":"Manufactured", "Name":"unknownenergycell", "Name_Localised":"Thargoid Energy Cell", "Count":3 })]
#[testcase({ "timestamp":"2026-08-09T13:52:51Z", "event":"MaterialCollected", "Category":"Raw", "Name":"sulphur", "Count":3 })]
pub struct EDLogMaterialCollected {
    #[serde(flatten)]
    pub category: CollectedMaterialCategory,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase")]
#[testcase({ "timestamp":"2016-06-10T14:32:03Z", "event":"MaterialDiscarded", "Category":"Raw", "Name":"sulphur", "Count": 5 })]
pub struct EDLogMaterialDiscarded {
    #[serde(flatten)]
    pub category: CollectedMaterialCategory,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase")]
#[testcase({ "timestamp":"2026-04-21T19:08:08Z", "event":"MaterialDiscovered", "Category":"Encoded", "Name":"tg_compositiondata", "Name_Localised":"Thargoid Material Composition Data", "DiscoveryNumber":45 })]
#[testcase({ "timestamp":"2022-10-10T10:46:51Z", "event":"MaterialDiscovered", "Category":"Manufactured", "Name":"guardian_techcomponent", "Name_Localised":"Guardian Technology Component", "DiscoveryNumber":49 })]
pub struct EDLogMaterialDiscovered {
    #[serde(flatten)]
    pub category: CollectedMaterialCategory,
    pub discovery_number: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "SystemName":"HIP 98965", "NumBodies":1 })]
pub struct DiscoveredSystem {
    system_name: EDString,
    num_bodies: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-24T18:42:46Z", "event":"MultiSellExplorationData", "Discovered":[ { "SystemName":"Ross 991", "NumBodies":2 } ], "BaseValue":16236, "Bonus":0, "TotalEarnings":16236 })]
pub struct EDLogMultiSellExplorationData {
    pub discovered: Vec<DiscoveredSystem>,
    pub base_value: Credits,
    pub bonus: Credits,
    pub total_earnings: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-11T19:55:33Z", "event":"NavBeaconScan", "SystemAddress":5306599838426, "NumBodies":23 })]
pub struct EDLogNavBeaconScan {
    pub system_address: u64,
    pub num_bodies: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-02-27T18:30:26Z", "event":"BuyExplorationData", "System":"Minerva", "Cost":1418 })]
pub struct EDLogBuyExplorationData {
    pub system: EDString,
    pub cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-30T12:31:30Z", "event":"SAAScanComplete", "BodyName":"Prae Drye XZ-P d5-4 7 f", 
    "SystemAddress":147194694067, "BodyID":69, "ProbesUsed":2, "EfficiencyTarget":4 })]
pub struct EDLogSAAScanComplete {
    pub body_name: EDString,
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub probes_used: u64,
    pub efficiency_target: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SAAGenus {
    pub genus: GenusType,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-02T19:24:58Z", "event":"SAASignalsFound", "BodyName":"Hypao Brai VV-X b14-18 B 2", 
    "SystemAddress":40055804550273, "BodyID":9, "Signals":[ 
        { "Type":"$SAA_SignalType_Biological;", "Type_Localised":"Biological", "Count":2 }, 
        { "Type":"$PlanetaryMiningLocation_Name;", "Type_Localised":"Planetary Mining Location", "Count":10 } ], 
    "Genuses":[ { "Genus":"$Codex_Ent_Bacterial_Genus_Name;", "Genus_Localised":"Bacterium" }, 
    { "Genus":"$Codex_Ent_Stratum_Genus_Name;", "Genus_Localised":"Stratum" } ] })]
#[testcase({"timestamp": "2026-08-30T01:56:01Z","event": "SAASignalsFound","BodyName": "Shinrarta Dezhra AB 3 c",
    "SystemAddress": 3932277478106,"BodyID": 61,"Signals": [
        {"Type": "$SAA_SignalType_Geological;","Type_Localised": "Geological","Count": 2},
        {"Type": "$PlanetaryMiningLocation_Name;","Type_Localised": "Planetary Mining Location","Count": 18},
        {"Type": "$SAA_SignalType_Human;","Type_Localised": "Human","Count": 1}
    ],"Genuses": []})]
pub struct EDLogSAASignalsFound {
    pub body_name: EDString,
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub signals: Vec<BodySignal>,
    pub genuses: Option<Vec<SAAGenus>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-02T19:21:18Z", "event":"ScanBaryCentre", "StarSystem":"Hypao Brai VV-X b14-18", 
    "SystemAddress":40055804550273, "BodyID":13, "SemiMajorAxis":27088435888.290405, "Eccentricity":0.004286, 
    "OrbitalInclination":-0.025448, "Periapsis":116.879201, "OrbitalPeriod":6575543.642044, "AscendingNode":-48.544241, 
    "MeanAnomaly":160.297909 })]
pub struct EDLogScanBaryCentre {
    pub star_system: EDString,
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub orbital_inclination: f64,
    pub periapsis: f64,
    pub orbital_period: f64,
    pub ascending_node: f64,
    pub mean_anomaly: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T02:42:32Z", "event":"SellExplorationData", "Systems":[ "Sinann", "Alrai Sector DL-Y d82", "Alrai Sector DL-Y d110" ], "Discovered":[  ], "BaseValue":9998, "Bonus":0 })]
pub struct EDLogSellExplorationData {
    pub systems: Vec<EDString>,
    pub discovered: Vec<EDString>,
    pub base_value: Credits,
    pub bonus: Credits, // first discovery bonus
    pub total_earnings: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LocationOnBody {
    latitude: f64,
    longitude: f64,
    heading: u64,
    altitude: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-08-22T17:54:49Z", "event":"Screenshot", "Filename":"\\ED_Pictures\\Screenshot_0004.bmp", 
    "Width":3840, "Height":2160, "System":"Kyloarph HG-O d6-4818", "Body":"Kyloarph HG-O d6-4818 1 b" })]
#[testcase({ "timestamp":"2025-03-23T11:28:12Z", "event":"Screenshot", "Filename":"\\ED_Pictures\\Screenshot_0001.bmp", 
    "Width":3840, "Height":2160, "System":"Hyades Sector FI-R b5-2", "Body":"Hyades Sector FI-R b5-2 C 3 a", 
    "Latitude":-61.859398, "Longitude":-155.874405, "Heading":139, "Altitude":286.921753 })]
pub struct EDLogScreenshot {
    filename: EDString,
    width: u64,
    height: u64,
    system: Option<EDString>,
    body: Option<EDString>,
    #[serde(flatten)]
    location_on_body: Option<LocationOnBody>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStationBernalSphere {
    system_address: u64,
    signal_name: EDString,
    signal_type: SignalType,
    is_station: bool,
}

#[test]
fn test_exploration() {
    let json2 = r#"{
    "timestamp": "2024-08-25T17:30:19Z",
    "event": "Scan",
    "ScanType": "Detailed",
    "BodyName": "Dyava ABC 1",
    "BodyID": 18,
    "Parents": [
        {
            "Null": 1
        },
        {
            "Null": 0
        }
    ],
    "StarSystem": "Dyava",
    "SystemAddress": 3107710833346,
    "DistanceFromArrivalLS": 260.202177,
    "TidalLock": true,
    "TerraformState": "Terraforming",
    "PlanetClass": "High metal content body",
    "Atmosphere": "nitrogen atmosphere",
    "AtmosphereType": "Nitrogen",
    "AtmosphereComposition": [
        {
            "Name": "Nitrogen",
            "Percent": 94.372269
        },
        {
            "Name": "Oxygen",
            "Percent": 5.087193
        },
        {
            "Name": "CarbonDioxide",
            "Percent": 0.521460
        }
    ],
    "Volcanism": "",
    "MassEM": 0.670844,
    "Radius": 5387590.000000,
    "SurfaceGravity": 9.211745,
    "SurfaceTemperature": 273.893555,
    "SurfacePressure": 137397.921875,
    "Landable": false,
    "Composition": {
        "Ice": 0.000000,
        "Rock": 0.666544,
        "Metal": 0.333456
    },
    "SemiMajorAxis": 91770311594.009399,
    "Eccentricity": 0.001335,
    "OrbitalInclination": -0.137544,
    "Periapsis": 28.357709,
    "OrbitalPeriod": 14343910.813332,
    "AscendingNode": -111.177103,
    "MeanAnomaly": 129.366323,
    "RotationPeriod": 14343960.161468,
    "AxialTilt": 1.115560,
    "WasDiscovered": false,
    "WasMapped": true
}"#;
    let line2: crate::log_line::EDLogLine = serde_json::from_str(json2).expect("Should parse");

    assert!(matches!(
        line2.event(),
        crate::log_line::EDLogEvent::Scan(_)
    ));
    if let crate::log_line::EDLogEvent::Scan(header) = line2.event() {
        assert_eq!(header.body_name.as_str(), "Dyava ABC 1");
        assert_eq!(
            header
                .body_details
                .as_ref()
                .unwrap()
                .atmosphere_composition
                .len(),
            3
        );
    }
}

#[test]
fn test_signal_type_optional() {
    let json = r#"{"timestamp":"2022-09-05T15:35:31Z","event":"FSSSignalDiscovered",
    "SystemAddress":633474192082,"SignalName":"Wnuk-Lipinski Installation"}"#;

    let line: crate::log_line::EDLogLine = serde_json::from_str(json).expect("Should parse");

    assert!(matches!(
        line.event(),
        crate::log_line::EDLogEvent::FSSSignalDiscovered(_)
    ));
    if let crate::log_line::EDLogEvent::FSSSignalDiscovered(header) = line.event() {
        assert_eq!(header.signal_name.as_str(), "Wnuk-Lipinski Installation");
        assert!(header.signal_type.is_none());
    }
}
