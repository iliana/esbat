// Copyright (c) 2020 iliana destroyer of worlds <iliana@buttslol.net>
// SPDX-License-Identifier: CC-BY-NC-4.0
//
// This work is licensed under the Creative Commons Attribution-NonCommercial 4.0 International
// License. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/
// or send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.

use crate::calendar::{lunar_phase, lunar_phase_at_or_after, lunar_phase_at_or_before};
use crate::conv::{chrono_from_fixed, fixed_from_chrono};
use crate::phase::PrincipalPhase;
use chrono::{Date, DateTime, Duration, Utc};
use core::iter::FusedIterator;
use core::ops::{Bound, RangeBounds};

fn min_time() -> DateTime<Utc> {
    chrono::MIN_DATE.and_hms(0, 0, 0)
}

fn max_time() -> DateTime<Utc> {
    chrono::MAX_DATE.and_hms_nano(23, 59, 59, 999_999_999)
}

fn add_day(t: DateTime<Utc>, positive: bool) -> DateTime<Utc> {
    t.checked_add_signed(Duration::days(if positive { 1 } else { -1 })).unwrap_or_else(|| {
        if positive {
            max_time()
        } else {
            min_time()
        }
    })
}

fn handle_bound<T: Copy, F>(bound: Bound<&T>, default: F) -> (T, bool)
where
    F: Fn() -> T,
{
    match bound {
        Bound::Included(t) => (*t, false),
        Bound::Excluded(t) => (*t, true),
        Bound::Unbounded => (default(), false),
    }
}

/// Returns an iterator of principal phases and their moments.
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use esbat::PrincipalPhase;
///
/// let start = Utc.ymd(2020, 10, 1).and_hms(0, 0, 0);
/// let end = Utc.ymd(2020, 11, 1).and_hms(0, 0, 0);
/// let mut iter = esbat::lunar_phase_iter(start..end);
///
/// assert_eq!(iter.next().unwrap().0, PrincipalPhase::FullMoon);
/// assert_eq!(iter.next().unwrap().0, PrincipalPhase::LastQuarter);
/// assert_eq!(iter.next().unwrap().0, PrincipalPhase::NewMoon);
/// assert_eq!(iter.next().unwrap().0, PrincipalPhase::FirstQuarter);
/// assert_eq!(iter.next().unwrap().0, PrincipalPhase::FullMoon);
/// assert!(iter.next().is_none());
/// ```
pub fn lunar_phase_iter<B>(range: B) -> Iter
where
    B: RangeBounds<DateTime<Utc>>,
{
    let (start, start_excl) = handle_bound(range.start_bound(), min_time);
    let (end, end_excl) = handle_bound(range.end_bound(), max_time);
    Iter::new(start, start_excl, end, end_excl)
}

/// Principal phase iterator.
///
/// This struct is created by [`lunar_phase_iter`].
#[derive(Debug, Clone)]
pub struct Iter {
    bound: Option<(DateTime<Utc>, DateTime<Utc>)>,
    positive: bool,
}

impl Iter {
    fn new(
        mut start: DateTime<Utc>,
        start_excl: bool,
        mut end: DateTime<Utc>,
        end_excl: bool,
    ) -> Iter {
        fn close_to_phase(t: DateTime<Utc>) -> bool {
            let x = lunar_phase(fixed_from_chrono(t)).rem_euclid(90.0);
            x < 0.00001 || 89.99999 < x
        }

        let positive = start <= end;
        if start_excl && close_to_phase(start) {
            start = add_day(start, positive);
        }
        if end_excl && close_to_phase(end) {
            end = add_day(end, !positive);
        }
        Iter { bound: Some((start, end)), positive }
    }
}

impl Iterator for Iter {
    type Item = (PrincipalPhase, DateTime<Utc>);

    fn next(&mut self) -> Option<(PrincipalPhase, DateTime<Utc>)> {
        let (start, end) = self.bound?;
        let start = fixed_from_chrono(start);

        let phase = lunar_phase(start);
        let phase = if self.positive {
            if phase <= 90.0 {
                PrincipalPhase::FirstQuarter
            } else if phase <= 180.0 {
                PrincipalPhase::FullMoon
            } else if phase <= 270.0 {
                PrincipalPhase::LastQuarter
            } else {
                PrincipalPhase::NewMoon
            }
        } else if phase >= 270.0 {
            PrincipalPhase::LastQuarter
        } else if phase >= 180.0 {
            PrincipalPhase::FullMoon
        } else if phase >= 90.0 {
            PrincipalPhase::FirstQuarter
        } else {
            PrincipalPhase::NewMoon
        };

        let next_rd = if self.positive {
            lunar_phase_at_or_after(phase.as_angle(), start)
        } else {
            lunar_phase_at_or_before(phase.as_angle(), start)
        };

        if let Some(next) = chrono_from_fixed(next_rd) {
            if self.positive && next <= end {
                self.bound = Some((add_day(next, true), end));
                return Some((phase, next));
            } else if !self.positive && next >= end {
                self.bound = Some((add_day(next, false), end));
                return Some((phase, next));
            }
        }

        self.bound = None;
        None
    }
}

