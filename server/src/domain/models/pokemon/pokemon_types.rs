//! ポケモンの複合タイプの値オブジェクト

use crate::domain::models::pokemon::pokemon_type::PokemonType;
use std::convert::TryFrom;

/// ポケモンの複合タイプの定義。
#[derive(PartialEq, Eq, Clone)]
pub struct PokemonTypes(Vec<PokemonType>);

/// ポケモンの複合タイプの振る舞い：Vec<String> から PokemonTypes への変換。
/// タイプに定義されていないものは複合タイプに含めない。
impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.iter() {
                match PokemonType::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(()),
                }
            }
            Ok(Self(pts))
        }
    }
}

/// ポケモンの複合タイプから Vec<String> への変換処理の振る舞いを定義。
impl From<PokemonTypes> for Vec<String> {
    fn from(pts: PokemonTypes) -> Self {
        let mut ts = vec![];
        for pt in pts.0.into_iter() {
            ts.push(String::from(pt));
        }
        ts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pokemon_types_try_from_ok() {
        let good_type1 = String::from("Fire");
        let good_type2 = String::from("Water");
        let good_types = vec![good_type1, good_type2];
        let result = PokemonTypes::try_from(good_types);
        let expect = Ok(PokemonTypes(vec![PokemonType::Fire, PokemonType::Water]));

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_number_try_from_ng() {
        let bad_type1 = String::from("Hoge");
        let bad_type2 = String::from("Moge");
        let bad_types = vec![bad_type1, bad_type2];
        let result = PokemonTypes::try_from(bad_types);
        let expect = Err(());

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_number_try_from_partial_ok() {
        let good_type = String::from("Fire");
        let bad_type = String::from("Hoge");
        let partial_good_types = vec![good_type, bad_type];
        let result = PokemonTypes::try_from(partial_good_types);
        let expect = Err(());

        assert!(result.eq(&expect));
    }
}
