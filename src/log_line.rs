use crate::{
    EDString,
    combat::{
        EDLogBounty, EDLogCapitalShipBond, EDLogDied, EDLogEscapeInterdiction,
        EDLogFactionKillBond, EDLogFighterDestroyed, EDLogHullDamage, EDLogInterdicted,
        EDLogInterdiction, EDLogPVPKill, EDLogSRVDestroyed, EDLogShieldState, EDLogShipTargeted,
        EDLogUnderAttack,
    },
    commander::{EDLogCarrierLocation, EDLogRequestPowerMicroResources},
    common_types::{BodyInformation, EDLogName},
    exploration::{
        EDLogBuyExplorationData, EDLogCodexEntry, EDLogDiscoveryScan, EDLogFSSAllBodiesFound,
        EDLogFSSBodySignals, EDLogFSSDiscoveryScan, EDLogFSSSignalDiscovered,
        EDLogMaterialCollected, EDLogMaterialDiscarded, EDLogMaterialDiscovered,
        EDLogMultiSellExplorationData, EDLogNavBeaconScan, EDLogSAAScanComplete,
        EDLogSAASignalsFound, EDLogScan, EDLogScanBaryCentre, EDLogScreenshot,
        EDLogSellExplorationData,
    },
    exploration_old::EDLogStationBernalSphere,
    fleet_carrier::{
        EDLogCarrierBankTransfer, EDLogCarrierBuy, EDLogCarrierCancelDecommission,
        EDLogCarrierCrewServices, EDLogCarrierDecommission, EDLogCarrierDepositFuel,
        EDLogCarrierDockingPermission, EDLogCarrierFinance, EDLogCarrierJump,
        EDLogCarrierJumpCancelled, EDLogCarrierJumpRequest, EDLogCarrierModulePack,
        EDLogCarrierNameChange, EDLogCarrierShipPack, EDLogCarrierStats, EDLogCarrierTradeOrder,
    },
    market::{
        EDLogColonisationConstructionDepot, EDLogColonisationContribution,
        EDLogDeliverPowerMicroResources, EDLogMarketID,
    },
    modules::EDLogModuleBuyAndStore,
    navigation::EDLogLaunchVessel,
    odyssey::{
        EDLogBackpack, EDLogBackpackChange, EDLogBookDropship, EDLogBookTaxi,
        EDLogBuyMicroResources, EDLogBuySuit, EDLogBuyWeapon, EDLogCancelDropship, EDLogCancelTaxi,
        EDLogCollectItems, EDLogCreateSuitLoadout, EDLogDropItems, EDLogDropshipDeploy,
        EDLogEmbarkOrDisembark, EDLogFCMaterials, EDLogLoadoutEquipModule,
        EDLogLoadoutRemoveModule, EDLogRenameSuitLoadout, EDLogScanOrganic,
        EDLogSellMicroResources, EDLogSellOrganicData, EDLogSellSuit, EDLogSellWeapon,
        EDLogShipLocker, EDLogSuitLoadout, EDLogTradeMicroResources, EDLogUpgradeSuit,
        EDLogUpgradeWeapon, EDLogUseConsumable,
    },
    other::{
        EDLogAfmuRepairs, EDLogApproachSettlement, EDLogCargoTransfer, EDLogChangeCrewRole,
        EDLogCommitCrime, EDLogContinued, EDLogCrewLaunchFighter, EDLogCrewMemberJoins,
        EDLogCrewMemberQuits, EDLogCrewMemberRoleChange, EDLogCrimeVictim, EDLogDataScanned,
        EDLogDatalinkScan, EDLogDatalinkVoucher, EDLogDockFighter, EDLogDockSRV,
        EDLogEndCrewSession, EDLogFighterRebuilt, EDLogFriends, EDLogFuelScoop, EDLogJetConeBoost,
        EDLogJetConeDamage, EDLogJoinACrew, EDLogKickCrewMember, EDLogLaunchDrone,
        EDLogLaunchFighter, EDLogLaunchSRV, EDLogModuleInfo, EDLogMusic, EDLogNpcCrewPaidWage,
        EDLogNpcCrewRank, EDLogPromotion, EDLogProspectedAsteroid, EDLogQuitACrew,
        EDLogRebootRepair, EDLogReceiveText, EDLogRepairDrone, EDLogReservoirReplenished,
        EDLogResurrect, EDLogScanned, EDLogSendText, EDLogSupercruiseDestinationDrop,
        EDLogSynthesis, EDLogUSSDrop, EDLogVehicleSwitch, EDLogWingJoin,
    },
    powerplay::{
        EDLogHoloscreenHacked, EDLogPowerplayCollect, EDLogPowerplayDefect, EDLogPowerplayDeliver,
        EDLogPowerplayFastTrack, EDLogPowerplayJoin, EDLogPowerplayLeave, EDLogPowerplayMerits,
        EDLogPowerplayRank, EDLogPowerplaySalary, EDLogPowerplayVote, EDLogPowerplayVoucher,
    },
    ship_type::ShipType,
    shipyard::{EDLogShipRedeemed, EDLogShipyardRedeem},
    squadron::{
        EDLogAppliedToSquadron, EDLogDisbandedSquadron, EDLogInvitedToSquadron,
        EDLogJoinedSquadron, EDLogKickedFromSquadron, EDLogLeftSquadron,
        EDLogSharedBookmarkToSquadron, EDLogSquadronCreated, EDLogSquadronDemotion,
        EDLogSquadronPromotion, EDLogSquadronStartup, EDLogWonATrophyForSquadron,
    },
    startup::{
        EDLogCargo, EDLogClearSavedGame, EDLogCommander, EDLogLoadGame, EDLogLoadout,
        EDLogMaterials, EDLogMissions, EDLogNewCommander, EDLogPassengers, EDLogPowerplay,
        EDLogRank, EDLogReputation, EDLogStatistics,
    },
    station_services::{
        EDLogBuyAmmo, EDLogBuyDrones, EDLogCargoDepot, EDLogClearImpound, EDLogCommunityGoal,
        EDLogCommunityGoalDiscard, EDLogCommunityGoalJoin, EDLogCommunityGoalReward,
        EDLogCrewAssign, EDLogCrewFire, EDLogCrewHire, EDLogEngineerContribution,
        EDLogEngineerCraft, EDLogEngineerProgress, EDLogFetchRemoteModule, EDLogMarket,
        EDLogMassModuleStore, EDLogMaterialTrade, EDLogMissionAbandoned, EDLogMissionAccepted,
        EDLogMissionCompleted, EDLogMissionFailed, EDLogMissionRedirected, EDLogModuleBuy,
        EDLogModuleRetrieve, EDLogModuleSell, EDLogModuleSellRemote, EDLogModuleStore,
        EDLogModuleSwap, EDLogOutfitting, EDLogPayBounties, EDLogPayFines, EDLogRedeemVoucher,
        EDLogRefuelAll, EDLogRefuelPartial, EDLogRepair, EDLogRepairAll, EDLogRestockVehicle,
        EDLogScientificResearch, EDLogSearchAndRescue, EDLogSellDrones, EDLogSellShipOnRebuy,
        EDLogSetUserShipName, EDLogShipyard, EDLogShipyardBuy, EDLogShipyardNew, EDLogShipyardSell,
        EDLogShipyardSwap, EDLogShipyardTransfer, EDLogStoredModules, EDLogStoredShips,
        EDLogTechnologyBroker,
    },
    status::EDLogStatus,
    suits::EDLogDeleteSuitLoadout,
    trade::{
        EDLogAsteroidCracked, EDLogBuyTradeData, EDLogCollectCargo, EDLogEjectCargo,
        EDLogMarketBuy, EDLogMarketSell, EDLogMiningRefined,
    },
    travel::{
        EDLogDocked, EDLogDockingCancelled, EDLogDockingDenied, EDLogDockingGranted,
        EDLogDockingRequested, EDLogDockingTimeout, EDLogFSDJump, EDLogFSDTarget, EDLogLiftoff,
        EDLogLocation, EDLogNavRoute, EDLogStartJump, EDLogSupercruiseEntry, EDLogSupercruiseExit,
        EDLogTouchdown, EDLogUndocked,
    },
};
use chrono::{DateTime, Utc};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumDiscriminants, EnumIter};

