use crate::{EDString, common_types::Credits};
use ed_parse_log_files_macros::{Extractable, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StatisticsBankAccountOnFoot {
    #[serde(rename = "Spent_On_Suits")]
    spent_on_suits: Credits,
    #[serde(rename = "Spent_On_Weapons")]
    spent_on_weapons: Credits,
    #[serde(rename = "Spent_On_Suit_Consumables")]
    spent_on_suit_consumables: Credits,
    #[serde(rename = "Suits_Owned")]
    suits_owned: u64,
    #[serde(rename = "Weapons_Owned")]
    weapons_owned: u64,
    #[serde(rename = "Spent_On_Premium_Stock")]
    spent_on_premium_stock: Credits,
    #[serde(rename = "Premium_Stock_Bought")]
    premium_stock_bought: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Current_Wealth":12341234, "Spent_On_Ships":12341234, "Spent_On_Outfitting":12341234, 
    "Spent_On_Repairs":12341234, "Spent_On_Fuel":1253673, "Spent_On_Ammo_Consumables":12341234, "Insurance_Claims":88, 
    "Spent_On_Insurance":12341234, "Owned_Ship_Count":26, "Spent_On_Suits":12341234, "Spent_On_Weapons":12341234, 
    "Spent_On_Suit_Consumables":123443, "Suits_Owned":7, "Weapons_Owned":11, "Spent_On_Premium_Stock":12341234, 
    "Premium_Stock_Bought":17 })]
pub struct StatisticsBankAccount {
    #[serde(rename = "Current_Wealth")]
    pub current_wealth: Credits,
    #[serde(rename = "Spent_On_Ships")]
    pub spent_on_ships: Credits,
    #[serde(rename = "Spent_On_Outfitting")]
    pub spent_on_outfitting: Credits,
    #[serde(rename = "Spent_On_Repairs")]
    pub spent_on_repairs: Credits,
    #[serde(rename = "Spent_On_Fuel")]
    pub spent_on_fuel: Credits,
    #[serde(rename = "Spent_On_Ammo_Consumables")]
    pub spent_on_ammo_consumables: Credits,
    #[serde(rename = "Insurance_Claims")]
    pub insurance_claims: Credits,
    #[serde(rename = "Spent_On_Insurance")]
    pub spent_on_insurance: Credits,
    #[serde(rename = "Owned_Ship_Count")]
    pub owned_ship_count: Credits,
    #[serde(flatten)]
    pub onfoot_statistics: Option<StatisticsBankAccountOnFoot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StatisticsCombatOnFoot {
    #[serde(rename = "OnFoot_Combat_Bonds")]
    onfoot_combat_bonds: u64,
    #[serde(rename = "OnFoot_Combat_Bonds_Profits")]
    onfoot_combat_bonds_profits: Credits,
    #[serde(rename = "OnFoot_Vehicles_Destroyed")]
    onfoot_vehicles_destroyed: u64,
    #[serde(rename = "OnFoot_Ships_Destroyed")]
    onfoot_ships_destroyed: u64,
    #[serde(rename = "Dropships_Taken")]
    dropships_taken: u64,
    #[serde(rename = "Dropships_Booked")]
    dropships_booked: u64,
    #[serde(rename = "Dropships_Cancelled")]
    dropships_cancelled: u64,
    #[serde(rename = "ConflictZone_High")]
    conflictzone_high: u64,
    #[serde(rename = "ConflictZone_Medium")]
    conflictzone_medium: u64,
    #[serde(rename = "ConflictZone_Low")]
    conflictzone_low: u64,
    #[serde(rename = "ConflictZone_Total")]
    conflictzone_total: u64,
    #[serde(rename = "ConflictZone_High_Wins")]
    conflictzone_high_wins: u64,
    #[serde(rename = "ConflictZone_Medium_Wins")]
    conflictzone_medium_wins: u64,
    #[serde(rename = "ConflictZone_Low_Wins")]
    conflictzone_low_wins: u64,
    #[serde(rename = "ConflictZone_Total_Wins")]
    conflictzone_total_wins: u64,
    #[serde(rename = "Settlement_Defended")]
    settlement_defended: u64,
    #[serde(rename = "Settlement_Conquered")]
    settlement_conquered: u64,
    #[serde(rename = "OnFoot_Skimmers_Killed")]
    onfoot_skimmers_killed: u64,
    #[serde(rename = "OnFoot_Scavs_Killed")]
    onfoot_scavs_killed: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Bounties_Claimed":9450, "Bounty_Hunting_Profit":12341234, "Combat_Bonds":12341234, 
    "Combat_Bond_Profits":12341234, "Assassinations":284, "Assassination_Profits":12341234, 
    "Highest_Single_Reward":2107790, "Skimmers_Killed":162, "OnFoot_Combat_Bonds":14468, 
    "OnFoot_Combat_Bonds_Profits":12341234, "OnFoot_Vehicles_Destroyed":0, "OnFoot_Ships_Destroyed":0, 
    "Dropships_Taken":34, "Dropships_Booked":71, "Dropships_Cancelled":1, "ConflictZone_High":387, 
    "ConflictZone_Medium":40, "ConflictZone_Low":38, "ConflictZone_Total":465, "ConflictZone_High_Wins":387, 
    "ConflictZone_Medium_Wins":40, "ConflictZone_Low_Wins":38, "ConflictZone_Total_Wins":465, 
    "Settlement_Defended":33, "Settlement_Conquered":7, "OnFoot_Skimmers_Killed":71, "OnFoot_Scavs_Killed":384 })]
