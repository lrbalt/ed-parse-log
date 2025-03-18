use serde::{Deserialize, Serialize};

use crate::common_types::Credits;

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
pub struct EDLogStatisticsBankAccount {
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
pub struct EDLogStatisticsCombat {
    #[serde(rename = "Bounties_Claimed")]
    bounties_claimed: u64,
    #[serde(rename = "Bounty_Hunting_Profit")]
    bounty_hunting_profit: Credits,
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
pub struct EDLogStatisticsCrime {
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
pub struct EDLogStatisticsSmuggling {
    #[serde(rename = "Black_Markets_Traded_With")]
    black_markets_traded_with: u64,
    #[serde(rename = "Black_Markets_Profits")]
    black_markets_profits: Credits,
    #[serde(rename = "Resources_Smuggled")]
    resources_smuggled: u64,
    #[serde(rename = "Average_Profit")]
    average_profit: f64,  // TODO: Credits<f64>
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
pub struct EDLogStatisticsTrading {
    #[serde(rename = "Markets_Traded_With")]
    markets_traded_with: u64,
    #[serde(rename = "Market_Profits")]
    market_profits: i64,
    #[serde(rename = "Resources_Traded")]
    resources_traded: u64,
    #[serde(rename = "Average_Profit")]
    average_profit: f64,
    #[serde(rename = "Highest_Single_Transaction")]
    highest_single_transaction: Credits,
    #[serde(flatten)]
    trading_onfoot_statistics: Option<StatisticsTradingOnFoot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EDLogStatisticsMining {
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
pub struct EDLogStatisticsExploration {
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
pub struct EDLogStatisticsPassengers {
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
pub struct EDLogStatisticSearchAndRescue {
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
pub struct EDLogStatisticsTgEncounters {
    #[serde(rename = "TG_ENCOUNTER_KILLED")]
    tg_encounter_killed: Option<u64>,
    #[serde(rename = "TG_ENCOUNTER_TOTAL")]
    tg_encounter_total: u64,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SYSTEM")]
    tg_encounter_total_last_system: String,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_TIMESTAMP")]
    tg_encounter_total_last_timestamp: String,
    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SHIP")]
    tg_encounter_total_last_ship: String,
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
pub struct EDLogStatisticsCrafting {
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
pub struct EDLogStatisticsCrew {
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
pub struct EDLogStatisticsMulticrew {
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
pub struct EDLogStatisticsMaterialTraderStats {
    #[serde(rename = "Trades_Completed")]
    trades_completed: u64,
    #[serde(rename = "Materials_Traded")]
    materials_traded: u64,
    #[serde(flatten)]
    detailed_traded_statistics: Option<StatisticsMaterialTraderStatsDetail>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EDLogStatisticsFleetcarrier {
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
pub struct EDLogStatisticsExobiology {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogStatistics {
    #[serde(rename = "Bank_Account")]
    pub bank_account: EDLogStatisticsBankAccount,
    pub combat: EDLogStatisticsCombat,
    pub crime: EDLogStatisticsCrime,
    pub smuggling: EDLogStatisticsSmuggling,
    pub trading: EDLogStatisticsTrading,
    pub mining: EDLogStatisticsMining,
    pub exploration: EDLogStatisticsExploration,
    pub passengers: EDLogStatisticsPassengers,
    #[serde(rename = "Search_And_Rescue")]
    pub search_and_rescue: EDLogStatisticSearchAndRescue,
    #[serde(rename = "TG_ENCOUNTERS")]
    pub tg_encounters: Option<EDLogStatisticsTgEncounters>,
    pub crafting: EDLogStatisticsCrafting,
    pub crew: EDLogStatisticsCrew,
    pub multicrew: EDLogStatisticsMulticrew,
    #[serde(rename = "Material_Trader_Stats")]
    pub material_trader_stats: EDLogStatisticsMaterialTraderStats,
    #[serde(rename = "FLEETCARRIER")]
    pub fleetcarrier: Option<EDLogStatisticsFleetcarrier>,
    pub exobiology: Option<EDLogStatisticsExobiology>,
}
