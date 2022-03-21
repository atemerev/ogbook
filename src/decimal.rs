use std::fmt;
use std::fmt::Formatter;

pub struct Decimal(f64);

impl Decimal {
    pub fn safe(value: f64) -> Self {
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
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}