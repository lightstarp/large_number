use super::{HLarge, Sign};
use std::cmp::Ordering;

impl PartialEq for HLarge {
    fn eq(&self, rhs: &Self) -> bool {
        (self.entry1 == rhs.entry1 && self.entry0 == rhs.entry0)
        && 
        ((self.entry1 == -2 && self.entry0 == 10.0) || self.sign == rhs.sign)
    }
}
impl Eq for HLarge {}

impl PartialOrd for HLarge {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for HLarge {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let x = match (self.sign, rhs.sign) {
            (Sign::Plus , Sign::Plus ) => false,
            (Sign::Minus, Sign::Plus ) => return Ordering::Greater,
            (Sign::Plus , Sign::Minus) => return Ordering::Less,
            (Sign::Minus, Sign::Minus) => true,
        };
        let y = match (self.entry1 == rhs.entry1, self.entry1 > rhs.entry1, self.entry0 == rhs.entry0, self.entry0 > rhs.entry0) {
            (true , _    , true , _    ) => return Ordering::Equal,
            (true , _    , false, t    ) => t,
            (false, t    , _    , _    ) => t,
        };
        match x ^ y {
            true  => Ordering::Greater,
            false => Ordering::Less,
        }
    }
}