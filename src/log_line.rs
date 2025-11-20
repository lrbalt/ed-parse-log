use crate::{
    commander::{
        EDLogAppliedToSquadron, EDLogChangeCrewRole, EDLogCommander, EDLogCommitCrime,
        EDLogCrewAssign, EDLogCrewFire, EDLogCrewHire, EDLogCrewMemberJoins, EDLogCrewMemberQuits,
        EDLogCrewMemberRoleChange, EDLogCrimeVictim, EDLogDied, EDLogEmbarkOrDisembark,
        EDLogEndCrewSession, EDLogFriends, EDLogHoloscreenHacked, EDLogInvitedToSquadron,
        EDLogJoinACrew, EDLogNewCommander, EDLogPowerplay, EDLogPowerplayCollect,
        EDLogPowerplayDefect, EDLogPowerplayDeliver, EDLogPowerplayFastTrack, EDLogPowerplayJoin,
        EDLogPowerplayLeave, EDLogPowerplayMerits, EDLogPowerplayRank, EDLogPowerplaySalary,
        EDLogPromotion, EDLogQuitACrew, EDLogRank, EDLogReputation,
        EDLogRequestPowerMicroResources, EDLogResurrect, EDLogSharedBookmarkToSquadron,
        EDLogVehicleSwitch,
    },
    common_types::{
        BodyInformation, Credits, EDLogLeftSquadron, EDLogName, EDLogNpcCrewPaidWage,
        EDLogNpcCrewRank, EDLogSquadronStartup,
    },
    community_goal::{
        EDLogCommunityGoal, EDLogCommunityGoalDiscard, EDLogCommunityGoalJoin,
        EDLogCommunityGoalReward,
    },
    docking::{
        EDLogBuyAmmo, EDLogDocked, EDLogDockingCancelled, EDLogDockingDenied, EDLogDockingGranted,
        EDLogDockingRequested, EDLogDockingTimeout, EDLogMaterialCollected, EDLogMaterialTrade,
        EDLogOutfitting, EDLogPayBounties, EDLogPayFines, EDLogRefuelAll, EDLogRepair,
        EDLogRepairAll, EDLogRestockVehicle, EDLogTechnologyBroker, EDLogUndocked,
    },
    drone::{EDLogBuyDrones, EDLogLaunchDrone, EDLogRepairDrone, EDLogSellDrones},
    engineers::{EDLogEngineerContribution, EDLogEngineerCraft, EDLogEngineerProgress},
    exploration::{
        EDLogAsteroidCracked, EDLogBuyExplorationData, EDLogCodexEntry, EDLogDataScanned,
        EDLogDatalinkScan, EDLogDiscoveryScan, EDLogFSSAllBodiesFound, EDLogFSSBodySignals,
        EDLogFSSDiscoveryScan, EDLogFSSSignalDiscovered, EDLogMaterialDiscovered,
        EDLogMiningRefined, EDLogMultiSellExplorationData, EDLogNavBeaconScan,
        EDLogProspectedAsteroid, EDLogSAAScanComplete, EDLogSAASignalsFound, EDLogScan,
        EDLogScanBaryCentre, EDLogScanOrganic, EDLogScanned, EDLogSellExplorationData,
        EDLogStationBernalSphere,
    },
    fleet_carrier::{
        EDLogCarrierBankTransfer, EDLogCarrierBuy, EDLogCarrierCrewServices,
        EDLogCarrierDepositFuel, EDLogCarrierDockingPermission, EDLogCarrierFinance,
        EDLogCarrierJump, EDLogCarrierJumpCancelled, EDLogCarrierJumpRequest, EDLogCarrierLocation,
        EDLogCarrierModulePack, EDLogCarrierNameChange, EDLogCarrierStats, EDLogCarrierTradeOrder,
        EDLogFCMaterials,
    },
    loadout::EDLogLoadout,
    location::EDLogLocation,
    locker::EDLogShipLocker,
    market::{
        EDLogBuyMicroResources, EDLogBuyTradeData, EDLogCargoDepot,
        EDLogColonisationConstructionDepot, EDLogColonisationContribution,
        EDLogDeliverPowerMicroResources, EDLogMarket, EDLogMarketBuy, EDLogMarketSell,
        EDLogSellMicroResources, EDLogSellOrganicData, EDLogTradeMicroResources,
    },
    materials::EDLogMaterials,
    mission::{
        EDLogBounty, EDLogCapitalShipBond, EDLogDatalinkVoucher, EDLogFactionKillBond,
        EDLogMissionAbandoned, EDLogMissionAccepted, EDLogMissionCompleted, EDLogMissionFailed,
        EDLogMissionRedirected, EDLogMissions, EDLogPVPKill, EDLogPassengers, EDLogRedeemVoucher,
        EDLogSearchAndRescue,
    },
    modules::{
        EDLogFetchRemoteModule, EDLogMassModuleStore, EDLogModuleBuy, EDLogModuleBuyAndStore,
        EDLogModuleRetrieve, EDLogModuleSell, EDLogModuleSellRemote, EDLogModuleStore,
        EDLogModuleSwap, EDLogStoredModules,
    },
    navigation::{
        EDLogApproachSettlement, EDLogDockSRV, EDLogFSDJump, EDLogFSDTarget, EDLogFuelScoop,
        EDLogJetConeBoost, EDLogLaunchSRV, EDLogLiftoff, EDLogSRVDestroyed, EDLogStartJump,
        EDLogTouchdown,
    },
    ship::{
        EDLogAfmuRepairs, EDLogCargo, EDLogClearImpound, EDLogDockFighter, EDLogEjectCargo,
        EDLogEscapeInterdiction, EDLogFighterDestroyed, EDLogFighterRebuilt, EDLogHullDamage,
        EDLogInterdicted, EDLogInterdiction, EDLogLaunchFighter, EDLogRebootRepair,
        EDLogReservoirReplenished, EDLogSetUserShipName, EDLogShieldState, EDLogShipTargeted,
        EDLogSynthesis, EDLogUnderAttack, ShipType,
    },
    shipyard::{
        EDLogSellShipOnRebuy, EDLogShipRedeemed, EDLogShipyard, EDLogShipyardBuy, EDLogShipyardNew,
        EDLogShipyardRedeem, EDLogShipyardSell, EDLogShipyardSwap, EDLogShipyardTransfer,
        EDLogStoredShips,
    },
    statistics::EDLogStatistics,
    suits::{
        EDLogBackpack, EDLogBackpackChange, EDLogBuySuit, EDLogBuyWeapon, EDLogCollectItems,
        EDLogCreateSuitLoadout, EDLogDeleteSuitLoadout, EDLogDropItems, EDLogLoadoutEquipModule,
        EDLogRenameSuitLoadout, EDLogSellSuit, EDLogSellWeapon, EDLogSuitLoadout, EDLogUpgradeSuit,
        EDLogUpgradeWeapon, EDLogUseConsumable,
    },
    supercruise::{EDLogSupercruiseDestinationDrop, EDLogSupercruiseEntry, EDLogSupercruiseExit},
    transport::{
        EDLogBookDropship, EDLogBookTaxi, EDLogCancelDropship, EDLogCancelTaxi, EDLogDropshipDeploy,
    },
    wing::EDLogWingJoin,
};
use chrono::{DateTime, Utc};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumDiscriminants, EnumIter};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct GameModeGroup {
    group: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "game_mode")]
