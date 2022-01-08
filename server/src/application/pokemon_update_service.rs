//! ポケモン更新処理のためのアプリケーションサービス。
//! 更新処理のユースケースの振る舞いを定義する。

use crate::domain::models::pokemon::{
    pokemon::Pokemon, pokemon_name::PokemonName, pokemon_number::PokemonNumber,
    pokemon_types::PokemonTypes,
};
use crate::domain::services::pokemon_repository::PokemonRepository;
use anyhow::Result;
use getset::{Getters, Setters};
use std::convert::TryFrom;

/// アプリケーションサービスの構造体。
/// generics でリポジトリへの依存を表し、trait 境界を定義することで、DI を行う。
pub struct PokemonUpdateService<T>
where
    T: PokemonRepository,
{
    pokemon_repository: T,
}

/// アプリケーションサービスの振る舞いを定義。
impl<T: PokemonRepository> PokemonUpdateService<T> {
    /// コンストラクタ
    pub fn new(pokemon_repository: T) -> Self {
        Self { pokemon_repository }
    }

    /// 更新処理の実行。
    pub fn handle(&self, command: PokemonUpdateCommand) -> Result<Pokemon> {
        let target_no = PokemonNumber::try_from(*command.get_number()).unwrap();
        match self.pokemon_repository.find_by_number(&target_no) {
            Ok(mut result) => {
                result.name = match command.get_name() {
                    Some(value) => PokemonName::try_from(value.clone()).unwrap(),
                    None => PokemonName::try_from(String::from("名前未設定")).unwrap(),
                };
                result.types = match command.get_types() {
                    Some(value) => PokemonTypes::try_from(value.clone()).unwrap(),
                    None => PokemonTypes::try_from(vec![String::from("Unknown")]).unwrap(),
                };
                self.pokemon_repository.update(&result).unwrap();
                Ok(result)
            }
            Err(_) => Err(anyhow::anyhow!(
                "更新しようとしたポケモンが存在しません: no {:?}",
                target_no
            )),
        }
    }
}

/// ポケモン情報のアップデートコマンドオブジェクト
#[derive(Getters, Setters)]
pub struct PokemonUpdateCommand {
    #[getset(get = "pub with_prefix")]
    number: i32,
    #[getset(get = "pub with_prefix", set = "pub with_prefix")]
    name: Option<String>,
    #[getset(get = "pub with_prefix", set = "pub with_prefix")]
    types: Option<Vec<String>>,
}

/// ポケモン情報のアップデートコマンドオブジェクトの振る舞いを定義
impl PokemonUpdateCommand {
    /// コンストラクタ
    pub fn new(number: i32) -> Self {
        Self {
            number,
            name: None,
            types: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::pokemon::pokemon::Pokemon;
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
            Ok(())
        }

        fn delete(&self, _number: &PokemonNumber) -> Result<()> {
            unimplemented!();
        }
    }

    #[test]
    fn handle_ok_no_name_and_no_type() {
        let repository = MockPokemonRepositoryImpl::new();
        let service = PokemonUpdateService::new(repository);
        let command = PokemonUpdateCommand::new(1);
        let result = service.handle(command);
        assert!(result.is_ok());

        let result_pokemon = result.unwrap();
        let expect = Pokemon::new(
            PokemonNumber::try_from(1).unwrap(),
            PokemonName::try_from("名前未設定".to_string()).unwrap(),
            PokemonTypes::try_from(vec!["Unknown".to_string()]).unwrap(),
        );
        assert_eq!(result_pokemon, expect);
    }

    #[test]
    fn handle_ok_set_name_and_set_type() {
        let repository = MockPokemonRepositoryImpl::new();
        let service = PokemonUpdateService::new(repository);
        let mut command = PokemonUpdateCommand::new(1);
        command.set_name(Some("TestName".to_string()));
        command.set_types(Some(vec!["Fire".to_string()]));
        let result = service.handle(command);
        assert!(result.is_ok());

        let result_pokemon = result.unwrap();
        let expect = Pokemon::new(
            PokemonNumber::try_from(1).unwrap(),
            PokemonName::try_from("TestName".to_string()).unwrap(),
            PokemonTypes::try_from(vec!["Fire".to_string()]).unwrap(),
        );
        assert_eq!(result_pokemon, expect);
    }

    #[test]
    fn handle_ng() {
        let repository = MockPokemonRepositoryImpl::new();
        let service = PokemonUpdateService::new(repository);
        let command = PokemonUpdateCommand::new(2);
        let result = service.handle(command);
        assert!(result.is_err());
    }
}
