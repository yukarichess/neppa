use std::{fmt::Display, num::NonZeroU8, str::FromStr};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Square(NonZeroU8);

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const RANKS: [char; 9] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
        let square = self.into_inner();
        let file = square % 9;
        let rank = square / 9;
        write!(f, "{file}{}", RANKS[rank as usize])
    }
}

impl FromStr for Square {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(());
        }
        let chars = s.as_bytes();
        if !(b'1'..=b'9').contains(&chars[0]) {
            return Err(());
        }
        if !(b'a'..=b'i').contains(&chars[1]) {
            return Err(());
        }
        let file = chars[0] - b'1';
        let rank = chars[1] - b'a';
        // SAFETY: values are constrained above and the "plus one" ensures this will never be zero.
        let square = unsafe { NonZeroU8::new_unchecked((9 * rank + file) + 1) };
        Ok(Self(square))
    }
}

impl Square {
    pub const COUNT: usize = 81;

    pub fn into_inner(self) -> u8 {
        u8::from(self.0) - 1
    }
}
