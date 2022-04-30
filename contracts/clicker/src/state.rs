use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub length: i32,
    pub name: String,
    pub owner: Addr,
    pub scores: Vec<(Addr, u16)>,
}

pub const STORAGE: Item<State> = Item::new("state");
