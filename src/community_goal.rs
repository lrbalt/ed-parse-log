use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalTopTier {
    name: String,
    bonus: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoal {
    #[serde(rename = "CGID")]
    cgid: u64,
    title: String,
    system_name: String,
    market_name: String,
    expiry: String,
    is_complete: bool,
    current_total: u64,
    player_contribution: u64,
    num_contributors: u64,
    top_tier: CommunityGoalTopTier,
    tier_reached: Option<String>,
    player_percentile_band: u64,
    bonus: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EDLogCommunityGoal {
    current_goals: Vec<CommunityGoal>,
}
