/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

use core::fmt;
use std::str::FromStr;

use crate::constants::{
    from_devanagari_digits, to_devanagari_digits, MONTH_NAMES_EN, MONTH_NAMES_NP,
    MONTH_NAMES_SHORT_EN, WEEKDAY_NAMES_EN, WEEKDAY_NAMES_NP, WEEKDAY_NAMES_SHORT_NP,
};
use crate::error::BsError;
use crate::fiscal::{get_fiscal_quarter, get_fiscal_year, FiscalQuarter};
use crate::gregorian::GregorianDate;
use crate::table::{
    bs_to_days_from_ref, days_from_ref_to_bs, days_in_month, days_in_year, is_leap_year,
    REF_AD_DATE,
};

/// Represents a validated Bikram Sambat (B.S.) date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BsDate {
    year: i32,
    month: u8,
    day: u8,
}

impl BsDate {
    /// Constructs and validates a new `BsDate`.
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, BsError> {
        let max_days = days_in_month(year, month)?;
        if day == 0 || day > max_days {
            return Err(BsError::InvalidDay {
                year,
                month,
                day,
                max_days,
            });
        }
        Ok(Self { year, month, day })
    }

    /// Converts a Gregorian (A.D.) date to a Bikram Sambat (B.S.) date.
    pub fn from_gregorian(ad: GregorianDate) -> Result<Self, BsError> {
        let days_diff = ad.diff_days(&REF_AD_DATE);
        let (y, m, d) = days_from_ref_to_bs(days_diff)?;
        Ok(Self {
            year: y,
            month: m,
            day: d,
        })
    }

    /// Converts this Bikram Sambat date into a Gregorian (A.D.) date.
    pub fn to_gregorian(&self) -> GregorianDate {
        let days_from_ref = bs_to_days_from_ref(self.year, self.month, self.day)
            .expect("BsDate instances are always validated");
        REF_AD_DATE.add_days(days_from_ref)
    }

    /// Year component (e.g., 2083).
    #[inline]
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Month component (1..=12).
    #[inline]
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Day component (1..=32).
    #[inline]
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Number of days in current month.
    #[inline]
    pub fn days_in_month(&self) -> u8 {
        days_in_month(self.year, self.month).unwrap_or(30)
    }

    /// Number of days in current year.
    #[inline]
    pub fn days_in_year(&self) -> u16 {
        days_in_year(self.year).unwrap_or(365)
    }

    /// True if current B.S. year has 366 days.
    #[inline]
    pub fn is_leap_year(&self) -> bool {
        is_leap_year(self.year).unwrap_or(false)
    }

    /// Day of the week (0 = Sunday, 1 = Monday .. 6 = Saturday).
    pub fn day_of_week(&self) -> u8 {
        let ad = self.to_gregorian();
        // In Rata Die: rd % 7 == 0 -> Sunday, 1 -> Monday, ..., 6 -> Saturday.
        let rd = ad.to_rata_die();
        let dow = ((rd % 7) + 7) % 7;
        dow as u8
    }

