use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Decimal(f64);

impl Decimal {
    pub fn new(value: f64) -> Self {
        const FACTOR: f64 = 1e8;
        return if value > (f64::MAX / FACTOR) || value < (f64::MIN / FACTOR) {
            Decimal(value)
        } else {
            let pre = if value < 0.0 { value * FACTOR - 0.5 } else { value * FACTOR + 0.5 };
            Decimal(pre.floor() / FACTOR)
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.0 == other.0 { Ordering::Equal }
        else if self.0 < other.0 { Ordering::Less }
        else { Ordering::Greater };
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

impl Eq for Decimal {
}

impl Clone for Decimal {
    fn clone(&self) -> Self {
        Decimal(self.0)
    }
}

impl Copy for Decimal {}

impl Debug for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}