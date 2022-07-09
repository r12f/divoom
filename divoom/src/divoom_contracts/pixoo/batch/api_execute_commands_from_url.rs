#[doc = include_str!("./api_execute_commands_from_url.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Draw/UseHTTPCommandSource",
    DivoomPixooCommandBatchExecuteCommandsFromUrlRequest,
    DivoomPixooCommandBatchExecuteCommandsFromUrlRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandBatchExecuteCommandsFromUrlRequestPayload {
    pub command_url: String,
}

impl DivoomPixooCommandBatchExecuteCommandsFromUrlRequestPayload {
    pub fn new(command_url: String) -> DivoomPixooCommandBatchExecuteCommandsFromUrlRequestPayload {
        DivoomPixooCommandBatchExecuteCommandsFromUrlRequestPayload { command_url }
    }
}

// Response
define_pixoo_command_response_without_payload!(
    DivoomPixooCommandBatchExecuteCommandsFromUrlResponse
);
