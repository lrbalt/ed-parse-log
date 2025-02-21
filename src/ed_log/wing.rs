use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogWingJoin {
    others: Vec<String>,
}
