use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub merkle_root: String,
    pub owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const CLAIMED_ADDRESSES: Map<&str, bool> = Map::new("claimed_addresses");
