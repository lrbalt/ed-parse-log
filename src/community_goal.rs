use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CommunityGoalTopTier {
    name: String,
    bonus: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
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
    top_rank_size: Option<u64>,
    player_in_top_rank: Option<bool>,
    tier_reached: Option<String>,
    player_percentile_band: u64,
    bonus: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCommunityGoal {
    current_goals: Vec<CommunityGoal>,
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
