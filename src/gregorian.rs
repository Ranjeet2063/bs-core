/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

use core::fmt;

/// A lightweight, zero-dependency Gregorian (A.D.) calendar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GregorianDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl GregorianDate {
    /// Creates a new `GregorianDate` with strict bounds checking.
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, crate::error::BsError> {
        if !(1..=12).contains(&month) {
            return Err(crate::error::BsError::InvalidMonth(month));
        }
        let max_days = Self::days_in_month(year, month);
        if day == 0 || day > max_days {
            return Err(crate::error::BsError::InvalidDay {
                year,
                month,
                day,
                max_days,
            });
        }
        Ok(Self { year, month, day })
    }

    /// Returns whether the specified Gregorian year is a leap year.
    #[inline]
    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Returns the number of days in the specified Gregorian month.
    #[inline]
    pub fn days_in_month(year: i32, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }

    /// Converts this Gregorian date to Rata Die (days since 0001-01-01 A.D. proleptic Gregorian).
    #[inline]
    pub fn to_rata_die(&self) -> i64 {
        let mut y = self.year as i64;
        let mut m = self.month as i64;
        if m <= 2 {
            y -= 1;
            m += 12;
        }
        365 * y + y / 4 - y / 100 + y / 400 + (153 * (m - 3) + 2) / 5 + self.day as i64 - 306
    }

    /// Constructs a `GregorianDate` from a Rata Die day count.
    #[inline]
    pub fn from_rata_die(rd: i64) -> Self {
        let z = rd + 306;
        let h = 100 * z - 25;
        let a = h / 3652425;
        let b = a - a / 4;
        let mut year = ((100 * b + h) / 36525) as i32;
        let c = b + z - 365 * year as i64 - year as i64 / 4;
        let mut month = ((5 * c + 456) / 153) as u8;
        let day = (c - (153 * month as i64 - 457) / 5) as u8;
        if month > 12 {
            year += 1;
            month -= 12;
        }
        Self { year, month, day }
    }

    /// Computes the difference in days between two Gregorian dates (`self - other`).
    #[inline]
    pub fn diff_days(&self, other: &Self) -> i64 {
        self.to_rata_die() - other.to_rata_die()
    }

    /// Adds a given number of days to this Gregorian date.
    #[inline]
    pub fn add_days(&self, days: i64) -> Self {
        Self::from_rata_die(self.to_rata_die() + days)
    }

    /// Formats as ISO-8601 string `YYYY-MM-DD`.
    pub fn to_iso_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl fmt::Display for GregorianDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
