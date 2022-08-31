use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Raffle, COUNTER, RAFFLEMAP, ADMINS};

use cosmwasm_std::{StdResult, Deps, Binary, QueryRequest, BankQuery, to_binary};

use rand_core::{RngCore, SeedableRng};
use crate::rand::{sha_256, Prng};
use rand_chacha::ChaChaRng;

#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Timestamp, Uint128,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;

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
        ExecuteMsg::BeginRaffleRound {
            end_time_stamp, 
            minimum_stake, 
            winners_distribution
        } => begin_raffle_round(deps, env, info, end_time_stamp, minimum_stake, winners_distribution),
        ExecuteMsg::JoinRaffleRound {
            id
        } => join_raffle_round(deps, env, info, id),
        ExecuteMsg::EndRaffleRound {id,} => choose_winners(deps, env, info, id),
    }
}

pub fn begin_raffle_round(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    end_time_stamp: Timestamp,
    minimum_stake: Uint128,
    winners_distribution: Vec<u32>,
) -> Result<Response, ContractError>{
    if !is_admin(deps.as_ref(), info.sender)? {
        return Err(ContractError::Unauthorized {});
    }

    let mut counter = COUNTER.load(deps.storage)?;
    let id = counter.counter;

    let raffle = Raffle {
        id,
        begin_time_stamp: env.block.time,
        end_time_stamp,
        minimum_stake,
        winners_distribution,
        winners: Vec::new(),
        players: Vec::new(),
        winner_payouts: Vec::new(),
        active: true,
    };
    
    counter.counter += 1;
    COUNTER.save(deps.storage, &counter)?;

    RAFFLEMAP.save(deps.storage, &id.to_string(), &raffle)?;

    Ok(Response::default())
}

pub fn join_raffle_round(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u32,
) -> Result<Response, ContractError> {
    if can_register(deps.as_ref(), id)? {
        return Err(ContractError::RegistrationsClosed {});
    }

    if is_registered(deps.as_ref(), id, info.sender.to_string())? {
        return Err(ContractError::AlreadyRegistered {});
    }

    if info.funds.len() != 1 {
        return Err(ContractError::WrongPayment {});
    }

    if info.funds[0].denom != "ujuno" {
        return Err(ContractError::MustPayByJuno {});
    }

    let mut raffle = RAFFLEMAP.load(deps.storage, &id.to_string())?;

    if raffle.is_expired(&env.block) {
        return Err(ContractError::RaffleExpired {});
    }

    if info.funds[0].amount < raffle.minimum_stake {
        return Err(ContractError::NotSufficientFunds {});
    }

    raffle.players.push(info.sender.to_string());
    RAFFLEMAP.save(deps.storage, &id.to_string(), &raffle)?;

    Ok(Response::default())
}

pub fn choose_winners(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u32,
) -> Result<Response, ContractError> {
    if !is_admin(deps.as_ref(), info.clone().sender)? {
        return Err(ContractError::Unauthorized {});
    }

    let raffle = RAFFLEMAP.load(deps.storage, &id.to_string())?;

    let prng_seed: Vec<u8> = sha_256(base64::encode("entropy").as_bytes()).to_vec();
    let random_seed = new_entropy(&info, &env, prng_seed.as_ref(), prng_seed.as_ref());
    let mut rng = ChaChaRng::from_seed(random_seed);

    let nb_players = raffle.players.len() as u32;
    let total_shares = raffle.clone().winners_distribution.iter().sum::<u32>();
    let total_deposit = query_total_deposit(deps.as_ref(), env)?;

    let res = Response::new();
    let mut winner_addresses = vec![];
    let mut payouts = vec![];

    for counter in 0..raffle.winners_distribution.len() {
        let id_winner = (rng.next_u32() % nb_players) as usize;

        let winner_address = raffle.players[id_winner].to_owned();

        winner_addresses.push(winner_address.clone());

        let reward_per_share = total_deposit.checked_div(Uint128::from(total_shares)).unwrap();
        let reward = reward_per_share.checked_mul(Uint128::from(raffle.winners_distribution[counter])).unwrap();
        payouts.push(reward);

        res.clone().add_message(BankMsg::Send { 
            to_address: winner_address, 
            amount: vec![Coin {
                denom: String::from("ujuno"),
                amount: reward,
            }]
        });
    }

    let data = Raffle {
        id,
        begin_time_stamp: raffle.begin_time_stamp,
        end_time_stamp: raffle.end_time_stamp,
        players: raffle.players,
        winners: winner_addresses,
        minimum_stake: raffle.minimum_stake,
        winners_distribution: raffle.winners_distribution,
        winner_payouts: payouts,
        active: false,
    };

    RAFFLEMAP.save(deps.storage, &id.to_string(), &data)?;

    Ok(res)
}

pub fn is_admin(
    deps: Deps,
    addr: Addr,
) -> Result<bool, ContractError> {
    let admins = ADMINS.load(deps.storage)?;
    let is_admin = admins.contains(&addr);
    Ok(is_admin)
}

fn can_register(deps: Deps, id_lottery: u32) -> Result<bool, ContractError> {
    let raffle = RAFFLEMAP.load(deps.storage, &id_lottery.to_string())?;

    return Ok(raffle.active);
}

fn is_registered(deps: Deps, id_lottery: u32, caller: String) -> Result<bool, ContractError> {
    let raffle = RAFFLEMAP.may_load(deps.storage, &id_lottery.to_string())?;

    if raffle.unwrap().players.contains(&caller) {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub fn new_entropy(info: &MessageInfo, env: &Env, seed: &[u8], entropy: &[u8]) -> [u8; 32] {
    // 16 here represents the lengths in bytes of the block height and time.
    let entropy_len = 16 + info.sender.to_string().len() + entropy.len();
    let mut rng_entropy = Vec::with_capacity(entropy_len);
    rng_entropy.extend_from_slice(&env.block.height.to_be_bytes());
    rng_entropy.extend_from_slice(&info.sender.as_bytes());
    rng_entropy.extend_from_slice(entropy);

    let mut rng = Prng::new(seed, &rng_entropy);

    rng.rand_bytes()
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTotalDeposit { } => to_binary(&query_total_deposit(deps, env)?),
        QueryMsg::GetCount {  } => to_binary(&get_current_counter(deps)?),
        QueryMsg::GetRaffleInfo { id } => to_binary(&get_raffle_info(deps, id)?)
    }
}

pub fn query_total_deposit(deps: Deps, env: Env) -> StdResult<Uint128>{
    let balance = deps.querier.query(
        &QueryRequest::Bank(BankQuery::AllBalances{
            address: env.contract.address.to_string(),
        })
    )?;
    Ok(balance)
}

fn get_current_counter(deps: Deps) -> StdResult<u32> {
    let counter = COUNTER.load(deps.storage)?;
    Ok(counter.counter)
}

fn get_raffle_info(deps:Deps, id: u32) -> StdResult<Raffle> {
    let raffle = RAFFLEMAP.load(deps.storage, &id.to_string())?;
    Ok(raffle)
}

