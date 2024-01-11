use std::fmt::Display;
use std::str::FromStr;
use std::ops::{Add, Sub, Mul, Div};

use bigdecimal::BigDecimal;
use num_traits::Pow;

#[derive(Clone, Debug)]
pub enum Number {
    Int(i64),
    Decimal(Box<BigDecimal>)
}


impl Number {

    pub fn parse(input: &str) -> Self {
        if input.contains('.') {
            Number::Decimal(Box::new(BigDecimal::from_str(input).unwrap()))
        } else {
            match input.parse() {
                Ok(n) => Number::Int(n),
                Err(_) => Number::Decimal(Box::new(BigDecimal::from_str(input).unwrap()))
            }
        }
    }

    pub fn new(input: i64) -> Self {
        Self::Int(input)
    }

}


impl PartialEq<Number> for Number {

    fn eq(&self, other: &Number) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Decimal(l0), Self::Decimal(r0)) => l0 == r0,
            (Self::Int(l0), Self::Decimal(box r0)) => BigDecimal::from(*l0) == *r0,
            (Self::Decimal(box l0), Self::Int(r0)) => *l0 == BigDecimal::from(*r0)
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(n) => write!(f, "{}", n),
            Number::Decimal(box d) => write!(f, "{}", d.to_string()),
        }
    }
}

impl Pow<Number> for Number {
    type Output = Number;

    fn pow(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            //TODO: Handle powers of negative numbers elegantly
            (Number::Int(a), Number::Int(b)) => match a.checked_pow(b as u32) {
                Some(r) => Number::Int(r),
                None => Number::Decimal(Box::new(BigDecimal::from(a))), //TODO: Powers dont have method for this lirary, might have to go back to drawing board
            },
            (Number::Int(_), Number::Decimal(_)) => todo!(),
            (Number::Decimal(_), Number::Int(_)) => todo!(),
            (Number::Decimal(_), Number::Decimal(_)) => panic!("Powers of Big Decimals is not yet implemented"),
        }
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Int(a), Number::Int(b)) => match a.checked_mul(b) {
                Some(r) => Number::Int(r),
                None => Number::Decimal(Box::new(BigDecimal::from(a) * BigDecimal::from(b)))
            },
            (Number::Int(a), Number::Decimal(box b)) => Number::Decimal(Box::new(BigDecimal::from(a) * b)),
            (Number::Decimal(box a), Number::Int(b)) => Number::Decimal(Box::new(BigDecimal::from(b) * a)),
            (Number::Decimal(box a), Number::Decimal(box b)) => Number::Decimal(Box::new(a * b)),
        }
    }
}

impl Div<Number> for Number {
    type Output = Number;

    // May need to revisit this one
    fn div(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Int(a), Number::Int(b)) => if a % b == 0 { Number::Int(a / b) } else { Number::Decimal(Box::new(BigDecimal::from(a) / BigDecimal::from(b))) },
            (Number::Int(a), Number::Decimal(box b)) => Number::Decimal(Box::new(BigDecimal::from(a) / b)),
            (Number::Decimal(box a), Number::Int(b)) => Number::Decimal(Box::new(BigDecimal::from(b) / a)),
            (Number::Decimal(box a), Number::Decimal(box b)) => Number::Decimal(Box::new(a / b)),
        }
    }
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Int(a), Number::Int(b)) => match a.checked_add(b) {
                Some(r) => Number::Int(r),
                None => Number::Decimal(Box::new(BigDecimal::from(a) + BigDecimal::from(b)))
            },
            (Number::Int(a), Number::Decimal(box b)) => Number::Decimal(Box::new(BigDecimal::from(a) + b)),
            (Number::Decimal(box a), Number::Int(b)) => Number::Decimal(Box::new(BigDecimal::from(b) + a)),
            (Number::Decimal(box a), Number::Decimal(box b)) => Number::Decimal(Box::new(a + b)),
        }
    }
}


impl Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Int(a), Number::Int(b)) => match a.checked_sub(b) {
                Some(r) => Number::Int(r),
                None => Number::Decimal(Box::new(BigDecimal::from(a) - BigDecimal::from(b)))
            },
            (Number::Int(a), Number::Decimal(box b)) => Number::Decimal(Box::new(BigDecimal::from(a) - b)),
            (Number::Decimal(box a), Number::Int(b)) => Number::Decimal(Box::new(BigDecimal::from(b) - a)),
            (Number::Decimal(box a), Number::Decimal(box b)) => Number::Decimal(Box::new(a - b)),
        }
    }
}



