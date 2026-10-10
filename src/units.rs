//! Module to manage units for money and time

use rust_units::{si_add_dim, si_system::SISystemWithAngles};

mod amount;
mod date;

pub use amount::*;
pub use date::*;


si_add_dim! { SISystemWithAngles =>
    (
        /// Dimension for an amount of money.
        /// 
        /// The default unit is the euro (€).
        pub Money,

        /// Dimension ID for an amount of money.
        pub MoneyID; 
        "€"
    )

    = 
    /// The extended system of units used in this crate.
    pub DimSystem
}