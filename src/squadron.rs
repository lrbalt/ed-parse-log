use crate::EDString;
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogAppliedToSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDisbandedSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogInvitedToSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogJoinedSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogKickedFromSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLeftSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSharedBookmarkToSquadron {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSquadronCreated {
    squadron_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2018-10-17T16:17:55Z", "event":"SquadronDemotion","SquadronName":"TestSquadron", "OldRank":3, "NewRank":2 })]
pub struct EDLogSquadronDemotion {
    squadron_name: EDString,
    old_rank: u8,
    new_rank: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2018-10-17T16:17:55Z", "event":"SquadronPromotion", "SquadronName":"TestSquadron", "OldRank":2, "NewRank":3 })]
pub struct EDLogSquadronPromotion {
    squadron_name: EDString,
    old_rank: u8,
    new_rank: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-11-13T16:13:26Z", "event":"SquadronStartup", "SquadronID":75645, "SquadronName":"ENDURANCE EXPLORATION", "CurrentRank":4, "CurrentRankName":"Agent" })]
#[testcase({"timestamp":"2024-02-14T17:32:56Z","event":"SquadronStartup","SquadronName":"ENDURANCE EXPLORATION","CurrentRank":4})]
#[testcase({ "timestamp":"2025-11-13T16:13:26Z", "event":"SquadronStartup", "SquadronID":75645, "SquadronName":"ENDURANCE EXPLORATION", "CurrentRank":4, "CurrentRankName":"Agent" })]
pub struct EDLogSquadronStartup {
    #[serde(rename = "SquadronID")]
    squadrion_id: Option<u64>,
    squadron_name: EDString,
    current_rank: u64,
    current_rank_name: Option<EDString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogWonATrophyForSquadron {
    squadron_name: EDString,
}
