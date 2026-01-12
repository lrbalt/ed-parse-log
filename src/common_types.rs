use crate::{EDString, commander::CombatRank};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
    str::FromStr,
};
use strum::Display;

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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
pub struct Credits(pub i64);

impl Display for Credits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Credits {
    type Output = Credits;

    fn add(self, rhs: Self) -> Self::Output {
        Credits(self.0 + rhs.0)
    }
}

impl AddAssign for Credits {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Credits {
    type Output = Credits;

    fn sub(self, rhs: Self) -> Self::Output {
        Credits(self.0 - rhs.0)
    }
}

impl SubAssign for Credits {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Sum for Credits {
    fn sum<I: Iterator<Item = Credits>>(iter: I) -> Self {
        iter.fold(Credits(0), |acc, x| acc + x)
    }
}

impl Mul<i64> for Credits {
    type Output = Credits;

    fn mul(self, rhs: i64) -> Self::Output {
        Credits(rhs * self.0)
    }
}

impl Mul<u64> for Credits {
    type Output = Credits;

    fn mul(self, rhs: u64) -> Self::Output {
        self * (rhs as i64)
    }
}

impl Neg for Credits {
    type Output = Credits;

    fn neg(self) -> Self::Output {
        Credits(-self.0)
    }
}

impl Credits {
    pub fn to_human_readable_string(&self) -> String {
        crate::utils::to_human_readable_string(self.0)
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
    TacticalPlans,
    TroopDeploymentRecords,

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
    WeaponTestData,
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
    DockingMajorBlockingLandingPad,
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
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum StarClass {
    #[serde(rename = "AeBe")]
    Aebe,
    A,
    B,
    C,
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
    SupermassiveBlackHole,
    T,
    #[serde(rename = "TTS")]
    Tts,
    W,
    Y,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LuminosityType {
    Va,
    V,
    #[serde(rename = "I")]
    One,
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

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum FSSSignalType {
    Generic,
    #[strum(to_string = "Resource Extraction")]
    ResourceExtraction,
    Combat,
    #[strum(to_string = "Navigation Beacon")]
    NavBeacon,
    Outpost,
    Installation,
    #[strum(to_string = "Coriolis Station")]
    StationCoriolis,
    #[strum(to_string = "Dodec Station")]
    StationDodec,
    #[strum(to_string = "Asteroid Station")]
    StationAsteroid,
    #[strum(to_string = "Bernal Sphere Station")]
    StationBernalSphere,
    #[strum(to_string = "O'Neil Orbis Station")]
    StationONeilOrbis,
    #[strum(to_string = "O'Neil Cylinder Station")]
    StationONeilCylinder,
    #[strum(to_string = "Fleet Carrier")]
    FleetCarrier,
    #[strum(to_string = "Squadron Carrier")]
    SquadronCarrier,
    #[strum(to_string = "Mega Ship")]
    Megaship,
    #[strum(to_string = "Mega Ship (Station)")]
    StationMegaShip,
    #[strum(to_string = "Tourist Beacon")]
    TouristBeacon,
    Titan,
    #[serde(rename = "USS")]
    Uss,
    Codex,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Display)]
pub enum StationType {
    #[serde(rename = "")]
    #[strum(to_string = "None")]
    None,
    #[strum(to_string = "Asteroid Base")]
    AsteroidBase,
    Bernal,
    Coriolis,
    #[strum(to_string = "Crater Outpost")]
    CraterOutpost,
    #[strum(to_string = "Crater Port")]
    CraterPort,
    Dodec,
    #[strum(to_string = "Dockable Planet Station")]
    DockablePlanetStation,
    #[strum(to_string = "Fleet Carrier")]
    FleetCarrier,
    #[strum(to_string = "Mega Ship")]
    MegaShip,
    Ocellus,
    #[strum(to_string = "On Foot Settlement")]
    OnFootSettlement,
    Orbis,
    Outpost,
    #[strum(to_string = "Planetary Construction Depot")]
    PlanetaryConstructionDepot,
    #[strum(to_string = "Space Construction Depot")]
    SpaceConstructionDepot,
    #[strum(to_string = "Surface Station")]
    SurfaceStation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ShipScanType {
    Crime,
    Cargo,
    Data,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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
    FuelTransfer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum PowerplayState {
    Unoccupied,
    Exploited,
    Controlled,
    Fortified,
    Contested,
    #[strum(to_string = "Home System")]
    HomeSystem,
    Stronghold,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Power {
    #[serde(rename = "A. Lavigny-Duval")]
    #[strum(to_string = "A. Lavigny-Duval")]
    ALavignyDuval,
    #[serde(rename = "Aisling Duval")]
    #[strum(to_string = "Aisling Duval")]
    AislingDuval,
    #[serde(rename = "Archon Delaine")]
    #[strum(to_string = "Archon Delaine")]
    ArchonDelaine,
    #[serde(rename = "Denton Patreus")]
    #[strum(to_string = "Denton Patreus")]
    DentonPatreus,
    #[serde(rename = "Edmund Mahon")]
    #[strum(to_string = "Edmund Mahon")]
    EdmundMahon,
    #[serde(rename = "Felicia Winters")]
    #[strum(to_string = "Felicia Winters")]
    FeliciaWinters,
    #[serde(rename = "Jerome Archer")]
    #[strum(to_string = "Jerome Archer")]
    JeromeArcher,
    #[serde(rename = "Li Yong-Rui")]
    #[strum(to_string = "Li Yong-Rui")]
    LiYongRui,
    #[serde(rename = "Nakato Kaine")]
    #[strum(to_string = "Nakato Kaine")]
    NakatoKaine,
    #[serde(rename = "Pranav Antal")]
    #[strum(to_string = "Pranav Antal")]
    PranavAntal,
    #[serde(rename = "Yuri Grom")]
    #[strum(to_string = "Yuri Grom")]
    YuriGrom,
    #[serde(rename = "Zachary Hudson")]
    #[strum(to_string = "Zachary Hudson")]
    ZacharyHudson,
    #[serde(rename = "Zemina Torval")]
    #[strum(to_string = "Zemina Torval")]
    ZeminaTorval,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PowerplayConflictProgress {
    pub power: Power,
    pub conflict_progress: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Powers {
    pub controlling_power: Option<Power>,
    pub powers: Vec<Power>,
    pub powerplay_state: PowerplayState,
    pub powerplay_conflict_progress: Option<Vec<PowerplayConflictProgress>>,
    pub powerplay_state_control_progress: Option<f64>,
    pub powerplay_state_reinforcement: Option<u64>,
    pub powerplay_state_undermining: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StarSystemData {
    star_system: EDString,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    transfer_price: Credits,
    transfer_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BodyInformation {
    pub star_system: EDString,
    pub system_address: u64,
    pub body: EDString,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CodexBodyInformation {
    system: EDString,
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
    nearest_destination: EDString,
    #[serde(rename = "NearestDestination_Localised")]
    nearest_destination_localised: Option<EDString>,
    traits: Vec<EDString>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationInformation {
    pub station_name: EDString,
    #[serde(rename = "StationName_Localised")]
    pub station_name_localised: Option<EDString>,
    pub station_type: StationType,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_faction: FactionName,
    pub station_government: GovernmentType,
    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: EDString,
    pub station_allegiance: Option<Allegiance>,
    pub station_services: Vec<StationService>,
    pub station_economy: SystemEconomy,
    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: EDString,
    pub station_economies: Vec<StationEconomy>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StationService {
    #[serde(alias = "Dock")]
    Dock,
    #[serde(alias = "Autodock")]
    Autodock,
    #[serde(alias = "Commodities")]
    Commodities,
    #[serde(alias = "Contacts")]
    Contacts,
    #[serde(alias = "Exploration")]
    Exploration,
    #[serde(alias = "Missions")]
    Missions,
    #[serde(alias = "Outfitting")]
    Outfitting,
    #[serde(alias = "CrewLounge")]
    CrewLounge,
    #[serde(alias = "Rearm")]
    Rearm,
    #[serde(alias = "Refuel")]
    Refuel,
    #[serde(alias = "Repair")]
    Repair,
    Engineer,
    #[serde(alias = "MissionsGenerated")]
    MissionsGenerated,
    #[serde(alias = "Facilitator")]
    Facilitator,
    #[serde(alias = "FlightController")]
    FlightController,
    #[serde(alias = "StationOperations")]
    StationOperations,
    #[serde(alias = "Powerplay")]
    Powerplay,
    #[serde(alias = "SearchAndRescue")]
    SearchRescue,
    #[serde(rename = "stationMenu")]
    StationMenu,
    Livery,
    SocialSpace,
    Bartender,
    PioneerSupplies,
    ApexInterstellar,
    #[serde(alias = "BlackMarket")]
    BlackMarket,
    #[serde(alias = "Shipyard")]
    Shipyard,
    #[serde(alias = "Tuning")]
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
    #[serde(rename = "squadronBank")]
    SquadronBank,
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
    name: EDString,
    stake: EDString,
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
    estimated_remaining_time: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StationEconomy {
    name: SystemEconomy,
    #[serde(rename = "Name_Localised")]
    name_localised: EDString,
    proportion: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FactionName {
    pub name: EDString,
    pub faction_state: Option<FactionState>,
}

impl FromStr for FactionName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FactionName {
            name: s.into(),
            faction_state: None,
        })
    }
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

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Display)]
pub enum GovernmentType {
    #[serde(alias = "$government_Anarchy;")]
    Anarchy,
    #[serde(alias = "$government_Communism;")]
    Communism,
    #[serde(alias = "$government_Confederacy;")]
    Confederacy,
    #[serde(alias = "$government_Carrier;")]
    Carrier,
    #[serde(alias = "$government_Cooperative;")]
    Cooperative,
    #[serde(alias = "$government_Corporate;")]
    Corporate,
    #[serde(alias = "$government_Democracy;")]
    Democracy,
    #[serde(alias = "$government_Dictatorship;")]
    Dictatorship,
    #[serde(alias = "$government_Engineer;")]
    Engineer,
    #[serde(alias = "$government_Feudal;")]
    Feudal,
    // Imperial,
    #[serde(alias = "$government_Patronage;")]
    Patronage,
    #[serde(alias = "$government_Prison;")]
    Prison,
    #[serde(alias = "$government_PrisonColony;")]
    PrisonColony,
    #[serde(alias = "$government_Theocracy;")]
    Theocracy,
    #[serde(alias = "$government_None;")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum SystemEconomy {
    #[serde(alias = "$economy_Agri;")]
    Agriculture,
    #[serde(alias = "$economy_Colony;")]
    Colony,
    #[serde(alias = "$economy_Carrier;")]
    Carrier,
    #[serde(alias = "$economy_Damaged;")]
    Damaged,
    #[serde(alias = "$economy_Engineer;")]
    Engineer,
    #[serde(alias = "$economy_Extraction;")]
    Extraction,
    #[serde(alias = "$economy_HighTech;")]
    HighTech,
    #[serde(alias = "$economy_Industrial;")]
    Industrial,
    #[serde(alias = "$economy_Military;")]
    Military,
    #[serde(alias = "$economy_Prison;")]
    Prison,
    #[serde(alias = "$economy_Refinery;")]
    Refinery,
    #[serde(alias = "$economy_Rescue;")]
    Rescue,
    #[serde(alias = "$economy_Service;")]
    Service,
    #[serde(alias = "$economy_Terraforming;")]
    Terraforming,
    #[serde(alias = "$economy_Tourism;")]
    Tourism,
    #[serde(alias = "$economy_None;")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum SystemSecurity {
    #[serde(alias = "$SYSTEM_SECURITY_high;")]
    High,
    #[serde(alias = "$SYSTEM_SECURITY_medium;")]
    Medium,
    #[serde(alias = "$SYSTEM_SECURITY_low;")]
    Low,
    #[serde(alias = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Name":"Pilots Federation Local Branch", "FactionState":"None", 
    "Government":"Democracy", "Influence":0.000000, "Allegiance":"PilotsFederation" })]
#[testcase_struct({ "Name":"Murung Services", "FactionState":"Boom", "Government":"Corporate", 
    "Influence":0.332667, "Allegiance":"Federation" })]
pub struct Faction {
    pub name: EDString,
    pub faction_state: FactionState,
    pub government: GovernmentType,
    pub influence: f64,
    pub allegiance: EDString,
    pub happiness: Option<EDString>,
    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: Option<EDString>,
    pub squadron_faction: Option<bool>,
    pub my_reputation: Option<f64>,
    pub recovering_states: Option<Vec<FactionRecoveringState>>,
    pub active_states: Option<Vec<FactionActiveState>>,
    pub pending_states: Option<Vec<FactionPendingState>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EngineerModification {
    engineer_modifications: EDString,
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
    value_str: EDString,
    #[serde(rename = "ValueStr_Localised")]
    value_str_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "Label")]
#[testcase_struct({ "Label":"DamagePerSecond", "Value":367.200012, "OriginalValue":216.000000, "LessIsGood":0 })]
#[testcase_struct({ "Label":"DamageType", "ValueStr":"$Thermic;", "ValueStr_Localised":"Thermal" })]
pub enum ModuleEngineeringModifiers {
    AmmoClipSize(ModifierValue),
    AmmoMaximum(ModifierValue),
    ArmourPenetration(ModifierValue),
    BootTime(ModifierValue),
    BrokenRegenRate(ModifierValue),
    BurstRateOfFire(ModifierValue),
    BurstSize(ModifierValue),
    CargoCapacity(ModifierValue),
    Damage(ModifierValue),
    DamageFalloffRange(ModifierValue),
    DamagePerSecond(ModifierValue),
    DamageType(ModifierDescription),
    DefenceModifierHealthAddition(ModifierValue),
    DefenceModifierHealthMultiplier(ModifierValue),
    DefenceModifierShieldMultiplier(ModifierValue),
    #[serde(rename = "DSS_PatchRadius")]
    DssPatchRadius(ModifierValue),
    DistributorDraw(ModifierValue),
    EnergyPerRegen(ModifierValue),
    EngineHeatRate(ModifierValue),
    EngineOptimalMass(ModifierValue),
    EngineOptPerformance(ModifierValue),
    EnginesCapacity(ModifierValue),
    EnginesRecharge(ModifierValue),
    ExplosiveResistance(ModifierValue),
    FSDHeatRate(ModifierValue),
    FSDInterdictorRange(ModifierValue),
    FSDInterdictorFacingLimit(ModifierValue),
    FSDOptimalMass(ModifierValue),
    GuardianModuleResistance(ModifierDescription),
    HeatEfficiency(ModifierValue),
    Integrity(ModifierValue),
    Jitter(ModifierValue),
    KineticResistance(ModifierValue),
    Mass(ModifierValue),
    MaxAngle(ModifierValue),
    MaxFuelPerJump(ModifierValue),
    MaximumRange(ModifierValue),
    PowerCapacity(ModifierValue),
    PowerDraw(ModifierValue),
    Range(ModifierValue),
    RateOfFire(ModifierValue),
    RegenRate(ModifierValue),
    ReloadTime(ModifierValue),
    ScannerRange(ModifierValue),
    ScannerTimeToScan(ModifierValue),
    SensorTargetScanAngle(ModifierValue),
    ShieldBankDuration(ModifierValue),
    ShieldBankHeat(ModifierValue),
    ShieldBankReinforcement(ModifierValue),
    ShieldBankSpinUp(ModifierValue),
    ShieldGenOptimalMass(ModifierValue),
    ShieldGenStrength(ModifierValue),
    ShotSpeed(ModifierValue),
    SystemsCapacity(ModifierValue),
    SystemsRecharge(ModifierValue),
    ThermalLoad(ModifierValue),
    ThermicResistance(ModifierValue),
    WeaponsCapacity(ModifierValue),
    WeaponsRecharge(ModifierValue),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLeftSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-13T16:13:26Z", "event":"SquadronStartup", "SquadronID":75645, "SquadronName":"ENDURANCE EXPLORATION", "CurrentRank":4, "CurrentRankName":"Agent" })]
#[testcase({"timestamp":"2024-02-14T17:32:56Z","event":"SquadronStartup","SquadronName":"ENDURANCE EXPLORATION","CurrentRank":4})]
#[testcase({ "timestamp":"2025-11-13T16:13:26Z", "event":"SquadronStartup", "SquadronID":75645, "SquadronName":"ENDURANCE EXPLORATION", "CurrentRank":4, "CurrentRankName":"Agent" })]
pub struct EDLogSquadronStartup {
    #[serde(rename = "SquadronID")]
    squadrion_id: Option<u64>,
    squadron_name: EDString,
    current_rank: u64,
    current_rank_name: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogName {
    name: EDString,
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

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogNpcCrewRank {
    npc_crew_name: EDString,
    npc_crew_id: u64,
    rank_combat: CombatRank,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogNpcCrewPaidWage {
    npc_crew_name: EDString,
    npc_crew_id: u64,
    amount: Credits,
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
