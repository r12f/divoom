use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomScheduleConfigCronJob {
    pub cron: String,
    pub operations: Vec<String>,
}
