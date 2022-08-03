use serde::{Serialize, Deserialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomScheduleConfig {
    pub schedules: Vec<DivoomScheduleConfigCronJob>,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DivoomScheduleConfigCronJob {
    pub cron: String,
    pub operations: Vec<String>,
}