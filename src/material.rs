use ed_parse_log_files_macros::testcase_struct;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::EDString;

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum RawMaterialName {
    #[serde(alias = "antimony")]
    Antimony,
    #[serde(alias = "arsenic")]
    Arsenic,
    #[serde(alias = "boron")]
    Boron,
    #[serde(alias = "cadmium")]
    Cadmium,
    #[serde(alias = "carbon")]
    Carbon,
    #[serde(alias = "chromium")]
    Chromium,
    #[serde(alias = "germanium")]
    Germanium,
    #[serde(alias = "hydrogen")]
    Hydrogen,
    #[serde(alias = "iron")]
    Iron,
    #[serde(alias = "lead")]
    Lead,
    #[serde(alias = "manganese")]
    Manganese,
    #[serde(alias = "mercury")]
    Mercury,
    #[serde(alias = "molybdenum")]
    Molybdenum,
    #[serde(alias = "nickel")]
    Nickel,
    #[serde(alias = "niobium")]
    Niobium,
    #[serde(alias = "phosphorus")]
    Phosphorus,
    #[serde(alias = "polonium")]
    Polonium,
    #[serde(alias = "rhenium")]
    Rhenium,
    #[serde(alias = "ruthenium")]
    Ruthenium,
    #[serde(alias = "selenium")]
    Selenium,
    #[serde(alias = "sulphur")]
    Sulphur,
    #[serde(alias = "technetium")]
    Technetium,
    #[serde(alias = "tellurium")]
    Tellurium,
    #[serde(alias = "tin")]
    Tin,
    #[serde(alias = "tungsten")]
    Tungsten,
    #[serde(alias = "vanadium")]
    Vanadium,
    #[serde(alias = "yttrium")]
    Yttrium,
    #[serde(alias = "zinc")]
    Zinc,
    #[serde(alias = "zirconium")]
    Zirconium,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct RawMaterial {
    pub name: RawMaterialName,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum ManufacturedMaterialName {
    #[serde(alias = "basicconductors")]
    #[strum(to_string = "Basic Conductors")]
    BasicConductors,
    #[serde(alias = "biotechconductors")]
    #[strum(to_string = "Biotech Conductors")]
    BiotechConductors,
    #[serde(alias = "chemicaldistillery")]
    #[strum(to_string = "Chemical Distillery")]
    ChemicalDistillery,
    #[serde(alias = "chemicalmanipulators")]
    #[strum(to_string = "Chemical Manipulators")]
    ChemicalManipulators,
    #[serde(alias = "chemicalprocessors")]
    #[strum(to_string = "Chemical Processors")]
    ChemicalProcessors,
    #[serde(alias = "chemicalstorageunits")]
    #[strum(to_string = "Chemical Storage Units")]
    ChemicalStorageUnits,
    #[serde(alias = "compactcomposites")]
    #[strum(to_string = "Compact Composites")]
    CompactComposites,
    #[serde(alias = "compoundshielding")]
    #[strum(to_string = "Compound Shielding")]
    CompoundShielding,
    #[serde(alias = "conductiveceramics")]
    #[strum(to_string = "Conductive Ceramics")]
    ConductiveCeramics,
    #[serde(alias = "conductivecomponents")]
    #[strum(to_string = "Conductive Components")]
    ConductiveComponents,
    #[serde(alias = "conductivepolymers")]
    #[strum(to_string = "Conductive Polymers")]
    ConductivePolymers,
    #[serde(alias = "configurablecomponents")]
    #[strum(to_string = "Configurable Components")]
    ConfigurableComponents,
    #[serde(alias = "crystalshards")]
    #[strum(to_string = "Crystal Shards")]
    CrystalShards,
    #[serde(alias = "electrochemicalarrays")]
    #[strum(to_string = "Electrochemical Arrays")]
    ElectrochemicalArrays,
    #[serde(alias = "exquisitefocuscrystals")]
    #[strum(to_string = "Exquisite Focus Crystals")]
    ExquisiteFocusCrystals,
    #[serde(alias = "fedcorecomposites")]
    #[strum(to_string = "Core Dynamics Composites")]
    FEDCoreComposites,
    #[serde(alias = "fedproprietarycomposites")]
    #[strum(to_string = "Proprietary Composites")]
    FEDProprietaryComposites,
    #[serde(alias = "filamentcomposites")]
    #[strum(to_string = "Filament Composites")]
    FilamentComposites,
    #[serde(alias = "focuscrystals")]
    #[strum(to_string = "Focus Crystals")]
    FocusCrystals,
    #[serde(alias = "galvanisingalloys")]
    #[strum(to_string = "Galvanising Alloys")]
    GalvanisingAlloys,
    #[serde(alias = "gridresistors")]
    #[strum(to_string = "Grid Resistors")]
    GridResistors,
    #[serde(alias = "guardian_powercell")]
    #[strum(to_string = "Guardian Power Cell")]
    GuardianPowerCell,
    #[serde(alias = "guardian_powerconduit")]
    #[strum(to_string = "Guardian Power Conduit")]
    GuardianPowerConduit,
    #[serde(alias = "guardian_sentinel_wreckagecomponents")]
    #[strum(to_string = "Guardian Wreckage Components")]
    GuardianSentinelWreckageComponents,
    #[serde(alias = "guardian_techcomponent")]
    #[strum(to_string = "Guardian Technology Component")]
    GuardianTechComponent,
    #[serde(alias = "guardian_sentinel_weaponparts")]
    #[strum(to_string = "Guardian Sentinel Weapon Parts")]
    GuardianSentinelWeaponParts,
    #[serde(alias = "heatexchangers")]
    #[strum(to_string = "Heat Exchangers")]
    HeatExchangers,
    #[serde(alias = "heatconductionwiring")]
    #[strum(to_string = "Heat Conduction Wiring")]
    HeatConductionWiring,
    #[serde(alias = "heatdispersionplate")]
    #[strum(to_string = "Heat Dispersion Plate")]
    HeatDispersionPlate,
    #[serde(alias = "heatresistantceramics")]
    #[strum(to_string = "Heat Resistant Ceramics")]
    HeatResistantCeramics,
    #[serde(alias = "heatvanes")]
    #[strum(to_string = "Heat Vanes")]
    HeatVanes,
    #[serde(alias = "highdensitycomposites")]
    #[strum(to_string = "High Density Composites")]
    HighDensityComposites,
    #[serde(alias = "hybridcapacitors")]
    #[strum(to_string = "Hybrid Capacitors")]
    HybridCapacitors,
    #[serde(alias = "imperialshielding")]
    #[strum(to_string = "Imperial Shielding")]
    ImperialShielding,
    #[serde(alias = "improvisedcomponents")]
    #[strum(to_string = "Improvised Components")]
    ImprovisedComponents,
    #[serde(alias = "mechanicalcomponents")]
    #[strum(to_string = "Mechanical Components")]
    MechanicalComponents,
    #[serde(alias = "mechanicalequipment")]
    #[strum(to_string = "Mechanical Equipment")]
    MechanicalEquipment,
    #[serde(alias = "mechanicalscrap")]
    #[strum(to_string = "Mechanical Scrap")]
    MechanicalScrap,
    #[serde(alias = "militarygradealloys")]
    #[strum(to_string = "Military Grade Alloys")]
    MilitaryGradeAlloys,
    #[serde(alias = "militarysupercapacitors")]
    #[strum(to_string = "Military Supercapacitors")]
    MilitarySupercapacitors,
    #[serde(alias = "pharmaceuticalisolators")]
    #[strum(to_string = "Pharmaceutical Isolators")]
    PharmaceuticalIsolators,
    #[serde(alias = "phasealloys")]
    #[strum(to_string = "Phase Alloys")]
    PhaseAlloys,
    #[serde(alias = "precipitatedalloys")]
    #[strum(to_string = "Precipitated Alloys")]
    PrecipitatedAlloys,
    #[serde(alias = "protoheatradiators")]
    #[strum(to_string = "Proto Heat Radiators")]
    ProtoHeatRadiators,
    #[serde(alias = "protoradiolicalloys")]
    #[strum(to_string = "Proto Radiolic Alloys")]
    ProtoRadiocalciumAlloys,
    #[serde(alias = "polymercapacitors")]
    #[strum(to_string = "Polymer Capacitors")]
    PolymerCapacitors,
    #[serde(alias = "protolightalloys")]
    #[strum(to_string = "Proto Light Alloys")]
    ProtoLightAlloys,
    #[serde(alias = "refinedfocuscrystals")]
    #[strum(to_string = "Refined Focus Crystals")]
    RefinedFocusCrystals,
    #[serde(alias = "salvagedalloys")]
    #[strum(to_string = "Salvaged Alloys")]
    SalvagedAlloys,
    #[serde(alias = "shieldemitters")]
    #[strum(to_string = "Shield Emitters")]
    ShieldEmitters,
    #[serde(alias = "shieldingsensors")]
    #[strum(to_string = "Shielding Sensors")]
    ShieldingSensors,
    #[serde(alias = "temperedalloys")]
    #[strum(to_string = "Tempered Alloys")]
    TemperedAlloys,
    #[serde(alias = "tg_abrasion03")]
    #[strum(to_string = "Hardened Surface Fragments")]
    TgAbrasion03,
    #[serde(alias = "tg_biomechanicalconduits")]
    #[strum(to_string = "Bio-Mechanical Conduits")]
    TgBioMechanicalConduits,
    #[serde(alias = "tg_causticcrystal")]
    #[strum(to_string = "Caustic Crystal")]
    TgCausticCrystal,
    #[serde(alias = "tg_causticgeneratorparts")]
    #[strum(to_string = "Corrosive Mechanisms")]
    TgCausticGeneratorParts,
    #[serde(alias = "tg_causticshard")]
    #[strum(to_string = "Caustic Shard")]
    TgCausticShard,
    #[serde(alias = "tg_propulsionelement")]
    #[strum(to_string = "Propulsion Elements")]
    TgPropulsionElement,
    #[serde(alias = "tg_weaponparts")]
    #[strum(to_string = "Weapon Parts")]
    TgWeaponParts,
    #[serde(alias = "tg_wreckagecomponents")]
    #[strum(to_string = "Wreckage Components")]
    TgWreckageComponents,
    #[serde(alias = "thermicalloys")]
    #[strum(to_string = "Thermic Alloys")]
    ThermalAlloys,
    #[serde(alias = "uncutfocuscrystals")]
    #[strum(to_string = "Flawed Focus Crystals")]
    UncutFocusCrystals,
    #[serde(alias = "unknowncarapace")]
    #[strum(to_string = "Thargoid Carapace")]
    UnknownCarapace,
    #[serde(alias = "unknowncorechip")]
    #[strum(to_string = "Tactical Core Chip")]
    UnknownCoreChip,
    #[serde(alias = "unknownenergycell")]
    #[strum(to_string = "Thargoid Energy Cell")]
    UnknownEnergyCell,
    #[serde(alias = "unknownenergysource")]
    #[strum(to_string = "Sensor Fragment")]
    UnknownEnergySource,
    #[serde(alias = "unknownorganiccircuitry")]
    #[strum(to_string = "Thargoid Organic Circuitry")]
    UnknownOrganicCircuitry,
    #[serde(alias = "unknowntechnologycomponents")]
    #[strum(to_string = "Thargoid Technological Components")]
    UnknownTechnologyComponents,
    #[serde(alias = "wornshieldemitters")]
    #[strum(to_string = "Worn Shield Emitters")]
    WornShieldEmitters,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"heatexchangers", "Count":9 })]
