use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::Item;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Raffle
{
    pub id : i32,
    pub beginTimeStamp : Timestamp,
    pub endTimeStamp : Timestamp,
    pub players: Vec<Addr>,
    pub winners : Vec<Addr>,
    pub minimumStake : u128, // Size per slot
    pub winnersDistribution: Vec<i32>,
    pub winnerPayouts: Vec<i32>,
    pub active: bool,
}

impl fmt::Display for Raffle
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Raffle {{ id: {}, beginTimeStamp: {}, endTimeStamp: {}, players: {}, winners: {}, minimumStake: {}, winnersDistribution: {}, winnerPayouts: {}, active: {} }}", self.id, self.beginTimeStamp, self.endTimeStamp, self.players, self.winners, self.minimumStake, self.winnersDistribution, self.winnerPayouts, self.active)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Counter
{
    pub counter: i32,
}

pub const STATE: Item<Raffle> = Item::new("raffle");
pub const COUNTER: Item<Counter> = Item::new("counter");
pub const RAFFLEMAP: Map<&str, Raffle> = Map::new("escrow");
