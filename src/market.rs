use crate::{
    common_types::{CarrierDockingAccess, Credits, StationType},
    log_line::{EDLogEvent, Extractable},
};
use ed_parse_log_files_macros::testcase;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MarketItemType {
    Silver,
    Aluminium,
    Tritium,
    Platinum,
    Titanium,
    Cobalt,
    Steel,
    Palladium,
    Osmium,
    Gold,
    Painite,
    Beryllium,
    Bauxite,
    Gallite,
    Samarium,
    Lepidolite,
    Rutile,
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

    SyntheticFabrics,
    SyntheticReagents,
    SyntheticMeat,
    SuperConductors,
    AdvancedCatalysers,
    ReinforcedMountingPlate,
    MarineSupplies,
    Clothing,
    ConsumerTechnology,
    DomesticAppliances,
    MagneticeMitterCoil,
    PowerTransferConduits,
    Explosives,
    PowerGenerators,
    ComercialSamples,
    UssCargoTechnicalBlueprints,
    UssCargoTradeData,
    EncriptedDataStorage,
    AnimalMeat,
    Coffee,
    Grain,
    Fish,
    Algae,
    FruitAndVegetables,
    FoodCartridges,
    Liquor,
    Beer,
    Wine,
    BasicNarcotics,
    GeologicalEquipment,
    Biowaste,
    ChemicalWaste,
    XiheCompanions,
    Landmines,
    NerveAgents,
    LithiumHydroxide,
    Tobacco,
    Tea,
    ProgenitorCells,
    HydrogenPeroxide,
    MethaneClathRate,
    PersonalGifts,
    PersonalWeapons,
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

    Drones,
    PersonalEffects,
    DamagedEscapePod,
    UssCargoBlackbox,
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

    OccupiedCryoPod,

    AncientCasket,
    AncientRelic,
    AncientTablet,
    AncientTotem,
    AncientUrn,
    AncientOrb,
    AncientKey,

    ThargoidTitanDriveComponent,
    ThargoidBoneFragments,
    ThargoidGeneratorTissuesample,
    ThargoidCystSpecimen,
    ThargoidPod,
    ThargoidOrganSample,
    ThargoidTissueSampleType5,
    ThargoidTissueSampleType6,
    ThargoidTissueSampleType7,
    ThargoidTissueSampleType9a,
    ThargoidTissueSampleType9b,
    ThargoidTissueSampleType9c,
    ThargoidHeart,
    UnknownResin,
    UnknownArtifact,
    UnknownSack,
    MetaAlloys,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarketBuy {
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "Type")]
    buy_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    buy_type_localised: Option<String>,
    count: u64,
    buy_price: u64,
    total_cost: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T03:29:58Z", "event":"MarketSell", "Type":"biowaste", "Count":1, "SellPrice":10, "TotalSale":10, "AvgPricePaid":0 })]
pub struct EDLogMarketSell {
    #[serde(rename = "MarketID")]
    market_id: Option<u64>,
    #[serde(rename = "Type")]
    sell_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    sell_type_localised: Option<String>,
    count: u64,
    sell_price: u64,
    total_sale: u64,
    avg_price_paid: u64,
    illegal_goods: Option<bool>,
    stolen_goods: Option<bool>,
    black_market: Option<bool>,
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
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    category: MicroResourceType,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyMicroResources {
    total_count: Option<u64>,
    #[serde(flatten)]
    micro_resource: Option<MicroResource>,
    micro_resources: Option<Vec<MicroResource>>,
    price: Credits,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-28T19:16:15Z", "event":"BuyTradeData", "System":"Quator", "Cost":100 })]
pub struct EDLogBuyTradeData {
    system: String,
    cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogTradeMicroResources {
    offered: Vec<MicroResource>,
    total_count: u64,
    received: String,
    #[serde(rename = "Received_Localised")]
    received_localised: Option<String>,
    count: u64,
    category: MicroResourceType,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellMicroResources {
    total_count: u64,
    micro_resources: Vec<MicroResource>,
    price: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDeliverPowerMicroResources {
    total_count: u64,
    micro_resources: Vec<MicroResource>,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarket {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub station_type: StationType,
    pub carrier_docking_access: Option<CarrierDockingAccess>,
    pub star_system: String,
}

impl Extractable for EDLogMarket {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::Market(loc) = event {
            return Some(loc);
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCargoDepot {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    update_type: String,        // TODO: enum
    cargo_type: Option<String>, // TODO: enum
    #[serde(rename = "CargoType_Localised")]
    cargo_type_localised: Option<String>,
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
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,
    pub required_amount: u64,
    pub provided_amount: u64,
    pub payment: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ContributedResource {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationConstructionDepot {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub construction_progress: f64,
    pub construction_complete: bool,
    pub construction_failed: bool,
    pub resources_required: Vec<RequiredResource>,
}

impl Extractable for EDLogColonisationConstructionDepot {
    fn extract(event: &EDLogEvent) -> Option<&Self> {
        if let EDLogEvent::ColonisationConstructionDepot(loc) = event {
            return Some(loc);
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationContribution {
    #[serde(rename = "MarketID")]
    market_id: u64,
    contributions: Vec<ContributedResource>,
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
    value: Credits,
    bonus: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellOrganicData {
    #[serde(rename = "MarketID")]
    market_id: u64,
    bio_data: Vec<SoldBioData>,
}
