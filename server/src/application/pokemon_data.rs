//! ポケモンのドメインオブジェクトのための DTO

use crate::domain::models::pokemon::pokemon::Pokemon;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Clone, Getters, PartialEq, Eq, Debug)]
pub struct PokemonData {
    #[getset(get = "pub with_prefix")]
    number: i32,
    #[getset(get = "pub with_prefix")]
    name: String,
    #[getset(get = "pub with_prefix")]
    types: Vec<String>,
}

impl PokemonData {
    pub fn new(source: Pokemon) -> Self {
        Self {
            number: source.number.try_into().unwrap(),
            name: source.name.try_into().unwrap(),
            types: source.types.try_into().unwrap(),
        }
    }
}