#[testcase_struct({ "Bounties_Claimed":18741, "Bounty_Hunting_Profit":1390383055.75, "Combat_Bonds":6279, 
    "Combat_Bond_Profits":69816712, "Assassinations":138, "Assassination_Profits":57761837, "Highest_Single_Reward":2084395, 
    "Skimmers_Killed":28, "OnFoot_Combat_Bonds":54907, "OnFoot_Combat_Bonds_Profits":6931746150, "OnFoot_Vehicles_Destroyed":0, 
    "OnFoot_Ships_Destroyed":0, "Dropships_Taken":0, "Dropships_Booked":2, "Dropships_Cancelled":2, "ConflictZone_High":1700, 
    "ConflictZone_Medium":64, "ConflictZone_Low":6, "ConflictZone_Total":1770, "ConflictZone_High_Wins":1692, 
    "ConflictZone_Medium_Wins":62, "ConflictZone_Low_Wins":6, "ConflictZone_Total_Wins":1760, "Settlement_Defended":44, 
    "Settlement_Conquered":18, "OnFoot_Skimmers_Killed":218, "OnFoot_Scavs_Killed":918 })]
pub struct StatisticsCombat {
    #[serde(rename = "Bounties_Claimed")]
    bounties_claimed: u64,
    #[serde(rename = "Bounty_Hunting_Profit")]
    bounty_hunting_profit: f64, // TODO: only instance of Credits where f64 is used
    #[serde(rename = "Combat_Bonds")]
    combat_bonds: u64,
    #[serde(rename = "Combat_Bond_Profits")]
    combat_bond_profits: Credits,
    #[serde(rename = "Assassinations")]
    assassinations: u64,
    #[serde(rename = "Assassination_Profits")]
    assassination_profits: Credits,
    #[serde(rename = "Highest_Single_Reward")]
    highest_single_reward: Credits,
    #[serde(rename = "Skimmers_Killed")]
    skimmers_killed: u64,
    #[serde(flatten)]
    onfoot_combat_statistics: Option<StatisticsCombatOnFoot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StatisticsCrimeOnFoot {
    #[serde(rename = "Malware_Uploaded")]
    malware_uploaded: u64,
    #[serde(rename = "Settlements_State_Shutdown")]
    settlements_state_shutdown: u64,
    #[serde(rename = "Production_Sabotage")]
    production_sabotage: u64,
    #[serde(rename = "Production_Theft")]
    production_theft: u64,
    #[serde(rename = "Total_Murders")]
    total_murders: u64,
    #[serde(rename = "Citizens_Murdered")]
    citizens_murdered: u64,
    #[serde(rename = "Omnipol_Murdered")]
    omnipol_murdered: u64,
    #[serde(rename = "Guards_Murdered")]
    guards_murdered: u64,
    #[serde(rename = "Data_Stolen")]
    data_stolen: u64,
    #[serde(rename = "Goods_Stolen")]
    goods_stolen: u64,
    #[serde(rename = "Sample_Stolen")]
    sample_stolen: u64,
    #[serde(rename = "Total_Stolen")]
    total_stolen: u64,
    #[serde(rename = "Turrets_Destroyed")]
    turrets_destroyed: u64,
    #[serde(rename = "Turrets_Overloaded")]
    turrets_overloaded: u64,
    #[serde(rename = "Turrets_Total")]
    turrets_total: u64,
    #[serde(rename = "Value_Stolen_StateChange")]
    value_stolen_state_change: Credits,
    #[serde(rename = "Profiles_Cloned")]
    profiles_cloned: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Notoriety":0, "Fines":227, "Total_Fines":123412, "Bounties_Received":513, 
    "Total_Bounties":1234123, "Highest_Bounty":123412, "Malware_Uploaded":0, "Settlements_State_Shutdown":7, 
    "Production_Sabotage":0, "Production_Theft":127, "Total_Murders":919, "Citizens_Murdered":443, 
    "Omnipol_Murdered":1, "Guards_Murdered":475, "Data_Stolen":496, "Goods_Stolen":1234, "Sample_Stolen":3, 
    "Total_Stolen":1234, "Turrets_Destroyed":0, "Turrets_Overloaded":0, "Turrets_Total":0, 
    "Value_Stolen_StateChange":1234, "Profiles_Cloned":152 })]
