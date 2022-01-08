//! ポケモン取得処理のためのアプリケーションサービス。
//! 更新処理のユースケースの振る舞いを定義する

use super::pokemon_data::PokemonData;
use crate::domain::services::pokemon_repository::PokemonRepository;
use anyhow::Result;

/// アプリケーションサービスの構造体。
/// generics でリポジトリへの依存を表し、trait 境界を定義することで、DI を行う
pub struct PokemonListService<T>
where
    T: PokemonRepository,
{
    pokemon_repository: T,
}

impl<T: PokemonRepository> PokemonListService<T> {
    /// コンストラクタ
    pub fn new(pokemon_repository: T) -> Self {
        Self { pokemon_repository }
    }

    /// 登録されているポケモンの一覧を表示
    pub fn handle(&self) -> Result<Vec<PokemonData>> {
        match self.pokemon_repository.list() {
            Ok(value) => Ok(value
                .iter()
                .map(|c| PokemonData::new(c.clone()))
                .collect::<Vec<PokemonData>>()),
            Err(_) => Err(anyhow::anyhow!("登録されたポケモンが1つもありません。")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::pokemon::{
        pokemon::Pokemon, pokemon_name::PokemonName, pokemon_number::PokemonNumber,
        pokemon_types::PokemonTypes,
    };
    use std::convert::TryFrom;

    /// テストのためのモックリポジトリ
    pub struct OkMockPokemonRepositoryImpl {}

    impl OkMockPokemonRepositoryImpl {
        fn new() -> impl PokemonRepository {
            OkMockPokemonRepositoryImpl {}
        }
    }

    /// OK モックリポジトリの振る舞い
    impl PokemonRepository for OkMockPokemonRepositoryImpl {
        fn find_by_number(
            &self,
            _number: &PokemonNumber,
        ) -> Result<crate::domain::models::pokemon::pokemon::Pokemon> {
            unimplemented!();
        }

        fn list(&self) -> Result<Vec<crate::domain::models::pokemon::pokemon::Pokemon>> {
            let pokemon_1 = Pokemon::new(
                PokemonNumber::try_from(1).unwrap(),
                PokemonName::try_from("TestPokemon1".to_string()).unwrap(),
                PokemonTypes::try_from(vec!["Fire".to_string()]).unwrap(),
            );
            let pokemon_2 = Pokemon::new(
                PokemonNumber::try_from(2).unwrap(),
                PokemonName::try_from("TestPokemon2".to_string()).unwrap(),
                PokemonTypes::try_from(vec!["Water".to_string()]).unwrap(),
            );
            let result = vec![pokemon_1, pokemon_2];
            Ok(result)
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

    /// NG テストのためのモックリポジトリ
    pub struct NgMockPokemonRepositoryImpl {}

    impl NgMockPokemonRepositoryImpl {
        fn new() -> impl PokemonRepository {
            NgMockPokemonRepositoryImpl {}
        }
    }

    /// モックリポジトリの振る舞い
    impl PokemonRepository for NgMockPokemonRepositoryImpl {
        fn find_by_number(
            &self,
            _number: &PokemonNumber,
        ) -> Result<crate::domain::models::pokemon::pokemon::Pokemon> {
            unimplemented!();
        }

        fn list(&self) -> Result<Vec<crate::domain::models::pokemon::pokemon::Pokemon>> {
            Err(anyhow::anyhow!("Dummy Error"))
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
        let repository = OkMockPokemonRepositoryImpl::new();
        let service = PokemonListService::new(repository);
        let pokemon_1 = PokemonData::new(Pokemon::new(
            PokemonNumber::try_from(1).unwrap(),
            PokemonName::try_from("TestPokemon1".to_string()).unwrap(),
            PokemonTypes::try_from(vec!["Fire".to_string()]).unwrap(),
        ));
        let pokemon_2 = PokemonData::new(Pokemon::new(
            PokemonNumber::try_from(2).unwrap(),
            PokemonName::try_from("TestPokemon2".to_string()).unwrap(),
            PokemonTypes::try_from(vec!["Water".to_string()]).unwrap(),
        ));
        let expect = vec![pokemon_1, pokemon_2];
        let result = service.handle();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn handle_ng() {
        let repository = NgMockPokemonRepositoryImpl::new();
        let service = PokemonListService::new(repository);
        let result = service.handle();
        assert!(result.is_err());
    }
}