pub enum GameMode {
    Group(GameModeGroup),
    Solo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LoadGameShip {
    ship: ShipType,
    #[serde(rename = "Ship_Localised")]
    ship_localised: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    ship_name: String,
    ship_ident: String,
    fuel_level: f64,
    fuel_capacity: f64,
    #[serde(flatten)]
    game_mode: GameMode,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase")]
#[testcase({ "timestamp":"2017-10-14T18:41:37Z", "event":"LoadGame", "Commander":"JournalServer", "Ship":"CobraMkIII", "ShipID":1, "ShipName":"Flat Head", "ShipIdent":"UNSC-1", "FuelLevel":16.000000, "FuelCapacity":16.000000, "GameMode":"Open", "Credits":766731, "Loan":0 })]
pub struct EDLogLoadGame {
    #[serde(rename = "FID")]
    pub fid: Option<String>,
    pub commander: String,
    pub horizons: Option<bool>,
    pub odyssey: Option<bool>,
    #[serde(flatten)]
    pub ship: Option<LoadGameShip>,
    pub credits: Credits,
    pub loan: Credits,
    #[serde(rename = "language")]
    pub language: Option<String>,
    #[serde(rename = "gameversion")]
    pub gameversion: Option<String>,
    #[serde(rename = "build")]
    pub build: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMusic {
    music_track: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-07-30T14:40:36Z", "event":"ReceiveText", "From":"$ShipName_PassengerLiner_Cruise;", 
             "From_Localised":"Cruise Ship", "Message":"$CruiseLiner_SCPatrol05;", 
             "Message_Localised":"This is your captain. Due to some unforeseen delays, we will be arriving at our next destination later than scheduled.", 
             "Channel":"npc" })]
pub struct EDLogReceiveText {
    from: String,
    #[serde(rename = "From_Localised")]
    from_localised: Option<String>,
    message: String,
    #[serde(rename = "Message_Localised")]
    message_localised: Option<String>,
    channel: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSendText {
    to: String,
    message: String,
    sent: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogFileHeader {
    part: u64,
    language: String,
    #[serde(rename = "Odyssey")]
    odyssey: Option<bool>,
    gameversion: String,
    build: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum USSType {
    #[serde(rename = "$USS_Type_VeryValuableSalvage;")]
    VeryValueableSalvage,
    #[serde(rename = "$USS_Type_ValuableSalvage;")]
    ValueableSalvage,
    #[serde(rename = "$USS_Type_Salvage;")]
    Salvage,
    #[serde(rename = "$USS_Type_MissionTarget;")]
    MissionTarget,
    #[serde(rename = "$USS_Type_TradingBeacon;")]
    TradingBaecon,
    #[serde(rename = "$USS_Type_Ceremonial;")]
    Ceremonial,
    #[serde(rename = "$USS_Type_WeaponsFire;")]
    WeaponsFire,
    #[serde(rename = "$USS_Type_Aftermath;")]
    Aftermath,
    #[serde(rename = "$USS_Type_Refugee;")]
    Refugee,
    #[serde(rename = "$USS_Type_DistressSignal;")]
    DistressSignal,
    #[serde(rename = "$USS_Type_Convoy;")]
    Convoy,
    #[serde(rename = "$USS_Type_PowerConvoy;")]
    PowerplayConvoy,
    #[serde(rename = "$USS_Type_PowerplayConvoyDistressSignal;")]
    PowerplayConvoyDistressSignal,
    #[serde(rename = "$USS_Type_PowerEmissions;")]
    PowerEmissions,
    #[serde(rename = "$USS_Type_PowerWeaponsFire;")]
    PowerWeaponsFire,
    #[serde(rename = "$USS_Type_NonHuman;")]
    NonHuman,
    #[serde(rename = "$USS_Type_AXShips;")]
    AXShips,
    #[serde(rename = "$USS_Type_AXWeaponsFire;")]
    AXWaeponsFire,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogUSSDrop {
    #[serde(rename = "USSType")]
    uss_type: USSType,
    #[serde(rename = "USSType_Localised")]
    uss_type_localised: String,
    #[serde(rename = "USSThreat")]
    uss_threat: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogCollectCargo {
    #[serde(rename = "Type")]
    cargo_type: String,
    #[serde(rename = "Type_Localised")]
    cargo_type_localised: Option<String>,
    #[serde(rename = "Stolen")]
    stolen: bool,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CargoTransfer {
    #[serde(rename = "Type")]
    cargo_type: String,
    #[serde(rename = "Type_Localised")]
    cargo_type_localised: Option<String>,
    count: u32,
    direction: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCargoTransfer {
    transfers: Vec<CargoTransfer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LocationOnBody {
    latitude: f64,
    longitude: f64,
    heading: u64,
    altitude: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogScreenshot {
    filename: String,
    width: u64,
    height: u64,
    system: Option<String>,
    body: Option<String>,
    #[serde(flatten)]
    location_on_body: Option<LocationOnBody>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display, EnumDiscriminants)]
#[serde(tag = "event", deny_unknown_fields)]
#[strum_discriminants(derive(EnumIter, Display))]
#[allow(clippy::large_enum_variant)]
pub enum EDLogEvent {
    #[serde(rename = "Fileheader")]
    FileHeader(EDLogFileHeader),
    LoadGame(EDLogLoadGame),
    SystemsShutdown,
    Shutdown,
    Music(EDLogMusic),

    // Commander
    Commander(EDLogCommander),
    NewCommander(EDLogNewCommander),
    Rank(EDLogRank),
    Progress(EDLogRank),
    Promotion(EDLogPromotion),
    Reputation(EDLogReputation),
    Powerplay(EDLogPowerplay),
    PowerplayJoin(EDLogPowerplayJoin),
    PowerplayLeave(EDLogPowerplayLeave),
    PowerplayDefect(EDLogPowerplayDefect),
    PowerplayCollect(EDLogPowerplayCollect),
    PowerplayDeliver(EDLogPowerplayDeliver),
    PowerplayFastTrack(EDLogPowerplayFastTrack),
    PowerplayMerits(EDLogPowerplayMerits),
    PowerplaySalary(EDLogPowerplaySalary),
    PowerplayRank(EDLogPowerplayRank),
    RequestPowerMicroResources(EDLogRequestPowerMicroResources),
    Died(EDLogDied),
    Resurrect(EDLogResurrect),
    CommitCrime(EDLogCommitCrime),
    CrimeVictim(EDLogCrimeVictim),
    Embark(EDLogEmbarkOrDisembark),
    Disembark(EDLogEmbarkOrDisembark),
    VehicleSwitch(EDLogVehicleSwitch),
    Friends(EDLogFriends),
    InvitedToSquadron(EDLogInvitedToSquadron),
    AppliedToSquadron(EDLogAppliedToSquadron),
    SharedBookmarkToSquadron(EDLogSharedBookmarkToSquadron),
    HoloscreenHacked(EDLogHoloscreenHacked),
    CommunityGoal(EDLogCommunityGoal),
    CommunityGoalReward(EDLogCommunityGoalReward),
    CommunityGoalJoin(EDLogCommunityGoalJoin),
    CommunityGoalDiscard(EDLogCommunityGoalDiscard),

    // Locker
    ShipLocker(EDLogShipLocker),

    // Materials
    Materials(EDLogMaterials),

    // Statistics
    Statistics(EDLogStatistics),

    // Engineers
    EngineerProgress(EDLogEngineerProgress),
    EngineerContribution(EDLogEngineerContribution),
    EngineerCraft(EDLogEngineerCraft),

    // Docking
    DockingRequested(EDLogDockingRequested),
    DockingCancelled(EDLogDockingCancelled),
    DockingGranted(EDLogDockingGranted),
    DockingDenied(EDLogDockingDenied),
    DockingTimeout(EDLogDockingTimeout),
    Docked(EDLogDocked),
    Undocked(EDLogUndocked),
    RefuelAll(EDLogRefuelAll),
    BuyAmmo(EDLogBuyAmmo),
    Repair(EDLogRepair),
    RepairAll(EDLogRepairAll),
    Outfitting(EDLogOutfitting),
    MaterialTrade(EDLogMaterialTrade),
    MaterialCollected(EDLogMaterialCollected),
    PayBounties(EDLogPayBounties),
    PayFines(EDLogPayFines),
    TechnologyBroker(EDLogTechnologyBroker),
    RestockVehicle(EDLogRestockVehicle),

    // Modules
    StoredModules(EDLogStoredModules),
    ModuleBuyAndStore(EDLogModuleBuyAndStore),
    ModuleStore(EDLogModuleStore),
    MassModuleStore(EDLogMassModuleStore),
    ModuleRetrieve(EDLogModuleRetrieve),
    ModuleBuy(EDLogModuleBuy),
    ModuleSell(EDLogModuleSell),
    ModuleSellRemote(EDLogModuleSellRemote),
    ModuleSwap(EDLogModuleSwap),
    ModuleInfo,
    FetchRemoteModule(EDLogFetchRemoteModule),

    // Navigation
    NavRoute,
    NavRouteClear,
    FSDTarget(EDLogFSDTarget),
    FSDJump(EDLogFSDJump),
    FuelScoop(EDLogFuelScoop),
    ApproachSettlement(EDLogApproachSettlement),
    ApproachBody(BodyInformation),
    LeaveBody(BodyInformation),
    Liftoff(EDLogLiftoff),
    LaunchSRV(EDLogLaunchSRV),
    DockSRV(EDLogDockSRV),
    SRVDestroyed(EDLogSRVDestroyed),
    Touchdown(EDLogTouchdown),
    StartJump(EDLogStartJump),
    JetConeBoost(EDLogJetConeBoost),

    // Supercruise
    SupercruiseEntry(EDLogSupercruiseEntry),
    SupercruiseDestinationDrop(EDLogSupercruiseDestinationDrop),
    SupercruiseExit(EDLogSupercruiseExit),

    // Exploration
    FSSSignalDiscovered(EDLogFSSSignalDiscovered),
    FSSDiscoveryScan(EDLogFSSDiscoveryScan),
    FSSAllBodiesFound(EDLogFSSAllBodiesFound),
    FSSBodySignals(EDLogFSSBodySignals),
    SAAScanComplete(EDLogSAAScanComplete),
    SAASignalsFound(EDLogSAASignalsFound),
    ScanBaryCentre(EDLogScanBaryCentre),
    StationBernalSphere(EDLogStationBernalSphere),
    MultiSellExplorationData(EDLogMultiSellExplorationData),
    Scan(EDLogScan),
    Scanned(EDLogScanned),
    DiscoveryScan(EDLogDiscoveryScan),
    ScanOrganic(EDLogScanOrganic),
    DatalinkScan(EDLogDatalinkScan),
    NavBeaconScan(EDLogNavBeaconScan),
    SellOrganicData(EDLogSellOrganicData),
    CodexEntry(EDLogCodexEntry),
    ProspectedAsteroid(EDLogProspectedAsteroid),
    MiningRefined(EDLogMiningRefined),
    AsteroidCracked(EDLogAsteroidCracked),
    MaterialDiscovered(EDLogMaterialDiscovered),
    DataScanned(EDLogDataScanned),
    BuyExplorationData(EDLogBuyExplorationData),
    SellExplorationData(EDLogSellExplorationData),

    // FleetCarrier
    CarrierStats(EDLogCarrierStats),
    CarrierJumpRequest(EDLogCarrierJumpRequest),
    CarrierBankTransfer(EDLogCarrierBankTransfer),
    CarrierFinance(EDLogCarrierFinance),
    CarrierTradeOrder(EDLogCarrierTradeOrder),
    CarrierJump(EDLogCarrierJump),
    CarrierJumpCancelled(EDLogCarrierJumpCancelled),
    CarrierDepositFuel(EDLogCarrierDepositFuel),
    CarrierCrewServices(EDLogCarrierCrewServices),
    CarrierBuy(EDLogCarrierBuy),
    CarrierNameChange(EDLogCarrierNameChange),
    CarrierLocation(EDLogCarrierLocation),
    CarrierDockingPermission(EDLogCarrierDockingPermission),
    CarrierModulePack(EDLogCarrierModulePack),
    FCMaterials(EDLogFCMaterials),

    // Drone
    LaunchDrone(EDLogLaunchDrone),
    SellDrones(EDLogSellDrones),
    BuyDrones(EDLogBuyDrones),
    RepairDrone(EDLogRepairDrone),

    // Missions
    Missions(EDLogMissions),
    MissionAccepted(EDLogMissionAccepted),
    MissionRedirected(EDLogMissionRedirected),
    MissionCompleted(EDLogMissionCompleted),
    MissionFailed(EDLogMissionFailed),
    MissionAbandoned(EDLogMissionAbandoned),
    SearchAndRescue(EDLogSearchAndRescue),
    Bounty(EDLogBounty),
    RedeemVoucher(EDLogRedeemVoucher),
    DatalinkVoucher(EDLogDatalinkVoucher),
    FactionKillBond(EDLogFactionKillBond),
    #[serde(rename = "CapShipBond")]
    CapitalShipBond(EDLogCapitalShipBond),
    PVPKill(EDLogPVPKill),
    Passengers(EDLogPassengers),

    // Market
    Market(EDLogMarket),
    MarketBuy(EDLogMarketBuy),
    MarketSell(EDLogMarketSell),
    SellMicroResources(EDLogSellMicroResources),
    BuyMicroResources(EDLogBuyMicroResources),
    TradeMicroResources(EDLogTradeMicroResources),
    DeliverPowerMicroResources(EDLogDeliverPowerMicroResources),
    CargoDepot(EDLogCargoDepot),
    ColonisationConstructionDepot(EDLogColonisationConstructionDepot),
    ColonisationContribution(EDLogColonisationContribution),
    BuyTradeData(EDLogBuyTradeData),

    // Shipyard
    ShipyardSwap(EDLogShipyardSwap),
    ShipyardTransfer(EDLogShipyardTransfer),
    ShipyardNew(EDLogShipyardNew),
    Shipyard(EDLogShipyard),
    ShipyardBuy(EDLogShipyardBuy),
    ShipyardSell(EDLogShipyardSell),
    StoredShips(EDLogStoredShips),
    ShipyardRedeem(EDLogShipyardRedeem),
    ShipRedeemed(EDLogShipRedeemed),
    SellShipOnRebuy(EDLogSellShipOnRebuy),

    // Suit and backpack
    SuitLoadout(EDLogSuitLoadout),
    Backpack(EDLogBackpack),
    SwitchSuitLoadout(EDLogSuitLoadout),
    BackpackChange(EDLogBackpackChange),
    UseConsumable(EDLogUseConsumable),
    UpgradeWeapon(EDLogUpgradeWeapon),
    BuyWeapon(EDLogBuyWeapon),
    SellWeapon(EDLogSellWeapon),
    BuySuit(EDLogBuySuit),
    SellSuit(EDLogSellSuit),
    CreateSuitLoadout(EDLogCreateSuitLoadout),
    RenameSuitLoadout(EDLogRenameSuitLoadout),
    DeleteSuitLoadout(EDLogDeleteSuitLoadout),
    LoadoutEquipModule(EDLogLoadoutEquipModule),
    UpgradeSuit(EDLogUpgradeSuit),
    DropItems(EDLogDropItems),

    // Transport
    BookDropship(EDLogBookDropship),
    DropshipDeploy(EDLogDropshipDeploy),
    BookTaxi(EDLogBookTaxi),
    CancelTaxi(EDLogCancelTaxi),
    CancelDropship(EDLogCancelDropship),

    // Wing
    WingAdd(EDLogName),
    WingJoin(EDLogWingJoin),
    WingLeave,
    WingInvite(EDLogName),
    SquadronStartup(EDLogSquadronStartup),
    LeftSquadron(EDLogLeftSquadron),
    NpcCrewPaidWage(EDLogNpcCrewPaidWage),
    NpcCrewRank(EDLogNpcCrewRank),
    CrewHire(EDLogCrewHire),
    ChangeCrewRole(EDLogChangeCrewRole),
    CrewMemberRoleChange(EDLogCrewMemberRoleChange),
    CrewMemberJoins(EDLogCrewMemberJoins),
    EndCrewSession(EDLogEndCrewSession),
    CrewAssign(EDLogCrewAssign),
    CrewFire(EDLogCrewFire),
    JoinACrew(EDLogJoinACrew),
    QuitACrew(EDLogQuitACrew),
    CrewMemberQuits(EDLogCrewMemberQuits),

    // Location
    Location(EDLogLocation),

    // Loadout
    Loadout(EDLogLoadout),

    // Ship
    Cargo(EDLogCargo),
    ShieldState(EDLogShieldState),
    EjectCargo(EDLogEjectCargo),
    ReservoirReplenished(EDLogReservoirReplenished),
    ShipTargeted(EDLogShipTargeted),
    Interdiction(EDLogInterdiction),
    Interdicted(EDLogInterdicted),
    EscapeInterdiction(EDLogEscapeInterdiction),
    RebootRepair(EDLogRebootRepair),
    UnderAttack(EDLogUnderAttack),
    HullDamage(EDLogHullDamage),
    HeatWarning,
    HeatDamage,
    SelfDestruct,
    CockpitBreached,
    Resupply,
    Synthesis(EDLogSynthesis),
    AfmuRepairs(EDLogAfmuRepairs),
    SetUserShipName(EDLogSetUserShipName),
    ClearImpound(EDLogClearImpound),
    LaunchFighter(EDLogLaunchFighter),
    DockFighter(EDLogDockFighter),
    FighterDestroyed(EDLogFighterDestroyed),
    FighterRebuilt(EDLogFighterRebuilt),

    Screenshot(EDLogScreenshot),
    ReceiveText(EDLogReceiveText),
    SendText(EDLogSendText),
    USSDrop(EDLogUSSDrop),
    CollectItems(EDLogCollectItems),
    CollectCargo(EDLogCollectCargo),
    CargoTransfer(EDLogCargoTransfer),
}

pub trait Extractable {
    fn extract(event: &EDLogEvent) -> Option<&Self>;
}

impl EDLogEvent {
    pub fn extract<T: Extractable>(&self) -> Option<&T> {
        T::extract(self)
    }

    // from https://doc.rust-lang.org/reference/items/enumerations.html#casting
    pub fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EDLogLine {
    timestamp: DateTime<Utc>,
    #[serde(flatten)]
    event: EDLogEvent,
}

impl EDLogLine {
    pub fn event(&self) -> &EDLogEvent {
        &self.event
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn extract<T: Extractable>(&self) -> Option<&T> {
        self.event.extract::<T>()
    }
}

#[test]
fn test_fileheader() {
    let json = r#"{ "timestamp":"2025-03-10T18:19:38Z", "event":"Fileheader", "part":1, "language":"English/UK", "Odyssey":true, "gameversion":"4.1.0.100", "build":"r311607/r0 " }"#;
    let line: EDLogLine = serde_json::from_str(json).expect("Should parse");

    assert!(matches!(line.event(), EDLogEvent::FileHeader(_)));
    if let EDLogEvent::FileHeader(header) = line.event() {
        assert!(header.odyssey.unwrap());
        assert_eq!(header.gameversion, "4.1.0.100");
    }
}

#[test]
fn test_receivetext() {
    let json = r#"{ "timestamp":"2023-07-30T14:40:36Z", "event":"ReceiveText", 
                          "From":"$ShipName_PassengerLiner_Cruise;", "From_Localised":"Cruise Ship", 
                          "Message":"$CruiseLiner_SCPatrol05;", 
                          "Message_Localised":"This is your captain. Due to some unforeseen delays, we will be arriving at our next destination later than scheduled.", "Channel":"npc" }"#;
    let line: EDLogLine = serde_json::from_str(json).expect("Should parse");

    assert!(matches!(line.event(), EDLogEvent::ReceiveText(_)));
    if let EDLogEvent::ReceiveText(header) = line.event() {
        assert_eq!(header.message, "$CruiseLiner_SCPatrol05;");
    }
}