pub struct StatisticsCrime {
    #[serde(rename = "Notoriety")]
    notoriety: u64,
    #[serde(rename = "Fines")]
    fines: u64,
    #[serde(rename = "Total_Fines")]
    total_fines: Credits,
    #[serde(rename = "Bounties_Received")]
    bounties_received: u64,
    #[serde(rename = "Total_Bounties")]
    total_bounties: Credits,
    #[serde(rename = "Highest_Bounty")]
    highest_bounty: Credits,
    #[serde(flatten)]
    statistics_on_foot: Option<StatisticsCrimeOnFoot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Black_Markets_Traded_With":22, "Black_Markets_Profits":123412, "Resources_Smuggled":143, 
    "Average_Profit":12344.217391304, "Highest_Single_Transaction":139902 })]
pub struct StatisticsSmuggling {
    #[serde(rename = "Black_Markets_Traded_With")]
    black_markets_traded_with: u64,
    #[serde(rename = "Black_Markets_Profits")]
    black_markets_profits: Credits,
    #[serde(rename = "Resources_Smuggled")]
    resources_smuggled: u64,
    #[serde(rename = "Average_Profit")]
    average_profit: f64, // TODO: Credits<f64>
    #[serde(rename = "Highest_Single_Transaction")]
    highest_single_transaction: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StatisticsTradingOnFoot {
    #[serde(rename = "Data_Sold")]
    data_sold: u64,
    #[serde(rename = "Goods_Sold")]
    goods_sold: u64,
    #[serde(rename = "Assets_Sold")]
    assets_sold: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Markets_Traded_With":147, "Market_Profits":12341234, "Resources_Traded":199113, "Average_Profit":123412.0073607, 
    "Highest_Single_Transaction":12341234, "Data_Sold":756, "Goods_Sold":3932, "Assets_Sold":4311 })]
pub struct StatisticsTrading {
    #[serde(rename = "Markets_Traded_With")]
    markets_traded_with: u64,
    #[serde(rename = "Market_Profits")]
    market_profits: Credits,
    #[serde(rename = "Resources_Traded")]
    resources_traded: u64,
    #[serde(rename = "Average_Profit")]
    average_profit: f64, // TODO: floating point credits type
    #[serde(rename = "Highest_Single_Transaction")]
    highest_single_transaction: Credits,
    #[serde(flatten)]
    trading_onfoot_statistics: Option<StatisticsTradingOnFoot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Mining_Profits":12341234, "Quantity_Mined":7988, "Materials_Collected":56783 })]
pub struct StatisticsMining {
    #[serde(rename = "Mining_Profits")]
    mining_profits: Credits,
    #[serde(rename = "Quantity_Mined")]
    quantity_mined: u64,
    #[serde(rename = "Materials_Collected")]
    materials_collected: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StatisticsExplorationOnFoot {
    #[serde(rename = "OnFoot_Distance_Travelled")]
    on_foot_distance_travelled: u64,
    #[serde(rename = "Shuttle_Journeys")]
    shuttle_journeys: u64,
    #[serde(rename = "Shuttle_Distance_Travelled")]
    shuttle_distance_travelled: f64,
    #[serde(rename = "Spent_On_Shuttles")]
    spent_on_shuttles: Credits,
    #[serde(rename = "First_Footfalls")]
    first_footfalls: u64,
    #[serde(rename = "Planet_Footfalls")]
    planet_footfalls: u64,
    #[serde(rename = "Settlements_Visited")]
    settlements_visited: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Systems_Visited":5576, "Exploration_Profits":12341234, "Planets_Scanned_To_Level_2":57981, 
    "Planets_Scanned_To_Level_3":57981, "Efficient_Scans":970, "Highest_Payout":1234123, "Total_Hyperspace_Distance":270553, 
    "Total_Hyperspace_Jumps":9839, "Greatest_Distance_From_Start":24111.762273172, "Time_Played":7045860, 
    "OnFoot_Distance_Travelled":1124559, "Shuttle_Journeys":26, "Shuttle_Distance_Travelled":297.87090093009, 
    "Spent_On_Shuttles":146647, "First_Footfalls":156, "Planet_Footfalls":403, "Settlements_Visited":124 })]
