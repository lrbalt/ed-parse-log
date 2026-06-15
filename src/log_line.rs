use crate::{
    EDString,
    codex::EDLogCodexEntry,
    commander::{
        EDLogAppliedToSquadron, EDLogChangeCrewRole, EDLogCommander, EDLogCommitCrime,
        EDLogCrewAssign, EDLogCrewFire, EDLogCrewHire, EDLogCrewMemberJoins, EDLogCrewMemberQuits,
        EDLogCrewMemberRoleChange, EDLogCrimeVictim, EDLogDied, EDLogEmbarkOrDisembark,
        EDLogEndCrewSession, EDLogFriends, EDLogInvitedToSquadron, EDLogJoinACrew,
        EDLogNewCommander, EDLogPromotion, EDLogQuitACrew, EDLogRank, EDLogReputation,
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
        EDLogAsteroidCracked, EDLogBuyExplorationData, EDLogDataScanned, EDLogDatalinkScan,
        EDLogDiscoveryScan, EDLogFSSAllBodiesFound, EDLogFSSBodySignals, EDLogFSSDiscoveryScan,
        EDLogFSSSignalDiscovered, EDLogMaterialDiscovered, EDLogMiningRefined,
        EDLogMultiSellExplorationData, EDLogNavBeaconScan, EDLogProspectedAsteroid,
        EDLogSAAScanComplete, EDLogSAASignalsFound, EDLogScan, EDLogScanBaryCentre,
        EDLogScanOrganic, EDLogScanned, EDLogSellExplorationData, EDLogStationBernalSphere,
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
        EDLogSellMicroResources, EDLogSellOrganicData, EDLogTradeMicroResources, MarketItemType,
    },
    materials::EDLogMaterials,
    mission::{
        EDLogBounty, EDLogCapitalShipBond, EDLogDatalinkVoucher, EDLogFactionKillBond,
        EDLogMissionAbandoned, EDLogMissionAccepted, EDLogMissionCompleted, EDLogMissionFailed,
        EDLogMissionRedirected, EDLogMissions, EDLogPVPKill, EDLogPassengers, EDLogRedeemVoucher,
        EDLogScientificResearch, EDLogSearchAndRescue,
    },
    modules::{
        EDLogFetchRemoteModule, EDLogMassModuleStore, EDLogModuleBuy, EDLogModuleBuyAndStore,
        EDLogModuleRetrieve, EDLogModuleSell, EDLogModuleSellRemote, EDLogModuleStore,
        EDLogModuleSwap, EDLogStoredModules,
    },
    navigation::{
        EDLogApproachSettlement, EDLogDockSRV, EDLogFSDJump, EDLogFSDTarget, EDLogFuelScoop,
        EDLogJetConeBoost, EDLogJetConeDamage, EDLogLaunchSRV, EDLogLiftoff, EDLogSRVDestroyed,
        EDLogStartJump, EDLogTouchdown,
    },
    powerplay::{
        EDLogHoloscreenHacked, EDLogPowerplay, EDLogPowerplayCollect, EDLogPowerplayDefect,
        EDLogPowerplayDeliver, EDLogPowerplayFastTrack, EDLogPowerplayJoin, EDLogPowerplayLeave,
        EDLogPowerplayMerits, EDLogPowerplayRank, EDLogPowerplaySalary,
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
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumDiscriminants, EnumIter};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameMode {
    Group,
    Solo,
    Open,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({"Ship":"CobraMkIII", "ShipID":1, "ShipName":"Flat Head", "ShipIdent":"UNSC-1"})]
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
#[serde(rename_all = "PascalCase")]
#[testcase({ "timestamp":"2017-10-14T18:41:37Z", "event":"LoadGame", "Commander":"JournalServer", "Ship":"CobraMkIII", "ShipID":1, "ShipName":"Flat Head", "ShipIdent":"UNSC-1", "FuelLevel":16.000000, "FuelCapacity":16.000000, "GameMode":"Open", "Credits":766731, "Loan":0 })]
#[testcase({ "timestamp":"2022-11-10T18:50:06Z", "event":"LoadGame", "FID":"F1234567", "Commander":"Myself", "Horizons":true, "Odyssey":true, "Credits":1234431, "Loan":0, "language":"English/UK", "gameversion":"4.0.0.1450", "build":"r286858/r0 " })]
#[testcase({ "timestamp":"2022-09-12T18:45:38Z", "event":"LoadGame", "FID":"F1234567", "Commander":"MySelf", "Horizons":true, "Ship":"FerDeLance", "Ship_Localised":"Fer-de-Lance", "ShipID":34, "ShipName":"", "ShipIdent":"", "FuelLevel":7.689338, "FuelCapacity":8.000000, "GameMode":"Group", "Group":"REINIER", "Credits":123321, "Loan":0 })]
#[testcase({ "timestamp":"2025-11-30T20:10:08Z", "event":"LoadGame", "FID":"F1234567", "Commander":"MySelf", "Horizons":true, "Odyssey":true, "Ship":"Python_NX", "Ship_Localised":"Python Mk II", "ShipID":12, "ShipName":"MyName", "ShipIdent":"IDENT1", "FuelLevel":16.000000, "FuelCapacity":16.000000, "GameMode":"Solo", "Credits":12341234, "Loan":0, "language":"English/UK", "gameversion":"4.2.2.1", "build":"r321306/r0 " })]
pub struct EDLogLoadGame {
    #[serde(rename = "FID")]
    pub fid: Option<EDString>,
    pub commander: EDString,
    pub horizons: Option<bool>,
    pub odyssey: Option<bool>,
    #[serde(flatten)]
    pub ship: Option<LoadGameShip>,
    pub credits: Credits,
    pub loan: Credits,
    #[serde(rename = "language")]
    pub language: Option<EDString>,
    #[serde(rename = "gameversion")]
    pub gameversion: Option<EDString>,
    #[serde(rename = "build")]
    pub build: Option<EDString>,
    pub game_mode: Option<GameMode>,
    pub group: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMusic {
    music_track: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct FuelStatus {
    fuel_main: f64,
    fuel_reservoir: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Destination {
    system: u64,
    body: u64,
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LegalState {
    Clean,
    IllegalCargo,
    Speeding,
    Wanted,
    Hostile,
    PassengerWanted,
}

/*

TODO:
Flags:
Bit Value      Hex       Meaning
0   1          0000 0001 Docked, (on a landing pad)
1   2          0000 0002 Landed, (on planet surface)
2   4          0000 0004 Landing Gear Down
3   8          0000 0008 Shields Up
4   16         0000 0010 Supercruise
5   32         0000 0020 FlightAssist Off
6   64         0000 0040 Hardpoints Deployed
7   128        0000 0080 In Wing
8   256        0000 0100 LightsOn
9   512        0000 0200 Cargo Scoop Deployed
10  1024       0000 0400 Silent Running,
11  2048       0000 0800 Scooping Fuel
12  4096       0000 1000 Srv Handbrake
13  8192       0000 2000 Srv using Turret view
14  16384      0000 4000 Srv Turret retracted (close to ship)
15  32768      0000 8000 Srv DriveAssist
16  65536      0001 0000 Fsd MassLocked
17  131072     0002 0000 Fsd Charging
18  262144     0004 0000 Fsd Cooldown
19  524288     0008 0000 Low Fuel ( < 25% )
20  1048576    0010 0000 Over Heating ( > 100% )
21  2097152    0020 0000 Has Lat Long
22  4194304    0040 0000 IsInDanger
23  8388608    0080 0000 Being Interdicted
24  16777216   0100 0000 In MainShip
25  33554432   0200 0000 In Fighter
26  67108864   0400 0000 In SRV
27  134217728  0800 0000 Hud in Analysis mode
28  268435456  1000 0000 Night Vision
29  536870912  2000 0000 Altitude from Average radius
30  1073741824 4000 0000 fsdJump
31  2147483648 8000 0000 srvHighBeam

Flags2:
Bit Value      Hex       Meaning
0   1          0000 0001 OnFoot
1   2          0000 0002 InTaxi (or dropship/shuttle)
2   4          0000 0004 InMulticrew (ie in someone else’s ship)
3   8          0000 0008 OnFootInStation
4   16         0000 0010 OnFootOnPlanet
5   32         0000 0020 AimDownSight
6   64         0000 0040 LowOxygen
7   128        0000 0080 LowHealth
8   256        0000 0100 Cold
9   512        0000 0200 Hot
10  1024       0000 0400 VeryCold
11  2048       0000 0800 VeryHot
12  4096       0000 1000 Glide Mode
13  8192       0000 2000 OnFootInHangar
14  16384      0000 4000 OnFootSocialSpace
15  32768      0000 8000 OnFootExterior
16  65536      0001 0001 BreathableAtmosphere
17  131072     0002 0000 Telepresence Multicrew
18  262144     0004 0000 Physical Multicrew
19  524288     0008 0000 Fsd hyperdrive charging
20  1048576    0010 0000 Supercruise Overcharge
21  2097152    0020 0000 Supercruise Assist
22  4194304    0040 0000 NPC Crew Active

Guifocus

0  NoFocus
1  InternalPanel (right hand side)
2  ExternalPanel (left hand side)
3  CommsPanel (top)
4  RolePanel (bottom)
5  StationServices
6  GalaxyMap
7  SystemMap
8  Orrery
9  FSS mode
10 SAA mode
11 Codex

*/

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-12-07T10:31:37Z", "event":"Status", "Flags":16842765, "Pips":[2,8,2], "FireGroup":0, 
    "Fuel":{ "FuelMain":15.146626, "FuelReservoir":0.382796 }, "GuiFocus":5 })]
#[testcase({ "timestamp":"2017-12-07T12:03:14Z", "event":"Status", "Flags":18874376, "Pips":[4,8,0], "FireGroup":0,
    "Fuel":{ "FuelMain":15.146626, "FuelReservoir":0.382796 }, "GuiFocus":0, "Latitude":-28.584963,
    "Longitude":6.826313, "Heading":109, "Altitude": 404 })]
