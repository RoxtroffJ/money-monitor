#[derive(Debug, Clone, PartialEq, Eq)]
/// A date of the year
pub struct Date {
    day: u8,
    month: Month,
    year: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// 
    /// # Example
    /// ```
    /// use money_monitor::units::{Date, Month};
    /// 
    /// # fn main() -> Result<(),String> {
    /// let valid_date = Date::new(15, Month::September, 2025);
    /// let invalid_date = Date::new(42, Month::May, 2042);
    /// assert_eq!("15 septembre 2025", valid_date?.to_string()); // Yes this is French because I am French :)
    /// assert!(invalid_date.is_err());
    /// # Ok(())}
    /// ```
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

    /// Parses a string of the format yyyy mm dd, separated with `sep`.
    /// 
    /// # Example
    /// ```
    /// use money_monitor::units::{Date, Month};
    /// 
    /// # fn main() -> Result<(), String> {
    /// let valid_date = Date::from_yyyy_mm_dd("2025-09-15", '-').unwrap(); // Valid date
    /// let invalid_date1 = Date::from_yyyy_mm_dd("25-9-15", '-'); // Incorrect format
    /// let invalid_date2 = Date::from_yyyy_mm_dd("2025-09-42", '-'); // There is no 42nd of september
    /// 
    /// assert_eq!(Date::new(15, Month::September, 2025)?, valid_date);
    /// assert!(invalid_date1.is_none());
    /// assert!(invalid_date2.is_none());
    /// # Ok(())}
    /// ```
    pub fn from_yyyy_mm_dd<T: AsRef<str>>(text: T, sep: char) -> Option<Date> {
        let split = text.as_ref().split(sep).collect::<Vec<_>>();

        let day_str = split.get(2)?;
        if day_str.len() != 2 {None?}
        let day = day_str.parse().ok()?;

        let month_str = split.get(1)?;
        if month_str.len() != 2 {None?}
        let month = Month::from_number(month_str.parse().ok()?).ok()?;

        let year_str = split.get(0)?;
        if year_str.len() != 4 {None?}
        let year =  year_str.parse().ok()?;

        Some(Self::new(day, month, year).ok()?)
    }

    /// Checks whether a date is in a leap year or not (ie has 29 days in February).
    /// 
    /// # Example
    /// ```
    /// use money_monitor::units::{Date, Month};
    /// 
    /// let date1 = Date::new(15, Month::May, 2025).unwrap(); // Not leap year (can not be divided by 4)
    /// let date2 = Date::new(15, Month::May, 2024).unwrap(); // Leap year (year can be divided by 4 but not by 100)
    /// let date3 = Date::new(15, Month::May, 2000).unwrap(); // Leap year (year can be divided by 400)
    /// let date4 = Date::new(15, Month::May, 1900).unwrap(); // Not leap year (year can be divided by 100 but not 400)
    /// 
    /// assert!(!date1.is_leap_year());
    /// assert!(date2.is_leap_year());
    /// assert!(date3.is_leap_year());
    /// assert!(!date4.is_leap_year());
    /// ```
    pub fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0
    }

    /// Indicates how many days there are in a month of the year in self.
    /// 
    /// Example
    /// ```
    /// use money_monitor::units::{Date, Month};
    /// 
    /// let date = Date::new(15, Month::September, 2025).unwrap();
    /// 
    /// assert_eq!(31, date.nb_days_in_month(Month::January)); // There were 31 days in January 2025
    /// assert_eq!(28, date.nb_days_in_month(Month::February)); // There were 27 days in February 2025
    /// assert_eq!(30, date.nb_days_in_month(Month::April)); // There were 30 days in April 2025
    /// ```
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
    /// 
    /// # Example
    /// ```
    /// use money_monitor::units::Month;
    /// 
    /// assert_eq!(Ok(Month::September), Month::from_number(9));
    /// assert!(Month::from_number(42).is_err());
    /// ```
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

/// Display in French because I am French
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