impl FusedIterator for Iter {}

#[cfg(test)]
#[test]
fn test_iter_rev() {
    use chrono::TimeZone;

    let start = Utc.ymd(2020, 11, 1).and_hms(0, 0, 0);
    let end = Utc.ymd(2020, 10, 1).and_hms(0, 0, 0);
    let mut iter = lunar_phase_iter(start..end);

    assert_eq!(iter.next().unwrap().0, PrincipalPhase::FullMoon);
    assert_eq!(iter.next().unwrap().0, PrincipalPhase::FirstQuarter);
    assert_eq!(iter.next().unwrap().0, PrincipalPhase::NewMoon);
    assert_eq!(iter.next().unwrap().0, PrincipalPhase::LastQuarter);
    assert_eq!(iter.next().unwrap().0, PrincipalPhase::FullMoon);
    assert!(iter.next().is_none());
}

/// Returns an iterator of principal phases and the days they fall on.
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use esbat::PrincipalPhase;
///
/// let start = Utc.ymd(2020, 10, 1);
/// let end = Utc.ymd(2020, 11, 1);
/// let mut iter = esbat::daily_lunar_phase_iter(start..end);
///
/// assert_eq!(iter.next().unwrap(), (PrincipalPhase::FullMoon, Utc.ymd(2020, 10, 1)));
/// assert_eq!(iter.next().unwrap(), (PrincipalPhase::LastQuarter, Utc.ymd(2020, 10, 10)));
/// assert_eq!(iter.next().unwrap(), (PrincipalPhase::NewMoon, Utc.ymd(2020, 10, 16)));
/// assert_eq!(iter.next().unwrap(), (PrincipalPhase::FirstQuarter, Utc.ymd(2020, 10, 23)));
/// assert_eq!(iter.next().unwrap(), (PrincipalPhase::FullMoon, Utc.ymd(2020, 10, 31)));
/// assert!(iter.next().is_none());
/// ```
pub fn daily_lunar_phase_iter<B>(range: B) -> DailyIter
where
    B: RangeBounds<Date<Utc>>,
{
    fn fix(t: Date<Utc>, down: bool) -> DateTime<Utc> {
        if down {
            t.and_hms(0, 0, 0)
        } else {
            t.and_hms_nano(23, 59, 59, 999_999_999)
        }
    }

    let (start, start_excl) = handle_bound(range.start_bound(), || chrono::MIN_DATE);
    let (end, end_excl) = handle_bound(range.end_bound(), || chrono::MAX_DATE);
    let positive = start <= end;
    DailyIter { inner: Iter::new(fix(start, positive), start_excl, fix(end, !positive), end_excl) }
}

/// Principal phase iterator by day.
///
/// This struct is created by [`daily_lunar_phase_iter`].
#[derive(Debug, Clone)]
pub struct DailyIter {
    inner: Iter,
}

impl Iterator for DailyIter {
    type Item = (PrincipalPhase, Date<Utc>);

    fn next(&mut self) -> Option<(PrincipalPhase, Date<Utc>)> {
        let next = self.inner.next()?;
        dbg!(&self.inner);
        Some((next.0, next.1.date()))
    }
}

impl FusedIterator for DailyIter {}

#[cfg(test)]
#[test]
fn test_daily_iter_rev() {
    use chrono::TimeZone;

    let start = Utc.ymd(2020, 11, 1);
    let end = Utc.ymd(2020, 10, 1);
    let mut iter = daily_lunar_phase_iter(start..end);

    assert_eq!(iter.next().unwrap(), (PrincipalPhase::FullMoon, Utc.ymd(2020, 10, 31)));
    assert_eq!(iter.next().unwrap(), (PrincipalPhase::FirstQuarter, Utc.ymd(2020, 10, 23)));
    assert_eq!(iter.next().unwrap(), (PrincipalPhase::NewMoon, Utc.ymd(2020, 10, 16)));
    assert_eq!(iter.next().unwrap(), (PrincipalPhase::LastQuarter, Utc.ymd(2020, 10, 10)));
    assert_eq!(iter.next().unwrap(), (PrincipalPhase::FullMoon, Utc.ymd(2020, 10, 1)));
    assert!(iter.next().is_none());
}

#[cfg(test)]
#[test]
fn test_ranges() {
    use chrono::TimeZone;

    let start = Utc.ymd(2020, 10, 1).and_hms(0, 0, 0);
    let end = Utc.ymd(2020, 11, 1).and_hms(0, 0, 0);
    lunar_phase_iter(..);
    lunar_phase_iter(start..);
    lunar_phase_iter(..end);
    lunar_phase_iter(..=end);
    lunar_phase_iter(start..end);
    lunar_phase_iter(start..=end);

    let start = Utc.ymd(2020, 10, 1);
    let end = Utc.ymd(2020, 11, 1);
    daily_lunar_phase_iter(..);
    daily_lunar_phase_iter(start..);
    daily_lunar_phase_iter(..end);
    daily_lunar_phase_iter(..=end);
    daily_lunar_phase_iter(start..end);
    daily_lunar_phase_iter(start..=end);
}
