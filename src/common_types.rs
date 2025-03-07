use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Unknown {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FactionState {
    None,
    Expansion,
    Boom,
    Outbreak,
    War,
    Election,
    Bust,
    CivilWar,
    Drought,
    InfrastructureFailure,
    CivilUnrest,
    Terrorism,
    State,
    Blight,
    CivilLiberty,
    PublicHoliday,
    Retreat,
    PirateAttack,
    Famine,
    NaturalDisaster,
    Investment,
    Lockdown,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BodyType {
    Star,
    Station,
    Planet,
    PlanetaryRing,
    StellarRing,
    AsteroidCluster,
    Null,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CrimeType {
    CollidedAtSpeedInNoFireZone,
    DockingMinorBlockingAirlock,
    FireInNoFireZone,
    DumpingNearStation,
    StationTamperingMinor,
    Assault,
    DockingMinorBlockingLandingPad,
    DockingMinorTresspass,
    Murder,
    DumpingDangerous,
    RecklessWeaponsDischarge,
    DockingMajorTresspass,
    Interdiction,
    #[serde(rename = "onFoot_damagingDefences")]
    OnFootDamagingDefences,
    #[serde(rename = "onFoot_detectionOfWeapon")]
    OnFootDetectionOfWeapeon,
    #[serde(rename = "onFoot_murder")]
    OnFootMurder,
    #[serde(rename = "onFoot_trespass")]
    OnFootTrespass,
    #[serde(rename = "onFoot_dataTransfer")]
    OnFootDataTransfer,
    #[serde(rename = "onFoot_propertyTheft")]
    OnFootPropertyTheft,
    #[serde(rename = "onFoot_arcCutterUse")]
    OnFootArcCutterUse,
    #[serde(rename = "onFoot_failureToSubmitToPolice")]
    OnFootFailureToSubmitToPolice,
    #[serde(rename = "onFoot_carryingIllegalGoods")]
    OnFootCarryingIllegalGoods,
    #[serde(rename = "onFoot_recklessEndangerment")]
    OnFootRecklessEndangerment,
    #[serde(rename = "onFoot_identityTheft")]
    OnFootIdentityTheft,
    #[serde(rename = "onFoot_profileCloningIntent")]
    OnFootCloningIntent,
    #[serde(rename = "collidedAtSpeedInNoFireZone_hulldamage")]
    CollidedAtSpeedInNoFireZoneHulldamage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CarrierDockingAccess {
    All,
    Friends,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum StarClass {
    A,
    B,
    D,
    DA,
    #[serde(rename = "DAB")]
    Dab,
    #[serde(rename = "DAZ")]
    Daz,
    DC,
    DQ,
    F,
    G,
    H,
    K,
    #[serde(rename = "K_OrangeGiant")]
    KOrangeGiant,
    L,
    M,
    #[serde(rename = "M_RedGiant")]
    MRedGiant,
    N,
    T,
    #[serde(rename = "TTS")]
    Tts,
    Y,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LuminosityType {
    Va,
    V,
    #[serde(rename = "III")]
    Three,
    #[serde(rename = "IV")]
    Four,
    #[serde(rename = "VII")]
    Seven,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SignalType {
    StationBernalSphere,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MaterialCategory {
    Manufactured,
    Encoded,
    Raw,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TraderType {
    Manufactured,
    Encoded,
    Raw,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ScanType {
    AutoScan,
    Detailed,
    Basic,
    Log,     // Organic
    Sample,  //Organic
    Analyse, // Organic
    NavBeaconDetail,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SAASignalType {
    #[serde(rename = "tritium")]
    Tritium,
    Grandidierite,
    Opal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FSSSignalType {
    Generic,
    ResourceExtraction,
    Combat,
    NavBeacon,
    Outpost,
    Installation,
    StationCoriolis,
    StationAsteroid,
    StationBernalSphere,
    StationONeilOrbis,
    StationONeilCylinder,
    FleetCarrier,
    Megaship,
    StationMegaShip,
    TouristBeacon,
    Titan,
    #[serde(rename = "USS")]
    Uss,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StationType {
    #[serde(rename = "")]
    None,
    Outpost,
    Coriolis,
    Orbis,
    FleetCarrier,
    Ocellus,
    Bernal,
    CraterOutpost,
    CraterPort,
    MegaShip,
    SurfaceStation,
    OnFootSettlement,
    AsteroidBase,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ShipScanType {
    Crime,
    Cargo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Allegiance {
    #[serde(rename = "")]
    None,
    Independent,
    PilotsFederation,
    Federation,
    Empire,
    Guardian,
    Thargoid,
    Alliance,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DroneType {
    Drones,
    Collection,
    Prospector,
    Repair,
    Decontamination,
    Recon,
    Research,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PowerplayState {
    Unoccupied,
    Exploited,
    Controlled,
    Fortified,
    Contested,
    HomeSystem,
    Stronghold,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StarSystemData {
    star_system: String,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    transfer_price: u64,
    transfer_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationFaction {
    name: String,
    faction_state: Option<FactionState>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BodyInformation {
    star_system: String,
    system_address: u64,
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CodexBodyInformation {
    system: String,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
    nearest_destination: String,
    #[serde(rename = "NearestDestination_Localised")]
    nearest_destination_localised: Option<String>,
    traits: Vec<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationInformation {
    station_name: String,
    station_type: StationType,
    #[serde(rename = "MarketID")]
    market_id: u64,
    station_faction: StationFaction,
    station_government: String,
    #[serde(rename = "StationGovernment_Localised")]
    station_government_localised: String,
    station_allegiance: Option<Allegiance>,
    station_services: Vec<StationService>,
    station_economy: String,
    #[serde(rename = "StationEconomy_Localised")]
    station_economy_localised: String,
    station_economies: Vec<StationEconomy>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StationService {
    Dock,
    Autodock,
    Commodities,
    Contacts,
    Exploration,
    Missions,
    Outfitting,
    Crewlounge,
    Rearm,
    Refuel,
    Repair,
    Engineer,
    MissionsGenerated,
    Facilitator,
    FlightController,
    StationOperations,
    Powerplay,
    SearchRescue,
    #[serde(rename = "stationMenu")]
    StationMenu,
    Livery,
    SocialSpace,
    Bartender,
    PioneerSupplies,
    ApexInterstellar,
    BlackMarket,
    Shipyard,
    Tuning,
    Shop,
    VistaGenomics,
    FrontlineSolutions,
    #[serde(rename = "techBroker")]
    TechBroker,
    CarrierManagement,
    CarrierFuel,
    VoucherRedemption,
    MaterialTrader,
    ModulePacks,
    OnDockMission,
    CarrierVendor,
    RegisteringColonisation,
    ColonisationContribution,
    Refinery,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationEconomy {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    proportion: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionName {
    name: String,
    faction_state: Option<FactionState>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EngineerModification {
    engineer_modifications: String,
    level: u64,
    quality: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ModifierValue {
    value: f64,
    original_value: f64,
    less_is_good: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ModifierDescription {
    value_str: String,
    #[serde(rename = "ValueStr_Localised")]
    value_str_localised: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "Label")]
pub enum ModuleEngineeringModifiers {
    DamagePerSecond(ModifierValue),
    Damage(ModifierValue),
    DistributorDraw(ModifierValue),
    ThermalLoad(ModifierValue),
    RateOfFire(ModifierValue),
    DamageType(ModifierDescription),
    AmmoMaximum(ModifierValue),
    AmmoClipSize(ModifierValue),
    PowerDraw(ModifierValue),
    ShotSpeed(ModifierValue),
    Mass(ModifierValue),
    FSDOptimalMass(ModifierValue),
    Integrity(ModifierValue),
    DefenceModifierShieldMultiplier(ModifierValue),
    DefenceModifierHealthMultiplier(ModifierValue),
    KineticResistance(ModifierValue),
    ThermicResistance(ModifierValue),
    ExplosiveResistance(ModifierValue),
    ScannerRange(ModifierValue),
    ScannerTimeToScan(ModifierValue),
    PowerCapacity(ModifierValue),
    HeatEfficiency(ModifierValue),
    EngineOptimalMass(ModifierValue),
    EngineOptPerformance(ModifierValue),
    EngineHeatRate(ModifierValue),
    WeaponsCapacity(ModifierValue),
    WeaponsRecharge(ModifierValue),
    EnginesCapacity(ModifierValue),
    EnginesRecharge(ModifierValue),
    SystemsCapacity(ModifierValue),
    SystemsRecharge(ModifierValue),
    SensorTargetScanAngle(ModifierValue),
    Range(ModifierValue),
    ShieldGenStrength(ModifierValue),
    RegenRate(ModifierValue),
    BrokenRegenRate(ModifierValue),
    EnergyPerRegen(ModifierValue),
    DefenceModifierHealthAddition(ModifierValue),
    FSDInterdictorRange(ModifierValue),
    FSDInterdictorFacingLimit(ModifierValue),
    MaximumRange(ModifierValue),
    DamageFalloffRange(ModifierValue),
    ReloadTime(ModifierValue),
    ShieldGenOptimalMass(ModifierValue),
    #[serde(rename = "DSS_PatchRadius")]
    DssPatchRadius(ModifierValue),
    ArmourPenetration(ModifierValue),
    Jitter(ModifierValue),
    MaxAngle(ModifierValue),
    MaxFuelPerJump(ModifierValue),
    BootTime(ModifierValue),
    ShieldBankReinforcement(ModifierValue),
    ShieldBankHeat(ModifierValue),
    FSDHeatRate(ModifierValue),
    GuardianModuleResistance(ModifierDescription),
    ShieldBankSpinUp(ModifierValue),
    BurstRateOfFire(ModifierValue),
    BurstSize(ModifierValue),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogName {
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PilotRank {
    #[serde(rename = "Mostly Harmless")]
    MostlyHarmless,
    Harmless,
    Novice,
    Competent,
    Expert,
    Dangerous,
    Deadly,
    Master,
    Elite,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TechBrokerType {
    Sirius,
    Rescue,
    Guardian,
    Human,
    Salvation,
}
