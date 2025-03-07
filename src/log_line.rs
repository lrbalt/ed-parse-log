use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    commander::{
        EDLogCommander, EDLogCommitCrime, EDLogCommunityGoalJoin, EDLogCommunityGoalReward,
        EDLogCrimeVictim, EDLogDied, EDLogEmbarkOrDisembark, EDLogEndCrewSession, EDLogFriends,
        EDLogHoloscreenHacked, EDLogInvitedToSquadron, EDLogNewCommander, EDLogPowerplay,
        EDLogPowerplayCollect, EDLogPowerplayDeliver, EDLogPowerplayFastTrack, EDLogPowerplayJoin,
        EDLogPowerplaySalary, EDLogPromotion, EDLogRank, EDLogReputation, EDLogResurrect,
        EDLogVehicleSwitch,
    },
    common_types::{BodyInformation, EDLogName},
    community_goal::EDLogCommunityGoal,
    docking::{
        EDLogBuyAmmo, EDLogDocked, EDLogDockingCancelled, EDLogDockingDenied, EDLogDockingGranted,
        EDLogDockingRequested, EDLogDockingTimeout, EDLogMaterialCollected, EDLogMaterialTrade,
        EDLogOutfitting, EDLogPayBounties, EDLogPayFines, EDLogRefuelAll, EDLogRepair,
        EDLogRepairAll, EDLogRestockVehicle, EDLogTechnologyBroker, EDLogUndocked,
    },
    drone::{EDLogBuyDrones, EDLogLaunchDrone, EDLogRepairDrone, EDLogSellDrones},
    engineers::{EDLogEngineerContribution, EDLogEngineerCraft, EDLogEngineerProgress},
    exploration::{
        EDLogBuyExplorationData, EDLogCodexEntry, EDLogDataScanned, EDLogDatalinkScan,
        EDLogDiscoveryScan, EDLogFSSAllBodiesFound, EDLogFSSBodySignals, EDLogFSSDiscoveryScan,
        EDLogFSSSignalDiscovered, EDLogMaterialDiscovered, EDLogMiningRefined,
        EDLogMultiSellExplorationData, EDLogNavBeaconScan, EDLogProspectedAsteroid,
        EDLogSAAScanComplete, EDLogSAASignalsFound, EDLogScan, EDLogScanBaryCentre,
        EDLogScanOrganic, EDLogScanned, EDLogSellExplorationData, EDLogSellOrganicData,
        EDLogStationBernalSphere,
    },
    fleet_carrier::{
        EDLogCarrierBankTransfer, EDLogCarrierBuy, EDLogCarrierCrewServices,
        EDLogCarrierDepositFuel, EDLogCarrierFinance, EDLogCarrierJump, EDLogCarrierJumpCancelled,
        EDLogCarrierJumpRequest, EDLogCarrierNameChange, EDLogCarrierStats, EDLogCarrierTradeOrder,
        EDLogFCMaterials,
    },
    loadout::EDLogLoadout,
    location::EDLogLocation,
    locker::EDLogShipLocker,
    market::{
        EDLogBuyMicroResources, EDLogCargoDepot, EDLogDeliverPowerMicroResources, EDLogMarket,
        EDLogMarketBuy, EDLogMarketSell, EDLogSellMicroResources, EDLogTradeMicroResources,
    },
    materials::EDLogMaterials,
    mission::{
        EDLogBounty, EDLogDatalinkVoucher, EDLogFactionKillBond, EDLogMissionAbandoned,
        EDLogMissionAccepted, EDLogMissionCompleted, EDLogMissionFailed, EDLogMissionRedirected,
        EDLogMissions, EDLogRedeemVoucher, EDLogSearchAndRescue,
    },
    modules::{
        EDLogFetchRemoteModule, EDLogModuleBuy, EDLogModuleBuyAndStore, EDLogModuleRetrieve,
        EDLogModuleSell, EDLogModuleSellRemote, EDLogModuleStore, EDLogModuleSwap,
        EDLogStoredModules,
    },
    navigation::{
        EDLogApproachSettlement, EDLogDockSRV, EDLogFSDJump, EDLogFSDTarget, EDLogFuelScoop,
        EDLogLaunchSRV, EDLogLiftoff, EDLogSRVDestroyed, EDLogStartJump, EDLogTouchdown,
    },
    ship::{
        EDLogAfmuRepairs, EDLogCargo, EDLogClearImpound, EDLogDockFighter, EDLogEjectCargo,
        EDLogEscapeInterdiction, EDLogFighterDestroyed, EDLogFighterRebuilt, EDLogHullDamage,
        EDLogInterdicted, EDLogInterdiction, EDLogLaunchFighter, EDLogRebootRepair,
        EDLogReservoirReplenished, EDLogSetUserShipName, EDLogShieldState, EDLogShipTargeted,
        EDLogSynthesis, EDLogUnderAttack,
    },
    shipyard::{
        EDLogShipRedeemed, EDLogShipyard, EDLogShipyardBuy, EDLogShipyardNew, EDLogShipyardRedeem,
        EDLogShipyardSell, EDLogShipyardSwap, EDLogShipyardTransfer, EDLogStoredShips,
    },
    statistics::EDLogStatistics,
    suits::{
        BackpackItemType, EDLogBackpack, EDLogBackpackChange, EDLogBuySuit, EDLogBuyWeapon,
        EDLogCreateSuitLoadout, EDLogDeleteSuitLoadout, EDLogDropItems, EDLogLoadoutEquipModule,
        EDLogRenameSuitLoadout, EDLogSellSuit, EDLogSellWeapon, EDLogSuitLoadout, EDLogUpgradeSuit,
        EDLogUpgradeWeapon, EDLogUseConsumable,
    },
    supercruise::{EDLogSupercruiseDestinationDrop, EDLogSupercruiseEntry, EDLogSupercruiseExit},
    transport::{EDLogBookDropship, EDLogBookTaxi, EDLogCancelDropship, EDLogDropshipDeploy},
    wing::EDLogWingJoin,
};

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
    ship: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogLoadGame {
    #[serde(rename = "FID")]
    fid: String,
    commander: String,
    horizons: bool,
    odyssey: Option<bool>,
    #[serde(flatten)]
    ship: Option<LoadGameShip>,
    credits: u64,
    loan: u64,
    #[serde(rename = "language")]
    language: Option<String>,
    #[serde(rename = "gameversion")]
    gameversion: Option<String>,
    #[serde(rename = "build")]
    build: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMusic {
    music_track: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogReceivedText {
    from: String,
    #[serde(rename = "From_Localised")]
    from_localised: Option<String>,
    message: String,
    #[serde(rename = "Message_Localised")]
    message_localised: Option<String>,
    channel: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSendText {
    to: String,
    message: String,
    sent: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EDLogFileHeader {
    part: u64,
    language: String,
    #[serde(rename = "Odyssey")]
    odyssey: bool,
    gameversion: String,
    build: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EDLogUSSDrop {
    #[serde(rename = "USSType")]
    uss_type: String,
    #[serde(rename = "USSType_Localised")]
    uss_type_localised: String,
    #[serde(rename = "USSThreat")]
    uss_threat: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCollectItems {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    #[serde(rename = "Type")]
    item_type: BackpackItemType,
    #[serde(rename = "OwnerID")]
    owner_id: u64,
    count: u64,
    stolen: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCargoTransfer {
    transfers: Vec<CargoTransfer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogScreenshot {
    filename: String,
    width: u64,
    height: u64,
    system: String,
    body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "event", deny_unknown_fields)]
#[allow(clippy::large_enum_variant)]
pub enum EDLogEvent {
    #[serde(rename = "Fileheader")]
    FileHeader(EDLogFileHeader),
    LoadGame(EDLogLoadGame),
    SystemsShutdown,
    Shutdown,
    Music(EDLogMusic),
    CommunityGoal(EDLogCommunityGoal),

    // Commander
    Commander(EDLogCommander),
    NewCommander(EDLogNewCommander),
    Rank(EDLogRank),
    Progress(EDLogRank),
    Promotion(EDLogPromotion),
    Reputation(EDLogReputation),
    Powerplay(EDLogPowerplay),
    PowerplayJoin(EDLogPowerplayJoin),
    PowerplayCollect(EDLogPowerplayCollect),
    PowerplayDeliver(EDLogPowerplayDeliver),
    PowerplayFastTrack(EDLogPowerplayFastTrack),
    Died(EDLogDied),
    Resurrect(EDLogResurrect),
    PowerplaySalary(EDLogPowerplaySalary),
    CommitCrime(EDLogCommitCrime),
    CrimeVictim(EDLogCrimeVictim),
    Embark(EDLogEmbarkOrDisembark),
    Disembark(EDLogEmbarkOrDisembark),
    VehicleSwitch(EDLogVehicleSwitch),
    Friends(EDLogFriends),
    InvitedToSquadron(EDLogInvitedToSquadron),
    HoloscreenHacked(EDLogHoloscreenHacked),
    CommunityGoalReward(EDLogCommunityGoalReward),
    CommunityGoalJoin(EDLogCommunityGoalJoin),
    EndCrewSession(EDLogEndCrewSession),

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

    // Market
    Market(EDLogMarket),
    MarketBuy(EDLogMarketBuy),
    MarketSell(EDLogMarketSell),
    SellMicroResources(EDLogSellMicroResources),
    BuyMicroResources(EDLogBuyMicroResources),
    TradeMicroResources(EDLogTradeMicroResources),
    DeliverPowerMicroResources(EDLogDeliverPowerMicroResources),
    CargoDepot(EDLogCargoDepot),

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
    CancelDropship(EDLogCancelDropship),

    // Wing
    WingAdd(EDLogName),
    WingJoin(EDLogWingJoin),
    WingLeave,
    WingInvite(EDLogName),

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
    ReceiveText(EDLogReceivedText),
    SendText(EDLogSendText),
    USSDrop(EDLogUSSDrop),
    CollectItems(EDLogCollectItems),
    CollectCargo(EDLogCollectCargo),
    CargoTransfer(EDLogCargoTransfer),
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
}