pub struct StatisticsExploration {
    #[serde(rename = "Systems_Visited")]
    systems_visited: u64,
    #[serde(rename = "Exploration_Profits")]
    exploration_profits: Credits,
    #[serde(rename = "Planets_Scanned_To_Level_2")]
    planets_scanned_to_level_2: u64,
    #[serde(rename = "Planets_Scanned_To_Level_3")]
    planets_scanned_to_level_3: u64,
    #[serde(rename = "Efficient_Scans")]
    efficient_scans: u64,
    #[serde(rename = "Highest_Payout")]
    highest_payout: Credits,
    #[serde(rename = "Total_Hyperspace_Distance")]
    total_hyperspace_distance: u64,
    #[serde(rename = "Total_Hyperspace_Jumps")]
    total_hyperspace_jumps: u64,
    #[serde(rename = "Greatest_Distance_From_Start")]
    greatest_distance_from_start: f64,
    #[serde(rename = "Time_Played")]
    time_played: u64,
    #[serde(flatten)]
    statistics_exploration_onfoot: Option<StatisticsExplorationOnFoot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Passengers_Missions_Accepted":13, "Passengers_Missions_Bulk":6, "Passengers_Missions_VIP":104, 
    "Passengers_Missions_Delivered":110, "Passengers_Missions_Ejected":0 })]
pub struct StatisticsPassengers {
    #[serde(rename = "Passengers_Missions_Accepted")]
    passengers_missions_accepted: Option<u64>,
    #[serde(rename = "Passengers_Missions_Bulk")]
    passengers_missions_bulk: u64,
    #[serde(rename = "Passengers_Missions_VIP")]
    passengers_missions_vip: u64,
    #[serde(rename = "Passengers_Missions_Delivered")]
    passengers_missions_delivered: u64,
    #[serde(rename = "Passengers_Missions_Ejected")]
    passengers_missions_ejected: u64,
    #[serde(rename = "Passengers_Missions_Disgruntled")]
    passengers_missions_disgruntled: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StatisticsSeachAndRescueOnFoot {
    #[serde(rename = "Salvage_Legal_POI")]
    salvage_legal_poi: u64,
    #[serde(rename = "Salvage_Legal_Settlements")]
    salvage_legal_settlements: u64,
    #[serde(rename = "Salvage_Illegal_POI")]
    salvage_illegal_poi: u64,
    #[serde(rename = "Salvage_Illegal_Settlements")]
    salvage_illegal_settlements: u64,
    #[serde(rename = "Maglocks_Opened")]
    maglocks_opened: u64,
    #[serde(rename = "Panels_Opened")]
    panels_opened: u64,
    #[serde(rename = "Settlements_State_FireOut")]
    settlements_state_fire_out: u64,
    #[serde(rename = "Settlements_State_Reboot")]
    settlements_state_reboot: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "SearchRescue_Traded":1388, "SearchRescue_Profit":43947801, "SearchRescue_Count":170, "Salvage_Legal_POI":5986700, 
"Salvage_Legal_Settlements":58208700, "Salvage_Illegal_POI":10200000, "Salvage_Illegal_Settlements":405000, "Maglocks_Opened":1092, 
"Panels_Opened":354, "Settlements_State_FireOut":951, "Settlements_State_Reboot":66 })]
pub struct StatisticSearchAndRescue {
    #[serde(rename = "SearchRescue_Traded")]
    search_rescue_traded: u64,
    #[serde(rename = "SearchRescue_Profit")]
    search_rescue_profit: Credits,
    #[serde(rename = "SearchRescue_Count")]
    search_rescue_count: u64,
    #[serde(flatten)]
    statistics_search_and_rescue_onfoot: Option<StatisticsSeachAndRescueOnFoot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Squadron_Bank_Credits_Deposited":0, "Squadron_Bank_Credits_Withdrawn":0, "Squadron_Bank_Commodities_Deposited_Num":0, 
    "Squadron_Bank_Commodities_Deposited_Value":0, "Squadron_Bank_Commodities_Withdrawn_Num":0, "Squadron_Bank_Commodities_Withdrawn_Value":0, 
    "Squadron_Bank_PersonalAssets_Deposited_Num":0, "Squadron_Bank_PersonalAssets_Deposited_Value":0, "Squadron_Bank_PersonalAssets_Withdrawn_Num":0, 
    "Squadron_Bank_PersonalAssets_Withdrawn_Value":0, "Squadron_Bank_Ships_Deposited_Num":0, "Squadron_Bank_Ships_Deposited_Value":0, 
    "Squadron_Leaderboard_aegis_highestcontribution":0, "Squadron_Leaderboard_bgs_highestcontribution":0, 
    "Squadron_Leaderboard_bounty_highestcontribution":0, "Squadron_Leaderboard_colonisation_contribution_highestcontribution":0, 
    "Squadron_Leaderboard_combat_highestcontribution":0, "Squadron_Leaderboard_cqc_highestcontribution":0, 
    "Squadron_Leaderboard_exploration_highestcontribution":0, "Squadron_Leaderboard_mining_highestcontribution":0, 
    "Squadron_Leaderboard_powerplay_highestcontribution":0, "Squadron_Leaderboard_trade_highestcontribution":0, 
    "Squadron_Leaderboard_trade_illicit_highestcontribution":0, "Squadron_Leaderboard_podiums":0 })]
