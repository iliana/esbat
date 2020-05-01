// Copyright (c) 2018 Edward M. Reingold and Nachum Dershowitz
// Copyright (c) 2020 iliana destroyer of worlds <iliana@buttslol.net>
// SPDX-License-Identifier: CC-BY-NC-4.0
//
// This work is licensed under the Creative Commons Attribution-NonCommercial 4.0 International
// License. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/
// or send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.
//
// Functions from:
//
//     E. M. Reingold and N. Dershowitz, Calendrical Calculations: The Ultimate Edition.
//         Cambridge University Press, 2018. doi:10.1017/9781107415058
//
// Dates throughout these functions are Rata Die, where 1 R.D. is Monday, January 1, 1 in the
// proleptic Gregorian calendar. R.D. is represented as `i32` (which can represent all days
// representable by Chrono).
//
// Moments (e.g. Chrono's `DateTime`) are Rata Die. For example, 1.0 R.D. is Monday, January 1, 1
// at midnight UTC, and 730120.5 R.D. is January 1, 2000, at 12:00 UTC. Moments are represented as
// `f64`.
//
// The (year, month, date) structure is represented as (i32, u32, u32) for simple interoperability
// with Chrono.
//
// All other types are `f64`. Angles are always in degrees throughout these functions.

#![allow(clippy::unreadable_literal, clippy::wildcard_imports)]

use crate::data::*;
use crate::util::*;

fn is_gregorian_leap_year(year: i32) -> bool {
    year.rem_euclid(4) == 0 && ![100, 200, 300].contains(&year.rem_euclid(400))
}

#[allow(clippy::cast_possible_wrap)]
pub(crate) fn fixed_from_gregorian(year: i32, month: u32, day: u32) -> i32 {
    // `month` and `day` happen to be u32 in chrono, but cannot be greater than 12 and 31
    debug_assert!(month <= 2147483647 && day <= 2147483647);
    let month = month as i32;
    let day = day as i32;

    365 * (year - 1) + (year - 1).div_euclid(4) - (year - 1).div_euclid(100)
        + (year - 1).div_euclid(400)
        + (367 * month - 362) / 12
        + if month <= 2 {
            0
        } else if is_gregorian_leap_year(year) {
            -1
        } else {
            -2
        }
        + day
}

fn gregorian_new_year(year: i32) -> i32 {
    fixed_from_gregorian(year, 1, 1)
}

fn gregorian_year_from_fixed(date: f64) -> i32 {
    let d0 = i32((date - 1.0).floor());
    let n400 = d0.div_euclid(146097);
    let d1 = d0.rem_euclid(146097);
    let n100 = d1.div_euclid(36524);
    let d2 = d1.rem_euclid(36524);
    let n4 = d2.div_euclid(1461);
    let d3 = d2.rem_euclid(1461);
    let n1 = d3.div_euclid(365);
    let year = 400 * n400 + 100 * n100 + 4 * n4 + n1;
    if n100 == 4 || n1 == 4 {
        year
    } else {
        year + 1
    }
}

