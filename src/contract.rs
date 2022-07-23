#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    attr, coin, entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Storage, Uint128,
};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use cw20::{Balance, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg, Denom};
use cw4::{
    Member, MemberChangedHookMsg, MemberDiff, MemberListResponse, MemberResponse,
    TotalWeightResponse,
};
use cw_storage_plus::Bound;
use cw_utils::{maybe_addr, NativeBalance};
use crate::coin_helpers::validate_sent_sufficient_coin;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:fury";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const STAKING_TOKEN: &str = "staking_token";
// pub const DEFAULT_END_HEIGHT_BLOCKS: &u64 = &100_800_u64;
const MIN_STAKE_AMOUNT: u128 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Stake {amount,denom,staker,} => stake(deps, env, info, amount, denom, staker),
        ExecuteMsg::BeginRaffleRound {end_height_blocks} => begin_raffle_round(deps, env, info, end_height_blocks),
}

pub fn Stake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: u128,
    denom: Denom,
    staker: Addr,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.owner != info.sender {
        return Err(ContractError::Unauthorized);
    }
    let mut state = state.clone();
    let mut state = state.stake(amount, denom, staker);
    STATE.save(deps.storage, &state)?;
    Ok(Response::new())
}

pub fn BeginRaffleRound(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    end_height_blocks: u64,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.owner != info.sender {
        return Err(ContractError::Unauthorized);
    }
    let mut state = state.clone();
    let mut state = state.begin_raffle_round(end_height_blocks);
    STATE.save(deps.storage, &state)?;
    Ok(Response::new())
}

pub fn

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { count: state.count })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
