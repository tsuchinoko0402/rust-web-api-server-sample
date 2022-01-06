use std::convert::TryInto;

use crate::domain::models::pokemon::pokemon::Pokemon;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
pub struct PokemonRequest {
    pub number: i32,
    pub name: String,
    pub types: Vec<String>,
}

impl PokemonRequest {
    pub fn of(&self) -> Pokemon {
        Pokemon::new(
            self.number.try_into().unwrap(),
            self.name.clone().try_into().unwrap(),
            self.types.clone().try_into().unwrap(),
        )
    }
}