pub struct ManufacturedMaterial {
    pub name: ManufacturedMaterialName,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum EncodedMaterialName {
    #[serde(alias = "adaptiveencryptors")]
    #[strum(to_string = "Adaptive Encryptors Capture")]
    AdaptiveEncryptors,
    #[serde(alias = "ancientbiologicaldata")]
    #[strum(to_string = "Pattern Alpha Obelisk Data")]
    AncientBiologicalData,
    #[serde(alias = "ancientculturaldata")]
    #[strum(to_string = "Pattern Beta Obelisk Data")]
    AncientCulturalData,
    #[serde(alias = "ancienthistoricaldata")]
    #[strum(to_string = "Pattern Gamma Obelisk Data")]
    AncientHistoricalData,
    #[serde(alias = "ancientlanguagedata")]
    #[strum(to_string = "Pattern Delta Obelisk Data")]
    AncientLanguageData,
    #[serde(alias = "ancienttechnologicaldata")]
    #[strum(to_string = "Pattern Epsilon Obelisk Data")]
    AncientTechnologicalData,
    #[serde(alias = "archivedemissiondata")]
    #[strum(to_string = "Irregular Emission Data")]
    ArchivedEmissionData,
    #[serde(alias = "bulkscandata")]
    #[strum(to_string = "Anomalous Bulk Scan Data")]
    BulkScanData,
    #[serde(alias = "classifiedscandata")]
    #[strum(to_string = "Classified Scan Fragment")]
    ClassifiedScanData,
    #[serde(alias = "compactemissionsdata")]
    #[strum(to_string = "Abnormal Compact Emissions Data")]
    CompactEmissionData,
    #[serde(alias = "consumerfirmware")]
    #[strum(to_string = "Modified Consumer Firmware")]
    ConsumerFirmware,
    #[serde(alias = "dataminedwake")]
    #[strum(to_string = "Datamined Wake Exceptions")]
    DataminedWake,
    #[serde(alias = "decodedemissiondata")]
    #[strum(to_string = "Decoded Emission Data")]
    DecodedEmissionData,
    #[serde(alias = "disruptedwakeechoes")]
    #[strum(to_string = "Atypical Disrupted Wake Echoes")]
    DisruptedWakeEchoes,
    #[serde(alias = "embeddedfirmware")]
    #[strum(to_string = "Modified Embedded Firmware")]
    EmbeddedFirmware,
    #[serde(alias = "emissiondata")]
    #[strum(to_string = "Unexpected Emission Data")]
    EmissionData,
    #[serde(alias = "encodedscandata")]
    #[strum(to_string = "Divergent Scan Data")]
    EncodedScanData,
    #[serde(alias = "encryptedfiles")]
    #[strum(to_string = "Unusual Encrypted Files")]
    EncryptedFiles,
    #[serde(alias = "encryptionarchives")]
    #[strum(to_string = "Atypical Encryption Archives")]
    EncryptionArchives,
    #[serde(alias = "encryptioncodes")]
    #[strum(to_string = "Tagged Encryption Codes")]
    EncryptionCodes,
    #[serde(alias = "fsdtelemetry")]
    #[strum(to_string = "Anomalous FSD Telemetry")]
    FSDTelemetry,
    #[serde(alias = "guardian_moduleblueprint")]
    #[strum(to_string = "Guardian Module Blueprint Fragment")]
    GuardianModuleBlueprint,
    #[serde(alias = "guardian_vesselblueprint")]
    #[strum(to_string = "Guardian Vessel Blueprint Fragment")]
    GuardianVesselBlueprint,
    #[serde(alias = "guardian_weaponblueprint")]
    #[strum(to_string = "Guardian Weapon Blueprint Fragment")]
    GuardianWeaponBlueprint,
    #[serde(alias = "hyperspacetrajectories")]
    #[strum(to_string = "Eccentric Hyperspace Trajectories")]
    HyperspaceTrajectories,
    #[serde(alias = "industrialfirmware")]
    #[strum(to_string = "Cracked Industrial Firmware")]
    IndustrialFirmware,
    #[serde(alias = "legacyfirmware")]
    #[strum(to_string = "Specialised Legacy Firmware")]
    LegacyFirmware,
    #[serde(alias = "scanarchives")]
    #[strum(to_string = "Unidentified Scan Archives")]
    ScanArchives,
    #[serde(alias = "scandatabanks")]
    #[strum(to_string = "Classified Scan Databanks")]
    ScanDatabanks,
    #[serde(alias = "scrambledemissiondata")]
    #[strum(to_string = "Exceptional Scrambled Emission Data")]
    ScrambledEmissionData,
    #[serde(alias = "securityfirmware")]
    #[strum(to_string = "Security Firmware Patch")]
    SecurityFirmware,
    #[serde(alias = "shieldcyclerecordings")]
    #[strum(to_string = "Distorted Shield Cycle Recordings")]
    ShieldCycleRecordings,
    #[serde(alias = "shielddensityreports")]
    #[strum(to_string = "Untypical Shield Scans")]
    ShieldDensityReports,
    #[serde(alias = "shieldfrequencydata")]
    #[strum(to_string = "Peculiar Shield Frequency Data")]
    ShieldFrequencyData,
    #[serde(alias = "shieldpatternanalysis")]
    #[strum(to_string = "Aberrant Shield Pattern Analysis")]
    ShieldPatternAnalysis,
    #[serde(alias = "shieldsoakanalysis")]
    #[strum(to_string = "Inconsistent Shield Soak Analysis")]
    ShieldSoakAnalysis,
    #[serde(alias = "symmetrickeys")]
    #[strum(to_string = "Open Symmetric Keys")]
    SymmetricKeys,
    #[serde(alias = "tg_interdictiondata")]
    #[strum(to_string = "Thargoid Interdiction Telemetry")]
    TgInterdictionData,
    #[serde(alias = "tg_shipflightdata")]
    #[strum(to_string = "Ship Flight Data")]
    TgShipFlightData,
    #[serde(alias = "tg_shipsystemsdata")]
    #[strum(to_string = "Ship Systems Data")]
    TgShipSystemsData,
    #[serde(alias = "tg_shutdowndata")]
    #[strum(to_string = "Massive Energy Surge Analytics")]
    TgShutdownData,
    #[serde(alias = "tg_structuraldata")]
    #[strum(to_string = "Thargoid Structural Data")]
    TgStructuralData,
    #[serde(alias = "tg_compositiondata")]
    #[strum(to_string = "Thargoid Material Composition Data")]
    TgCompositionData,
    #[serde(alias = "tg_residuedata")]
    #[strum(to_string = "Thargoid Residue Data")]
    TgResidueData,
    #[serde(alias = "unknownshipsignature")]
    #[strum(to_string = "Thargoid Ship Signature")]
    UnknownShipSignature,
    #[serde(alias = "unknownwakedata")]
    #[strum(to_string = "Thargoid Wake Data")]
    UnknownWakeData,
    #[serde(alias = "wakesolutions")]
    #[strum(to_string = "Strange Wake Solutions")]
    WakeSolutions,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"decodedemissiondata", "Count":9 })]
pub struct EncodedMaterial {
    pub name: EncodedMaterialName,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub count: u64,
}
