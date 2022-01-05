//! ポケモンに関するアプリケーションサービスを定義する

use anyhow::Result;
use getset::{Getters, Setters};
use std::convert::TryFrom;

use crate::domain::{
    models::pokemon::{
        pokemon::Pokemon, pokemon_name::PokemonName, pokemon_number::PokemonNumber,
        pokemon_types::PokemonTypes,
    },
    services::pokemon_repository::PokemonRepository,
};

use super::pokemon_data::PokemonData;

pub struct PokemonApplicationService<T: PokemonRepository> {
    pokemon_repository: T,
}

impl<T: PokemonRepository> PokemonApplicationService<T> {
    pub fn new(pokemon_repository: T) -> Self {
        Self { pokemon_repository }
    }

    /// ポケモンの登録処理
    pub fn register(&self, no: i32, name: String, types: Vec<String>) -> Result<()> {
        let pokemon = Pokemon::new(
            PokemonNumber::try_from(no).unwrap(),
            PokemonName::try_from(name).unwrap(),
            PokemonTypes::try_from(types).unwrap(),
        );

        if self.pokemon_repository.exists(&pokemon) {
            return Err(anyhow::anyhow!(
                "作成しようとしたポケモンが既に存在しています。"
            ));
        } else {
            self.pokemon_repository.insert(&pokemon);
        }
        Ok(())
    }

    /// ポケモン情報の取得処理
    pub fn get(&self, no: i32) -> Result<PokemonData> {
        let number = PokemonNumber::try_from(no).unwrap();
        match self.pokemon_repository.find_by_number(&number) {
            Some(value) => Ok(PokemonData::new(value)),
            None => Err(anyhow::anyhow!("取得しようとしたポケモンが存在しません。")),
        }
    }

    /// ポケモンデータの更新処理
    pub fn update(&self, command: PokemonUpdateCommand) -> Result<()> {
        let target_no = PokemonNumber::try_from(*command.get_number()).unwrap();
        match self.pokemon_repository.find_by_number(&target_no) {
            Some(mut result) => {
                result.name = match command.get_name() {
                    Some(value) => PokemonName::try_from(value.clone()).unwrap(),
                    None => PokemonName::try_from(String::from("名前未設定")).unwrap(),
                };
                result.types = match command.get_types() {
                    Some(value) => PokemonTypes::try_from(value.clone()).unwrap(),
                    None => PokemonTypes::try_from(vec![String::from("タイプ未設定")]).unwrap(),
                };
                self.pokemon_repository.update(&result).unwrap();
                Ok(())
            }
            None => Err(anyhow::anyhow!("更新しようとしたポケモンが存在しません。")),
        }
    }

    /// ポケモンデータの消去処理
    pub fn delete(&self, command: PokemonDeleteCommand) -> Result<()> {
        let target_no = PokemonNumber::try_from(*command.get_number()).unwrap();
        match self.pokemon_repository.find_by_number(&target_no) {
            Some(result) => {
                self.pokemon_repository.delete(&result).unwrap();
                Ok(())
            }
            None => Ok(()),
        }
    }
}

#[derive(Getters, Setters)]
/// ポケモン情報のアップデートコマンドオブジェクト
pub struct PokemonUpdateCommand {
    #[getset(get = "pub with_prefix")]
    number: i32,
    #[getset(get = "pub with_prefix", set = "pub with_prefix")]
    name: Option<String>,
    #[getset(get = "pub with_prefix", set = "pub with_prefix")]
    types: Option<Vec<String>>,
}

impl PokemonUpdateCommand {
    pub fn new(number: i32) -> Self {
        Self {
            number,
            name: None,
            types: None,
        }
    }
}

#[derive(Getters, Setters)]
/// ポケモン情報の消去コマンドオブジェクト
pub struct PokemonDeleteCommand {
    #[getset(get = "pub with_prefix")]
    number: i32,
}

impl PokemonDeleteCommand {
    pub fn new(number: i32) -> Self {
        Self { number }
    }
}
