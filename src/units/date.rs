#[derive(Debug, Clone)]
/// A date of the year
pub struct Date {
    day: u8,
    month: Month,
    year: u16,
}

#[derive(Debug, Clone, Copy)]
/// The month of the year
pub enum Month {
    /// January (01)
    January,
    /// February (02)
    February,
    /// March (03)
    March,
    /// April (04)
    April,
    /// May (05)
    May,
    /// June (06)
    June,
    /// July (07)
    July,
    /// August (08)
    August,
    /// September (09)
    September,
    /// October (10)
    October,
    /// November (11)
    November,
    /// December (12)
    December,
}

impl Date {
    /// Builds a date from day month and year.
    ///
    /// Fails if the date does not exist in the calendar.
    pub fn new(day: u8, month: Month, year: u16) -> Result<Self, String> {
        let date = Self { day, month, year };
        let max_day = date.nb_days_in_month(month);
        if day > max_day {
            Err(format!(
                "In the year {year}, the month {month} has {max_day} days. {date} does not exist"
            ))
        } else if day < 1 {
            Err(format!("Day {day} does not exist. Days start at 1."))
        } else {
            Ok(date)
        }
    }

    /// Parses a string of the format yyyy mm dd, separated with [sep]
    pub fn from_yyyy_mm_dd<T: AsRef<str>>(text: T, sep: char) -> Option<Date> {
        let split = text.as_ref().split(sep).collect::<Vec<_>>();

        let day = split.get(2)?.parse().ok()?;
        let month = Month::from_number(split.get(1)?.parse().ok()?).ok()?;
        let year =  split.get(0)?.parse().ok()?;

        Some(Self::new(day, month, year).ok()?)
    }

    /// Checks whether a date is in a leap year or not (ie has 29 days in February).
    pub fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && !self.year % 100 == 0 || self.year % 400 == 0
    }

    /// Indicates how many days there are in a month of the year in self.
    pub fn nb_days_in_month(&self, month: Month) -> u8 {
        match month {
            Month::January
            | Month::March
            | Month::May
            | Month::July
            | Month::August
            | Month::October
            | Month::December => 31,
            Month::April | Month::June | Month::September | Month::November => 30,
            Month::February => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
        }
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.day, self.month, self.year)
    }
}

impl Month {
    /// Creates a month from a number.
    ///
    /// Fails if the number is not between 1 and 12.
    pub fn from_number(number: u8) -> Result<Self, String> {
        match number {
            1 => Ok(Self::January),
            2 => Ok(Self::February),
            3 => Ok(Self::March),
            4 => Ok(Self::April),
            5 => Ok(Self::May),
            6 => Ok(Self::June),
            7 => Ok(Self::July),
            8 => Ok(Self::August),
            9 => Ok(Self::September),
            10 => Ok(Self::October),
            11 => Ok(Self::November),
            12 => Ok(Self::December),
            _ => Err(format!("Month {number} does not exist")),
        }
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word = match self {
            Self::January => "janvier",
            Self::February => "février",
            Self::March => "mars",
            Self::April => "avril",
            Self::May => "mai",
            Self::June => "juin",
            Self::July => "juillet",
            Self::August => "août",
            Self::September => "septembre",
            Self::October => "octobre",
            Self::November => "novembre",
            Self::December => "décembre",
        };

        write!(f, "{word}")
    }
}
