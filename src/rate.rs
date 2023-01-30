use crate::error::Error;
use crate::magnification::Magnification;
use crate::unit::UnitId;

use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Rate {
    from: UnitId,
    to: UnitId,
    magnification: Magnification,
}

pub type RateResult<T> = Result<T, Error>;

impl Rate {
    pub fn new(from: UnitId, to: UnitId, magnification: Magnification) -> Self {
        Self {
            from: from,
            to: to,
            magnification: magnification,
        }
    }
    pub fn is_zero(&self) -> bool {
        self.magnification.is_zero()
    }
}
impl Rate {
    pub fn devide_magnification(&self, rhs: &Self) -> RateResult<Magnification> {
        if rhs.is_zero() {
            panic!("Cannot devide by zero-valued `Rate`!");
        }
        // if self.from != rhs.from {
        //     panic!("Cannot devide by not same from `Unit`");
        // }
        // if self.to != rhs.to {
        //     panic!("Cannot devide by not same to `Unit`");
        // }
        Ok(self.magnification / rhs.magnification)
    }
}
impl PartialEq for Rate {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}
impl Eq for Rate {}
impl PartialOrd for Rate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.from.cmp(&other.from))
    }
}
impl Ord for Rate {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        let table = vec![
            [Less, Less, Less],
            [Less, Equal, Greater],
            [Greater, Greater, Greater],
        ];
        let from_index = 1 + self.from.cmp(&other.from) as i32;
        let to_index = 1 + self.to.cmp(&other.to) as i32;
        table[from_index as usize][to_index as usize]
    }
}
