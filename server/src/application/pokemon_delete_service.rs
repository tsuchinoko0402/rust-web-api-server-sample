//! ポケモン削除処理のためのアプリケーションサービス。
//! 削除処理のユースケースの振る舞いを定義する。

use crate::domain::models::pokemon::{
    pokemon_number::PokemonNumber, pokemon_repository::PokemonRepository,
};
use anyhow::Result;
use std::convert::TryFrom;

/// アプリケーションサービスの構造体。
/// generics でリポジトリへの依存を表し、trait 境界を定義することで、DI を行う。
pub struct PokemonDeleteService<T>
where
    T: PokemonRepository,
{
    pokemon_repository: T,
}

/// アプリケーションサービスの振る舞いを定義。
impl<T: PokemonRepository> PokemonDeleteService<T> {
    /// コンストラクタ
    pub fn new(pokemon_repository: T) -> Self {
        Self { pokemon_repository }
    }

    // 削除処理の実行。
    pub fn handle(&self, number: i32) -> Result<()> {
        let target_no = PokemonNumber::try_from(number).unwrap();
        match self.pokemon_repository.find_by_number(&target_no) {
            Ok(_) => {
                self.pokemon_repository.delete(&target_no).unwrap();
                Ok(())
            }
            // エラーの場合でも対象のデータが存在しないので、消去は成功しているものとして OK にする。
            Err(_) => Ok(()),
        }
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
            unimplemented!();
        }

        fn update(
            &self,
            _pokemon: &crate::domain::models::pokemon::pokemon::Pokemon,
        ) -> Result<()> {
            unimplemented!();
        }

        fn delete(&self, _number: &PokemonNumber) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn handle_ok_exist_no() {
        let repository = MockPokemonRepositoryImpl::new();
        let service = PokemonDeleteService::new(repository);
        let result = service.handle(1);
        assert!(result.is_ok());
    }

    #[test]
    fn handle_ok_not_exist_no() {
        let repository = MockPokemonRepositoryImpl::new();
        let service = PokemonDeleteService::new(repository);
        let result = service.handle(2);
        assert!(result.is_ok());
    }
}
