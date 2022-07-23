use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub struct Raffle
{
    pub id : i32,
    pub beginTimeStamp : i64,
    pub endTimeStamp : i64,
    pub players: Vec<Addr>,
    pub winner : Vec<Addr>,
    pub minimumStake : i32,

}

pub const STATE: Item<State> = Item::new("state");
