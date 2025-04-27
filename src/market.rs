use crate::common_types::{CarrierDockingAccess, Credits, StationType};
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
pub struct EDLogMarketSell {
    #[serde(rename = "MarketID")]
    market_id: u64,
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
    market_id: u64,
    station_name: String,
    station_type: StationType,
    carrier_docking_access: Option<CarrierDockingAccess>,
    star_system: String,
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
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    required_amount: u64,
    provided_amount: u64,
    payment: Credits,
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
    market_id: u64,
    construction_progress: f64,
    construction_complete: bool,
    construction_failed: bool,
    resources_required: Vec<RequiredResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationContribution {
    #[serde(rename = "MarketID")]
    market_id: u64,
    contributions: Vec<ContributedResource>,
}
