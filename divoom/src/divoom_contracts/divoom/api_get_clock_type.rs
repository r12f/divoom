#[doc = include_str!("./api_get_clock_type.md")]
use crate::divoom_contracts::divoom::*;
use serde::{Deserialize, Serialize};

// Response definition
define_divoom_service_api_response!(
    DivoomAPIResponseGetClockType,
    DivoomAPIResponseGetClockTypePayload,
    Vec<String>
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomAPIResponseGetClockTypePayload {
    pub dial_type_list: Vec<String>,
}

impl DivoomAPIResponseGetClockTypePayload {
    pub fn destructive_into(self) -> Vec<String> {
        self.dial_type_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divoom_service_api_get_dial_type_payload_serialization_should_work() {
        let serialized = r#"{
            "ReturnCode": 0,
            "ReturnMessage": "",
            "DialTypeList": [
                "Social",
                "normal",
                "financial",
                "Game",
                "HOLIDAYS",
                "TOOLS",
                "DESIGN-64"
            ]
        }"#;

        let deserialized: DivoomAPIResponseGetClockType = serde_json::from_str(serialized).unwrap();

        assert_eq!(
            deserialized,
            DivoomAPIResponseGetClockType {
                result: DivoomServiceAPIResultDetails {
                    return_code: 0,
                    return_message: "".into()
                },
                payload: DivoomAPIResponseGetClockTypePayload {
                    dial_type_list: vec![
                        "Social".into(),
                        "normal".into(),
                        "financial".into(),
                        "Game".into(),
                        "HOLIDAYS".into(),
                        "TOOLS".into(),
                        "DESIGN-64".into(),
                    ],
                }
            }
        );
    }
}
