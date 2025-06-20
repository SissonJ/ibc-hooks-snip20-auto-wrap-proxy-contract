use cosmwasm_std::{Addr, Binary, Uint128};
use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {}

#[cw_serde]
pub struct Snip20ReceiveMsg {
    pub sender: String,
    pub from: String,
    pub amount: Uint128,
    pub memo: Option<String>,
    pub msg: Option<Binary>,
}

#[cw_serde]
pub struct UnwrapTransfer {
    pub channel: String,
    pub recipient_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecMsg {
    WrapDeposit {
        snip20_address: String,
        snip20_code_hash: String,
        recipient_address: String,
    },
    Receive(Snip20ReceiveMsg),
    RegisterSnip20 {
        snip20_address: String,
        snip20_code_hash: String,
        denom: String,
    },
}

/// SNIP20 token handle messages
#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Snip20HandleMsg {
    Deposit {
        padding: Option<String>,
    },
    Redeem{
        amount: Uint128,
        denom: String,
        padding: Option<String>,
    },
    // Basic SNIP20 functions
    Transfer {
        recipient: String,
        amount: Uint128,
        memo: Option<String>,
        padding: Option<String>,
    },
    RegisterReceive {
        code_hash: String,
        padding: Option<String>,
    },
}

#[cw_serde]
pub struct RegisteredSnip20 {
    pub snip20_address: Addr,
    pub snip20_code_hash: String,
    pub denom: String,
}
