use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Coin, CosmosMsg, DepsMut, Env, IbcTimeout, MessageInfo, Response, StdResult
};
use crate::msg::{ExecMsg, InitMsg, RegisteredSnip20, Snip20HandleMsg, UnwrapTransfer};
use cosmwasm_std::Storage;

pub fn save_snip20(storage: &mut dyn Storage, key: &str, snip: &RegisteredSnip20) -> StdResult<()> {
    storage.set(key.as_bytes(), &to_binary(snip)?);
    Ok(())
}

pub fn load_snip20(storage: &dyn Storage, key: &str) -> StdResult<RegisteredSnip20> {
    let data = storage.get(key.as_bytes()).ok_or_else(|| {
        cosmwasm_std::StdError::generic_err("No snip20 registration found")
    })?;
    from_binary(&cosmwasm_std::Binary::from(data))
}

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
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
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
                let api = deps.api;
                let decoded_msg = from_binary::<UnwrapTransfer>(&snip_msg)?;
                let snip20 = load_snip20(deps.storage, &info.sender.to_string())?;
                Ok(Response::default().add_messages([
                    CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                        contract_addr: snip20.snip20_address.clone().to_string(),
                        code_hash: snip20.snip20_code_hash.clone(),
                        msg: to_binary(&Snip20HandleMsg::Deposit { padding: None }).unwrap(),
                        funds: info.funds.clone(),
                    }),
                    CosmosMsg::Ibc(cosmwasm_std::IbcMsg::Transfer {
                        channel_id: decoded_msg.channel,
                        to_address: decoded_msg.recipient_address,
                        amount: Coin{
                            amount: recv_msg.amount.clone(),
                            denom: snip20.denom,
                        },
                        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
                        memo: "".to_string(),
                    })
                ]))
            } else {
                Err(cosmwasm_std::StdError::generic_err("Invalid receive message"))
            }
        },
        ExecMsg::RegisterSnip20 {
            snip20_address,
            snip20_code_hash,
            denom,
        } => {
            let snip20 = RegisteredSnip20 {
                snip20_address: Addr::unchecked(snip20_address),
                snip20_code_hash,
                denom,
            };
            save_snip20(deps.storage, &snip20.snip20_address.to_string(), &snip20)?;
            Ok(Response::default().add_message(
                CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: snip20.snip20_address.clone().to_string(),
                    code_hash: snip20.snip20_code_hash.clone(),
                    msg: to_binary(&Snip20HandleMsg::RegisterReceive{ code_hash: env.contract.code_hash, padding: None }).unwrap(),
                    funds: vec![],
                })
            ))
        },
    }
}
