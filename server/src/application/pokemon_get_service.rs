//! ポケモン取得処理のためのアプリケーションサービス。
//! 更新処理のユースケースの振る舞いを定義する。

use super::pokemon_data::PokemonData;
use crate::domain::models::pokemon::pokemon_number::PokemonNumber;
use crate::domain::models::pokemon::pokemon_repository::PokemonRepository;
use anyhow::Result;
use std::convert::TryFrom;

/// アプリケーションサービスの構造体。
/// generics でリポジトリへの依存を表し、trait 境界を定義することで、DI を行う。
pub struct PokemonGetService<T>
where
    T: PokemonRepository,
{
    pokemon_repository: T,
}

/// アプリケーションサービスの振る舞いを定義。
impl<T: PokemonRepository> PokemonGetService<T> {
    /// コンストラクタ
    pub fn new(pokemon_repository: T) -> Self {
        Self { pokemon_repository }
    }

    /// 取得処理の実行。
    pub fn handle(&self, no: i32) -> Result<PokemonData> {
        let number = PokemonNumber::try_from(no).unwrap();
        match self.pokemon_repository.find_by_number(&number) {
            Ok(value) => Ok(PokemonData::new(value)),
            Err(_) => Err(anyhow::anyhow!(
                "取得しようとしたポケモンが存在しません: no {:?}",
                number
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::pokemon::{
        pokemon::Pokemon, pokemon_name::PokemonName, pokemon_types::PokemonTypes,
    };
    use anyhow::Result;
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
            unimplemented!();
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
    fn handle_ok() {
        let repository = MockPokemonRepositoryImpl::new();
        let service = PokemonGetService::new(repository);
        let result = service.handle(1);
        assert!(result.is_ok());

        let result_pokemon = result.unwrap();
        let expect = Pokemon::new(
            PokemonNumber::try_from(1).unwrap(),
            PokemonName::try_from("TestPokemon".to_string()).unwrap(),
            PokemonTypes::try_from(vec!["Fire".to_string()]).unwrap(),
        );
        assert_eq!(result_pokemon, PokemonData::new(expect));
    }

    #[test]
    fn handle_ng() {
        let repository = MockPokemonRepositoryImpl::new();
        let service = PokemonGetService::new(repository);
        let result = service.handle(2);
        assert!(result.is_err());
    }
}
