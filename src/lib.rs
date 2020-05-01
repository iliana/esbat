// Copyright (c) 2020 iliana destroyer of worlds <iliana@buttslol.net>
// SPDX-License-Identifier: CC-BY-NC-4.0
//
// This work is licensed under the Creative Commons Attribution-NonCommercial 4.0 International
// License. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/
// or send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.

//! esbat provides functions for lunar phase calculations.
//!
//! The algorithms are as published in [<i>Calendrical Calculations: The Ultimate
//! Edition</i>][book] by Edward M. Reingold and Nachum Dershowitz, used with permission. The book
//! notes:
//!
//! > ... the algorithms are centered around the present date, for which they are accurate to
//! > within about 2 minutes. Their accuracy decreases for the far-distant past or future.
//!
//! # License
//!
//! **This crate uses a non-commercial license**, a [Creative Commons Attribution-NonCommercial 4.0
//! International License][license], at the request of the authors of <i>Calendrical
//! Calculations</i>. Please contact the author of this crate at
//! [iliana@buttslol.net][mail] for any licensing questions.
//!
//! [book]: https://doi.org/10.1017/9781107415058
//! [license]: https://creativecommons.org/licenses/by-nc/4.0/
//! [mail]: mailto:iliana@buttslol.net

#![warn(
    absolute_paths_not_starting_with_crate,
    trivial_casts,
    trivial_numeric_casts,
    rust_2018_idioms,
    clippy::pedantic
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value
)]

mod calendar;
mod conv;
mod data;
mod iter;
mod phase;
mod util;

pub use crate::iter::{daily_lunar_phase_iter, lunar_phase_iter, DailyIter, Iter};
pub use crate::phase::{Phase, PrincipalPhase};

use crate::conv::fixed_from_chrono;
use chrono::{Date, DateTime, Duration, TimeZone, Utc};

/// Calculates the lunar phase for a given moment.
///
/// This determines the difference in longitudes of the Sun and the Moon, in degrees, for the
/// moment `t`. The result is clamped to 0&deg;&nbsp;&le;&nbsp;<i>x</i>&nbsp;&lt;&nbsp;360&deg;.
///
/// The new moon is 0&deg;, the first-quarter moon is 90&deg;, the full moon is 180&deg;, and the
/// last-quarter moon is 270&deg;.
///
/// ```
/// use chrono::{DateTime, TimeZone, Utc};
/// use esbat::lunar_phase;
///
/// let t = Utc.ymd(2020, 10, 31).and_hms_milli(14, 48, 59, 300);
/// assert!((lunar_phase(t) - 180.0).abs() < 0.00001);
/// ```
pub fn lunar_phase<Tz: TimeZone>(t: DateTime<Tz>) -> f64 {
    calendar::lunar_phase(fixed_from_chrono(t.with_timezone(&Utc)))
}

/// Calculates the lunar phase for a given date.
///
/// This determines the principal phase (new moon, first quarter, full moon, or third quarter) that
/// occurs on the date `t` or the intermediate phase between the previous and next principal
/// phases.
///
/// ```
/// use chrono::{Date, TimeZone, Utc};
/// use esbat::{Phase, daily_lunar_phase};
///
/// let t = Utc.ymd(2020, 10, 31);
/// assert_eq!(daily_lunar_phase(t), Phase::FullMoon);
/// ```
pub fn daily_lunar_phase<Tz: TimeZone>(t: Date<Tz>) -> Phase {
    let t = t.and_hms(0, 0, 0).with_timezone(&Utc);
    Phase::from_phase_range(lunar_phase(t), lunar_phase(t + Duration::days(1)))
}
