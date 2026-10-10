//! New si dimension for amount of money. 

use std::num::ParseFloatError;

use extended_typenum::rational;
use rust_units::{Quantity, Unit, si_system::units::{SIPropUnit, SimpleSIPropUnit}};
use typenum::{P1, Z0};

use crate::units::Money;

/// Euro currency unit.
pub const EURO: SimpleSIPropUnit<Money, rational!(P1), Z0, &'static str> = SIPropUnit::new("€");

/// An amount of money.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Amount {
    qty: Quantity<f64, Money>
}

impl Amount {
    /// Creates a new amount of money
    pub fn new<U>(value: f64, unit: &U) -> Self
    where U: Unit<f64, Dimension = Money>
    {
        Self {
            qty: Quantity::from(value, unit)
        }
    }

    /// Parses a string into an amount of money, in euros.
    pub fn parse_euro(string: &str) -> Result<Self, ParseFloatError> 
    {
        let value = string.parse::<f64>()?;
        Ok(Self::new(value, &EURO))
    }
}