#[derive(Serialize, Deserialize, Clone, Debug, Copy, Display)]
pub enum GameMode {
    Group,
    Solo,
    Open,
    MainGame,
    Operation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Ship":"CobraMkIII", "ShipID":1, "ShipName":"Flat Head", "ShipIdent":"UNSC-1"})]
#[testcase_struct({"Ship":"Combat_Multicrew_SRV_01", "Ship_Localised":"SRV Scorpion", "ShipID":49, "ShipName":"", "ShipIdent":"", "FuelLevel":0.000000, "FuelCapacity":0.000000})]
pub struct LoadGameShip {
    pub ship: ShipType,
    #[serde(rename = "Ship_Localised")]
    pub ship_localised: Option<EDString>,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub ship_name: EDString,
    pub ship_ident: EDString,
    pub fuel_level: Option<f64>,
    pub fuel_capacity: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-09-02T18:51:13Z", "event":"GameModeChange", "GameMode":"MainGame" })]
pub struct EDLogGameModeChange {
    game_mode: GameMode,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogFileHeader {
    part: u64,
    language: EDString,
    #[serde(rename = "Odyssey")]
    odyssey: Option<bool>,
    gameversion: EDString,
    build: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display, EnumDiscriminants)]
#[serde(tag = "event", deny_unknown_fields)]
#[strum_discriminants(derive(EnumIter, Display))]
// Some variants use Box<_> for the data to keep all variants below 64 bytes (cache line)
pub enum EDLogEvent {
    #[serde(rename = "Fileheader")]
    FileHeader(EDLogFileHeader),

