use crate::query::{ChainResponse, InterchainQueries, QueryMsg};
use cosmwasm_std::{
    entry_point, to_binary, to_vec, Binary, ContractResult, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdError, StdResult, SystemResult,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}
use neutron_sdk::sudo::msg::SudoMsg;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: Instantiate");
    Ok(Response::default())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Send { to: String, amount: u128 },
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, _: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: execute: received msg: {:?}", msg).as_str());
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps<InterchainQueries>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Reflect(payload) => to_binary(&query_with_payload(deps, env, payload)?),
    }
}

fn query_with_payload(
    deps: Deps<InterchainQueries>,
    _env: Env,
    icq_query: QueryRequest<InterchainQueries>,
) -> StdResult<ChainResponse> {
    let raw = to_vec(&icq_query).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;
    match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(ChainResponse { data: value }),
    }
}

#[entry_point]
pub fn sudo(_deps: DepsMut, _env: Env, _msg: SudoMsg) -> StdResult<Response> {
    Ok(Response::default())
}
