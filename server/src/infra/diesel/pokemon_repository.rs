//! Diesel を用いてポケモンのデータをやり取りするためのリポジトリ。

use super::schema::pokemon;
use super::schema::pokemon::dsl::*;
use crate::domain::models::pokemon::{
    pokemon::Pokemon, pokemon_number::PokemonNumber, pokemon_repository::PokemonRepository,
};
use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::convert::TryInto;

/// Diesel が直接利用するデータモデル。
#[derive(Debug, Queryable, Clone)]
pub struct PokemonEntity {
    pub no: i32,
    pub name: String,
    pub type_: Vec<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "pokemon"]
pub struct NewPokemon {
    pub no: i32,
    pub name: String,
    pub type_: Vec<String>,
}

/// Pokemon の振る舞い： PokemonEntity から Pokemon への変換処理。
impl From<PokemonEntity> for Pokemon {
    fn from(entity: PokemonEntity) -> Pokemon {
        Pokemon {
            number: entity.no.try_into().unwrap(),
            name: entity.name.try_into().unwrap(),
            types: entity.type_.try_into().unwrap(),
        }
    }
}

pub struct PokemonRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl PokemonRepository for PokemonRepositoryImpl {
    /// ポケモンの一覧を出力する
    fn list(&self) -> Result<Vec<Pokemon>> {
        let conn = self.pool.get().context("failed to get connection")?;
        match pokemon.load::<PokemonEntity>(&conn) {
            Ok(result) => match result.len() {
                0 => Err(anyhow::anyhow!("Not Found Pokemon List")),
                _ => Ok(result
                    .iter()
                    .map(|c| Pokemon::from(c.clone()))
                    .collect::<Vec<Pokemon>>()),
            },
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    /// 引数で渡した図鑑 No のポケモンを返却する
    fn find_by_number(&self, number: &PokemonNumber) -> Result<Pokemon> {
        let conn = self.pool.get().context("failed to get connection")?;
        let target_num: i32 = number.clone().try_into().unwrap();
        match pokemon
            .filter(pokemon::no.eq(target_num))
            .load::<PokemonEntity>(&conn)
        {
            Ok(result) => match result.get(0) {
                Some(value) => Ok(Pokemon::from(value.clone())),
                None => Err(anyhow::anyhow!("Not Found Pokemon number:{}", target_num)),
            },
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    /// ポケモンデータを挿入する
    fn insert(&self, data: &Pokemon) -> Result<()> {
        let conn = self.pool.get().context("failed to get connection")?;
        let new_pokemon = NewPokemon {
            no: data.number.clone().try_into().unwrap(),
            name: data.name.clone().try_into().unwrap(),
            type_: data.types.clone().try_into().unwrap(),
        };

        diesel::insert_into(pokemon::table)
            .values(&new_pokemon)
            .execute(&conn)
            .expect("Error saving new pokemon");
        Ok(())
    }

    /// ポケモンデータを更新する
    fn update(&self, data: &Pokemon) -> Result<()> {
        let conn = self.pool.get().context("failed to get connection")?;
        let target_number: i32 = data.number.clone().try_into().unwrap();
        let target_name: String = data.name.clone().try_into().unwrap();
        let target_types: Vec<String> = data.types.clone().try_into().unwrap();
        diesel::update(pokemon.find(target_number))
            .set((name.eq(target_name), type_.eq(target_types)))
            .execute(&conn)
            .expect(&format!("Unable to find pokemon {}", target_number));
        Ok(())
    }

    /// ポケモンデータを削除する
    fn delete(&self, number: &PokemonNumber) -> Result<()> {
        let conn = self.pool.get().context("failed to get connection")?;
        let target_number: i32 = number.clone().try_into().unwrap();
        diesel::delete(pokemon.find(target_number))
            .execute(&conn)
            .expect("Error deleting pokemon");
        Ok(())
    }
}
