/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BsError {
    InvalidYear(i32),
    InvalidMonth(u8),
    InvalidDay {
        year: i32,
        month: u8,
        day: u8,
        max_days: u8,
    },
    DateOutOfRange {
        year: i32,
        min_year: i32,
        max_year: i32,
    },
    ParseError(String),
}

impl fmt::Display for BsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BsError::InvalidYear(y) => write!(f, "Invalid year: {}", y),
            BsError::InvalidMonth(m) => write!(f, "Invalid month: {} (must be 1..=12)", m),
            BsError::InvalidDay {
                year,
                month,
                day,
                max_days,
            } => write!(
                f,
                "Invalid day {} for year {} month {} (max allowed is {})",
                day, year, month, max_days
            ),
            BsError::DateOutOfRange {
                year,
                min_year,
                max_year,
            } => write!(
                f,
                "Date out of supported range: year {} is outside [{}..={}]",
                year, min_year, max_year
            ),
            BsError::ParseError(msg) => write!(f, "Failed to parse date string: {}", msg),
        }
    }
}

impl std::error::Error for BsError {}