    // Startup
    ClearSavedGame(EDLogClearSavedGame),
    Cargo(EDLogCargo),
    Commander(EDLogCommander),
    Loadout(Box<EDLogLoadout>),
    Materials(Box<EDLogMaterials>),
    Missions(Box<EDLogMissions>),
    NewCommander(EDLogNewCommander),
    LoadGame(Box<EDLogLoadGame>),
    Passengers(EDLogPassengers),
    Powerplay(EDLogPowerplay),
    Rank(EDLogRank),
    Progress(EDLogRank),
    Reputation(Box<EDLogReputation>),
    Statistics(Box<EDLogStatistics>),

    // Travel
    ApproachBody(BodyInformation),
    Docked(Box<EDLogDocked>),
    DockingCancelled(EDLogDockingCancelled),
    DockingDenied(EDLogDockingDenied),
    DockingRequested(Box<EDLogDockingRequested>),
    DockingGranted(EDLogDockingGranted),
    DockingTimeout(EDLogDockingTimeout),
    FSDJump(Box<EDLogFSDJump>),
    FSDTarget(EDLogFSDTarget),
    LeaveBody(BodyInformation),
    Liftoff(Box<EDLogLiftoff>),
    Location(Box<EDLogLocation>),
    StartJump(EDLogStartJump),
    SupercruiseEntry(EDLogSupercruiseEntry),
    SupercruiseExit(EDLogSupercruiseExit),
    Touchdown(Box<EDLogTouchdown>),
    Undocked(EDLogUndocked),
    NavRoute(EDLogNavRoute),
    NavRouteClear,

    // Combat
    Bounty(Box<EDLogBounty>),
    #[serde(rename = "CapShipBond")]
    CapitalShipBond(EDLogCapitalShipBond),
    Died(EDLogDied),
    EscapeInterdiction(EDLogEscapeInterdiction),
    FactionKillBond(EDLogFactionKillBond),
    FighterDestroyed(EDLogFighterDestroyed),
    HeatDamage,
    HeatWarning,
    HullDamage(EDLogHullDamage),
    Interdiction(EDLogInterdiction),
    Interdicted(EDLogInterdicted),
    PVPKill(EDLogPVPKill),
    ShieldState(EDLogShieldState),
    ShipTargeted(Box<EDLogShipTargeted>),
    SRVDestroyed(EDLogSRVDestroyed),
    UnderAttack(EDLogUnderAttack),

    // Exploration
    CodexEntry(Box<EDLogCodexEntry>),
    DiscoveryScan(EDLogDiscoveryScan),
    Scan(Box<EDLogScan>),
    FSSAllBodiesFound(EDLogFSSAllBodiesFound),
    FSSBodySignals(EDLogFSSBodySignals),
    FSSDiscoveryScan(EDLogFSSDiscoveryScan),
    FSSSignalDiscovered(Box<EDLogFSSSignalDiscovered>),
    MaterialCollected(EDLogMaterialCollected),
    MaterialDiscarded(EDLogMaterialDiscarded),
    MaterialDiscovered(EDLogMaterialDiscovered),
    MultiSellExplorationData(EDLogMultiSellExplorationData),
    NavBeaconScan(EDLogNavBeaconScan),
    BuyExplorationData(EDLogBuyExplorationData),
    SAAScanComplete(EDLogSAAScanComplete),
    SAASignalsFound(Box<EDLogSAASignalsFound>),
    ScanBaryCentre(Box<EDLogScanBaryCentre>),
    SellExplorationData(Box<EDLogSellExplorationData>),
    Screenshot(Box<EDLogScreenshot>),

