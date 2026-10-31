/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

use core::fmt;

/// Nepal Fiscal Year Quarters according to the Ministry of Finance & Inland Revenue Department.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FiscalQuarter {
    /// First Quarter: Shrawan 1 – Ashwin end (Months 4, 5, 6)
    Q1,
    /// Second Quarter: Kartik 1 – Poush end (Months 7, 8, 9)
    Q2,
    /// Third Quarter: Magh 1 – Chaitra end (Months 10, 11, 12)
    Q3,
    /// Fourth Quarter: Baisakh 1 – Ashadh end (Months 1, 2, 3)
    Q4,
}

impl FiscalQuarter {
    /// Returns the numerical quarter (1..=4).
    pub fn to_number(&self) -> u8 {
        match self {
            FiscalQuarter::Q1 => 1,
            FiscalQuarter::Q2 => 2,
            FiscalQuarter::Q3 => 3,
            FiscalQuarter::Q4 => 4,
        }
    }

    /// English description.
    pub fn name_en(&self) -> &'static str {
        match self {
            FiscalQuarter::Q1 => "First Quarter (Q1)",
            FiscalQuarter::Q2 => "Second Quarter (Q2)",
            FiscalQuarter::Q3 => "Third Quarter (Q3)",
            FiscalQuarter::Q4 => "Fourth Quarter (Q4)",
        }
    }

    /// Devanagari description.
    pub fn name_np(&self) -> &'static str {
        match self {
            FiscalQuarter::Q1 => "प्रथम त्रैमासिक (Q1)",
            FiscalQuarter::Q2 => "दोस्रो त्रैमासिक (Q2)",
            FiscalQuarter::Q3 => "तेस्रो त्रैमासिक (Q3)",
            FiscalQuarter::Q4 => "चौथो त्रैमासिक (Q4)",
        }
    }

    /// Returns the statutory tax depreciation entry fraction according to Section 19 of the
    /// Nepal Income Tax Act 2058 (आयकर ऐन २०५८):
    /// - Q1 additions: 3/3 (100% of standard depreciation pool allowable in current fiscal year)
    /// - Q2 additions: 2/3 (66.67% of standard depreciation pool allowable in current fiscal year)
    /// - Q3 & Q4 additions: 1/3 (33.33% of standard depreciation pool allowable in current fiscal year)
    pub fn tax_depreciation_fraction(&self) -> (u8, u8) {
        match self {
            FiscalQuarter::Q1 => (3, 3),
            FiscalQuarter::Q2 => (2, 3),
            FiscalQuarter::Q3 | FiscalQuarter::Q4 => (1, 3),
        }
    }
}

impl fmt::Display for FiscalQuarter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Q{}", self.to_number())
    }
}

/// Computes the official Nepal Fiscal Year label (e.g., "2080/81") from a Bikram Sambat date.
pub fn get_fiscal_year(bs_year: i32, bs_month: u8) -> String {
    if bs_month >= 4 {
        let next_yy = (bs_year + 1) % 100;
        format!("{}/{:02}", bs_year, next_yy)
    } else {
        let prev_year = bs_year - 1;
        let cur_yy = bs_year % 100;
        format!("{}/{:02}", prev_year, cur_yy)
    }
}

/// Computes the fiscal quarter from a Bikram Sambat month (1..=12).
pub fn get_fiscal_quarter(bs_month: u8) -> FiscalQuarter {
    match bs_month {
        4..=6 => FiscalQuarter::Q1,
        7..=9 => FiscalQuarter::Q2,
        10..=12 => FiscalQuarter::Q3,
        1..=3 => FiscalQuarter::Q4,
        _ => FiscalQuarter::Q1,
    }
}
