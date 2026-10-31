/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

import { BS_MONTH_DAYS, MAX_BS_YEAR, MIN_BS_YEAR, REF_AD_DAY, REF_AD_MONTH, REF_AD_YEAR } from "./tableData.ts";

export const MONTH_NAMES_EN = [
  "Baisakh", "Jestha", "Ashadh", "Shrawan", "Bhadra", "Ashwin",
  "Kartik", "Mangsir", "Poush", "Magh", "Falgun", "Chaitra"
] as const;

export const MONTH_NAMES_NP = [
  "बैशाख", "जेठ", "असार", "साउन", "भदौ", "असोज",
  "कात्तिक", "मंसिर", "पुस", "माघ", "फागुन", "चैत"
] as const;

export const WEEKDAY_NAMES_EN = [
  "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
] as const;

export const WEEKDAY_NAMES_NP = [
  "आइतबार", "सोमबार", "मङ्गलबार", "बुधबार", "बिहिबार", "शुक्रबार", "शनिबार"
] as const;

export const DEVANAGARI_DIGITS = ["०", "१", "२", "३", "४", "५", "६", "७", "८", "९"] as const;

export function toDevanagariDigits(num: number | string): string {
  return String(num).replace(/\d/g, (d) => DEVANAGARI_DIGITS[Number(d)]);
}

export function fromDevanagariDigits(str: string): string {
  const map: Record<string, string> = {
    "०": "0", "१": "1", "२": "2", "३": "3", "४": "4",
    "५": "5", "६": "6", "७": "7", "८": "8", "९": "9"
  };
  return str.replace(/[०-९]/g, (d) => map[d] ?? d);
}

export function isGregorianLeapYear(year: number): boolean {
  return (year % 4 === 0 && year % 100 !== 0) || (year % 400 === 0);
}

export function gregorianDaysInMonth(year: number, month: number): number {
  switch (month) {
    case 1: case 3: case 5: case 7: case 8: case 10: case 12: return 31;
    case 4: case 6: case 9: case 11: return 30;
    case 2: return isGregorianLeapYear(year) ? 29 : 28;
    default: return 0;
  }
}

export function gregorianToRataDie(year: number, month: number, day: number): number {
  let y = year;
  let m = month;
  if (m <= 2) {
    y -= 1;
    m += 12;
  }
  return 365 * y + Math.floor(y / 4) - Math.floor(y / 100) + Math.floor(y / 400) +
    Math.floor((153 * (m - 3) + 2) / 5) + day - 306;
}

export function rataDieToGregorian(rd: number): [number, number, number] {
  const z = rd + 306;
  const h = 100 * z - 25;
  const a = Math.floor(h / 3652425);
  const b = a - Math.floor(a / 4);
  let year = Math.floor((100 * b + h) / 36525);
  const c = b + z - 365 * year - Math.floor(year / 4);
  let month = Math.floor((5 * c + 456) / 153);
  const day = c - Math.floor((153 * month - 457) / 5);
  if (month > 12) {
    year += 1;
    month -= 12;
  }
  return [year, month, day];
}

const REF_RATA_DIE = gregorianToRataDie(REF_AD_YEAR, REF_AD_MONTH, REF_AD_DAY);

export function getBsDaysInMonth(year: number, month: number): number {
  if (month < 1 || month > 12) {
    throw new Error(`Invalid BS month: ${month}`);
  }
  if (year >= MIN_BS_YEAR && year <= MAX_BS_YEAR) {
    return BS_MONTH_DAYS[year - MIN_BS_YEAR][month - 1];
  } else if (year >= 1970 && year < MIN_BS_YEAR) {
    const historical = [
      [30, 32, 31, 32, 31, 30, 30, 30, 29, 30, 29, 31],
      [31, 31, 32, 31, 31, 31, 30, 29, 30, 29, 30, 30],
      [31, 31, 32, 32, 31, 30, 30, 29, 30, 29, 30, 30],
      [31, 32, 31, 32, 31, 30, 30, 30, 29, 29, 30, 31],
      [30, 32, 31, 32, 31, 30, 30, 30, 29, 30, 29, 31]
    ];
    return historical[year - 1970][month - 1];
  } else if (year > MAX_BS_YEAR && year <= 2150) {
    const templates = [
      [31, 31, 32, 32, 31, 30, 30, 30, 29, 30, 29, 31],
      [30, 32, 31, 32, 31, 30, 30, 29, 30, 29, 30, 30],
      [31, 31, 32, 31, 31, 31, 30, 30, 29, 30, 29, 31],
      [31, 32, 31, 32, 31, 30, 30, 29, 30, 29, 30, 31]
    ];
    return templates[year % 4][month - 1];
  }
  throw new Error(`BS year ${year} is out of supported range [1970..2150]`);
}

export function getBsDaysInYear(year: number): number {
  let sum = 0;
  for (let m = 1; m <= 12; m++) {
    sum += getBsDaysInMonth(year, m);
  }
  return sum;
}

