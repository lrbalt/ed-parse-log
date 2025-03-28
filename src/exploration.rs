use crate::common_types::{
    CodexBodyInformation, FSSSignalType, LuminosityType, MaterialCategory, ScanType, ShipScanType,
    SignalType, StarClass, Unknown,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct DiscoveredSystem {
    system_name: String,
    num_bodies: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SpawningInfo {
    spawning_state: String,
    #[serde(rename = "SpawningState_Localised")]
    spawning_state_localised: String,
    spawning_faction: String,
    #[serde(rename = "SpawningFaction_Localised")]
    spawning_faction_localised: String,
    spawning_power: Option<String>,
    opposing_power: Option<String>,
    threat_level: Option<u32>,
    time_remaining: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFSSSignalDiscovered {
    system_address: u64,
    signal_name: String,
    #[serde(rename = "SignalName_Localised")]
    signal_name_localised: Option<String>,
    signal_type: Option<FSSSignalType>,
    #[serde(rename = "USSType")]
    uss_type: Option<String>,
    #[serde(rename = "USSType_Localised")]
    uss_type_localised: Option<String>,
    #[serde(flatten)]
    spawning_info: Option<SpawningInfo>,
    is_station: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AtmosphereComposition {
    name: String,
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
    name: String,
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
    name: String,
    ring_class: RingClass,
    #[serde(rename = "MassMT")]
    mass_mt: f64,
    inner_rad: f64,
    outer_rad: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ScannedBodyDetails {
    tidal_lock: bool,
    terraform_state: String,
    planet_class: String,
    atmosphere: String,
    atmosphere_type: String,
    atmosphere_composition: Option<Vec<AtmosphereComposition>>,
    volcanism: String,
    #[serde(rename = "MassEM")]
    mass_em: f64,
    radius: f64,
    surface_gravity: f64,
    surface_pressure: f64,
    landable: bool,
    materials: Option<Vec<MaterialOnBody>>,
    composition: BodyComposition,
    rings: Option<Vec<Ring>>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogScan {
    pub scan_type: ScanType,
    pub body_name: String,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub parents: Option<Vec<BodyParent>>,
    pub star_system: String,
    pub system_address: u64,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f64,
    #[serde(flatten)]
    pub star_details: Option<ScannedStarDetails>,
    #[serde(flatten)]
    pub body_details: Option<ScannedBodyDetails>,
    #[serde(flatten)]
    pub common_details: Option<ScannedCommonDetails>,
    pub reserve_level: Option<ReserveLevel>,
    pub was_discovered: bool,
    pub was_mapped: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogScanned {
    scan_type: ShipScanType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDiscoveryScan {
    pub system_address: u64,
    pub bodies: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDatalinkScan {
    message: String,
    #[serde(rename = "Message_Localised")]
    message_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogNavBeaconScan {
    pub system_address: u64,
    pub num_bodies: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogScanOrganic {
    pub scan_type: ScanType,
    pub genus: String,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: String,
    pub species: String,
    #[serde(rename = "Species_Localised")]
    pub species_localised: String,
    pub variant: Option<String>,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: Option<String>,
    pub system_address: u64,
    pub body: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCodexEntry {
    #[serde(rename = "EntryID")]
    entry_id: u64,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    sub_category: String,
    #[serde(rename = "SubCategory_Localised")]
    sub_category_localised: String,
    category: String,
    #[serde(rename = "Category_Localised")]
    category_localised: String,
    region: String,
    #[serde(rename = "Region_Localised")]
    region_localised: String,
    system: String,
    system_address: u64,
    #[serde(flatten)]
    body_information: Option<CodexBodyInformation>,
    is_new_entry: Option<bool>,
    voucher_amount: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ProspectedMaterial {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    proportion: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogProspectedAsteroid {
    materials: Vec<ProspectedMaterial>,
    motherlode_material: Option<String>,
    content: String,
    #[serde(rename = "Content_Localised")]
    content_localised: String,
    remaining: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMiningRefined {
    #[serde(rename = "Type")]
    material_type: String,
    #[serde(rename = "Type_Localised")]
    material_type_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMaterialDiscovered {
    category: MaterialCategory,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    discovery_number: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDataScanned {
    #[serde(rename = "Type")]
    data_type: String,
    #[serde(rename = "Type_Localised")]
    data_type_localised: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyExplorationData {
    system: String,
    cost: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellExplorationData {
    systems: Vec<String>,
    discovered: Vec<Unknown>,
    base_value: u64,
    bonus: u64,
    total_earnings: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SoldBioData {
    genus: String,
    #[serde(rename = "Genus_Localised")]
    genus_localised: String,
    species: String,
    #[serde(rename = "Species_Localised")]
    species_localised: String,
    variant: Option<String>,
    #[serde(rename = "Variant_Localised")]
    variant_localised: Option<String>,
    value: u64,
    bonus: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellOrganicData {
    #[serde(rename = "MarketID")]
    market_id: u64,
    bio_data: Vec<SoldBioData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFSSDiscoveryScan {
    pub progress: f64,
    pub body_count: u64,
    pub non_body_count: u64,
    pub system_name: String,
    pub system_address: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStationBernalSphere {
    system_address: u64,
    signal_name: String,
    signal_type: SignalType,
    is_station: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSAAScanComplete {
    body_name: String,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
    probes_used: u64,
    efficiency_target: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogScanBaryCentre {
    star_system: String,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
    semi_major_axis: f64,
    eccentricity: f64,
    orbital_inclination: f64,
    periapsis: f64,
    orbital_period: f64,
    ascending_node: f64,
    mean_anomaly: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFSSAllBodiesFound {
    system_name: String,
    system_address: u64,
    count: u64,
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
    body_signal_type: BodySignalType,
    #[serde(rename = "Type_Localised")]
    body_signal_type_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogFSSBodySignals {
    body_name: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    system_address: u64,
    signals: Vec<BodySignal>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SAAGenus {
    genus: String,
    #[serde(rename = "Genus_Localised")]
    genus_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSAASignalsFound {
    body_name: String,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
    signals: Vec<BodySignal>,
    genuses: Option<Vec<SAAGenus>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMultiSellExplorationData {
    discovered: Vec<DiscoveredSystem>,
    base_value: u64,
    bonus: u64,
    total_earnings: u64,
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
        assert_eq!(&header.body_name, "Dyava ABC 1");
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
