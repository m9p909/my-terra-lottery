use std::collections::HashMap;
use std::iter::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use crate::UserModel::{Ticket, User};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
    pub users: HashMap<Addr, User>,
}

pub const STATE: Item<State> = Item::new("state");
