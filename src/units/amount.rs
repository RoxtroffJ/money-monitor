use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
/// An amount of money
pub struct Amount {
    /// Amount in euros
    amount: f64
}

impl Amount {
    /// Creates an amount from a value in euros.
    pub fn euro(amount: f64) -> Self {
        Self { amount }
    }

    /// Parses an amount in euro.
    /// 
    /// The input string must be just a float (no €) with a . as decimal separator.
    pub fn parse_euro<S: AsRef<str>>(string: S) -> Option<Self> {
        let amount = string.as_ref().parse().ok()?;
        Some(Self::euro(amount))
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}€", self.amount)
    }
}