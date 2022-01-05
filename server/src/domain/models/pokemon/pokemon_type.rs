//! ポケモンのタイプを表す値オブジェクト。

use std::convert::TryFrom;

/// ポケモンのタイプを表す。
#[derive(PartialEq, Eq, Clone)]
pub enum PokemonType {
    Fire,     // ほのお
    Water,    // みず
    Grass,    // くさ
    Electric, // でんき
    Flying,   // ひこう
}

/// ポケモンのタイプの振る舞い: 文字列からタイプへの変換。
/// 指定の文字列以外は NG とする。
impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Fire" => Ok(Self::Fire),
            "Water" => Ok(Self::Water),
            "Grass" => Ok(Self::Grass),
            "Electric" => Ok(Self::Electric),
            "Flying" => Ok(Self::Flying),
            _ => Err(()),
        }
    }
}

/// ポケモンのタイプから String への変換処理の振る舞いを定義。
impl From<PokemonType> for String {
    fn from(t: PokemonType) -> Self {
        String::from(match t {
            PokemonType::Fire => "Fire",
            PokemonType::Water => "Water",
            PokemonType::Grass => "Grass",
            PokemonType::Electric => "Electric",
            PokemonType::Flying => "Flying",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pokemon_type_try_from_ok_fire() {
        let good_type = String::from("Fire");
        let result = PokemonType::try_from(good_type.clone());
        let expect = Ok(PokemonType::Fire);

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_type_try_from_ok_water() {
        let good_type = String::from("Water");
        let result = PokemonType::try_from(good_type.clone());
        let expect = Ok(PokemonType::Water);

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_type_try_from_ok_grass() {
        let good_type = String::from("Grass");
        let result = PokemonType::try_from(good_type.clone());
        let expect = Ok(PokemonType::Grass);

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_type_try_from_ok_electric() {
        let good_type = String::from("Electric");
        let result = PokemonType::try_from(good_type.clone());
        let expect = Ok(PokemonType::Electric);

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_type_try_from_ok_flying() {
        let good_type = String::from("Flying");
        let result = PokemonType::try_from(good_type.clone());
        let expect = Ok(PokemonType::Flying);

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_type_try_from_ng() {
        let bad_type = String::from("Hoge");
        let result = PokemonType::try_from(bad_type.clone());
        let expect = Err(());

        assert!(result.eq(&expect));
    }
}
