use crate::common_types::{CarrierDockingAccess, StationType};
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

    SyntheticFabrics,
    SyntheticReagents,
    SyntheticMeat,
    SuperConductors,
    AdvancedCatalysers,
    ReinforcedMountingPlate,
    MarineSupplies,
    ConsumerTechnology,
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

    KamitraCigars,
    FujinTea,
    LavianBrandy,
    KonggaAle,
    SoontillRelics,
    VegaSlimWeed,
    EraninPearlWhisky,

    OccupiedCryoPod,

    AncientCasket,
    AncientRelic,
    AncientTablet,
    AncientTotem,
    AncientUrn,
    AncientOrb,

    ThargoidTitanDriveComponent,
    ThargoidBoneFragments,
    ThargoidGeneratorTissuesample,
    ThargoidCystSpecimen,
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
    #[serde(flatten)]
    micro_resource: MicroResource,
    price: u64,
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
    update_type: String, // TODO: enum
    cargo_type: String,  // TODO: enum
    #[serde(rename = "CargoType_Localised")]
    cargo_type_localised: Option<String>,
    count: u64,
    #[serde(rename = "StartMarketID")]
    start_market_id: u64,
    #[serde(rename = "EndMarketID")]
    end_market_id: u64,
    items_collected: u64,
    items_delivered: u64,
    total_items_to_deliver: u64,
    progress: f64,
}