pub(crate) fn gregorian_from_fixed(date: f64) -> (i32, u32, u32) {
    let year = gregorian_year_from_fixed(date);
    let date = i32(date.floor());
    let prior_days = date - gregorian_new_year(year);
    let correction = if date < fixed_from_gregorian(year, 3, 1) {
        0
    } else if is_gregorian_leap_year(year) {
        1
    } else {
        2
    };
    let month = nonneg((12 * (prior_days + correction) + 373) / 367);
    let day = nonneg(date - fixed_from_gregorian(year, month, 1) + 1);
    (year, month, day)
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

fn ephemeris_correction(t: f64) -> f64 {
    let year_i = gregorian_year_from_fixed(t);
    let year = f64::from(year_i);
    match year_i {
        2051..=2150 => {
            (-20.0 + 32.0 * ((year - 1820.0) / 100.0).powi(2) + 0.5628 * (2150.0 - year)) / 86400.0
        }
        2006..=2050 => polynomial(year - 2000.0, &[62.92, 0.32217, 0.005589]) / 86400.0,
        1987..=2005 => {
            polynomial(
                year - 2000.0,
                &[63.86, 0.3345, -0.060374, 0.0017275, 0.000651814, 0.00002373599],
            ) / 86400.0
        }
        1900..=1986 => polynomial(
            f64::from(fixed_from_gregorian(year_i, 7, 1) - fixed_from_gregorian(1900, 1, 1))
                / 36525.0,
            &[-0.00002, 0.000297, 0.025184, -0.181133, 0.553040, -0.861938, 0.677066, -0.212591],
        ),
        1800..=1899 => polynomial(
            f64::from(fixed_from_gregorian(year_i, 7, 1) - fixed_from_gregorian(1900, 1, 1))
                / 36525.0,
            &[
                -0.000009, 0.003844, 0.083563, 0.865736, 4.867575, 15.845535, 31.332267, 38.291999,
                28.316289, 11.636204, 2.043794,
            ],
        ),
        1700..=1799 => {
            polynomial(year - 1700.0, &[8.118780842, -0.005092142, 0.003336121, -0.0000266484])
                / 86400.0
        }
        1600..=1699 => {
            polynomial(year - 1600.0, &[120.0, -0.9808, -0.01532, 0.000140272128]) / 86400.0
        }
        500..=1599 => {
            polynomial(
                (year - 1000.0) / 100.0,
                &[1574.2, -556.01, 71.23472, 0.319781, -0.8503463, -0.005050998, 0.0083572073],
            ) / 86400.0
        }
        -499..=499 => {
            polynomial(
                year / 100.0,
                &[10583.6, -1014.41, 33.78311, -5.952053, -0.1798452, 0.022174192, 0.0090316521],
            ) / 86400.0
        }
        _ => polynomial((year - 1820.0) / 100.0, &[-20.0, 0.0, 32.0]) / 86400.0,
    }
}

fn dynamical_from_universal(t: f64) -> f64 {
    t + ephemeris_correction(t)
}

fn universal_from_dynamical(t: f64) -> f64 {
    t - ephemeris_correction(t)
}

const J2000: f64 = 730120.5;

fn julian_centuries(t: f64) -> f64 {
    (dynamical_from_universal(t) - J2000) / 36525.0
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

fn solar_longitude(t: f64) -> f64 {
    let c = julian_centuries(t);
    let lambda = 282.7771834
        + 36000.76953744 * c
        + 0.000005729577951308232 * sigma(&SOLAR_LONGITUDE_TABLE, |(x, y, z)| x * sin(y + z * c));
    clamp_angle(lambda + aberration(c) + nutation(c))
}

// Takes `julian_centuries(t)` instead of `t`
fn nutation(c: f64) -> f64 {
    let a = polynomial(c, &[124.90, -1934.134, 0.002063]);
    let b = polynomial(c, &[201.11, 72001.5377, 0.00057]);
    -0.004778 * sin(a) - 0.0003667 * sin(b)
}

// Takes `julian_centuries(t)` instead of `t`
fn aberration(c: f64) -> f64 {
    0.0000974 * cos(177.63 + 35999.01848 * c) - 0.005575
}

const MEAN_SYNODIC_MONTH: f64 = 29.530588861;

fn nth_new_moon(n: i32) -> f64 {
    let k = f64::from(n - 24724);
    let c = k / 1236.85;
    let approx = J2000
        + polynomial(
            c,
            &[5.09766, MEAN_SYNODIC_MONTH * 1236.85, 0.00015437, -0.000000150, 0.00000000073],
        );
    let e = polynomial(c, &[1.0, -0.002516, -0.0000074]);
    let solar_anomaly = polynomial(c, &[2.5534, 1236.85 * 29.10535670, -0.0000014, -0.00000011]);
    let lunar_anomaly =
        polynomial(c, &[201.5643, 385.81693528 * 1236.85, 0.0107582, 0.00001238, -0.000000058]);
    let moon_argument =
        polynomial(c, &[160.7108, 390.67050284 * 1236.85, -0.0016118, -0.00000227, 0.000000011]);
    let omega = polynomial(c, &[124.7746, -1.56375588 * 1236.85, 0.0020672, 0.00000215]);
    let correction = -0.00017 * sin(omega)
        + sigma(&NTH_NEW_MOON_CORRECTION_TABLE, |(v, w, x, y, z)| {
            v * e.powi(w) * sin(x * solar_anomaly + y * lunar_anomaly + z * moon_argument)
        });
    let extra = 0.000325 * sin(polynomial(c, &[299.77, 132.8475848, -0.009173]));
    let additional = sigma(&NTH_NEW_MOON_ADDITIONAL_TABLE, |(i, j, l)| l * sin(i + j * k));
    universal_from_dynamical(approx + correction + extra + additional)
}

#[cfg(test)]
#[allow(clippy::maybe_infinite_iter)]
fn new_moon_at_or_after(t: f64) -> f64 {
    let t0 = nth_new_moon(0);
    let phi = lunar_phase(t);
    let n = i32((((t - t0) / MEAN_SYNODIC_MONTH) - (phi / 360.0)).round());
    nth_new_moon((n..).find(|k| nth_new_moon(*k) >= t).unwrap())
}

fn lunar_longitude(t: f64) -> f64 {
    let c = julian_centuries(t);
    let mean_lunar_longitude = clamp_angle(polynomial(
        c,
        &[218.3164477, 481267.88123421, -0.0015786, 538841_f64.recip(), -(65194000_f64.recip())],
    ));
    let lunar_elongation = clamp_angle(polynomial(
        c,
        &[297.8501921, 445267.1114034, -0.0018819, 545868_f64.recip(), -(113065000_f64.recip())],
    ));
    let solar_anomaly =
        clamp_angle(polynomial(c, &[357.5291092, 35999.0502909, -0.0001536, 24490000_f64.recip()]));
    let lunar_anomaly = clamp_angle(polynomial(
        c,
        &[134.9633964, 477198.8675055, 0.0087414, 69699_f64.recip(), -(14712000_f64.recip())],
    ));
    let moon_node = clamp_angle(polynomial(
        c,
        &[93.2720950, 483202.0175233, -0.0036539, -(3526000_f64.recip()), 863310000_f64.recip()],
    ));
    let e = polynomial(c, &[1.0, -0.002516, -0.0000074]);
    let correction = sigma(&LUNAR_LONGITUDE_CORRECTION_TABLE, |(v, w, x, y, z)| {
        v * e.powi(i32::abs(x))
            * sin(w * lunar_elongation
                + f64::from(x) * solar_anomaly
                + y * lunar_anomaly
                + z * moon_node)
    }) / 1000000.0;
    let venus = 0.003958 * sin(119.75 + c * 131.849);
    let jupiter = 0.000318 * sin(53.09 + c * 479264.29);
    let flat_earth = 0.001962 * sin(mean_lunar_longitude - moon_node);
    clamp_angle(mean_lunar_longitude + correction + venus + jupiter + flat_earth + nutation(c))
}

pub(crate) fn lunar_phase(t: f64) -> f64 {
    let phi = clamp_angle(lunar_longitude(t) - solar_longitude(t));
    let t0 = nth_new_moon(0);
    let n = i32(((t - t0) / MEAN_SYNODIC_MONTH).round());
    let phi_prime = 360.0 * ((t - nth_new_moon(n)) / MEAN_SYNODIC_MONTH).rem_euclid(1.0);
    if (phi - phi_prime).abs() > 180.0 {
        phi_prime
    } else {
        phi
    }
}

pub(crate) fn lunar_phase_at_or_before(phase: f64, t: f64) -> f64 {
    let tau = t - MEAN_SYNODIC_MONTH / 360.0 * clamp_angle(lunar_phase(t) - phase);
    inv_angle(lunar_phase, phase, tau - 2.0, t.min(tau + 2.0))
}

pub(crate) fn lunar_phase_at_or_after(phase: f64, t: f64) -> f64 {
    let tau = t + MEAN_SYNODIC_MONTH / 360.0 * clamp_angle(phase - lunar_phase(t));
    inv_angle(lunar_phase, phase, t.max(tau - 2.0), tau + 2.0)
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

#[cfg(test)]
#[test]
fn test_sample_data() {
    // Floating-point sample data is truncated at 6 decimal digits.
    macro_rules! assert_feq {
        ($a:expr, $b:expr) => {
            assert_eq!(($a * 1_000_000.0) as i64, ($b * 1_000_000.0) as i64)
        };
    }

    for (rd, (y, m, d), ephem, solar_l, lunar_l, new_moon_test) in TEST_DATA.iter().copied() {
        assert_eq!(gregorian_from_fixed(rd), (y, m, d));
        assert_eq!(fixed_from_gregorian(y, m, d), i32(rd));
        assert_feq!(ephemeris_correction(rd), ephem);
        assert_feq!(solar_longitude(rd + 0.5), solar_l);
        assert_feq!(lunar_longitude(rd), lunar_l);
        let new_moon = new_moon_at_or_after(rd);
        assert_feq!(new_moon, new_moon_test);
        assert!((lunar_phase_at_or_before(0.0, new_moon + 0.001) - new_moon).abs() < 2e-5);
        assert!((lunar_phase_at_or_before(0.0, rd + MEAN_SYNODIC_MONTH) - new_moon).abs() < 2e-5);
        assert!((lunar_phase_at_or_after(0.0, new_moon - 0.001) - new_moon).abs() < 2e-5);
        assert!((lunar_phase_at_or_after(0.0, rd) - new_moon).abs() < 2e-5);
    }
}