#[testcase({ "timestamp":"2026-05-15T12:51:43Z", "event":"Status", "Flags":0 })]
#[testcase({ "timestamp":"2026-06-06T13:01:47Z", "event":"Status", "Flags":151060485, "Flags2":0, "Pips":[2,8,2], 
    "FireGroup":0, "GuiFocus":0, "Fuel":{ "FuelMain":128.000000, "FuelReservoir":1.110000 }, "Cargo":1324.000000, 
    "LegalState":"Hostile", "Balance":28304956592, "Destination":{ "System":2869441275273, "Body":33, 
    "Name":"Val-rasha Starport" } })]
pub struct EDLogStatus {
    flags: u64,
    flags2: Option<u64>,
    pips: Option<[u8; 3]>,
    fire_group: Option<u64>,
    fuel: Option<FuelStatus>,
    gui_focus: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    heading: Option<u64>,
    altitude: Option<f64>,
    cargo: Option<f64>,
    legal_state: Option<LegalState>,
    balance: Option<Credits>,
    oxigen: Option<f64>,
    health: Option<f64>,
    temperature: Option<f64>,
    selected_weapon: Option<EDString>,
    #[serde(rename = "SelectedWeapon_Localised")]
    selected_weapon_localised: Option<EDString>,
    body_name: Option<EDString>,
    destination: Option<Destination>,
    planet_radius: Option<f64>,
    gravity: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2023-07-30T14:40:36Z", "event":"ReceiveText", "From":"$ShipName_PassengerLiner_Cruise;", 
             "From_Localised":"Cruise Ship", "Message":"$CruiseLiner_SCPatrol05;", 
             "Message_Localised":"This is your captain. Due to some unforeseen delays, we will be arriving at our next destination later than scheduled.", 
             "Channel":"npc" })]
