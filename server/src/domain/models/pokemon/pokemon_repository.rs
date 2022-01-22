//! ポケモンに関するドメインサービスを定義する。

use crate::domain::models::pokemon::{pokemon::Pokemon, pokemon_number::PokemonNumber};
use anyhow::Result;

/// Pokemon のリポジトリインタフェース
pub trait PokemonRepository {
    /// 番号からポケモンを探す
    fn find_by_number(&self, number: &PokemonNumber) -> Result<Pokemon>;

    /// ポケモン一覧を表示する
    fn list(&self) -> Result<Vec<Pokemon>>;

    /// オブジェクトを永続化（保存）する振る舞い
    fn insert(&self, pokemon: &Pokemon) -> Result<()>;

    /// オブジェクトを再構築する振る舞い
    fn update(&self, pokemon: &Pokemon) -> Result<()>;

    /// オブジェクトを永続化（破棄）する振る舞い
    fn delete(&self, number: &PokemonNumber) -> Result<()>;

    /// 作成したポケモンの重複確認を行う。
    fn exists(&self, pokemon: &Pokemon) -> bool {
        match self.find_by_number(&pokemon.number) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