pub struct StatisticsSquadron {
    #[serde(rename = "Squadron_Bank_Credits_Deposited")]
    squadron_bank_credits_deposited: Credits,
    #[serde(rename = "Squadron_Bank_Credits_Withdrawn")]
    squadron_bank_credits_withdrawn: Credits,
    #[serde(rename = "Squadron_Bank_Commodities_Deposited_Num")]
    squadron_bank_commodities_deposited_num: u64,
    #[serde(rename = "Squadron_Bank_Commodities_Deposited_Value")]
    squadron_bank_commodities_deposited_value: Credits,
    #[serde(rename = "Squadron_Bank_Commodities_Withdrawn_Num")]
    squadron_bank_commodities_withdrawn_num: u64,
    #[serde(rename = "Squadron_Bank_Commodities_Withdrawn_Value")]
    squadron_bank_commodities_withdrawn_value: Credits,
    #[serde(rename = "Squadron_Bank_PersonalAssets_Deposited_Num")]
    squadron_bank_personal_assets_deposited_num: u64,
    #[serde(rename = "Squadron_Bank_PersonalAssets_Deposited_Value")]
    squadron_bank_personal_assets_deposited_value: Credits,
    #[serde(rename = "Squadron_Bank_PersonalAssets_Withdrawn_Num")]
    squadron_bank_personal_assets_withdrawn_num: u64,
    #[serde(rename = "Squadron_Bank_PersonalAssets_Withdrawn_Value")]
    squadron_bank_personal_assets_withdrawn_value: Credits,
    #[serde(rename = "Squadron_Bank_Ships_Deposited_Num")]
    squadron_bank_ships_deposited_num: u64,
    #[serde(rename = "Squadron_Bank_Ships_Deposited_Value")]
    squadron_bank_ships_deposited_value: Credits,
    #[serde(rename = "Squadron_Leaderboard_aegis_highestcontribution")]
    squadron_leaderboard_aegis_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_bgs_highestcontribution")]
    squadron_leaderboard_bgs_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_bounty_highestcontribution")]
    squadron_leaderboard_bounty_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_colonisation_contribution_highestcontribution")]
    squadron_leaderboard_colonisation_contribution_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_combat_highestcontribution")]
    squadron_leaderboard_combat_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_cqc_highestcontribution")]
    squadron_leaderboard_cqc_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_exploration_highestcontribution")]
    squadron_leaderboard_exploration_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_mining_highestcontribution")]
    squadron_leaderboard_mining_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_powerplay_highestcontribution")]
    squadron_leaderboard_powerplay_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_trade_highestcontribution")]
    squadron_leaderboard_trade_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_trade_illicit_highestcontribution")]
    squadron_leaderboard_trade_illicit_highestcontribution: u64,
    #[serde(rename = "Squadron_Leaderboard_podiums")]
    squadron_leaderboard_podiums: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "TG_ENCOUNTER_KILLED":10032, "TG_ENCOUNTER_TOTAL":171, "TG_ENCOUNTER_TOTAL_LAST_SYSTEM":"Pleiades Sector SZ-O b6-0", 
    "TG_ENCOUNTER_TOTAL_LAST_TIMESTAMP":"3311-11-24 11:13", "TG_ENCOUNTER_TOTAL_LAST_SHIP":"Mandalay" })]
