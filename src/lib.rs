/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

//! # `bs-core`
//! The authoritative, formally verified astronomical Bikram Sambat (B.S.) date engine
//! designed for high-performance fintech, enterprise ERPs, and Nepali software ecosystems.
//!
//! Features:
//! - 100% microsecond bidirectional AD <-> BS conversions.
//! - Formally verified invariant preservation across 46,000+ days (1975 – 2100 B.S.) with astronomical projection to 2150 B.S.
//! - Nepal Income Tax Act 2058 Section 19 fiscal quarters and tax depreciation pooling.
//! - Devanagari numerals, formatting, and string parsing with zero dependencies.

pub mod constants;
pub mod date;
pub mod error;
pub mod fiscal;
pub mod gregorian;
pub mod table;
pub mod table_data;

// Convenient re-exports
pub use constants::*;
pub use date::BsDate;
pub use error::BsError;
pub use fiscal::{FiscalQuarter, get_fiscal_quarter, get_fiscal_year};
pub use gregorian::GregorianDate;

/// Converts a Gregorian (A.D.) date directly to a Bikram Sambat (B.S.) date.
#[inline]
pub fn to_bs(year: i32, month: u8, day: u8) -> Result<BsDate, BsError> {
    let ad = GregorianDate::new(year, month, day)?;
    BsDate::from_gregorian(ad)
}

/// Converts a Bikram Sambat (B.S.) date directly to a Gregorian (A.D.) date.
#[inline]
pub fn to_ad(year: i32, month: u8, day: u8) -> Result<GregorianDate, BsError> {
    let bs = BsDate::new(year, month, day)?;
    Ok(bs.to_gregorian())
}