    /// English weekday name (e.g., "Sunday").
    pub fn weekday_name_en(&self) -> &'static str {
        WEEKDAY_NAMES_EN[self.day_of_week() as usize]
    }

    /// Nepali / Devanagari weekday name (e.g., "आइतबार").
    pub fn weekday_name_np(&self) -> &'static str {
        WEEKDAY_NAMES_NP[self.day_of_week() as usize]
    }

    /// Short Nepali weekday name (e.g., "आइत").
    pub fn weekday_name_short_np(&self) -> &'static str {
        WEEKDAY_NAMES_SHORT_NP[self.day_of_week() as usize]
    }

    /// English month name (e.g., "Kartik").
    pub fn month_name_en(&self) -> &'static str {
        MONTH_NAMES_EN[(self.month - 1) as usize]
    }

    /// Nepali / Devanagari month name (e.g., "कात्तिक").
    pub fn month_name_np(&self) -> &'static str {
        MONTH_NAMES_NP[(self.month - 1) as usize]
    }

    /// Short English month name (e.g., "Kar").
    pub fn month_name_short_en(&self) -> &'static str {
        MONTH_NAMES_SHORT_EN[(self.month - 1) as usize]
    }

    /// Official Nepal Fiscal Year string (e.g., "2083/84").
    pub fn fiscal_year(&self) -> String {
        get_fiscal_year(self.year, self.month)
    }

    /// Nepal Fiscal Quarter (Q1, Q2, Q3, Q4).
    pub fn fiscal_quarter(&self) -> FiscalQuarter {
        get_fiscal_quarter(self.month)
    }

    /// Statutory Tax Depreciation Fraction under Nepal Income Tax Act 2058.
    pub fn tax_depreciation_fraction(&self) -> (u8, u8) {
        self.fiscal_quarter().tax_depreciation_fraction()
    }

    /// Difference in days between two B.S. dates (`self - other`).
    pub fn diff_days(&self, other: &Self) -> i64 {
        let self_days = bs_to_days_from_ref(self.year, self.month, self.day).unwrap_or(0);
        let other_days = bs_to_days_from_ref(other.year, other.month, other.day).unwrap_or(0);
        self_days - other_days
    }

    /// Adds a given number of days to this B.S. date.
    pub fn add_days(&self, days: i64) -> Result<Self, BsError> {
        let cur_days = bs_to_days_from_ref(self.year, self.month, self.day)?;
        let target_days = cur_days + days;
        let (y, m, d) = days_from_ref_to_bs(target_days)?;
        Self::new(y, m, d)
    }

    /// Subtracts a given number of days from this B.S. date.
    pub fn sub_days(&self, days: i64) -> Result<Self, BsError> {
        self.add_days(-days)
    }

    /// Formats as standard ISO string `YYYY-MM-DD`.
    pub fn to_iso_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Formats as full Devanagari string (e.g. "१४ कात्तिक २०८३").
    pub fn to_devanagari_string(&self) -> String {
        format!(
            "{} {} {}",
            to_devanagari_digits(self.day as i64),
            self.month_name_np(),
            to_devanagari_digits(self.year as i64)
        )
    }

    /// Flexible formatting supporting pattern specifiers:
    /// - `%Y`: 4-digit year (e.g. 2083)
    /// - `%y`: 2-digit year (e.g. 83)
    /// - `%m`: 2-digit month (01..12)
    /// - `%d`: 2-digit day (01..32)
    /// - `%j`: day without leading zero (1..32)
    /// - `%B`: Full month name in English (e.g. Kartik)
    /// - `%b`: Short month name in English (e.g. Kar)
    /// - `%A`: Full weekday in English (e.g. Saturday)
    /// - `%N`: Full month name in Devanagari (e.g. कात्तिक)
    /// - `%W`: Full weekday in Devanagari (e.g. शनिबार)
    /// - `%EY`: Devanagari year (e.g. २०८३)
    /// - `%Ed`: Devanagari day (e.g. १४)
    /// - `%FY`: Fiscal Year (e.g. 2083/84)
    /// - `%FQ`: Fiscal Quarter (e.g. Q2)
    pub fn format(&self, pattern: &str) -> String {
        let mut out = pattern.to_string();
        out = out.replace("%EY", &to_devanagari_digits(self.year as i64));
        out = out.replace("%Ed", &to_devanagari_digits(self.day as i64));
        out = out.replace("%FY", &self.fiscal_year());
        out = out.replace("%FQ", &self.fiscal_quarter().to_string());
        out = out.replace("%Y", &format!("{:04}", self.year));
        out = out.replace("%y", &format!("{:02}", self.year % 100));
        out = out.replace("%m", &format!("{:02}", self.month));
        out = out.replace("%d", &format!("{:02}", self.day));
        out = out.replace("%j", &self.day.to_string());
        out = out.replace("%B", self.month_name_en());
        out = out.replace("%b", self.month_name_short_en());
        out = out.replace("%A", self.weekday_name_en());
        out = out.replace("%N", self.month_name_np());
        out = out.replace("%W", self.weekday_name_np());
        out
    }

    /// Parses a Bikram Sambat date from string format `YYYY-MM-DD` or `YYYY/MM/DD`
    /// (supports both ASCII and Devanagari numerals).
    pub fn parse_str(input: &str) -> Result<Self, BsError> {
        let normalized = from_devanagari_digits(input.trim());
        let parts: Vec<&str> = normalized
            .split(|c| c == '-' || c == '/' || c == '.')
            .collect();
        if parts.len() != 3 {
            return Err(BsError::ParseError(format!(
                "Expected YYYY-MM-DD format, received '{}'",
                input
            )));
        }

        let year: i32 = parts[0]
            .parse()
            .map_err(|_| BsError::ParseError(format!("Invalid year string: '{}'", parts[0])))?;
        let month: u8 = parts[1]
            .parse()
            .map_err(|_| BsError::ParseError(format!("Invalid month string: '{}'", parts[1])))?;
        let day: u8 = parts[2]
            .parse()
            .map_err(|_| BsError::ParseError(format!("Invalid day string: '{}'", parts[2])))?;

        Self::new(year, month, day)
    }
}

impl fmt::Display for BsDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl FromStr for BsDate {
    type Err = BsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}
