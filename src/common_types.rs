use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Sub};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Merits(u64);

impl Merits {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Sub for Merits {
    type Output = Merits;

    fn sub(self, rhs: Self) -> Self::Output {
        Merits(self.0 - rhs.0)
    }
}

impl Display for Merits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Credits(pub i64);

impl Display for Credits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub for Credits {
    type Output = Credits;

    fn sub(self, rhs: Self) -> Self::Output {
        Credits(self.0 - rhs.0)
    }
}

impl Credits {
    pub fn to_human_readable_string(&self) -> String {
        match self.0 {
            -1_000..0 => format!("{}", self.0),
            -1_000_000..-1_000 => format!("{}K", self.0 / 1_000),
            -1_000_000_000..-1_000_000 => format!("{}M", self.0 / 1_000_000),
            ..-1_000_000_000 => format!("{}B", self.0 / 1_000_000_000),
            0..=999 => format!("{}", self.0),
            1_000..=999_999 => format!("{}K", self.0 / 1_000),
            1_000_000..=999_999_999 => format!("{}M", self.0 / 1_000_000),
            1_000_000_000.. => format!("{}B", self.0 / 1_000_000_000),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Unknown {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OnFootItem {
    AeroGel,
    AgriculturalProcessSample,
    BiochemicalAgent,
    Californium,
    CastFossil,
    ChemicalCatalyst,
    ChemicalSuperbase,
    ChemicalSample,
    ChemicalFormulae,
    DeepMantleSample,
    GeneticRepairMeds,
    GeneticSample,
    GMeds,
    Graphene,
    Hush,
    Infinity,
    InorganicContaminant,
    Kompromat,
    Lazarus,
    MutagenicCatalyst,
    NutritionalConcentrate,
    OxygenicBacteria,
    PetrifiedFossil,
    PHNeutraliser,
    Push,
    RDX,
    SyntheticGenome,
    TrueFormFossil,

    HealthPack,
    EnergyCell,
    #[serde(rename = "amm_grenade_emp")]
    AmmGrenadeEmp,
    #[serde(rename = "amm_grenade_frag")]
    AmmGrenadeFrag,
    #[serde(rename = "amm_grenade_shield")]
    AmmGrenadeShield,

    Epinephrine,
    MicrobialInhibitor,
    InertiaCanister,

    Bypass,
    CarbonfibrePlating,
    CircuitSwitch,
    CircuitBoard,
    CompactLibrary,
    CompressionLiquefiedGas,
    DegradedPowerRegulator,
    ElectricalWiring,
    ElectricalFuse,
    ElectroMagnet,
    EncryptedMemoryChip,
    EpoxyAdhesive,
    HealthMonitor,
    IonBattery,
    IonisedGas,
    LargeCapacityPowerRegulator,
    MemoryChip,
    MetalCoil,
    MicroElectrode,
    MicroHydraulics,
    MicroSupercapacitor,
    MicroThrusters,
    MicroTransformer,
    Motor,
    OpticalFibre,
    OpticalLens,
    PersonalComputer,
    PersonalDocuments,
    PowerAgriculture,
    PowerClassifiedData,
    PowerComputer,
    PowerElectronics,
    PowerEquipment,
    PowerEmployeeData,
    PowerExperiment,
    PowerExtraction,
    PowerFinancialRecords,
    PowerIndustrial,
    PowerInventory,
    PowerMedical,
    PowerMiscComputer,
    PowerMiscIndust,
    PowerPower,
    PowerPropagandaData,
    PowerRegulator,
    PowerSecurity,
    PowerResearch,
    PowerResearchData,
    PowerplayMilitary,
    PyrolyticCatalyst,

    Scrambler,
    SurveillanceEquipment,
    SyntheticPathogen,
    Transmitter,
    TitaniumPlating,
    TungstenCarbide,
    UniversalTranslator,
    ViscoElasticPolymer,
    WeaponComponent,

    BuildingSchematic,
    ShipSchematic,
    SuitSchematic,
    VehicleSchematic,
    WeaponSchematic,

    AccidentLogs,
    AirQualityReports,
    AtmosphericData,
    AXcombatLogs,
    AudioLogs,
    BallisticsData,
    BiologicalWeaponData,
    BiomechanicalComponent,
    BiometricData,
    BlacklistData,
    BloodTestResults,
    CampaignPlans,
    CatMedia,
    CensusData,
    ChemicalExperimentData,
    ChemicalInventory,
    ChemicalPatents,
    ChemicalProcessSample,
    ChemicalWeaponData,
    ClassicEntertainment,
    CocktailRecipes,
    CombatantPerformance,
    CombatTrainingMaterial,
    ConflictHistory,
    CriminalRecords,
    CropYieldAnalysis,
    CulinaryRecipes,
    DigitalDesigns,
    DutyRota,
    EmployeeDirectory,
    EmployeeExpenses,
    EmployeeGeneticData,
    EmploymentHistory,
    EnhancedInterrogationRecordings,
    ExplorationJournals,
    ExtractionYieldData,
    EvacuationProtocols,
    FactionAssociates,
    FactionDonatorList,
    FactionNews,
    FinancialProjections,
    FleetRegistry,
    GeneSequencingData,
    GeneticResearch,
    GeologicalData,
    HydroponicData,
    IncidentLogs,
    InfluenceProjections,
    InsightEntertainmentSuite,
    InsightDatabank,
    Insight,
    InternalCorrespondence,
    InterrogationRecordings,
    InterviewRecordings,
    JobApplications,
    LiteraryFiction,
    ManufacturingInstructions,
    MaintenanceLogs,
    MineralSurvey,
    MiningAnalytics,
    MeetingMinutes,
    MedicalRecords,
    MedicalTrialRecords,
    MultiMediaEntertainment,
    NetworkAccessHistory,
    NetworkSecurityProtocols,
    NextOfKinRecords,
    NOCData,
    OperationalManual,
    OpinionPolls,
    PatientHistory,
    PatrolRoutes,
    PayrollInformation,
    PersonalLogs,
    PharmaceuticalPatents,
    PhotoAlbums,
    PoliticalAffiliations,
    PrisonerLogs,
    ProductionReports,
    ProductionSchedule,
    Propaganda,
    PurchaseRecords,
    PurchaseRequests,
    RadioactivityData,
    ResidentialDirectory,
    ReactorOutputReview,
    RecyclingLogs,
    RefinementProcessSample,
    RiskAssessments,
    SalesRecords,
    SeedGeneaology,
    SecurityExpenses,
    SettlementAssaultPlans,
    SettlementDefencePlans,
    ShareholderInformation,
    SlushFundLogs,
    SmearCampaignPlans,
    SpectralAnalysisData,
    StellarActivityLogs,
    SurveilleanceLogs,
    TaxRecords,
    TravelPermits,
    TopographicalSurveys,
    UnionMembership,
    VaccinationRecords,
    VaccineResearch,
    VIPSecurityDetail,
    VirologyData,
    VisitorRegister,
    WeaponInventory,
    XenoDefenceProtocols,
}

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
    PlanetaryConstructionDepot,
    DockablePlanetStation,
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
pub struct PowerplayConflictProgress {
    power: String,
    conflict_progress: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Powers {
    controlling_power: Option<String>,
    powers: Vec<String>,
    powerplay_state: PowerplayState,
    powerplay_conflict_progress: Option<Vec<PowerplayConflictProgress>>,
    powerplay_state_control_progress: Option<f64>,
    powerplay_state_reinforcement: Option<u64>,
    powerplay_state_undermining: Option<u64>,
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
    station_faction: FactionName,
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
#[serde(rename_all = "lowercase")]
pub enum WarType {
    Election,
    War,
    CivilWar,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConflictStatus {
    #[serde(rename = "")]
    None,
    Active,
    Pending,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ConflictFaction {
    name: String,
    stake: String,
    won_days: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Conflict {
    war_type: WarType,
    status: ConflictStatus,
    faction1: ConflictFaction,
    faction2: ConflictFaction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ThargoidWarState {
    Unknown,
    #[serde(rename = "")]
    None,
    #[serde(rename = "Thargoid_Harvest")]
    Harvest,
    #[serde(rename = "Thargoid_Recovery")]
    Recovery,
    #[serde(rename = "Thargoid_Controlled")]
    Controlled,
    #[serde(rename = "Thargoid_Stronghold")]
    Stronghold,
    #[serde(rename = "Thargoid_Probing")]
    Probing,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ThargoidWar {
    current_state: ThargoidWarState,
    next_state_success: ThargoidWarState,
    next_state_failure: ThargoidWarState,
    success_state_reached: bool,
    war_progress: f64,
    remaining_ports: u64,
    estimated_remaining_time: Option<String>,
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
pub struct FactionRecoveringState {
    state: FactionState,
    trend: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionActiveState {
    state: FactionState,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionPendingState {
    state: FactionState,
    trend: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Faction {
    name: String,
    faction_state: FactionState,
    government: String,
    influence: f64,
    allegiance: String,
    happiness: String,
    #[serde(rename = "Happiness_Localised")]
    happiness_localised: Option<String>,
    my_reputation: f64,
    recovering_states: Option<Vec<FactionRecoveringState>>,
    active_states: Option<Vec<FactionActiveState>>,
    pending_states: Option<Vec<FactionPendingState>>,
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

#[test]
fn test_faction() {
    let json = r#"{ "Name":"People's Madjandji Resistance", "FactionState":"None", "Government":"Democracy", "Influence":0.063555,
          "Allegiance":"Federation", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":0.000000 }"#;
    let line: Result<Faction, _> = serde_json::from_str(json);
    assert!(line.is_ok());

    let json = r#"{ "Name":"DaVinci Corp.", "FactionState":"Blight", "Government":"Corporate", "Influence":0.599801, "Allegiance":"Independent",
    "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", "MyReputation":61.321701, 
    "ActiveStates":[ { "State":"Blight" } ] }"#;
    let line: Result<Faction, _> = serde_json::from_str(json);
    assert!(line.is_ok());

    let json = r#"{ "Name":"Phoenix Flight Explorers Commune", "FactionState":"None", "Government":"Cooperative",
    "Influence":0.266137, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Happy", 
    "MyReputation":86.373100 } 
    "#;
    let line: Result<Faction, _> = serde_json::from_str(json);
    assert!(line.is_ok());
}

#[test]
fn test_power() {
    let json = r#"{ 
        "Powers":[ "Pranav Antal", "Jerome Archer" ], 
        "PowerplayState":"Unoccupied", 
        "PowerplayConflictProgress":[ 
            { "Power":"Pranav Antal", "ConflictProgress":0.005875 }, 
            { "Power":"Jerome Archer", "ConflictProgress":0.478375 } 
        ] 
    }"#;
    let _line: Powers = serde_json::from_str(json).expect("should parse");

    let json = r#"{ 
        "ControllingPower": "Jerome Archer",
        "Powers": ["Jerome Archer"],
        "PowerplayState": "Exploited"
    }"#;
    let _line: Powers = serde_json::from_str(json).expect("should parse");

    let json = r#"{ 
        "ControllingPower": "Jerome Archer",
        "Powers": ["Pranav Antal","Jerome Archer"],
        "PowerplayState": "Fortified",
        "PowerplayStateControlProgress": 0.337526,
        "PowerplayStateReinforcement": 792,
        "PowerplayStateUndermining": 0
    }"#;
    let _line: Powers = serde_json::from_str(json).expect("should parse");
}

#[test]
fn test_station_info() {
    let json = r#"{
    "StationName": "Otiman Dock",
    "StationType": "Bernal",
    "MarketID": 3227675648,
    "StationFaction": {
        "Name": "Nagii Union",
        "FactionState": "Expansion"
    },
    "StationGovernment": "$government_Communism;",
    "StationGovernment_Localised": "Communist",
    "StationServices": [
        "dock",
        "autodock",
        "blackmarket",
        "commodities",
        "contacts",
        "exploration",
        "missions",
        "outfitting",
        "crewlounge",
        "rearm",
        "refuel",
        "repair",
        "shipyard",
        "tuning",
        "engineer",
        "missionsgenerated",
        "facilitator",
        "flightcontroller",
        "stationoperations",
        "powerplay",
        "searchrescue",
        "stationMenu",
        "shop",
        "livery",
        "socialspace",
        "bartender",
        "vistagenomics",
        "pioneersupplies",
        "apexinterstellar",
        "frontlinesolutions"
    ],
    "StationEconomy": "$economy_Industrial;",
    "StationEconomy_Localised": "Industrial",
    "StationEconomies": [
        {
            "Name": "$economy_Industrial;",
            "Name_Localised": "Industrial",
            "Proportion": 1.000000
        }
    ]}"#;
    let _line: StationInformation = serde_json::from_str(json).expect("should parse");
}