pub struct EDLogReceiveText {
    from: EDString,
    #[serde(rename = "From_Localised")]
    from_localised: Option<EDString>,
    message: EDString,
    #[serde(rename = "Message_Localised")]
    message_localised: Option<EDString>,
    channel: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSendText {
    to: String,
    message: EDString,
    sent: bool,
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
    uss_type_localised: EDString,
    #[serde(rename = "USSThreat")]
    uss_threat: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(deny_unknown_fields)]
pub struct EDLogCollectCargo {
    #[serde(rename = "Type")]
    cargo_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    cargo_type_localised: Option<EDString>,
    #[serde(rename = "Stolen")]
    stolen: bool,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CargoTransfer {
    #[serde(rename = "Type")]
    cargo_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    cargo_type_localised: Option<EDString>,
    count: u32,
    direction: EDString,
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
    filename: EDString,
    width: u64,
    height: u64,
    system: Option<EDString>,
    body: Option<EDString>,
    #[serde(flatten)]
    location_on_body: Option<LocationOnBody>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display, EnumDiscriminants)]
#[serde(tag = "event", deny_unknown_fields)]
#[strum_discriminants(derive(EnumIter, Display))]
pub enum EDLogEvent {
    #[serde(rename = "Fileheader")]
    FileHeader(EDLogFileHeader),
    LoadGame(Box<EDLogLoadGame>),
    SystemsShutdown,
    Shutdown,
    Music(EDLogMusic),
    Status(Box<EDLogStatus>),

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
    ShipLocker(Box<EDLogShipLocker>),

