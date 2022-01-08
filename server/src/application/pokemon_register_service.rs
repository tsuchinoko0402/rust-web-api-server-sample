//! ポケモン登録処理のためのアプリケーションサービス。
//! 登録処理のユースケースの振る舞いを定義する

use anyhow::Result;
use std::convert::TryFrom;

use crate::domain::{
    models::pokemon::{
        pokemon::Pokemon, pokemon_name::PokemonName, pokemon_number::PokemonNumber,
        pokemon_types::PokemonTypes,
    },
    services::pokemon_repository::PokemonRepository,
};

use super::pokemon_data::PokemonData;

/// アプリケーションサービスの構造体。
/// generics でリポジトリへの依存を表し、trait 境界を定義することで、DI を行う。
pub struct PokemonRegisterService<T: PokemonRepository> {
    pokemon_repository: T,
}

impl<T: PokemonRepository> PokemonRegisterService<T> {
    // コンストラクタ
    pub fn new(pokemon_repository: T) -> Self {
        Self { pokemon_repository }
    }

    /// ポケモンの登録処理
    pub fn handle(&self, data: PokemonData) -> Result<()> {
        let pokemon = Pokemon::new(
            PokemonNumber::try_from(data.get_number().clone()).unwrap(),
            PokemonName::try_from(data.get_name().clone()).unwrap(),
            PokemonTypes::try_from(data.get_types().clone()).unwrap(),
        );

        if self.pokemon_repository.exists(&pokemon) {
            return Err(anyhow::anyhow!(
                "作成しようとしたポケモンが既に存在しています。"
            ));
        } else {
            self.pokemon_repository.insert(&pokemon).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::pokemon::{
        pokemon::Pokemon, pokemon_name::PokemonName, pokemon_types::PokemonTypes,
    };
    use std::convert::TryInto;

    /// テストのためのモックリポジトリ
    pub struct MockPokemonRepositoryImpl {}

    impl MockPokemonRepositoryImpl {
        fn new() -> impl PokemonRepository {
            MockPokemonRepositoryImpl {}
        }
    }

    /// モックリポジトリの振る舞い
    impl PokemonRepository for MockPokemonRepositoryImpl {
        fn find_by_number(
            &self,
            number: &PokemonNumber,
        ) -> Result<crate::domain::models::pokemon::pokemon::Pokemon> {
            let target_no: i32 = number.clone().try_into().unwrap();
            match target_no {
                1 => Ok(Pokemon::new(
                    number.clone(),
                    PokemonName::try_from("TestPokemon".to_string()).unwrap(),
                    PokemonTypes::try_from(vec!["Fire".to_string()]).unwrap(),
                )),
                _ => Err(anyhow::anyhow!("Dummy Error")),
            }
        }

        fn list(&self) -> Result<Vec<crate::domain::models::pokemon::pokemon::Pokemon>> {
            unimplemented!();
        }

        fn insert(
            &self,
            _pokemon: &crate::domain::models::pokemon::pokemon::Pokemon,
        ) -> Result<()> {
            Ok(())
        }

        fn update(
            &self,
            _pokemon: &crate::domain::models::pokemon::pokemon::Pokemon,
        ) -> Result<()> {
            unimplemented!();
        }

        fn delete(&self, _number: &PokemonNumber) -> Result<()> {
            unimplemented!();
        }
    }

    #[test]
    fn handle_ok_not_exist_no() {
        let repository = MockPokemonRepositoryImpl::new();
        let service = PokemonRegisterService::new(repository);
        let data = Pokemon::new(
            PokemonNumber::try_from(2).unwrap(),
            PokemonName::try_from("TestPokemon".to_string()).unwrap(),
            PokemonTypes::try_from(vec!["Fire".to_string()]).unwrap(),
        );
        let result = service.handle(PokemonData::new(data));
        assert!(result.is_ok());
    }

    #[test]
    fn handle_ng_exist_no() {
        let repository = MockPokemonRepositoryImpl::new();
        let service = PokemonRegisterService::new(repository);
        let data = Pokemon::new(
            PokemonNumber::try_from(1).unwrap(),
            PokemonName::try_from("TestPokemon".to_string()).unwrap(),
            PokemonTypes::try_from(vec!["Fire".to_string()]).unwrap(),
        );
        let result = service.handle(PokemonData::new(data));
        assert!(result.is_err());
    }
}
