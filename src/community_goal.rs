use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommunityGoalTopTier {
    pub name: String,
    pub bonus: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommunityGoal {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub title: String,
    pub system_name: String,
    pub market_name: String,
    pub expiry: DateTime<Utc>,
    pub is_complete: bool,
    pub current_total: u64,
    pub player_contribution: u64,
    pub num_contributors: u64,
    pub top_tier: CommunityGoalTopTier,
    pub top_rank_size: Option<u64>,
    pub player_in_top_rank: Option<bool>,
    pub tier_reached: Option<String>,
    pub player_percentile_band: u64,
    pub bonus: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoalDiscard {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: String,
    #[serde(rename = "System")]
    pub system_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoal {
    pub current_goals: Vec<CommunityGoal>,
}

#[test]
fn test_community_goal() {
    let json1 = r#"{ "timestamp":"2024-09-20T15:50:48Z", "event":"CommunityGoal", "CurrentGoals":[ { "CGID":810, "Title":"Defend Shinrarta Dezhra Against Thargoid Invasion", "SystemName":"V886 Centauri", "MarketName":"Rescue Ship Cornwallis", "Expiry":"2024-09-26T07:00:00Z", "IsComplete":false, "CurrentTotal":864224475278, "PlayerContribution":0, "NumContributors":3199, "TopTier":{ "Name":"Tier 4", "Bonus":"" }, "TierReached":"Tier 1", "PlayerPercentileBand":100, "Bonus":10000000 } ] }"#;
    let _line1: crate::log_line::EDLogLine = serde_json::from_str(json1).expect("Should parse");

    let json2 = r#"{ "timestamp":"2025-03-02T13:41:22Z", "event":"CommunityGoal", "CurrentGoals":[ { "CGID":813, "Title":"Brewer Corporation Trailblazer Fleet Initiative", "SystemName":"Minerva", "MarketName":"Starlace Station", "Expiry":"2025-03-06T07:00:00Z", "IsComplete":false, "CurrentTotal":65000213, "PlayerContribution":7024, "NumContributors":13124, "TopTier":{ "Name":"Tier 8", "Bonus":"" }, "TopRankSize":10, "PlayerInTopRank":false, "TierReached":"Tier 4", "PlayerPercentileBand":25, "Bonus":245000000 }, { "CGID":814, "Title":"Protect Deliveries to Minerva", "SystemName":"Minerva", "MarketName":"Starlace Station", "Expiry":"2025-03-06T07:00:00Z", "IsComplete":false, "CurrentTotal":241388732091, "PlayerContribution":66513181, "NumContributors":10704, "TopTier":{ "Name":"Tier 5", "Bonus":"" }, "TopRankSize":10, "PlayerInTopRank":false, "TierReached":"Tier 4", "PlayerPercentileBand":25, "Bonus":300000000 } ] }"#;
    let line2: crate::log_line::EDLogLine = serde_json::from_str(json2).expect("Should parse");

    assert!(matches!(
        line2.event(),
        crate::log_line::EDLogEvent::CommunityGoal(_)
    ));
    if let crate::log_line::EDLogEvent::CommunityGoal(header) = line2.event() {
        assert_eq!(&header.current_goals[0].top_tier.name, "Tier 8");
    }
}