pub struct StatisticsTgEncounters {
    #[serde(rename = "TG_ENCOUNTER_IMPRINT")]
    tg_encounter_imprint: Option<u64>,
    #[serde(rename = "TG_ENCOUNTER_KILLED")]
    tg_encounter_killed: Option<u64>,
    #[serde(rename = "TG_ENCOUNTER_TOTAL")]
    tg_encounter_total: Option<u64>,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SYSTEM")]
    tg_encounter_total_last_system: Option<EDString>,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_TIMESTAMP")]
    tg_encounter_total_last_timestamp: Option<EDString>,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SHIP")]
    tg_encounter_total_last_ship: Option<EDString>,
    #[serde(rename = "TG_ENCOUNTER_WAKES")]
    tg_encounter_wakes: Option<u64>,
    #[serde(rename = "TG_SCOUT_COUNT")]
    tg_scout_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StatisticsCraftingOnFoot {
    #[serde(rename = "Suit_Mods_Applied")]
    suit_mods_applied: u64,
    #[serde(rename = "Weapon_Mods_Applied")]
    weapon_mods_applied: u64,
    #[serde(rename = "Suits_Upgraded")]
    suits_upgraded: u64,
    #[serde(rename = "Weapons_Upgraded")]
    weapons_upgraded: u64,
    #[serde(rename = "Suits_Upgraded_Full")]
    suits_upgraded_full: u64,
    #[serde(rename = "Weapons_Upgraded_Full")]
    weapons_upgraded_full: u64,
    #[serde(rename = "Suit_Mods_Applied_Full")]
    suit_mods_applied_full: u64,
    #[serde(rename = "Weapon_Mods_Applied_Full")]
    weapon_mods_applied_full: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Count_Of_Used_Engineers":25, "Recipes_Generated":6364, "Recipes_Generated_Rank_1":902, 
    "Recipes_Generated_Rank_2":972, "Recipes_Generated_Rank_3":1250, "Recipes_Generated_Rank_4":1413, 
    "Recipes_Generated_Rank_5":1827, "Suit_Mods_Applied":9, "Weapon_Mods_Applied":18, "Suits_Upgraded":6, 
    "Weapons_Upgraded":24, "Suits_Upgraded_Full":0, "Weapons_Upgraded_Full":0, "Suit_Mods_Applied_Full":3, 
    "Weapon_Mods_Applied_Full":5 })]
pub struct StatisticsCrafting {
    #[serde(rename = "Count_Of_Used_Engineers")]
    count_of_used_engineers: u64,
    #[serde(rename = "Recipes_Generated")]
    recipes_generated: u64,
    #[serde(rename = "Recipes_Generated_Rank_1")]
    recipes_generated_rank_1: u64,
    #[serde(rename = "Recipes_Generated_Rank_2")]
    recipes_generated_rank_2: u64,
    #[serde(rename = "Recipes_Generated_Rank_3")]
    recipes_generated_rank_3: u64,
    #[serde(rename = "Recipes_Generated_Rank_4")]
    recipes_generated_rank_4: u64,
    #[serde(rename = "Recipes_Generated_Rank_5")]
    recipes_generated_rank_5: u64,
    #[serde(flatten)]
    statistics_crafting_onfoot: Option<StatisticsCraftingOnFoot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "NpcCrew_TotalWages":0, "NpcCrew_Hired":0, "NpcCrew_Fired":0, "NpcCrew_Died":0 })]
pub struct StatisticsCrew {
    #[serde(rename = "NpcCrew_TotalWages")]
    npc_crew_total_wages: Option<Credits>,
    #[serde(rename = "NpcCrew_Hired")]
    npc_crew_hired: Option<u64>,
    #[serde(rename = "NpcCrew_Fired")]
    npc_crew_fired: Option<u64>,
    #[serde(rename = "NpcCrew_Died")]
    npc_crew_died: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Multicrew_Time_Total":870, "Multicrew_Gunner_Time_Total":0, "Multicrew_Fighter_Time_Total":0, 
    "Multicrew_Credits_Total":0, "Multicrew_Fines_Total":0 })]
