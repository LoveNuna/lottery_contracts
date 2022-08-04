use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Raffle
{
    pub id : i32,
    pub beginTimeStamp : Timestamp,
    pub endTimeStamp : Timestamp,
    pub players: Vec<Addr>,
    pub winners : Vec<Addr>,
    pub minimumStake : i32, // Size per slot
    pub winnerDistribution: Vec<i32>,
    pub winnerPayouts: Vec<i32>,
    pub active: bool,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Counter
{
    pub counter: i32,
}

pub const STATE: Item<Raffle> = Item::new("raffle");
pub const COUNTER: Item<Counter> = Item::new("counter");
