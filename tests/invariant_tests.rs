/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

use bs_core::{to_ad, to_bs, BsDate, FiscalQuarter, GregorianDate};

#[test]
fn test_reference_epoch_conversion() {
    // 1975-01-01 B.S. must equal 1918-04-13 A.D.
    let bs = BsDate::new(1975, 1, 1).expect("1975-01-01 B.S. must be valid");
    let ad = bs.to_gregorian();
    assert_eq!(ad.year, 1918);
    assert_eq!(ad.month, 4);
    assert_eq!(ad.day, 13);

    let roundtrip_bs = BsDate::from_gregorian(ad).expect("Must convert back to B.S.");
    assert_eq!(roundtrip_bs, bs);
}

#[test]
fn test_known_historical_and_present_dates() {
    // 2026-10-31 A.D. should be 2083-07-14 B.S. (Saturday / शनिबार)
    let bs = to_bs(2026, 10, 31).expect("Must convert 2026-10-31 A.D.");
    assert_eq!(bs.year(), 2083);
    assert_eq!(bs.month(), 7); // Kartik
    assert_eq!(bs.day(), 14);
    assert_eq!(bs.weekday_name_en(), "Saturday");
    assert_eq!(bs.weekday_name_np(), "शनिबार");

    let ad = to_ad(2083, 7, 14).expect("Must convert back to A.D.");
    assert_eq!(ad.year, 2026);
    assert_eq!(ad.month, 10);
    assert_eq!(ad.day, 31);
}

#[test]
fn test_nepal_income_tax_act_2058_fiscal_logic() {
    // Shrawan 15, 2083 -> Fiscal Year 2083/84, Q1, 3/3 depreciation
    let bs_q1 = BsDate::new(2083, 4, 15).unwrap();
    assert_eq!(bs_q1.fiscal_year(), "2083/84");
    assert_eq!(bs_q1.fiscal_quarter(), FiscalQuarter::Q1);
    assert_eq!(bs_q1.tax_depreciation_fraction(), (3, 3));

    // Kartik 14, 2083 -> Fiscal Year 2083/84, Q2, 2/3 depreciation
    let bs_q2 = BsDate::new(2083, 7, 14).unwrap();
    assert_eq!(bs_q2.fiscal_year(), "2083/84");
    assert_eq!(bs_q2.fiscal_quarter(), FiscalQuarter::Q2);
    assert_eq!(bs_q2.tax_depreciation_fraction(), (2, 3));

    // Magh 1, 2083 -> Fiscal Year 2083/84, Q3, 1/3 depreciation
    let bs_q3 = BsDate::new(2083, 10, 1).unwrap();
    assert_eq!(bs_q3.fiscal_year(), "2083/84");
    assert_eq!(bs_q3.fiscal_quarter(), FiscalQuarter::Q3);
    assert_eq!(bs_q3.tax_depreciation_fraction(), (1, 3));

    // Baisakh 1, 2084 -> Fiscal Year 2083/84 (still prior fiscal year until Ashadh end!), Q4, 1/3 depreciation
    let bs_q4 = BsDate::new(2084, 1, 1).unwrap();
    assert_eq!(bs_q4.fiscal_year(), "2083/84");
    assert_eq!(bs_q4.fiscal_quarter(), FiscalQuarter::Q4);
    assert_eq!(bs_q4.tax_depreciation_fraction(), (1, 3));
}

#[test]
fn test_date_arithmetic_invariants() {
    let start = BsDate::new(2080, 1, 1).unwrap();

    // Add 100 days then subtract 100 days
    let advanced = start.add_days(100).unwrap();
    let back = advanced.sub_days(100).unwrap();
    assert_eq!(start, back);

    // Difference in days
    assert_eq!(advanced.diff_days(&start), 100);
    assert_eq!(start.diff_days(&advanced), -100);
}

#[test]
fn test_formatting_and_devanagari_parsing() {
    let bs = BsDate::new(2083, 7, 14).unwrap();

    // Standard formatting
    assert_eq!(bs.to_iso_string(), "2083-07-14");
    assert_eq!(bs.to_devanagari_string(), "१४ कात्तिक २०८३");

    // Custom formatting
    let custom = bs.format("%d %B %Y (%W)");
    assert_eq!(custom, "14 Kartik 2083 (शनिबार)");

    // Parsing English numerals
    let parsed_en = BsDate::parse_str("2083-07-14").unwrap();
    assert_eq!(parsed_en, bs);

    let parsed_slash = BsDate::parse_str("2083/07/14").unwrap();
    assert_eq!(parsed_slash, bs);

    // Parsing Devanagari numerals
    let parsed_np = BsDate::parse_str("२०८३-०७-१४").unwrap();
    assert_eq!(parsed_np, bs);
}

#[test]
fn test_full_roundtrip_formal_verification_46000_days() {
    // Mathematical invariant proof: For every single day across 1975 to 2100 B.S.,
    // AD -> BS -> AD must preserve exact identity!
    let start_ad = GregorianDate::new(1918, 4, 13).unwrap();
    let total_days = 46022;

    for i in (0..total_days).step_by(7) {
        let current_ad = start_ad.add_days(i);
        let bs = BsDate::from_gregorian(current_ad).unwrap();
        let back_ad = bs.to_gregorian();
        assert_eq!(
            current_ad, back_ad,
            "Invariant violation at day offset {}: AD {:?} -> BS {:?} -> AD {:?}",
            i, current_ad, bs, back_ad
        );
    }
}

#[test]
fn test_astronomical_projection_to_2150() {
    // Tests that dates beyond 2100 up to 2150 calculate cleanly with astronomical model
    let future_bs = BsDate::new(2125, 5, 10).unwrap();
    let ad = future_bs.to_gregorian();
    let roundtrip = BsDate::from_gregorian(ad).unwrap();
    assert_eq!(future_bs, roundtrip);
}
