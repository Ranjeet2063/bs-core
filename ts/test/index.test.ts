/* Authorized Protocol Quality Assurance & Formal Verification Test Suite */

import test from "node:test";
import assert from "node:assert/strict";
import { toBs, toAd, BsDate } from "../src/index.ts";

test("Reference epoch conversion: 1975-01-01 BS == 1918-04-13 AD", () => {
  const bs = new BsDate(1975, 1, 1);
  const [y, m, d] = bs.toGregorian();
  assert.equal(y, 1918);
  assert.equal(m, 4);
  assert.equal(d, 13);

  const backBs = toBs(1918, 4, 13);
  assert.equal(backBs.year, 1975);
  assert.equal(backBs.month, 1);
  assert.equal(backBs.day, 1);
});

test("Present known date conversion: 2026-10-31 AD == 2083-07-14 BS (Saturday)", () => {
  const bs = toBs(2026, 10, 31);
  assert.equal(bs.year, 2083);
  assert.equal(bs.month, 7);
  assert.equal(bs.day, 14);
  assert.equal(bs.weekdayNameEn, "Saturday");
  assert.equal(bs.weekdayNameNp, "शनिबार");

  const [gy, gm, gd] = toAd(2083, 7, 14);
  assert.equal(gy, 2026);
  assert.equal(gm, 10);
  assert.equal(gd, 31);
});

test("Nepal Income Tax Act 2058 fiscal quarters and depreciation", () => {
  // Q1
  const q1 = new BsDate(2083, 4, 10);
  assert.equal(q1.fiscalYear, "2083/84");
  assert.equal(q1.fiscalQuarter, "Q1");
  assert.deepEqual(q1.taxDepreciationFraction, [3, 3]);

  // Q2
  const q2 = new BsDate(2083, 7, 14);
  assert.equal(q2.fiscalYear, "2083/84");
  assert.equal(q2.fiscalQuarter, "Q2");
  assert.deepEqual(q2.taxDepreciationFraction, [2, 3]);

  // Q3
  const q3 = new BsDate(2083, 10, 5);
  assert.equal(q3.fiscalYear, "2083/84");
  assert.equal(q3.fiscalQuarter, "Q3");
  assert.deepEqual(q3.taxDepreciationFraction, [1, 3]);

  // Q4
  const q4 = new BsDate(2084, 1, 1);
  assert.equal(q4.fiscalYear, "2083/84");
  assert.equal(q4.fiscalQuarter, "Q4");
  assert.deepEqual(q4.taxDepreciationFraction, [1, 3]);
});

test("Date arithmetic and difference", () => {
  const d1 = new BsDate(2083, 1, 1);
  const d2 = d1.addDays(50);
  assert.equal(d2.diffDays(d1), 50);
  assert.equal(d1.diffDays(d2), -50);
});

test("Parsing English and Devanagari numerals", () => {
  const bsEn = BsDate.parse("2083-07-14");
  const bsNp = BsDate.parse("२०८३-०७-१४");
  assert.equal(bsEn.year, 2083);
  assert.equal(bsNp.year, 2083);
  assert.equal(bsEn.day, 14);
  assert.equal(bsNp.day, 14);
  assert.equal(bsEn.toDevanagariString(), "१४ कात्तिक २०८३");
});
