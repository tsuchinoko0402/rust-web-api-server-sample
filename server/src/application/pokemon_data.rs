//! ポケモンのドメインオブジェクトのための DTO

use std::convert::TryInto;

use serde::{Serialize, Deserialize};

use crate::domain::models::pokemon::pokemon::Pokemon;

#[derive(Serialize, Deserialize, Clone)]
pub struct PokemonData {
    number: i32,
    name: String,
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

    pub fn get_number(&self) -> i32 {
        self.number
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_types(&self) -> &Vec<String> {
        &self.types
    }
}
