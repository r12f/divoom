#[doc = include_str!("./api_execute_commands.md")]
use crate::divoom_contracts::pixoo::common::*;
use serde::{Deserialize, Serialize};

// Request
define_pixoo_command_request!(
    "Draw/CommandList",
    DivoomPixooCommandBatchExecuteCommandsRequest,
    DivoomPixooCommandBatchExecuteCommandsRequestPayload
);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DivoomPixooCommandBatchExecuteCommandsRequestPayload {
    pub command_list: Vec<serde_json::Value>,
}

// Response
define_pixoo_command_response_without_payload!(DivoomPixooCommandBatchExecuteCommandsResponse);