pub struct StatisticsMulticrew {
    #[serde(rename = "Multicrew_Time_Total")]
    multicrew_time_total: u64,
    #[serde(rename = "Multicrew_Gunner_Time_Total")]
    multicrew_gunner_time_total: u64,
    #[serde(rename = "Multicrew_Fighter_Time_Total")]
    multicrew_fighter_time_total: u64,
    #[serde(rename = "Multicrew_Credits_Total")]
    multicrew_credits_total: u64,
    #[serde(rename = "Multicrew_Fines_Total")]
    multicrew_fines_total: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StatisticsMaterialTraderStatsDetail {
    #[serde(rename = "Encoded_Materials_Traded")]
    encoded_materials_traded: u64,
    #[serde(rename = "Raw_Materials_Traded")]
    raw_materials_traded: u64,
    #[serde(rename = "Grade_1_Materials_Traded")]
    grade_1_materials_traded: u64,
    #[serde(rename = "Grade_2_Materials_Traded")]
    grade_2_materials_traded: u64,
    #[serde(rename = "Grade_3_Materials_Traded")]
    grade_3_materials_traded: u64,
    #[serde(rename = "Grade_4_Materials_Traded")]
    grade_4_materials_traded: u64,
    #[serde(rename = "Grade_5_Materials_Traded")]
    grade_5_materials_traded: u64,
    #[serde(rename = "Assets_Traded_In")]
    assets_traded_in: u64,
    #[serde(rename = "Assets_Traded_Out")]
    assets_traded_out: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Trades_Completed":3819, "Materials_Traded":101524, "Encoded_Materials_Traded":34911, 
    "Raw_Materials_Traded":17768, "Grade_1_Materials_Traded":19443, "Grade_2_Materials_Traded":22717, 
    "Grade_3_Materials_Traded":21980, "Grade_4_Materials_Traded":20526, "Grade_5_Materials_Traded":16858, 
    "Assets_Traded_In":307, "Assets_Traded_Out":1146 })]
pub struct StatisticsMaterialTraderStats {
    #[serde(rename = "Trades_Completed")]
    trades_completed: u64,
    #[serde(rename = "Materials_Traded")]
    materials_traded: u64,
    #[serde(flatten)]
    detailed_traded_statistics: Option<StatisticsMaterialTraderStatsDetail>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "FLEETCARRIER_EXPORT_TOTAL":8486, "FLEETCARRIER_IMPORT_TOTAL":3347, "FLEETCARRIER_TRADEPROFIT_TOTAL":848492, 
    "FLEETCARRIER_TRADESPEND_TOTAL":335341, "FLEETCARRIER_STOLENPROFIT_TOTAL":0, "FLEETCARRIER_STOLENSPEND_TOTAL":0, 
    "FLEETCARRIER_DISTANCE_TRAVELLED":55819.323803593, "FLEETCARRIER_TOTAL_JUMPS":197, "FLEETCARRIER_SHIPYARD_SOLD":0, 
    "FLEETCARRIER_SHIPYARD_PROFIT":0, "FLEETCARRIER_OUTFITTING_SOLD":254, "FLEETCARRIER_OUTFITTING_PROFIT":0, "FLEETCARRIER_REARM_TOTAL":233, 
    "FLEETCARRIER_REFUEL_TOTAL":757, "FLEETCARRIER_REFUEL_PROFIT":1913, "FLEETCARRIER_REPAIRS_TOTAL":465, "FLEETCARRIER_VOUCHERS_REDEEMED":954,
    "FLEETCARRIER_VOUCHERS_PROFIT":12341234 })]
