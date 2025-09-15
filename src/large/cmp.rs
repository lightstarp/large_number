use super::Lnum;
use std::cmp::Ordering;

// impl PartialEq for Lnum {
//     fn eq(&self, rhs: &Self) -> bool {
//         self.tetra == rhs.tetra && self.unit == rhs.unit
//     }
// }

impl PartialOrd for Lnum {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let x = match (self.sign, rhs.sign) {
            (1.0, 1.0) => false,
            (1.0, _  ) => return Some(Ordering::Greater),
            (_  , 1.0) => return Some(Ordering::Less),
            (_  , _  ) => true,
        };
        let y = match (self.tetra.cmp(&rhs.tetra), self.unit.partial_cmp(&rhs.unit)) {
            (Ordering::Equal  , None                   ) => return None,
            (Ordering::Equal  , Some(Ordering::Equal)  ) => return Some(Ordering::Equal),
            (Ordering::Equal  , Some(Ordering::Greater)) => true,
            (Ordering::Equal  , Some(Ordering::Less)   ) => false,
            (Ordering::Greater, _                      ) => true,
            (Ordering::Less   , _                      ) => false,
        };
        match x ^ y {
            true  => Some(Ordering::Greater),
            false => Some(Ordering::Less),
        }
    }
}

impl Lnum {
    pub fn max(self, rhs: Lnum) -> Lnum {
        if self > rhs {
            self
        } else {
            rhs
        }
    }
    pub fn min(self, rhs: Lnum) -> Lnum {
        if self > rhs {
            rhs
        } else {
            self
        }
    }
}