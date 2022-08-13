use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Raffle, COUNTER, RAFFLEMAP};
use std::string::ToString;
use cosmwasm_std::{StdResult, StdError};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Timestamp, Uint128,
};
use cw2::set_contract_version;
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg};

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
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // ExecuteMsg::Stake_CW {amount,denom,staker,} => stake&(deps, env, info, amount, denom, staker),
        // ExecuteMsg::BeginRaffleRound {begin_time_stamp, end_time_stamp, minimum_stake, winners_distribution,} => begin_raffle_round(deps, env, info, id, endTimeStamp, players, minimumStake),
        // ExecuteMsg::EndRaffleRound {id,} => end_raffle_round(deps, env, info, id),
        // ExecuteMsg::ClaimWinning {id,} => claim_winning(deps, env, info, id),
}

    fn Stake_CW(deps: DepsMut, env: Env, info: MessageInfo, amount: Uin128, cw20_addr: Addr) {
        let cw20 = Cw20Contract(cw20_addr);
        let msg = cw20.call(Cw20ExecuteMsg::Transfer {
            recipient: env.contract.address.to_string(),
            amount: amount,
        });
        Ok(msg);
    }

    pub fn Stake_Native(deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) {
        let msg = BankMsg::Send {
            to_address: env.contract.address.to_string(),
            amount: vec![Coin {
                denom: "juno".to_string(),
                amount: amount,
            }],
        };
    }

    fn getCurrentCounter(deps: DepsMut) -> Result<i32, ContractError> {
        let counter = COUNTER.load(deps.storage)?;
        Ok(counter.counter)
    }

    fn incrementCounter(deps: DepsMut) -> Result<i32, ContractError> {
        let mut counter = COUNTER.load(deps.storage)?;
        counter.counter += 1;
        COUNTER.save(deps.storage, &counter)?;
        Ok(counter.counter)
    }

    pub fn getRaffleObject(deps: DepsMut, id: u64) -> Result<String> {
        let raffle = RAFFLEMAP.load(deps.storage, &id.to_string());
        Ok("Doesn't work yet, type mismatch".to_string())
    }

    pub fn begin_raffle_round(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: i32,
        endTimeStamp: Timestamp,
        players: Vec<Addr>,
        minimumStake: u128,
        winnersDistribution: Vec<i32>,
    ) -> Result<Response, ContractError> {
        let raffle = Raffle {
            id: id,
            beginTimeStamp: env.block.time,
            endTimeStamp: endTimeStamp,
            players: players,
            winners: Vec::new(),
            minimumStake: minimumStake,
            winnersDistribution: winnersDistribution,
            winnerPayouts: Vec::new(),
            active: true,
        };
        let mut state =
            RAFFLEMAP.save(deps.storage, getCurrentCounter(deps).to_string(), &raffle)?;

        Ok(Response::new().add_attribute("method", "delete_raffle_round"))
    }

    pub fn join_raffle_round(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: u64,
    ) -> Result<Response, ContractError> {
        let mut endBlock = env.block.height - 10;
        // get Raffle object
        let mut raffle = getRaffleObject(deps, id)?;
        Stake_CW(
            deps,
            env,
            info,
            raffle.minimumStake,
            info.sender,
            info.sender,
        );
        raffle.players.push(info.sender);
        Ok(Response::new().add_attribute("method", "join_raffle_round"))
    }

    pub fn execute_receive(
        deps: DepsMut,
        info: MessageInfo,
        wrapped: Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        // cw20 address authentication
        let config = CONFIG.load(deps.storage)?;
        if config.cw20_addr != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        let msg: ReceiveMsg = from_binary(&wrapped.msg)?;
        match msg {
            ReceiveMsg::Send { id } => receive_send(deps, id, wrapped.amount, info.sender),
        }
    }

    pub fn delete_raffle_round(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: i32,
    ) -> Result<Response, ContractError> {
        let state = RAFFLEMAP.load(deps.storage)?;
        let mut state =
            RAFFLEMAP.save(
                state.id = id,
                state.beginTimeStamp = env.block.time,
                state.endTimeStamp = env.block.time,
                state.players = vec![],
                state.minimumStake = 0,
                state.winnersDistribution = vec![],
            );
        Ok(Response::new().add_attribute("method", "delete_raffle_round"))
    }

    pub fn RNG(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: i32,
    ) -> Result<Response, ContractError> {
        let state = RAFFLEMAP.load(deps.storage)?;
        let mut rng = env.block.time;
        Ok(Response::new().add_attribute("method", "RNG"))
    }

    pub fn choose_winners(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: i32,
    ) -> Result<Response, ContractError> {
        let state = RAFFLEMAP.load(deps.storage)?;
        let winners_number = RNG(deps, env, info, id);
        let winners_address = players[winners_number];
        let mut state =
            RAFFLEMAP.update(deps.storage, |mut state| -> Result<_, ContractError> {
                state.winners = winners_address;
                state.active = false;
                Ok(state)
            })?;
        Ok(Response::new().add_attribute("method", "choose_winners"))
    }

    // pub fn end_raffle_round(deps: DepsMut, env: Env, info: MessageInfo, id: i32) -> Result<Response, ContractError> {
    //     let winner_payouts = calculate_winner_payouts(deps, env, info, id);
    //     // transfer amount to players
    //     for winner_payout in winner_payouts {
    //         let winner_address = winner_payout.winner_address;
    //         let winner_amount = winner_payout.1;
    //         let mut state = RAFFLEMAP.update(deps.storage, |mut state| -> Result<_, ContractError> {
    //             state.winners = winner_address;
    //             state.active = false;
    //             Ok(state)
    //         })?;
    //         msg.amount = winner_amount;
    //         msg.to = winner_address;
    //         msg.memo = "winner".to_string();
    //         transfer(deps, env, info, msg)?;
    //     }
    //     Ok(Response::new().add_attribute("method", "end_raffle_round"))
    // }

    pub fn calculate_winner_payouts(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: i32,
    ) -> Result<Response, ContractError> {
        let state = RAFFLEMAP.load(deps.storage)?;
        // calculate winner payouts from state.winnersDistribution
        let winnerPayouts = winnersDistribution.iter().map(|&x| x * minimumStake).sum();
        Ok(Response::new().add_attribute("winnerPayouts", winnerPayouts));
    }
}

#[test]
fn proper_initialization() {
    assert_eq!(1, 1);
}
