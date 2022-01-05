//! ポケモンの名前を表す値オブジェクト。

use std::convert::TryFrom;

/// ポケモンの名前を表す。
#[derive(PartialEq, Eq, Clone)]
pub struct PokemonName(String);

/// ポケモンの名前の振る舞い：String から PokemonName への変換。
/// 名前は空白の場合を NG としている。
impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.is_empty() {
            Err(())
        } else {
            Ok(Self(name))
        }
    }
}

/// String からポケモン名への変換処理の振る舞いを定義。
impl From<PokemonName> for String {
    fn from(n: PokemonName) -> Self {
        n.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pokemon_name_try_from_ok() {
        let good_name = String::from("hogehoge");
        let result = PokemonName::try_from(good_name.clone());
        let expect = Ok(PokemonName(good_name.clone()));

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_name_try_from_ng() {
        let bad_name = String::from("");
        let result = PokemonName::try_from(bad_name);
        let expect = Err(());

        assert!(result.eq(&expect));
    }
}
