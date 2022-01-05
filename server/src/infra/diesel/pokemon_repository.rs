//! Diesel を用いてポケモンのデータをやり取りするためのリポジトリ。

use crate::domain::models::pokemon::{pokemon::Pokemon, pokemon_number::PokemonNumber};
use crate::domain::services::pokemon_repository::PokemonRepository;
use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::convert::TryInto;

/// Diesel が直接利用するデータモデル。
#[derive(Debug, Queryable)]
pub struct PokemonEntity {
    pub no: i32,
    pub name: String,
    pub type_: Vec<String>,
}

/// Pokemon の振る舞い： PokemonEntity から Pokemon への変換処理。
impl From<PokemonEntity> for Pokemon {
    fn from(pokemon: PokemonEntity) -> Pokemon {
        Pokemon {
            number: pokemon.no.try_into().unwrap(),
            name: pokemon.name.try_into().unwrap(),
            types: pokemon.type_.try_into().unwrap(),
        }
    }
}

pub struct PokemonRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl PokemonRepository for PokemonRepositoryImpl {
    /// ポケモンの一覧を出力する
    fn list(&self) -> Result<Vec<Pokemon>> {
        use super::schema::pokemon::dsl::*;

        let conn = self.pool.get()?;
        let res = pokemon.load::<PokemonEntity>(&conn).unwrap();
        Ok(res.into_iter().map(|e| e.into()).collect())
    }

    /// 引数で渡した図鑑 No のポケモンを返却する
    fn find_by_number(&self, number: &PokemonNumber) -> Option<Pokemon> {
        todo!()
    }

    /// ポケモンデータを挿入する
    fn insert(&self, pokemon: &Pokemon) -> Result<()> {
        todo!()
    }

    /// ポケモンデータを更新する
    fn update(&self, pokemon: &Pokemon) -> Result<()> {
        todo!()
    }

    /// ポケモンデータを削除する
    fn delete(&self, pokemon: &Pokemon) -> Result<()> {
        todo!()
    }
}
