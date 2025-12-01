use crate::EDString;
use chrono::{DateTime, Utc};
use ed_parse_log_files_macros::testcase;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommunityGoalTopTier {
    pub name: EDString,
    pub bonus: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoalReward {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub system: EDString,
    pub reward: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoalJoin {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub system: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommunityGoal {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub title: EDString,
    pub system_name: EDString,
    pub market_name: EDString,
    pub expiry: DateTime<Utc>,
    pub is_complete: bool,
    pub current_total: u64,
    pub player_contribution: u64,
    pub num_contributors: u64,
    pub top_tier: CommunityGoalTopTier,
    pub top_rank_size: Option<u64>,
    pub player_in_top_rank: Option<bool>,
    pub tier_reached: Option<EDString>,
    pub player_percentile_band: u64,
    pub bonus: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoalDiscard {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: EDString,
    #[serde(rename = "System")]
    pub system_name: EDString,
}

#[testcase({ "timestamp":"2024-09-20T15:50:48Z", "event":"CommunityGoal", "CurrentGoals":[ { "CGID":810, "Title":"Defend Shinrarta Dezhra Against Thargoid Invasion", "SystemName":"V886 Centauri", "MarketName":"Rescue Ship Cornwallis", "Expiry":"2024-09-26T07:00:00Z", "IsComplete":false, "CurrentTotal":864224475278, "PlayerContribution":0, "NumContributors":3199, "TopTier":{ "Name":"Tier 4", "Bonus":"" }, "TierReached":"Tier 1", "PlayerPercentileBand":100, "Bonus":10000000 } ] })]
#[testcase({ "timestamp":"2025-03-02T13:41:22Z", "event":"CommunityGoal", "CurrentGoals":[ { "CGID":813, "Title":"Brewer Corporation Trailblazer Fleet Initiative", "SystemName":"Minerva", "MarketName":"Starlace Station", "Expiry":"2025-03-06T07:00:00Z", "IsComplete":false, "CurrentTotal":65000213, "PlayerContribution":7024, "NumContributors":13124, "TopTier":{ "Name":"Tier 8", "Bonus":"" }, "TopRankSize":10, "PlayerInTopRank":false, "TierReached":"Tier 4", "PlayerPercentileBand":25, "Bonus":245000000 }, { "CGID":814, "Title":"Protect Deliveries to Minerva", "SystemName":"Minerva", "MarketName":"Starlace Station", "Expiry":"2025-03-06T07:00:00Z", "IsComplete":false, "CurrentTotal":241388732091, "PlayerContribution":66513181, "NumContributors":10704, "TopTier":{ "Name":"Tier 5", "Bonus":"" }, "TopRankSize":10, "PlayerInTopRank":false, "TierReached":"Tier 4", "PlayerPercentileBand":25, "Bonus":300000000 } ] })]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoal {
    pub current_goals: Vec<CommunityGoal>,
}
