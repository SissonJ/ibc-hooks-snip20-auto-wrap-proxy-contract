use cosmwasm_std::{Binary, Uint128};
use cosmwasm_schema::cw_serde;

#[cw_serde]
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
    pub code_hash: String,
    pub denom: String,
}

#[cw_serde]
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
    },
}

#[cw_serde]
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
