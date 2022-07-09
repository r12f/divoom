#[doc = include_str!("./api_get_dial_font_list.md")]
use crate::divoom_contracts::divoom::*;
use crate::{DivoomFontInfo, DivoomFontType};
use serde::{Deserialize, Serialize};

// Response definition
define_divoom_service_api_response!(
    DivoomAPIResponseGetTimeDialFontList,
    DivoomAPIResponseGetTimeDialFontListPayload,
    Vec<DivoomFontInfo>
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomAPIResponseGetTimeDialFontListPayload {
    pub font_list: Vec<DivoomAPIResponseFontInfo>,
}

impl DivoomAPIResponseGetTimeDialFontListPayload {
    pub fn destructive_into(self) -> Vec<DivoomFontInfo> {
        self.font_list
            .into_iter()
            .map(|x| x.destructive_into())
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct DivoomAPIResponseFontInfo {
    pub id: i32,
    pub name: String,
    pub width: String,
    pub high: String,
    pub charset: String,

    #[serde(rename = "type")]
    pub font_type: i32,
}

impl DivoomAPIResponseFontInfo {
    pub fn destructive_into(self) -> DivoomFontInfo {
        DivoomFontInfo {
            id: self.id,
            name: self.name,
            width: self.width.parse().unwrap_or(0),
            height: self.high.parse().unwrap_or(0),
            charset: self.charset,
            font_type: match self.font_type {
                0 => DivoomFontType::Scrollable,
                1 => DivoomFontType::NotScrollable,
                _ => DivoomFontType::Raw(self.font_type),
            },
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
            "FontList": [
               {
                   "id": 2,
                   "name": "8*8 English letters, Arabic figures,punctuation",
                   "width": "8",
                   "high": "8",
                   "charset": "abcdefg",
                   "type": 1
               }
           ]
        }"#;

        let deserialized: DivoomAPIResponseGetTimeDialFontList =
            serde_json::from_str(serialized).unwrap();

        assert_eq!(
            deserialized,
            DivoomAPIResponseGetTimeDialFontList {
                result: DivoomServiceAPIResultDetails {
                    return_code: 0,
                    return_message: "".into()
                },
                payload: DivoomAPIResponseGetTimeDialFontListPayload {
                    font_list: vec![DivoomAPIResponseFontInfo {
                        id: 2,
                        name: "8*8 English letters, Arabic figures,punctuation".into(),
                        width: "8".into(),
                        high: "8".into(),
                        charset: "abcdefg".into(),
                        font_type: 1,
                    }]
                },
            }
        );
    }
}
