//! ポケモンの図鑑 No の値オブジェクト。
use std::convert::TryFrom;

/// ポケモンの図鑑 No を表す。
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct PokemonNumber(i32);

/// ポケモンの図鑑 No の振る舞い：u16 から PokemonNumber への変換。
/// 現時点でポケモンの図鑑 No は898 までなので、
/// それ以上にならないように決めている。
impl TryFrom<i32> for PokemonNumber {
    type Error = ();

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        if n > 0 && n < 899 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

/// 図鑑 No から u16 への変換処理の振る舞いを定義。
impl From<PokemonNumber> for i32 {
    fn from(n: PokemonNumber) -> Self {
        n.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pokemon_number_try_from_ok() {
        let good_number = 1;
        let result = PokemonNumber::try_from(good_number);
        let expect = Ok(PokemonNumber(1));

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_number_try_from_ng_over_num() {
        let bad_number = 900;
        let result = PokemonNumber::try_from(bad_number);
        let expect = Err(());

        assert!(result.eq(&expect));
    }

    #[test]
    fn pokemon_number_try_from_ng_lower_num() {
        let bad_number = 0;
        let result = PokemonNumber::try_from(bad_number);
        let expect = Err(());

        assert!(result.eq(&expect));
    }
}
