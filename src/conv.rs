// Copyright (c) 2020 iliana destroyer of worlds <iliana@buttslol.net>
// SPDX-License-Identifier: CC-BY-NC-4.0
//
// This work is licensed under the Creative Commons Attribution-NonCommercial 4.0 International
// License. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/
// or send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.

use crate::calendar::{fixed_from_gregorian, gregorian_from_fixed};
use crate::util::u32;
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};

pub(crate) fn fixed_from_chrono(t: DateTime<Utc>) -> f64 {
    let rd_sec = f64::from(t.hour() * 3600 + t.minute() * 60 + t.second())
        + (f64::from(t.nanosecond()) / 1_000_000_000.0);
    f64::from(fixed_from_gregorian(t.year(), t.month(), t.day())) + (rd_sec / 86400.0)
}

pub(crate) fn chrono_from_fixed(t: f64) -> Option<DateTime<Utc>> {
    let (year, month, day) = gregorian_from_fixed(t);
    let fract = t.rem_euclid(1.0) * 86400.0;
    let hour = u32(fract / 3600.0);
    let min = u32(fract % 3600.0 / 60.0);
    let sec = u32(fract % 60.0);
    let nano = u32((fract % 1.0) * 1_000_000_000.0);
    Utc.ymd_opt(year, month, day).and_hms_nano_opt(hour, min, sec, nano).single()
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
#[test]
fn test_chrono_conversions() {
    let input = [
        (730_120.5, Utc.ymd(2000, 1, 1).and_hms(12, 0, 0)),
        (-61_387.0 + 256_f64.recip(), Utc.ymd(-168, 12, 5).and_hms_milli(0, 5, 37, 500)),
        (-95_746_495.0, chrono::MIN_DATE.and_hms(0, 0, 0)),
        (95_745_764.0, chrono::MAX_DATE.and_hms(0, 0, 0)),
    ];
    let iter = crate::data::TEST_DATA
        .iter()
        .copied()
        .map(|(rd, (y, m, d), _, _, _, _)| (rd, Utc.ymd(y, m, d).and_hms(0, 0, 0)))
        .chain(input.iter().copied());
    for (rd, datetime) in iter {
        assert_eq!(rd, fixed_from_chrono(datetime));
        assert_eq!(chrono_from_fixed(rd).unwrap(), datetime);
    }

    assert!(chrono_from_fixed(-95_746_496.0).is_none());
    assert!(chrono_from_fixed(95_745_765.0).is_none());
}
