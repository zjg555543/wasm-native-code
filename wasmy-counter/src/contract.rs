#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use sha3::{Digest, Keccak256};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    // deps.api.ed25519_verify(message, signature, public_key);

    match msg {
        // ExecuteMsg::Test { } => try_test_contract(deps),
        ExecuteMsg::Test { vm } => test(deps, vm),
    }
}

pub fn test(deps: DepsMut, vm: bool) -> Result<Response, ContractError> {
    if vm {
        return try_test_vm(deps);
    }else{
        return try_test_contract(deps);
    }
}

pub fn try_test_vm(deps: DepsMut) -> Result<Response, ContractError> {

    let sign_bytes  = hex::decode("hello").to_owned();

    let result = deps.api.keccak256_digest(&sign_bytes.to_owned().unwrap());

    let temp: String = hex::encode(result.unwrap());

    Ok(Response::new().add_attribute("try_test_vm", temp.to_string()))
}

pub fn try_test_contract(deps: DepsMut) -> Result<Response, ContractError> {

    let sign_bytes  = hex::decode("hello").to_owned();
    let hash = Keccak256::digest(&sign_bytes.to_owned().unwrap());

    let result: String = hex::encode(hash);

    Ok(Response::new().add_attribute("try_test_contract", result.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCounter {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<i32> {
    Ok(1)
}