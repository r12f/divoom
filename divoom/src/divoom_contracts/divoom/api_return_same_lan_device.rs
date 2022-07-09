#[doc = include_str!("./api_return_same_lan_device.md")]
use crate::divoom_contracts::divoom::*;
use crate::DivoomDeviceInfo;
use serde::{Deserialize, Serialize};

// Response definition
define_divoom_service_api_response!(
    DivoomAPIResponseReturnSameLANDevice,
    DivoomAPIResponseReturnSameLANDevicePayload,
    Vec<DivoomDeviceInfo>
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomAPIResponseReturnSameLANDevicePayload {
    pub device_list: Vec<DivoomAPIResponseDeviceInfo>,
}

impl DivoomAPIResponseReturnSameLANDevicePayload {
    pub fn destructive_into(self) -> Vec<DivoomDeviceInfo> {
        self.device_list
            .into_iter()
            .map(|x| x.destructive_into())
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomAPIResponseDeviceInfo {
    pub device_name: String,
    pub device_id: u64,

    #[serde(rename = "DevicePrivateIP")]
    pub device_private_ip: String,
}

impl DivoomAPIResponseDeviceInfo {
    pub fn destructive_into(self) -> DivoomDeviceInfo {
        DivoomDeviceInfo {
            device_name: self.device_name,
            device_id: self.device_id,
            device_private_ip: self.device_private_ip,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divoom_service_api_return_same_lan_device_payload_serialization_should_work() {
        let serialized = r#"{
            "ReturnCode": 0,
            "ReturnMessage": "",
            "DeviceList": [
                {
                    "DeviceName": "Pixoo64",
                    "DeviceId": 300000020,
                    "DevicePrivateIP": "10.0.0.100"
                }
            ]
        }"#;

        let deserialized: DivoomAPIResponseReturnSameLANDevice =
            serde_json::from_str(serialized).unwrap();

        assert_eq!(
            deserialized,
            DivoomAPIResponseReturnSameLANDevice {
                result: DivoomServiceAPIResultDetails {
                    return_code: 0,
                    return_message: "".into()
                },
                payload: DivoomAPIResponseReturnSameLANDevicePayload {
                    device_list: vec![DivoomAPIResponseDeviceInfo {
                        device_name: "Pixoo64".into(),
                        device_id: 300000020,
                        device_private_ip: "10.0.0.100".into(),
                    }],
                }
            }
        );
    }

    #[test]
    fn divoom_api_response_return_same_lan_device_destructive_into_should_work() {
        let from = DivoomAPIResponseReturnSameLANDevice {
            result: DivoomServiceAPIResultDetails {
                return_code: 0,
                return_message: "".into(),
            },
            payload: DivoomAPIResponseReturnSameLANDevicePayload {
                device_list: vec![DivoomAPIResponseDeviceInfo {
                    device_name: "Pixoo64".into(),
                    device_id: 300000020,
                    device_private_ip: "10.0.0.100".into(),
                }],
            },
        };
        let to = from.destructive_into();
        assert_eq!(
            to,
            vec![DivoomDeviceInfo {
                device_name: "Pixoo64".into(),
                device_id: 300000020,
                device_private_ip: "10.0.0.100".into()
            }]
        )
    }
}
