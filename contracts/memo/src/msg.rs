use schemars::JsonSchema;
use secret_toolkit::permit::Permit;
use serde::{Deserialize, Serialize};
use cosmwasm_std::HumanAddr;
use crate::state::Message;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub owner: Option<HumanAddr>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    SendMemo { to: HumanAddr, message: String },
    SetViewingKey {
        key: String,
        padding: Option<String>,
    },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetMemo { address: HumanAddr, auth: ViewingPermissions, page: Option<u32>, page_size: Option<u32> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ViewingPermissions {
    pub permit: Option<Permit>,
    pub key: Option<String>
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MsgsResponse {
    pub msgs: Vec<Message>,
    pub length: u32
}


