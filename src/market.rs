use crate::{
    EDString,
    common_types::{CarrierDockingAccess, Credits, StationType},
};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MarketItemType {
    Silver,
    Aluminium,
    Tritium,
    Platinum,
    Titanium,
    #[serde(alias = "Cobalt")]
    Cobalt,
    Steel,
    Palladium,
    Osmium,
    #[serde(alias = "Gold")]
    Gold,
    #[serde(alias = "Painite")]
    Painite,
    Beryllium,
    Bauxite,
    Gallite,
    Samarium,
    Lepidolite,
    Rutile,
    #[serde(alias = "Uraninite")]
    Uraninite,
    Moissanite,
    LowTemperatureDiamond,
    Taaffeite,
    Jadeite,
    Gallium,
    Uranium,
    Indite,
    Coltan,
    Praseodymium,
    Copper,
    Water,
    Indium,
    Bromellite,
    Bertrandite,
    Opal,
    GrandIdierite,
    Serendibite,
    Haematite,
    Monazite,
    Thorium,

    #[serde(alias = "SyntheticFabrics")]
    SyntheticFabrics,
    SyntheticReagents,
    SyntheticMeat,
    SuperConductors,
    AdvancedCatalysers,
    ReinforcedMountingPlate,
    MarineSupplies,
    #[serde(alias = "Clothing")]
    Clothing,
    #[serde(alias = "ConsumerTechnology")]
    ConsumerTechnology,
    DomesticAppliances,
    MagneticeMitterCoil,
    PowerTransferConduits,
    Explosives,
    PowerGenerators,
    ComercialSamples,
    #[serde(alias = "USSCargoTechnicalBlueprints")]
    UssCargoTechnicalBlueprints,
    #[serde(alias = "USSCargoTradeData")]
    UssCargoTradeData,
    #[serde(alias = "USSCargoPrototypeTech")]
    UssCargoPrototypeTech,
    #[serde(alias = "USSCargoBlackBox")]
    UssCargoBlackBox,
    #[serde(alias = "USSCargoMilitaryPlans")]
    UssCargoMilitaryPlans,
    #[serde(alias = "EncriptedDataStorage")]
    EncriptedDataStorage,
    AnimalMeat,
    Coffee,
    #[serde(alias = "Grain")]
    Grain,
    Fish,
    #[serde(alias = "Algae")]
    Algae,
    FruitAndVegetables,
    #[serde(alias = "FoodCartridges")]
    FoodCartridges,
    #[serde(alias = "Liquor")]
    Liquor,
    Beer,
    #[serde(alias = "Wine")]
    Wine,
    BasicNarcotics,
    GeologicalEquipment,
    Biowaste,
    ChemicalWaste,
    XiheCompanions,
    Landmines,
    NerveAgents,
    LithiumHydroxide,
    #[serde(alias = "Tobacco")]
    Tobacco,
    Tea,
    ProgenitorCells,
    HydrogenPeroxide,
    MethaneClathRate,
    PersonalGifts,
    PersonalWeapons,
    #[serde(alias = "PersonalEffects")]
    PersonalEffects,
    MilitarygradeFabrics,
    BattleWeapons,
    NonLethalWeapons,
    AgronomicTreatment,
    LiquidOxygen,
    MethanolMonoHydrateCrystals,
    AutoFabricators,
    ResonatingSeparators,
    PowerConverter,
    MicroControllers,
    HazardousEnvironmentSuits,
    ComputerComponents,
    ModularTerminals,
    Pesticides,
    RadiationBaffle,
    NeofabricInsulation,
    HnShockMount,
    WaterPurifiers,
    PerformanceEnhancers,
    TerrainenrichmentSystems,
    HelioStaticFurnaces,
    SemiConductors,
    #[serde(alias = "Polymers")]
    Polymers,
    InsulatingMembrane,
    CMMComposite,
    MedicalDiagnosticEquipment,
    HydrogenFuel,
    CeramicComposites,
    SurvivalEquipment,
    BuildingFabricators,
    EvacuationShelter,
    EmergencyPowercells,
    SurfaceStabilisers,
    StructuralRegulators,
    CropHarvesters,
    DiagnosticSensor,
    Hostage,
    #[serde(alias = "Scrap")]
    Scrap,
    CoolingHoses,
    ReactiveArmour,
    ConductiveFabrics,
    AdvancedMedicines,
    Lanthanum,
    CombatStabilisers,
    ArticulationMotors,
    ThermalCoolingUnits,
    SkimerComponents,
    MineralExtractors,
    GeologicalSamples,
    EncryptedCorrespondence,
    ScientificSamples,
    #[serde(alias = "ComercialSamples")]
    CommercialSamples,
    BootLegLiquor,
    PyrophylLite,
    Leather,
    Robotics,
    NaturalFabrics,
    ScientificResearch,

    Drones,
    #[serde(alias = "DamagedEscapePod")]
    DamagedEscapePod,
    #[serde(alias = "WreckageComponents")]
    WreckageComponents,
    AislingMediaMaterials,
    AislingPromotionalMaterials,
    RepublicanFieldSupplies,
    RepublicanGarisonSupplies,
    ImperialSlaves,

    KamitraCigars,
    FujinTea,
    LavianBrandy,
    KonggaAle,
    SoontillRelics,
    VegaSlimWeed,
    EraninPearlWhisky,
    DamnaCarapaces,
    AlienEggs,

    #[serde(alias = "OccupiedCryoPod")]
    OccupiedCryoPod,

    AncientCasket,
    #[serde(alias = "AncientRelic")]
    AncientRelic,
    AncientTablet,
    AncientTotem,
    AncientUrn,
    AncientOrb,
    AncientKey,

    #[serde(alias = "ThargoidTitanDriveComponent")]
    ThargoidTitanDriveComponent,
    #[serde(alias = "ThargoidBoneFragments")]
    ThargoidBoneFragments,
    #[serde(alias = "ThargoidGeneratorTissueSample")]
    ThargoidGeneratorTissueSample,
    #[serde(alias = "ThargoidCystSpecimen")]
    ThargoidCystSpecimen,
    #[serde(alias = "ThargoidPod")]
    ThargoidPod,
    #[serde(alias = "ThargoidOrganSample")]
    ThargoidOrganSample,
    #[serde(alias = "ThargoidTissueSampleType5")]
    ThargoidTissueSampleType5,
    #[serde(alias = "ThargoidTissueSampleType6")]
    ThargoidTissueSampleType6,
    #[serde(alias = "ThargoidTissueSampleType7")]
    ThargoidTissueSampleType7,
    #[serde(alias = "ThargoidTissueSampleType9a")]
    ThargoidTissueSampleType9a,
    #[serde(alias = "ThargoidTissueSampleType9b")]
    ThargoidTissueSampleType9b,
    #[serde(alias = "ThargoidTissueSampleType9c")]
    ThargoidTissueSampleType9c,
    #[serde(alias = "ThargoidHeart")]
    ThargoidHeart,
    #[serde(alias = "UnknownResin")]
    UnknownResin,
    #[serde(alias = "UnknownArtifact")]
    UnknownArtifact,
    #[serde(alias = "UnknownSack")]
    UnknownSack,
    #[serde(alias = "MetaAlloys")]
    MetaAlloys,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarketBuy {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    #[serde(rename = "Type")]
    pub buy_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub buy_type_localised: Option<EDString>,
    pub count: u64,
    pub buy_price: Credits,
    pub total_cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T03:29:58Z", "event":"MarketSell", "Type":"biowaste", "Count":1, "SellPrice":10, "TotalSale":10, "AvgPricePaid":0 })]
pub struct EDLogMarketSell {
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
    #[serde(rename = "Type")]
    pub sell_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub sell_type_localised: Option<EDString>,
    pub count: u64,
    pub sell_price: Credits,
    pub total_sale: Credits,
    pub avg_price_paid: Credits,
    pub illegal_goods: Option<bool>,
    pub stolen_goods: Option<bool>,
    pub black_market: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MicroResourceType {
    Data,
    Item,
    Component,
    Consumable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MicroResource {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    category: MicroResourceType,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyMicroResources {
    pub total_count: Option<u64>,
    #[serde(flatten)]
    pub micro_resource: Option<MicroResource>,
    pub micro_resources: Option<Vec<MicroResource>>,
    pub price: Credits,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-28T19:16:15Z", "event":"BuyTradeData", "System":"Quator", "Cost":100 })]
pub struct EDLogBuyTradeData {
    system: EDString,
    cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogTradeMicroResources {
    offered: Vec<MicroResource>,
    total_count: u64,
    received: EDString,
    #[serde(rename = "Received_Localised")]
    received_localised: Option<EDString>,
    count: u64,
    category: MicroResourceType,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-21T20:24:20Z", "event":"SellMicroResources", "TotalCount":44, 
    "MicroResources":[ 
        { "Name":"compactlibrary", "Name_Localised":"Compact Library", "Category":"Item", "Count":1 }, 
        { "Name":"insight", "Category":"Item", "Count":1 } ], 
    "Price":479000, "MarketID":3228823296 })]
pub struct EDLogSellMicroResources {
    pub total_count: u64,
    pub micro_resources: Vec<MicroResource>,
    pub price: Credits,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDeliverPowerMicroResources {
    total_count: u64,
    micro_resources: Vec<MicroResource>,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarket {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: EDString,
    pub station_type: StationType,
    pub carrier_docking_access: Option<CarrierDockingAccess>,
    pub star_system: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCargoDepot {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    update_type: EDString,        // TODO: enum
    cargo_type: Option<EDString>, // TODO: enum
    #[serde(rename = "CargoType_Localised")]
    cargo_type_localised: Option<EDString>,
    count: Option<u64>,
    #[serde(rename = "StartMarketID")]
    start_market_id: u64,
    #[serde(rename = "EndMarketID")]
    end_market_id: u64,
    items_collected: u64,
    items_delivered: u64,
    total_items_to_deliver: u64,
    progress: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct RequiredResource {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub required_amount: u64,
    pub provided_amount: u64,
    pub payment: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ContributedResource {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationConstructionDepot {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub construction_progress: f64,
    pub construction_complete: bool,
    pub construction_failed: bool,
    pub resources_required: Vec<RequiredResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationContribution {
    #[serde(rename = "MarketID")]
    market_id: u64,
    contributions: Vec<ContributedResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SoldBioData {
    pub genus: EDString,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: EDString,
    pub species: EDString,
    #[serde(rename = "Species_Localised")]
    pub species_localised: EDString,
    pub variant: Option<EDString>,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: Option<EDString>,
    pub value: Credits,
    pub bonus: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellOrganicData {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub bio_data: Vec<SoldBioData>,
}
