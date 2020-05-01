// Copyright (c) 2020 iliana destroyer of worlds <iliana@buttslol.net>
// SPDX-License-Identifier: CC-BY-NC-4.0
//
// This work is licensed under the Creative Commons Attribution-NonCommercial 4.0 International
// License. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/
// or send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.

use crate::util::clamp_angle;

const NEW_MOON: f64 = 0.0;
const NEW_MOON_HIGH: f64 = 360.0;
const FIRST_QUARTER: f64 = 90.0;
const FULL_MOON: f64 = 180.0;
const LAST_QUARTER: f64 = 270.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// The eight principal and intermediate phases of the Moon.
pub enum Phase {
    /// 🌑 The Moon appears completely dark.
    NewMoon,
    /// 🌒 The Moon is less than half visible after the new moon and before the full moon.
    WaxingCrescent,
    /// 🌓 The Moon is half visible after the new moon and before the full moon.
    FirstQuarter,
    /// 🌔 The Moon is more than half visible after the new moon and before the full moon.
    WaxingGibbous,
    /// 🌕 The Moon is fully visible.
    FullMoon,
    /// 🌖 The Moon is more than half visible after the full moon and before the new moon.
    WaningGibbous,
    /// 🌗 The Moon is half visible after the full moon and before the new moon.
    LastQuarter,
    /// 🌘 The Moon is less than half visible after the full moon and before the new moon.
    WaningCrescent,
}

impl Phase {
    /// Returns the Unicode emoji representation of the moon phase.
    ///
    /// ```
    /// # use esbat::Phase;
    /// assert_eq!(Phase::FullMoon.as_emoji(), '🌕');
    /// ```
    pub fn as_emoji(self) -> char {
        match self {
            Phase::NewMoon => '\u{1f311}',
            Phase::WaxingCrescent => '\u{1f312}',
            Phase::FirstQuarter => '\u{1f313}',
            Phase::WaxingGibbous => '\u{1f314}',
            Phase::FullMoon => '\u{1f315}',
            Phase::WaningGibbous => '\u{1f316}',
            Phase::LastQuarter => '\u{1f317}',
            Phase::WaningCrescent => '\u{1f318}',
        }
    }

    /// Views the eight moon phases as an iterator.
    ///
    /// ```
    /// # use esbat::Phase;
    /// assert_eq!(Phase::iter().next(), Some(&Phase::NewMoon));
    /// assert_eq!(Phase::iter().len(), 8);
    /// ```
    pub fn iter() -> core::slice::Iter<'static, Phase> {
        [
            Phase::NewMoon,
            Phase::WaxingCrescent,
            Phase::FirstQuarter,
            Phase::WaxingGibbous,
            Phase::FullMoon,
            Phase::WaningGibbous,
            Phase::LastQuarter,
            Phase::WaningCrescent,
        ]
        .iter()
    }

    /// Returns true if the phase is one of the four principal phases.
    ///
    /// ```
    /// # use esbat::Phase;
    /// assert_eq!(Phase::NewMoon.is_principal(), true);
    /// ```
    pub fn is_principal(self) -> bool {
        match self {
            Phase::NewMoon | Phase::FirstQuarter | Phase::FullMoon | Phase::LastQuarter => true,
            _ => false,
        }
    }

    pub(crate) fn from_phase_range(start: f64, mut end: f64) -> Phase {
        debug_assert!((clamp_angle(start) - start).abs() < f64::EPSILON);
        debug_assert!((clamp_angle(end) - end).abs() < f64::EPSILON);
        if end < start {
            end += 360.0;
        }

        if (start..end).contains(&NEW_MOON) || (start..end).contains(&NEW_MOON_HIGH) {
            Phase::NewMoon
        } else if (start..end).contains(&FIRST_QUARTER) {
            Phase::FirstQuarter
        } else if (start..end).contains(&FULL_MOON) {
            Phase::FullMoon
        } else if (start..end).contains(&LAST_QUARTER) {
            Phase::LastQuarter
        } else if start < FIRST_QUARTER {
            Phase::WaxingCrescent
        } else if start < FULL_MOON {
            Phase::WaxingGibbous
        } else if start < LAST_QUARTER {
            Phase::WaningGibbous
        } else {
            Phase::WaningCrescent
        }
    }
}

