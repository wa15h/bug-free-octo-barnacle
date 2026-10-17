/// Raw steps in one whole unit, so `Fixed(1_500_000)` is 1.5. Decimal, so a printed ledger shows
/// exact values. Proto and persist will share it; Q10 (open-questions.md) asks if it needs an ADR.
pub const SCALE: i64 = 1_000_000;

/// A fixed-point number, its value times `SCALE`: the sim's positions, weights, and money.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed(pub i64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixedError {
    Overflow,
    DivideByZero,
}

/// Each operation works out the exact answer in `i128`, which cannot overflow (no product of two
/// `i64`s reaches 2^127), and rounds a product or quotient toward zero. An answer outside the `i64`
/// range is `Overflow`, never a panic or a wrap, so debug and release builds agree.
impl Fixed {
    pub fn checked_add(self, rhs: Fixed) -> Result<Fixed, FixedError> {
        narrow(i128::from(self.0) + i128::from(rhs.0))
    }
    pub fn checked_sub(self, rhs: Fixed) -> Result<Fixed, FixedError> {
        narrow(i128::from(self.0) - i128::from(rhs.0))
    }
    pub fn checked_mul(self, rhs: Fixed) -> Result<Fixed, FixedError> {
        narrow(i128::from(self.0) * i128::from(rhs.0) / i128::from(SCALE))
    }
    pub fn checked_div(self, rhs: Fixed) -> Result<Fixed, FixedError> {
        if rhs.0 == 0 {
            return Err(FixedError::DivideByZero);
        }
        narrow(i128::from(self.0) * i128::from(SCALE) / i128::from(rhs.0))
    }
}

fn narrow(exact: i128) -> Result<Fixed, FixedError> {
    match i64::try_from(exact) {
        Ok(raw) => Ok(Fixed(raw)),
        Err(_) => Err(FixedError::Overflow),
    }
}
