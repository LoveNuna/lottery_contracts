#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    attr, coin, entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Storage, Uint128,
};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp};
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
    env: Env,
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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Stake {amount,denom,staker,} => stake(deps, env, info, amount, denom, staker),
        ExecuteMsg::BeginRaffleRound {begin_time_stamp, end_time_stamp, minimum_stake, winner_distribution,} => begin_raffle_round(deps, env, info, begin_time_stamp, end_time_stamp, minimum_stake, winner_distribution)
}

pub fn Stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: u128,
    denom: Denom,
    staker: Addr,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    Ok(Response::new())
}

pub fn getCurrentCounter(deps: DepsMut) -> Result<i32, ContractError> {
    let counter = COUNTER.load(deps.storage)?;
    Ok(counter.counter)
}

pub fn begin_raffle_round(deps: DepsMut, env: Env, info: MessageInfo, id: i32, endTimeStamp: Timestamp, players: Vec<Addr>, minimumStake: i32, winnerDistribution: Vec<i32>) -> Result<Response, ContractError> {
    // let state = STATE.load(deps.storage)?;
    let mut state = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.id = getCurrentCounter();
        state.beginTimeStamp = env.block.time;
        state.endTimeStamp = endTimeStamp;
        state.players = players;
        state.minimumStake = minimumStake;
        state.winnerDistribution = winnerDistribution;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "begin_raffle_round"))
}

pub fn delete_raffle_round(deps: DepsMut, env: Env, info: MessageInfo, id: i32) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let mut state = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.id = id;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "delete_raffle_round"))
}
}
