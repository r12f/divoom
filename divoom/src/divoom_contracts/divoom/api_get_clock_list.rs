#[doc = include_str!("./api_get_clock_list.md")]
use crate::divoom_contracts::divoom::*;
use crate::{DivoomClockInfo, DivoomClockInfoPage};
use serde::{Deserialize, Serialize};

// Request definition
#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomAPIRequestGetClockList {
    pub dial_type: String,
    pub page: i32,
}

// Response definition
define_divoom_service_api_response!(
    DivoomAPIResponseGetClockList,
    DivoomAPIResponseGetClockListPayload,
    DivoomClockInfoPage
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomAPIResponseGetClockListPayload {
    pub total_num: i32,
    pub dial_list: Vec<DivoomAPIResponseClockInfo>,
}

impl DivoomAPIResponseGetClockListPayload {
    pub fn destructive_into(self) -> DivoomClockInfoPage {
        DivoomClockInfoPage {
            total_num: self.total_num,
            dial_list: self
                .dial_list
                .into_iter()
                .map(|x| x.destructive_into())
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomAPIResponseClockInfo {
    pub clock_id: i32,
    pub name: String,
}

impl DivoomAPIResponseClockInfo {
    pub fn destructive_into(self) -> DivoomClockInfo {
        DivoomClockInfo {
            clock_id: self.clock_id,
            name: self.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divoom_service_api_get_dial_list_payload_serialization_should_work() {
        let serialized = r#"{
            "ReturnCode": 0,
            "ReturnMessage": "",
            "TotalNum": 100,
            "DialList": [
                {
                    "ClockId": 10,
                    "Name": "Classic Digital Clock"
                }
            ]
        }"#;

        let deserialized: DivoomAPIResponseGetClockList = serde_json::from_str(serialized).unwrap();

        assert_eq!(
            deserialized,
            DivoomAPIResponseGetClockList {
                result: DivoomServiceAPIResultDetails {
                    return_code: 0,
                    return_message: "".into()
                },
                payload: DivoomAPIResponseGetClockListPayload {
                    total_num: 100,
                    dial_list: vec![DivoomAPIResponseClockInfo {
                        clock_id: 10,
                        name: "Classic Digital Clock".into()
                    },],
                }
            }
        );
    }
}