export function bsToDaysFromRef(year: number, month: number, day: number): number {
  let days = 0;
  if (year >= MIN_BS_YEAR) {
    for (let y = MIN_BS_YEAR; y < year; y++) {
      days += getBsDaysInYear(y);
    }
  } else {
    for (let y = year; y < MIN_BS_YEAR; y++) {
      days -= getBsDaysInYear(y);
    }
  }
  for (let m = 1; m < month; m++) {
    days += getBsDaysInMonth(year, m);
  }
  days += (day - 1);
  return days;
}

export function daysFromRefToBs(totalDays: number): [number, number, number] {
  let days = totalDays;
  let year = MIN_BS_YEAR;
  if (days >= 0) {
    while (true) {
      const yDays = getBsDaysInYear(year);
      if (days < yDays) break;
      days -= yDays;
      year += 1;
      if (year > 2150) throw new Error("BS date overflow");
    }
  } else {
    while (true) {
      year -= 1;
      if (year < 1970) throw new Error("BS date underflow");
      const yDays = getBsDaysInYear(year);
      days += yDays;
      if (days >= 0) break;
    }
  }

  let month = 1;
  for (let m = 1; m <= 12; m++) {
    const mDays = getBsDaysInMonth(year, m);
    if (days < mDays) {
      month = m;
      break;
    }
    days -= mDays;
  }
  const day = days + 1;
  return [year, month, day];
}

export class BsDate {
  readonly year: number;
  readonly month: number;
  readonly day: number;

  constructor(year: number, month: number, day: number) {
    const maxDays = getBsDaysInMonth(year, month);
    if (day < 1 || day > maxDays) {
      throw new Error(`Invalid BS day: ${day} for ${year}-${month} (max is ${maxDays})`);
    }
    this.year = year;
    this.month = month;
    this.day = day;
  }

  static fromGregorian(year: number, month: number, day: number): BsDate {
    const rd = gregorianToRataDie(year, month, day);
    const diff = rd - REF_RATA_DIE;
    const [by, bm, bd] = daysFromRefToBs(diff);
    return new BsDate(by, bm, bd);
  }

  toGregorian(): [number, number, number] {
    const diff = bsToDaysFromRef(this.year, this.month, this.day);
    const targetRd = REF_RATA_DIE + diff;
    return rataDieToGregorian(targetRd);
  }

  get dayOfWeek(): number {
    const [gy, gm, gd] = this.toGregorian();
    const rd = gregorianToRataDie(gy, gm, gd);
    return ((rd % 7) + 7) % 7;
  }

  get weekdayNameEn(): string {
    return WEEKDAY_NAMES_EN[this.dayOfWeek];
  }

  get weekdayNameNp(): string {
    return WEEKDAY_NAMES_NP[this.dayOfWeek];
  }

  get monthNameEn(): string {
    return MONTH_NAMES_EN[this.month - 1];
  }

  get monthNameNp(): string {
    return MONTH_NAMES_NP[this.month - 1];
  }

  get fiscalYear(): string {
    if (this.month >= 4) {
      const nextYy = String((this.year + 1) % 100).padStart(2, "0");
      return `${this.year}/${nextYy}`;
    }
    const prevYear = this.year - 1;
    const curYy = String(this.year % 100).padStart(2, "0");
    return `${prevYear}/${curYy}`;
  }

  get fiscalQuarter(): "Q1" | "Q2" | "Q3" | "Q4" {
    if (this.month >= 4 && this.month <= 6) return "Q1";
    if (this.month >= 7 && this.month <= 9) return "Q2";
    if (this.month >= 10 && this.month <= 12) return "Q3";
    return "Q4";
  }

  get taxDepreciationFraction(): [number, number] {
    switch (this.fiscalQuarter) {
      case "Q1": return [3, 3];
      case "Q2": return [2, 3];
      case "Q3":
      case "Q4": return [1, 3];
    }
  }

  addDays(days: number): BsDate {
    const cur = bsToDaysFromRef(this.year, this.month, this.day);
    const [y, m, d] = daysFromRefToBs(cur + days);
    return new BsDate(y, m, d);
  }

  diffDays(other: BsDate): number {
    const a = bsToDaysFromRef(this.year, this.month, this.day);
    const b = bsToDaysFromRef(other.year, other.month, other.day);
    return a - b;
  }

  toIsoString(): string {
    const mm = String(this.month).padStart(2, "0");
    const dd = String(this.day).padStart(2, "0");
    return `${this.year}-${mm}-${dd}`;
  }

  toDevanagariString(): string {
    return `${toDevanagariDigits(this.day)} ${this.monthNameNp} ${toDevanagariDigits(this.year)}`;
  }

  static parse(str: string): BsDate {
    const normalized = fromDevanagariDigits(str.trim());
    const parts = normalized.split(/[-/.]/);
    if (parts.length !== 3) {
      throw new Error(`Invalid date string: ${str}`);
    }
    const y = parseInt(parts[0], 10);
    const m = parseInt(parts[1], 10);
    const d = parseInt(parts[2], 10);
    return new BsDate(y, m, d);
  }
}

export function toBs(year: number, month: number, day: number): BsDate {
  return BsDate.fromGregorian(year, month, day);
}

export function toAd(year: number, month: number, day: number): [number, number, number] {
  return new BsDate(year, month, day).toGregorian();
}
