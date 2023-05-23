#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,Uint256};
use cw_storage_plus::{Map};
use cosmwasm_std::{Addr, Uint128};
use cosmwasm_std::Order::Ascending;
use cosmwasm_std::Order::Descending;
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{COUNTER_VALUE};

pub const BALANCES: Map<&Addr, Uint128> = Map::new("before--");
pub const BALANCES2: Map<&Addr, Uint128> = Map::new("after--");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let counter:Uint256 = _msg.init_count;
    COUNTER_VALUE.save(deps.storage, &counter)?; 
    let owner_addr1 = deps.api.addr_validate(&"0x5A8D648DEE57b2fc90D98DC17fa887159b69638b".to_string())?;
    let owner_addr2 = deps.api.addr_validate(&"0x8651e94972a56e69F3C0897d9E8faCbDAEb98386".to_string())?;
    let owner_addr3 = deps.api.addr_validate(&"0x9536354AE32852A7E7C4BFe7415b104016d5Fb04".to_string())?;
    let owner_addr4 = deps.api.addr_validate(&"0xB26C63498bBa95589704F3d5A1fE2DF763C8B7a4".to_string())?;

    BALANCES.update(
        deps.storage,
        &owner_addr1,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(Uint128::zero())?)
        },
    )?;
    BALANCES.update(
        deps.storage,
        &owner_addr2,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(Uint128::zero())?)
        },
    )?;
    BALANCES.update(
        deps.storage,
        &owner_addr3,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(Uint128::zero())?)
        },
    )?;BALANCES.update(
        deps.storage,
        &owner_addr4,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(Uint128::zero())?)
        },
    )?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Add { delta } => try_add(deps,delta),
        ExecuteMsg::Subtract {} => try_sub(deps),
    }
}

pub fn try_add(deps: DepsMut,delta:Uint256) -> Result<Response, ContractError> {

    let mut counter: Uint256 = COUNTER_VALUE
    .may_load(deps.storage)?.unwrap();
    counter = delta;
    COUNTER_VALUE.save(deps.storage, &counter)?;

    // let owner_addr1 = deps.api.addr_validate(&"0x5A8D648DEE57b2fc90D98DC17fa887159b69638b".to_string())?;
    // let owner_addr2 = deps.api.addr_validate(&"0x8651e94972a56e69F3C0897d9E8faCbDAEb98386".to_string())?;
    // let owner_addr3 = deps.api.addr_validate(&"0x9536354AE32852A7E7C4BFe7415b104016d5Fb04".to_string())?;
    // let owner_addr4 = deps.api.addr_validate(&"0xB26C63498bBa95589704F3d5A1fE2DF763C8B7a4".to_string())?;

    // let data: Vec<(Addr, Uint128)> = BALANCES
    //         .range(deps.storage, None, None, Descending)
    //         .collect::<StdResult<Vec<_>>>()?;
    // let mut i = 1;
    // for (owner, allowance) in &data {
    //     if i == 3{
    //         BALANCES2.save(deps.storage, &owner, &allowance)?;
    //     }

    //     i = i + 1;
    // }

    // let data: Vec<(Addr, Uint128)> = BALANCES
    //         .range(deps.storage, Some(Bound::exclusive(&owner_addr1)), Some(Bound::exclusive(&owner_addr4)), Descending)
    //         .collect::<StdResult<Vec<_>>>()?;
    // let mut i = 1;
    // for (owner, allowance) in &data {
        // if i == 1{
        //     BALANCES2.save(deps.storage, &owner, &allowance)?;
        // }

    //     i = i + 1;
    // }

    Ok(Response::new().add_attribute("Added", counter).add_attribute("Changed", counter))
}

pub fn try_sub(deps: DepsMut) -> Result<Response, ContractError> {
    let mut counter = COUNTER_VALUE
    .may_load(deps.storage)?.unwrap();
    if counter == Uint256::zero(){
        ContractError::TooLow {};
    }
    counter -= Uint256::from(1u128);
    COUNTER_VALUE.save(deps.storage, &counter)?;
    Ok(Response::new().add_attribute("Changed", counter))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCounter {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<Uint256> {
    let info = COUNTER_VALUE.may_load(deps.storage).unwrap_or_default();
    Ok(info.unwrap())

}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {init_count: Uint256::from_u128(0) };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCounter {}).unwrap();
        let value: Uint256 = from_binary(&res).unwrap();
        assert_eq!(Uint256::zero(), value);
    }

    #[test]
    fn add() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {init_count: Uint256::from_u128(0)};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Add {delta:Uint256::from(1u128)};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCounter {}).unwrap();
        let value: Uint256 = from_binary(&res).unwrap();
        assert_eq!(Uint256::from(1u128), value);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { init_count: Uint256::from_u128(0)};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();


        let info = mock_info("anyone", &coins(2, "token"));
        let _addmsg = ExecuteMsg::Add {delta:Uint256::from(5u128)};
        execute(deps.as_mut(), mock_env(), info, _addmsg).unwrap();
    
        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Subtract { };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 4
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCounter {}).unwrap();
        let value: Uint256 = from_binary(&res).unwrap();
        assert_eq!(Uint256::from(4u128), value);
    }
}
