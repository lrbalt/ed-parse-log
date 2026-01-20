use crate::{
    EDString,
    common_types::{
        CodexBodyInformation, Credits, FSSSignalType, LuminosityType, MaterialCategory, ScanType,
        ShipScanType, SignalType, StarClass, Unknown,
    },
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct DiscoveredSystem {
    system_name: EDString,
    num_bodies: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SpawningInfo {
    spawning_state: EDString,
    #[serde(rename = "SpawningState_Localised")]
    spawning_state_localised: EDString,
    spawning_faction: EDString,
    #[serde(rename = "SpawningFaction_Localised")]
    spawning_faction_localised: EDString,
    spawning_power: Option<EDString>,
    opposing_power: Option<EDString>,
    threat_level: Option<u32>,
    time_remaining: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-21T14:55:03Z", "event":"FSSSignalDiscovered", "SystemAddress":908486218450, 
    "SignalName":"U | HOFI", "SignalType":"SquadronCarrier", "IsStation":true })]
#[testcase({"timestamp":"2022-09-05T15:35:31Z","event":"FSSSignalDiscovered",
    "SystemAddress":633474192082,"SignalName":"Wnuk-Lipinski Installation"})]
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
pub struct AtmosphereComposition {
    name: AtmosphereType,
    percent: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BodyParent {
    ring: Option<u64>,
    star: Option<u64>,
    null: Option<u64>,
    planet: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MaterialOnBody {
    name: EDString,
    percent: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BodyComposition {
    ice: f64,
    rock: f64,
    metal: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ScannedStarDetails {
    star_type: Option<StarClass>,
    subclass: u64,
    stellar_mass: f64,
    absolute_magnitude: f64,
    #[serde(rename = "Age_MY")]
    age_my: u64,
    surface_temperature: f64,
    luminosity: LuminosityType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RingClass {
    #[serde(rename = "eRingClass_Rocky")]
    Rocky,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Ring {
    name: EDString,
    ring_class: RingClass,
    #[serde(rename = "MassMT")]
    mass_mt: f64,
    inner_rad: f64,
    outer_rad: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum AtmosphereType {
    None,

    Ammonia,
    AmmoniaOxygen,
    AmmoniaRich,
    Argon,
    ArgonRich,
    CarbonDioxide,
    CarbonDioxideRich,
    EarthLike,
    Hydrogen,
    Helium,
    Iron,
    MetallicVapour,
    Methane,
    MethaneRich,
    Neon,
    NeonRich,
    Nitrogen,
    Oxygen,
    Silicates,
    SilicateVapour,
    SulphurDioxide,
    Water,
    WaterRich,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum TerraformState {
    #[serde(rename = "")]
    None,
    Terraformable,
    Terraforming,
    Terraformed,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Display)]
pub enum PlanetClass {
    #[serde(rename = "Ammonia world")]
    #[strum(to_string = "Ammonia world")]
    AmmoniaWorld,
    #[serde(rename = "Earthlike body")]
    #[strum(to_string = "Earthlike body")]
    EarthlikeBody,
    #[serde(rename = "Gas giant with ammonia based life")]
    #[strum(to_string = "Gas giant with ammonia based life")]
    GasGiantWithAmmoniaBasedLife,
    #[serde(rename = "Gas giant with water based life")]
    #[strum(to_string = "Gas giant with water based life")]
    GasGiantWithWaterBasedLife,
    #[serde(rename = "High metal content body")]
    #[strum(to_string = "High metal content body")]
    HighMetalContentBody,
    #[serde(rename = "Helium rich gas giant")]
    #[strum(to_string = "Helium rich gas giant")]
    HeliumRichGasGiant,
    #[serde(rename = "Icy body")]
    #[strum(to_string = "Icy body")]
    IcyBody,
    #[serde(rename = "Metal rich body")]
    #[strum(to_string = "Metal rich body")]
    MetalRichBody,
    #[serde(rename = "Rocky ice body")]
    #[strum(to_string = "Rocky ice body")]
    RockyIceBody,
    #[serde(rename = "Rocky body")]
    #[strum(to_string = "Rocky body")]
    RockyBody,
    #[serde(rename = "Sudarsky class I gas giant")]
    #[strum(to_string = "Sudarsky class I gas giant")]
    SudarskyClassIGasGiant,
    #[serde(rename = "Sudarsky class II gas giant")]
    #[strum(to_string = "Sudarsky class II gas giant")]
    SudarskyClassIIGasGiant,
    #[serde(rename = "Sudarsky class III gas giant")]
    #[strum(to_string = "Sudarsky class III gas giant")]
    SudarskyClassIIIGasGiant,
    #[serde(rename = "Sudarsky class IV gas giant")]
    #[strum(to_string = "Sudarsky class IV gas giant")]
    SudarskyClassIVGasGiant,
    #[serde(rename = "Sudarsky class V gas giant")]
    #[strum(to_string = "Sudarsky class V gas giant")]
    SudarskyClassVGasGiant,
    #[serde(rename = "Water giant")]
    #[strum(to_string = "Water giant")]
    WaterGiant,
    #[serde(rename = "Water world")]
    #[strum(to_string = "Water world")]
    WaterWorld,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ScannedBodyDetails {
    pub tidal_lock: bool,
    pub terraform_state: TerraformState,
    pub planet_class: PlanetClass,
    pub atmosphere: EDString,
    pub atmosphere_type: AtmosphereType,
    pub atmosphere_composition: Option<Vec<AtmosphereComposition>>,
    pub volcanism: EDString,
    #[serde(rename = "MassEM")]
    pub mass_em: f64,
    pub radius: f64,
    pub surface_gravity: f64,
    pub surface_pressure: f64,
    pub landable: bool,
    pub materials: Option<Vec<MaterialOnBody>>,
    pub composition: BodyComposition,
    pub rings: Option<Vec<Ring>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ScannedCommonDetails {
    semi_major_axis: f64,
    eccentricity: f64,
    orbital_inclination: f64,
    periapsis: f64,
    orbital_period: f64,
    ascending_node: f64,
    mean_anomaly: f64,
    rotation_period: f64,
    axial_tilt: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ReserveLevel {
    #[serde(rename = "MajorResources")]
    Major,
    #[serde(rename = "PristineResources")]
    Pristine,
    #[serde(rename = "LowResources")]
    Low,
    #[serde(rename = "CommonResources")]
    Common,
    #[serde(rename = "DepletedResources")]
    Depleted,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T03:05:10Z", "event":"Scan", "BodyName":"Wolf 865 A B Belt Cluster 1", "DistanceFromArrivalLS":237.092957 })]
pub struct EDLogScan {
    pub scan_type: Option<ScanType>,
    pub body_name: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: Option<u64>,
    pub parents: Option<Vec<BodyParent>>,
    pub star_system: Option<EDString>,
    pub system_address: Option<u64>,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f64,
    #[serde(flatten)]
    pub star_details: Option<ScannedStarDetails>,
    #[serde(flatten)]
    pub body_details: Option<ScannedBodyDetails>,
    #[serde(flatten)]
    pub common_details: Option<ScannedCommonDetails>,
    pub reserve_level: Option<ReserveLevel>,
    pub was_discovered: Option<bool>,
    pub was_mapped: Option<bool>,
    pub was_footfalled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogScanned {
    scan_type: ShipScanType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDiscoveryScan {
    pub system_address: u64,
    pub bodies: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDatalinkScan {
    message: EDString,
    #[serde(rename = "Message_Localised")]
    message_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T03:05:10Z", "event":"NavBeaconScan", "NumBodies":24 })]
pub struct EDLogNavBeaconScan {
    pub system_address: Option<u64>,
    pub num_bodies: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-13T20:22:43Z", "event":"ScanOrganic", "ScanType":"Sample", "Genus":"$Codex_Ent_Ingensradices_Genus_Name;", "Genus_Localised":"Radicoida", "Species":"$Codex_Ent_Ingensradices_Unicus_Name;", "Species_Localised":"Radicoida Unica", "Variant":"$Codex_Ent_Ingensradices_Unicus_Name;", "Variant_Localised":"Radicoida Unica", "WasLogged":false, "SystemAddress":147882789259, "Body":3 })]
pub struct EDLogScanOrganic {
    pub scan_type: ScanType,
    pub genus: EDString,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: EDString,
    pub species: EDString,
    #[serde(rename = "Species_Localised")]
    pub species_localised: EDString,
    pub variant: Option<EDString>,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: Option<EDString>,
    pub system_address: u64,
    pub body: u64,
    pub was_logged: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCodexEntry {
    #[serde(rename = "EntryID")]
    entry_id: u64,
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: EDString,
    sub_category: EDString,
    #[serde(rename = "SubCategory_Localised")]
    sub_category_localised: EDString,
    category: EDString,
    #[serde(rename = "Category_Localised")]
    category_localised: EDString,
    region: EDString,
    #[serde(rename = "Region_Localised")]
    region_localised: EDString,
    system: EDString,
    system_address: u64,
    #[serde(flatten)]
    body_information: Option<CodexBodyInformation>,
    is_new_entry: Option<bool>,
    voucher_amount: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ProspectedMaterial {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    proportion: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogProspectedAsteroid {
    materials: Vec<ProspectedMaterial>,
    motherlode_material: Option<EDString>,
    #[serde(rename = "MotherlodeMaterial_Localised")]
    motherlode_material_localised: Option<EDString>,
    content: EDString,
    #[serde(rename = "Content_Localised")]
    content_localised: EDString,
    remaining: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogAsteroidCracked {
    body: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMiningRefined {
    #[serde(rename = "Type")]
    material_type: EDString,
    #[serde(rename = "Type_Localised")]
    material_type_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterialDiscovered {
    category: MaterialCategory,
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    discovery_number: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDataScanned {
    #[serde(rename = "Type")]
    data_type: EDString,
    #[serde(rename = "Type_Localised")]
    data_type_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyExplorationData {
    pub system: EDString,
    pub cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T02:42:32Z", "event":"SellExplorationData", "Systems":[ "Sinann", "Alrai Sector DL-Y d82", "Alrai Sector DL-Y d110" ], "Discovered":[  ], "BaseValue":9998, "Bonus":0 })]
pub struct EDLogSellExplorationData {
    pub systems: Vec<EDString>,
    pub discovered: Vec<Unknown>,
    pub base_value: Credits,
    pub bonus: Credits,
    pub total_earnings: Option<Credits>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMultiSellExplorationData {
    pub discovered: Vec<DiscoveredSystem>,
    pub base_value: Credits,
    pub bonus: Credits,
    pub total_earnings: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-12-17T10:30:50Z", "event":"FSSDiscoveryScan", "Progress":1.000000, 
    "BodyCount":2, "NonBodyCount":23, "SystemName":"HIP 12355", "SystemAddress":138741286052 })]
pub struct EDLogFSSDiscoveryScan {
    pub progress: f64,
    pub body_count: u64,
    pub non_body_count: u64,
    pub system_name: EDString,
    pub system_address: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStationBernalSphere {
    system_address: u64,
    signal_name: EDString,
    signal_type: SignalType,
    is_station: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-08-30T12:31:30Z", "event":"SAAScanComplete", 
    "BodyName":"Prae Drye XZ-P d5-4 7 f", "SystemAddress":147194694067, "BodyID":69, "ProbesUsed":2, "EfficiencyTarget":4 })]
pub struct EDLogSAAScanComplete {
    pub body_name: EDString,
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub probes_used: u64,
    pub efficiency_target: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
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
#[testcase({ "timestamp":"2026-01-07T19:57:12Z", "event":"FSSAllBodiesFound", 
    "SystemName":"Hyades Sector MX-T b3-2", "SystemAddress":5068732245337, "Count":1 })]
pub struct EDLogFSSAllBodiesFound {
    pub system_name: EDString,
    pub system_address: u64,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BodySignalType {
    #[serde(rename = "$SAA_SignalType_Biological;")]
    Biological,
    #[serde(rename = "$SAA_SignalType_Geological;")]
    Geological,
    #[serde(rename = "$SAA_SignalType_Guardian;")]
    Guardian,
    #[serde(rename = "$SAA_SignalType_Human;")]
    Human,
    #[serde(rename = "$SAA_SignalType_Other;")]
    Other,
    #[serde(rename = "$SAA_SignalType_Thargoid;")]
    Thargoid,

    Alexandrite,
    Benitoite,
    Bromellite,
    Grandidierite,
    LowTemperatureDiamond,
    Monazite,
    Musgravite,
    Opal,
    Painite,
    Platinum,
    Rhodplumsite,
    Serendibite,
    #[serde(alias = "tritium")]
    Tritium,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BodySignal {
    #[serde(rename = "Type")]
    pub body_signal_type: BodySignalType,
    #[serde(rename = "Type_Localised")]
    pub body_signal_type_localised: Option<EDString>,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFSSBodySignals {
    pub body_name: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub system_address: u64,
    pub signals: Vec<BodySignal>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum GenusType {
    #[serde(rename = "$Codex_Ent_Aleoids_Genus_Name;")]
    Aleoida,
    #[serde(rename = "$Codex_Ent_Bacterial_Genus_Name;")]
    Bacterium,
    #[serde(rename = "$Codex_Ent_Brancae_Name;")]
    #[strum(to_string = "Brain Trees")]
    BrainTrees,
    #[serde(rename = "$Codex_Ent_Cactoid_Genus_Name;")]
    Cactoida,
    #[serde(rename = "$Codex_Ent_Clypeus_Genus_Name;")]
    Clypeus,
    #[serde(rename = "$Codex_Ent_Conchas_Genus_Name;")]
    Concha,
    #[serde(rename = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    #[strum(to_string = "Crystalline Shards")]
    CrystallineShards,
    #[serde(rename = "$Codex_Ent_Electricae_Genus_Name;")]
    Electricae,
    #[serde(rename = "$Codex_Ent_Fonticulus_Genus_Name;")]
    Fonticulua,
    #[serde(rename = "$Codex_Ent_Fumerolas_Genus_Name;")]
    Fumerola,
    #[serde(rename = "$Codex_Ent_Shrubs_Genus_Name;")]
    Frutexa,
    #[serde(rename = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,
    #[serde(rename = "$Codex_Ent_Osseus_Genus_Name;")]
    Osseus,
    #[serde(rename = "$Codex_Ent_Recepta_Genus_Name;")]
    Recepta,
    #[serde(rename = "$Codex_Ent_Stratum_Genus_Name;")]
    Stratum,
    #[serde(rename = "$Codex_Ent_Tubus_Genus_Name;")]
    Tubus,
    #[serde(rename = "$Codex_Ent_Tussocks_Genus_Name;")]
    Tussock,
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
pub struct EDLogSAASignalsFound {
    pub body_name: EDString,
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub signals: Vec<BodySignal>,
    pub genuses: Option<Vec<SAAGenus>>,
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
                .as_ref()
                .unwrap()
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
