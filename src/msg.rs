use cosmwasm_std::{Addr, Uint128, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
    pub admins: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    BeginRaffleRound { 
        end_time_stamp: Timestamp, 
        minimum_stake: Uint128,
        winners_distribution: Vec<u32>
    },
    JoinRaffleRound {
        id: u32
    },
    EndRaffleRound {id: u32},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    // GetWinner {},
    GetTotalDeposit {},
    GetRaffleInfo { id: u32 },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetCountResponse {
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetWinnerResponse {
    pub winner: Vec<Addr>,
}