    // Trade
    AsteroidCracked(EDLogAsteroidCracked),
    BuyTradeData(EDLogBuyTradeData),
    CollectCargo(EDLogCollectCargo),
    EjectCargo(EDLogEjectCargo),
    MarketBuy(EDLogMarketBuy),
    MarketSell(Box<EDLogMarketSell>),
    MiningRefined(EDLogMiningRefined),

    // Station Services
    BuyAmmo(EDLogBuyAmmo),
    BuyDrones(EDLogBuyDrones),
    CargoDepot(Box<EDLogCargoDepot>),
    CommunityGoal(EDLogCommunityGoal),
    CommunityGoalReward(EDLogCommunityGoalReward),
    CommunityGoalJoin(EDLogCommunityGoalJoin),
    CommunityGoalDiscard(EDLogCommunityGoalDiscard),
    CrewHire(EDLogCrewHire),
    CrewAssign(EDLogCrewAssign),
    CrewFire(EDLogCrewFire),
    EngineerContribution(Box<EDLogEngineerContribution>),
    EngineerProgress(Box<EDLogEngineerProgress>),
    EngineerCraft(Box<EDLogEngineerCraft>),
    FetchRemoteModule(Box<EDLogFetchRemoteModule>),
    Market(EDLogMarket),
    MassModuleStore(EDLogMassModuleStore),
    MaterialTrade(EDLogMaterialTrade),
    MissionAbandoned(EDLogMissionAbandoned),
    MissionAccepted(Box<EDLogMissionAccepted>),
    MissionCompleted(Box<EDLogMissionCompleted>),
    MissionRedirected(EDLogMissionRedirected),
    MissionFailed(EDLogMissionFailed),
    ModuleBuy(Box<EDLogModuleBuy>),
    ModuleRetrieve(Box<EDLogModuleRetrieve>),
    ModuleSell(EDLogModuleSell),
    ModuleSellRemote(EDLogModuleSellRemote),
    ModuleStore(Box<EDLogModuleStore>),
    ModuleSwap(EDLogModuleSwap),
    Outfitting(Box<EDLogOutfitting>),
    PayBounties(EDLogPayBounties),
    PayFines(EDLogPayFines),
    RedeemVoucher(EDLogRedeemVoucher),
    RefuelAll(EDLogRefuelAll),
    RefuelPartial(EDLogRefuelPartial),
    Repair(EDLogRepair),
    RepairAll(EDLogRepairAll),
    RestockVehicle(EDLogRestockVehicle),
    ScientificResearch(EDLogScientificResearch),
    SearchAndRescue(EDLogSearchAndRescue),
    SellDrones(EDLogSellDrones),
    SellShipOnRebuy(EDLogSellShipOnRebuy),
    SetUserShipName(EDLogSetUserShipName),
    ShipyardSwap(Box<EDLogShipyardSwap>),
    ShipyardTransfer(Box<EDLogShipyardTransfer>),
    ShipyardNew(EDLogShipyardNew),
    Shipyard(EDLogShipyard),
    ShipyardBuy(Box<EDLogShipyardBuy>),
    ShipyardSell(EDLogShipyardSell),
    StoredModules(EDLogStoredModules),
    StoredShips(Box<EDLogStoredShips>),
    TechnologyBroker(Box<EDLogTechnologyBroker>),
    ClearImpound(EDLogClearImpound),

    // Powerplay
    PowerplayCollect(EDLogPowerplayCollect),
    PowerplayDefect(EDLogPowerplayDefect),
    PowerplayDeliver(EDLogPowerplayDeliver),
    PowerplayJoin(EDLogPowerplayJoin),
    PowerplayLeave(EDLogPowerplayLeave),
    PowerplayFastTrack(EDLogPowerplayFastTrack),
    PowerplaySalary(EDLogPowerplaySalary),
    PowerplayVote(EDLogPowerplayVote),
    PowerplayVoucher(EDLogPowerplayVoucher),
    // not in manual, but found in logs
    PowerplayMerits(EDLogPowerplayMerits),
    PowerplayRank(EDLogPowerplayRank),
    HoloscreenHacked(EDLogHoloscreenHacked),