#[cfg(test)]
#[test]
fn test_from_phase_range() {
    assert_eq!(Phase::from_phase_range(359.0, 1.0), Phase::NewMoon);
    assert_eq!(Phase::from_phase_range(0.0, 2.0), Phase::NewMoon);
    assert_eq!(Phase::from_phase_range(37.0, 39.0), Phase::WaxingCrescent);
    assert_eq!(Phase::from_phase_range(88.0, 90.0), Phase::WaxingCrescent);
    assert_eq!(Phase::from_phase_range(89.0, 91.0), Phase::FirstQuarter);
    assert_eq!(Phase::from_phase_range(90.0, 92.0), Phase::FirstQuarter);
    assert_eq!(Phase::from_phase_range(132.0, 134.0), Phase::WaxingGibbous);
    assert_eq!(Phase::from_phase_range(178.0, 180.0), Phase::WaxingGibbous);
    assert_eq!(Phase::from_phase_range(179.0, 181.0), Phase::FullMoon);
    assert_eq!(Phase::from_phase_range(180.0, 182.0), Phase::FullMoon);
    assert_eq!(Phase::from_phase_range(216.0, 218.0), Phase::WaningGibbous);
    assert_eq!(Phase::from_phase_range(268.0, 270.0), Phase::WaningGibbous);
    assert_eq!(Phase::from_phase_range(269.0, 271.0), Phase::LastQuarter);
    assert_eq!(Phase::from_phase_range(270.0, 272.0), Phase::LastQuarter);
    assert_eq!(Phase::from_phase_range(314.0, 316.0), Phase::WaningCrescent);
    assert_eq!(Phase::from_phase_range(358.0, 0.0), Phase::WaningCrescent);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// The four principal phases of the Moon.
pub enum PrincipalPhase {
    /// 🌑 The Moon appears completely dark.
    NewMoon,
    /// 🌓 The Moon is half visible after the new moon and before the full moon.
    FirstQuarter,
    /// 🌕 The Moon is fully visible.
    FullMoon,
    /// 🌗 The Moon is half visible after the full moon and before the new moon.
    LastQuarter,
}

impl PrincipalPhase {
    /// Returns the Unicode emoji representation of the moon phase.
    ///
    /// ```
    /// # use esbat::PrincipalPhase;
    /// assert_eq!(PrincipalPhase::FullMoon.as_emoji(), '🌕');
    /// ```
    pub fn as_emoji(self) -> char {
        Phase::from(self).as_emoji()
    }

    /// Views the four principal moon phases as an iterator.
    ///
    /// ```
    /// # use esbat::PrincipalPhase;
    /// assert_eq!(PrincipalPhase::iter().next(), Some(&PrincipalPhase::NewMoon));
    /// assert_eq!(PrincipalPhase::iter().len(), 4);
    /// ```
    pub fn iter() -> core::slice::Iter<'static, PrincipalPhase> {
        [
            PrincipalPhase::NewMoon,
            PrincipalPhase::FirstQuarter,
            PrincipalPhase::FullMoon,
            PrincipalPhase::LastQuarter,
        ]
        .iter()
    }

    pub(crate) fn as_angle(self) -> f64 {
        match self {
            PrincipalPhase::NewMoon => NEW_MOON,
            PrincipalPhase::FirstQuarter => FIRST_QUARTER,
            PrincipalPhase::FullMoon => FULL_MOON,
            PrincipalPhase::LastQuarter => LAST_QUARTER,
        }
    }
}

impl From<PrincipalPhase> for Phase {
    fn from(phase: PrincipalPhase) -> Phase {
        match phase {
            PrincipalPhase::NewMoon => Phase::NewMoon,
            PrincipalPhase::FirstQuarter => Phase::FirstQuarter,
            PrincipalPhase::FullMoon => Phase::FullMoon,
            PrincipalPhase::LastQuarter => Phase::LastQuarter,
        }
    }
}
