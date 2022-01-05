//! Diesel を用いてポケモンのデータをやり取りするためのリポジトリ。

use super::schema::pokemon;
use super::schema::pokemon::dsl::*;
use crate::domain::models::pokemon::{pokemon::Pokemon, pokemon_number::PokemonNumber};
use crate::domain::services::pokemon_repository::PokemonRepository;
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
        let conn = self.pool.get()?;
        let res = pokemon.load::<PokemonEntity>(&conn).unwrap();
        Ok(res.into_iter().map(|e| e.into()).collect())
    }

    /// 引数で渡した図鑑 No のポケモンを返却する
    fn find_by_number(&self, number: &PokemonNumber) -> Option<Pokemon> {
        let conn = self.pool.get().unwrap();
        let target_num: i32 = number.clone().try_into().unwrap();
        match pokemon
            .filter(pokemon::no.eq(target_num))
            .load::<PokemonEntity>(&conn)
        {
            Ok(result) => match result.get(0) {
                Some(value) => Some(Pokemon::from(value.clone())),
                None => None,
            },
            Err(_) => None,
        }
    }

    /// ポケモンデータを挿入する
    fn insert(&self, data: &Pokemon) -> Result<()> {
        let conn = self.pool.get()?;
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
        todo!()
    }

    /// ポケモンデータを削除する
    fn delete(&self, data: &Pokemon) -> Result<()> {
        todo!()
    }
}