    // Squadrons
    AppliedToSquadron(EDLogAppliedToSquadron),
    DisbandedSquadron(EDLogDisbandedSquadron),
    InvitedToSquadron(EDLogInvitedToSquadron),
    JoinedSquadron(EDLogJoinedSquadron),
    KickedFromSquadron(EDLogKickedFromSquadron),
    LeftSquadron(EDLogLeftSquadron),
    SharedBookmarkToSquadron(EDLogSharedBookmarkToSquadron),
    SquadronCreated(EDLogSquadronCreated),
    SquadronDemotion(EDLogSquadronDemotion),
    SquadronPromotion(EDLogSquadronPromotion),
    SquadronStartup(EDLogSquadronStartup),
    WonATrophyForSquadron(EDLogWonATrophyForSquadron),

    // FleetCarrier
    CarrierJump(Box<EDLogCarrierJump>),
    CarrierBuy(EDLogCarrierBuy),
    CarrierStats(Box<EDLogCarrierStats>),
    CarrierJumpRequest(EDLogCarrierJumpRequest),
    CarrierDecommission(EDLogCarrierDecommission),
    CarrierCancelDecommission(EDLogCarrierCancelDecommission),
    CarrierBankTransfer(EDLogCarrierBankTransfer),
    CarrierDepositFuel(EDLogCarrierDepositFuel),
    CarrierCrewServices(EDLogCarrierCrewServices),
    CarrierFinance(EDLogCarrierFinance),
    CarrierShipPack(EDLogCarrierShipPack),
    CarrierModulePack(EDLogCarrierModulePack),
    CarrierTradeOrder(EDLogCarrierTradeOrder),
    CarrierDockingPermission(EDLogCarrierDockingPermission),
    CarrierNameChange(EDLogCarrierNameChange),
    CarrierJumpCancelled(EDLogCarrierJumpCancelled),

    // Odyssey
    Backpack(Box<EDLogBackpack>),
    BackpackChange(EDLogBackpackChange),
    BookDropship(EDLogBookDropship),
    BookTaxi(EDLogBookTaxi),
    BuyMicroResources(EDLogBuyMicroResources),
    BuyWeapon(EDLogBuyWeapon),
    BuySuit(EDLogBuySuit),
    CancelTaxi(EDLogCancelTaxi),
    CancelDropship(EDLogCancelDropship),
    CollectItems(EDLogCollectItems),
    CreateSuitLoadout(EDLogCreateSuitLoadout),
    Embark(EDLogEmbarkOrDisembark),
    Disembark(EDLogEmbarkOrDisembark),
    DropItems(EDLogDropItems),
    DropshipDeploy(EDLogDropshipDeploy),
    FCMaterials(EDLogFCMaterials),
    LoadoutEquipModule(EDLogLoadoutEquipModule),
    LoadoutRemoveModule(EDLogLoadoutRemoveModule),
    RenameSuitLoadout(EDLogRenameSuitLoadout),
    ScanOrganic(EDLogScanOrganic),
    SellMicroResources(EDLogSellMicroResources),
    SellOrganicData(EDLogSellOrganicData),
    SellSuit(EDLogSellSuit),
    SellWeapon(EDLogSellWeapon),
    ShipLocker(Box<EDLogShipLocker>),
    SwitchSuitLoadout(EDLogSuitLoadout),
    TradeMicroResources(EDLogTradeMicroResources),
    UpgradeSuit(EDLogUpgradeSuit),
    UpgradeWeapon(EDLogUpgradeWeapon),
    UseConsumable(EDLogUseConsumable),

