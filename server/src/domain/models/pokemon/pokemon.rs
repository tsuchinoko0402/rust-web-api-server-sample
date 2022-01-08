//! ポケモンのエンティティの定義

use crate::domain::models::pokemon::{
    pokemon_name::PokemonName, pokemon_number::PokemonNumber, pokemon_types::PokemonTypes,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Pokemon {
    pub number: PokemonNumber,
    pub name: PokemonName,
    pub types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}