    // Materials
    Materials(EDLogMaterials),

    // Statistics
    // EDLogStatistics is boxed to reduce the size of EDLogEvent
    Statistics(Box<EDLogStatistics>),

    // Engineers
    EngineerProgress(EDLogEngineerProgress),
    EngineerContribution(EDLogEngineerContribution),
    EngineerCraft(Box<EDLogEngineerCraft>),

    // Docking
    DockingRequested(EDLogDockingRequested),
    DockingCancelled(EDLogDockingCancelled),
    DockingGranted(EDLogDockingGranted),
    DockingDenied(EDLogDockingDenied),
    DockingTimeout(EDLogDockingTimeout),
    Docked(Box<EDLogDocked>),
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
    FSDJump(Box<EDLogFSDJump>),
    FuelScoop(EDLogFuelScoop),
    ApproachSettlement(Box<EDLogApproachSettlement>),
    ApproachBody(BodyInformation),
    LeaveBody(BodyInformation),
    Liftoff(EDLogLiftoff),
    LaunchSRV(EDLogLaunchSRV),
    DockSRV(EDLogDockSRV),
    SRVDestroyed(EDLogSRVDestroyed),
    Touchdown(EDLogTouchdown),
    StartJump(EDLogStartJump),
    JetConeBoost(EDLogJetConeBoost),
    JetConeDamage(EDLogJetConeDamage),

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
    Scan(Box<EDLogScan>),
    Scanned(EDLogScanned),
    DiscoveryScan(EDLogDiscoveryScan),
    ScanOrganic(EDLogScanOrganic),
    DatalinkScan(EDLogDatalinkScan),
    NavBeaconScan(EDLogNavBeaconScan),
    SellOrganicData(EDLogSellOrganicData),
    CodexEntry(Box<EDLogCodexEntry>),
    ProspectedAsteroid(EDLogProspectedAsteroid),
    MiningRefined(EDLogMiningRefined),
    AsteroidCracked(EDLogAsteroidCracked),
    MaterialDiscovered(EDLogMaterialDiscovered),
    DataScanned(EDLogDataScanned),
    BuyExplorationData(EDLogBuyExplorationData),
    SellExplorationData(EDLogSellExplorationData),

    // FleetCarrier
    CarrierStats(Box<EDLogCarrierStats>),
    CarrierJumpRequest(EDLogCarrierJumpRequest),
    CarrierBankTransfer(EDLogCarrierBankTransfer),
    CarrierFinance(EDLogCarrierFinance),
    CarrierTradeOrder(EDLogCarrierTradeOrder),
    CarrierJump(Box<EDLogCarrierJump>),
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
    MissionAccepted(Box<EDLogMissionAccepted>),
    MissionRedirected(EDLogMissionRedirected),
    MissionCompleted(Box<EDLogMissionCompleted>),
    MissionFailed(EDLogMissionFailed),
    MissionAbandoned(EDLogMissionAbandoned),
    SearchAndRescue(EDLogSearchAndRescue),
    Bounty(Box<EDLogBounty>),
    RedeemVoucher(EDLogRedeemVoucher),
    DatalinkVoucher(EDLogDatalinkVoucher),
    FactionKillBond(EDLogFactionKillBond),
    #[serde(rename = "CapShipBond")]
    CapitalShipBond(EDLogCapitalShipBond),
    PVPKill(EDLogPVPKill),
    Passengers(EDLogPassengers),
    ScientificResearch(EDLogScientificResearch),

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
    Backpack(Box<EDLogBackpack>),
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
    Location(Box<EDLogLocation>),

    // Loadout
    Loadout(Box<EDLogLoadout>),

    // Ship
    Cargo(EDLogCargo),
    ShieldState(EDLogShieldState),
    EjectCargo(EDLogEjectCargo),
    ReservoirReplenished(EDLogReservoirReplenished),
    ShipTargeted(Box<EDLogShipTargeted>),
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
