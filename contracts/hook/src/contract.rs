use cosmwasm_std::{
    entry_point, from_binary, to_binary, Coin, CosmosMsg, DepsMut, Env, IbcTimeout, MessageInfo, Response, StdResult
};
use crate::msg::{ExecMsg, InitMsg, Snip20HandleMsg, UnwrapTransfer};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(_deps: DepsMut, env: Env, info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
    match msg {
        ExecMsg::WrapDeposit {
            snip20_address,
            snip20_code_hash,
            recipient_address,
        } => Ok(Response::default().add_messages(vec![
            CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                contract_addr: snip20_address.clone(),
                code_hash: snip20_code_hash.clone(),
                msg: to_binary(&Snip20HandleMsg::Deposit { padding: None }).unwrap(),
                funds: info.funds.clone(),
            }),
            CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                contract_addr: snip20_address,
                code_hash: snip20_code_hash,
                msg: to_binary(&Snip20HandleMsg::Transfer {
                    recipient: recipient_address,
                    amount: info.funds[0].amount,
                    memo: None,
                    padding: None,
                })
                .unwrap(),
                funds: vec![],
            }),
        ])),
        ExecMsg::Receive(recv_msg) => {
            if let Some(snip_msg) = recv_msg.msg {
                let decoded_msg = from_binary::<UnwrapTransfer>(&snip_msg)?;
                Ok(Response::default().add_messages([
                    CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                        contract_addr: info.sender.clone().to_string(),
                        code_hash: decoded_msg.snip20_code_hash.clone(),
                        msg: to_binary(&Snip20HandleMsg::Redeem { amount: recv_msg.amount.clone(), denom: decoded_msg.denom.clone(), padding: None }).unwrap(),
                        funds: info.funds.clone(),
                    }),
                    CosmosMsg::Ibc(cosmwasm_std::IbcMsg::Transfer {
                        channel_id: decoded_msg.channel,
                        to_address: decoded_msg.recipient_address,
                        amount: Coin{
                            amount: recv_msg.amount.clone(),
                            denom: decoded_msg.denom,
                        },
                        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
                        memo: decoded_msg.memo.clone().unwrap_or("".to_string()),
                    })
                ]))
            } else {
                Err(cosmwasm_std::StdError::generic_err("Receive message does not contain a valid UnwrapTransfer message"))
            }
        },
        ExecMsg::RegisterSnip20 {
            snip20_address,
            snip20_code_hash,
        } => {
            Ok(Response::default().add_message(
                CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: snip20_address.clone().to_string(),
                    code_hash: snip20_code_hash.clone(),
                    msg: to_binary(&Snip20HandleMsg::RegisterReceive{ code_hash: env.contract.code_hash, padding: None }).unwrap(),
                    funds: vec![],
                })
            ))
        },
    }
}
