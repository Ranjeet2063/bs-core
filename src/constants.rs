/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

/// English names of the 12 Bikram Sambat months (1-indexed).
pub const MONTH_NAMES_EN: [&str; 12] = [
    "Baisakh",
    "Jestha",
    "Ashadh",
    "Shrawan",
    "Bhadra",
    "Ashwin",
    "Kartik",
    "Mangsir",
    "Poush",
    "Magh",
    "Falgun",
    "Chaitra",
];

/// Devanagari / Nepali names of the 12 Bikram Sambat months (1-indexed).
pub const MONTH_NAMES_NP: [&str; 12] = [
    "बैशाख",
    "जेठ",
    "असार",
    "साउन",
    "भदौ",
    "असोज",
    "कात्तिक",
    "मंसिर",
    "पुस",
    "माघ",
    "फागुन",
    "चैत",
];

/// English abbreviated names of the 12 Bikram Sambat months.
pub const MONTH_NAMES_SHORT_EN: [&str; 12] = [
    "Bai", "Jes", "Ash", "Shr", "Bha", "Asw", "Kar", "Man", "Pou", "Mag", "Fal", "Cha",
];

/// English names of weekdays starting from Sunday (0 = Sunday .. 6 = Saturday).
pub const WEEKDAY_NAMES_EN: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

/// Nepali / Devanagari names of weekdays starting from Sunday.
pub const WEEKDAY_NAMES_NP: [&str; 7] = [
    "आइतबार",
    "सोमबार",
    "मङ्गलबार",
    "बुधबार",
    "बिहिबार",
    "शुक्रबार",
    "शनिबार",
];

/// Short Nepali names of weekdays.
pub const WEEKDAY_NAMES_SHORT_NP: [&str; 7] = [
    "आइत", "सोम", "मङ्गल", "बुध", "बिहि", "शुक्र", "शनि",
];

/// Devanagari numerals corresponding to digits 0..=9.
pub const DEVANAGARI_DIGITS: [char; 10] = [
    '०', '१', '२', '३', '४', '५', '६', '७', '८', '९',
];

/// Converts an ASCII integer string into Devanagari digits.
pub fn to_devanagari_digits(num: i64) -> String {
    let s = num.to_string();
    let mut result = String::with_capacity(s.len() * 3);
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            let digit_idx = (ch as u8 - b'0') as usize;
            result.push(DEVANAGARI_DIGITS[digit_idx]);
        } else {
            result.push(ch);
        }
    }
    result
}

/// Converts Devanagari digits in a string back to standard ASCII digits.
pub fn from_devanagari_digits(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '०' => result.push('0'),
            '१' => result.push('1'),
            '२' => result.push('2'),
            '३' => result.push('3'),
            '४' => result.push('4'),
            '५' => result.push('5'),
            '६' => result.push('6'),
            '७' => result.push('7'),
            '८' => result.push('8'),
            '९' => result.push('9'),
            other => result.push(other),
        }
    }
    result
}