pub struct StatisticsFleetcarrier {
    #[serde(rename = "FLEETCARRIER_EXPORT_TOTAL")]
    fleetcarrier_export_total: u64,
    #[serde(rename = "FLEETCARRIER_IMPORT_TOTAL")]
    fleetcarrier_import_total: u64,
    #[serde(rename = "FLEETCARRIER_TRADEPROFIT_TOTAL")]
    fleetcarrier_tradeprofit_total: Credits,
    #[serde(rename = "FLEETCARRIER_TRADESPEND_TOTAL")]
    fleetcarrier_tradespend_total: Credits,
    #[serde(rename = "FLEETCARRIER_STOLENPROFIT_TOTAL")]
    fleetcarrier_stolenprofit_total: Credits,
    #[serde(rename = "FLEETCARRIER_STOLENSPEND_TOTAL")]
    fleetcarrier_stolenspend_total: Credits,
    #[serde(rename = "FLEETCARRIER_DISTANCE_TRAVELLED")]
    fleetcarrier_distance_travelled: f64,
    #[serde(rename = "FLEETCARRIER_TOTAL_JUMPS")]
    fleetcarrier_total_jumps: u64,
    #[serde(rename = "FLEETCARRIER_SHIPYARD_SOLD")]
    fleetcarrier_shipyard_sold: u64,
    #[serde(rename = "FLEETCARRIER_SHIPYARD_PROFIT")]
    fleetcarrier_shipyard_profit: Credits,
    #[serde(rename = "FLEETCARRIER_OUTFITTING_SOLD")]
    fleetcarrier_outfitting_sold: u64,
    #[serde(rename = "FLEETCARRIER_OUTFITTING_PROFIT")]
    fleetcarrier_outfitting_profit: Credits,
    #[serde(rename = "FLEETCARRIER_REARM_TOTAL")]
    fleetcarrier_rearm_total: u64,
    #[serde(rename = "FLEETCARRIER_REFUEL_TOTAL")]
    fleetcarrier_refuel_total: u64,
    #[serde(rename = "FLEETCARRIER_REFUEL_PROFIT")]
    fleetcarrier_refuel_profit: u64,
    #[serde(rename = "FLEETCARRIER_REPAIRS_TOTAL")]
    fleetcarrier_repairs_total: u64,
    #[serde(rename = "FLEETCARRIER_VOUCHERS_REDEEMED")]
    fleetcarrier_vouchers_redeemed: u64,
    #[serde(rename = "FLEETCARRIER_VOUCHERS_PROFIT")]
    fleetcarrier_vouchers_profit: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StatisticsCQCStats {
    #[serde(rename = "CQC_Credits_Earned")]
    cqc_credits_earned: Option<Credits>,
    #[serde(rename = "CQC_Time_Played")]
    cqc_time_played: u64,
    #[serde(rename = "CQC_KD")]
    cqc_kd: f64,
    #[serde(rename = "CQC_WL")]
    cqc_wl: f64,
    #[serde(rename = "CQC_Kills")]
    cqc_kills: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[testcase_struct({ "Organic_Genus_Encountered":16, "Organic_Species_Encountered":66, "Organic_Variant_Encountered":225, 
    "Organic_Data_Profits":12341234, "Organic_Data":515, "First_Logged_Profits":12341234, "First_Logged":336, "Organic_Systems":128, 
    "Organic_Planets":210, "Organic_Genus":11, "Organic_Species":19 })]
pub struct StatisticsExobiology {
    #[serde(rename = "Organic_Genus_Encountered")]
    organic_genus_encountered: u64,
    #[serde(rename = "Organic_Species_Encountered")]
    organic_species_encountered: u64,
    #[serde(rename = "Organic_Variant_Encountered")]
    organic_variant_encountered: u64,
    #[serde(rename = "Organic_Data_Profits")]
    organic_data_profits: Credits,
    #[serde(rename = "Organic_Data")]
    organic_data: u64,
    #[serde(rename = "First_Logged_Profits")]
    first_logged_profits: Credits,
    #[serde(rename = "First_Logged")]
    first_logged: u64,
    #[serde(rename = "Organic_Systems")]
    organic_systems: u64,
    #[serde(rename = "Organic_Planets")]
    organic_planets: u64,
    #[serde(rename = "Organic_Genus")]
    organic_genus: u64,
    #[serde(rename = "Organic_Species")]
    organic_species: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStatistics {
    #[serde(rename = "Bank_Account")]
    pub bank_account: StatisticsBankAccount,
    pub combat: StatisticsCombat,
    pub crime: StatisticsCrime,
    pub smuggling: StatisticsSmuggling,
    pub trading: StatisticsTrading,
    pub mining: StatisticsMining,
    pub exploration: StatisticsExploration,
    pub passengers: StatisticsPassengers,
    #[serde(rename = "Search_And_Rescue")]
    pub search_and_rescue: StatisticSearchAndRescue,
    pub squadron: Option<StatisticsSquadron>,
    #[serde(rename = "TG_ENCOUNTERS")]
    pub tg_encounters: Option<StatisticsTgEncounters>,
    pub crafting: StatisticsCrafting,
    pub crew: StatisticsCrew,
    pub multicrew: StatisticsMulticrew,
    #[serde(rename = "Material_Trader_Stats")]
    pub material_trader_stats: StatisticsMaterialTraderStats,
    #[serde(rename = "CQC")]
    pub cqc_stats: Option<StatisticsCQCStats>,
    #[serde(rename = "FLEETCARRIER")]
    pub fleetcarrier: Option<StatisticsFleetcarrier>,
    pub exobiology: Option<StatisticsExobiology>,
}