    // Other
    AfmuRepairs(EDLogAfmuRepairs),
    ApproachSettlement(Box<EDLogApproachSettlement>),
    CockpitBreached,
    ChangeCrewRole(EDLogChangeCrewRole),
    CommitCrime(EDLogCommitCrime),
    Continued(EDLogContinued),
    CrewLaunchFighter(EDLogCrewLaunchFighter),
    CrimeVictim(EDLogCrimeVictim),
    DatalinkScan(EDLogDatalinkScan),
    CrewMemberRoleChange(EDLogCrewMemberRoleChange),
    CrewMemberJoins(EDLogCrewMemberJoins),
    CrewMemberQuits(EDLogCrewMemberQuits),
    DataScanned(EDLogDataScanned),
    DatalinkVoucher(EDLogDatalinkVoucher),
    DockFighter(EDLogDockFighter),
    DockSRV(EDLogDockSRV),
    EndCrewSession(EDLogEndCrewSession),
    Friends(EDLogFriends),
    FuelScoop(EDLogFuelScoop),
    JetConeBoost(EDLogJetConeBoost),
    JetConeDamage(EDLogJetConeDamage),
    FighterRebuilt(EDLogFighterRebuilt),
    JoinACrew(EDLogJoinACrew),
    KickCrewMember(EDLogKickCrewMember),
    LaunchDrone(EDLogLaunchDrone),
    LaunchSRV(EDLogLaunchSRV),
    LaunchFighter(EDLogLaunchFighter),
    ModuleInfo(EDLogModuleInfo),
    Music(EDLogMusic),
    NpcCrewPaidWage(EDLogNpcCrewPaidWage),
    NpcCrewRank(EDLogNpcCrewRank),
    Promotion(EDLogPromotion),
    ProspectedAsteroid(EDLogProspectedAsteroid),
    QuitACrew(EDLogQuitACrew),
    RebootRepair(EDLogRebootRepair),
    ReceiveText(EDLogReceiveText),
    RepairDrone(EDLogRepairDrone),
    ReservoirReplenished(EDLogReservoirReplenished),
    SelfDestruct,
    Shutdown,
    Resurrect(EDLogResurrect),
    Scanned(EDLogScanned),
    Synthesis(EDLogSynthesis),
    SendText(EDLogSendText),
    SystemsShutdown,
    USSDrop(EDLogUSSDrop),
    WingAdd(EDLogName),
    VehicleSwitch(EDLogVehicleSwitch),
    WingJoin(EDLogWingJoin),
    WingLeave,
    WingInvite(EDLogName),
    CargoTransfer(EDLogCargoTransfer),
    SupercruiseDestinationDrop(EDLogSupercruiseDestinationDrop),

    // Status
    Status(Box<EDLogStatus>),

    GameModeChange(EDLogGameModeChange),

    // Commander
    RequestPowerMicroResources(EDLogRequestPowerMicroResources),
    CarrierLocation(EDLogCarrierLocation),

    // Modules
    ModuleBuyAndStore(EDLogModuleBuyAndStore),

    // Navigation
    LaunchVessel(EDLogLaunchVessel),

    // Exploration (old)
    StationBernalSphere(EDLogStationBernalSphere),

    // Market
    MarketID(EDLogMarketID),
    DeliverPowerMicroResources(EDLogDeliverPowerMicroResources),
    ColonisationConstructionDepot(EDLogColonisationConstructionDepot),
    ColonisationContribution(EDLogColonisationContribution),

    // Shipyard
    ShipyardRedeem(EDLogShipyardRedeem),
    ShipRedeemed(EDLogShipRedeemed),

    // Suits and backpack
    SuitLoadout(EDLogSuitLoadout),
    DeleteSuitLoadout(EDLogDeleteSuitLoadout),

    // Ship
    Resupply,
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
        assert_eq!(header.gameversion.as_str(), "4.1.0.100");
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
        assert_eq!(header.message.as_str(), "$CruiseLiner_SCPatrol05;");
    }
}
#[test]
fn test_optional_loadgame() {
    let json = r#"{
        "timestamp":"2025-11-30T20:10:08Z", "event":"LoadGame", "FID":"F1234567", "Commander":"MySelf", 
        "Horizons":true, "Odyssey":true, 
        "Ship":"Python_NX", "Ship_Localised":"Python Mk II", "ShipID":99, "ShipName":"myship", "ShipIdent":"IDNT-12", 
        "FuelLevel":16.000000, "FuelCapacity":16.000000, "GameMode":"Solo", "Credits":123456, "Loan":0, 
        "language":"English/UK", "gameversion":"4.2.2.1", "build":"r321306/r0 " }"#;
    let line: EDLogLine = serde_json::from_str(json).expect("Should parse");

    assert!(matches!(line.event(), EDLogEvent::LoadGame(_)));

    if let EDLogEvent::LoadGame(content) = line.event() {
        assert!(content.fid.is_some(), "data has a FID");
        assert_eq!(content.fid.unwrap().as_str(), "F1234567");
        assert!(content.ship.is_some(), "data should contain a ship");
    }
}
