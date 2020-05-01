// Copyright (c) 2020 iliana destroyer of worlds <iliana@buttslol.net>
// SPDX-License-Identifier: CC-BY-NC-4.0
//
// This work is licensed under the Creative Commons Attribution-NonCommercial 4.0 International
// License. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/
// or send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.

macro_rules! f64_to {
    ($ty:ident) => {
        #[allow(clippy::cast_possible_truncation)]
        pub(crate) fn $ty(x: f64) -> $ty {
            let x = x.trunc();
            debug_assert!(f64::from($ty::MIN) <= x && x <= f64::from($ty::MAX));
            x as $ty
        }
    };
}
f64_to!(i32);
f64_to!(u32);

#[allow(clippy::cast_sign_loss)]
pub(crate) fn nonneg(x: i32) -> u32 {
    debug_assert!(!x.is_negative());
    x as u32
}

pub(crate) fn sin(x: f64) -> f64 {
    x.to_radians().sin()
}

pub(crate) fn cos(x: f64) -> f64 {
    x.to_radians().cos()
}

pub(crate) fn clamp_angle(x: f64) -> f64 {
    x.rem_euclid(360.0)
}

pub(crate) fn polynomial(base: f64, coefficients: &[f64]) -> f64 {
    coefficients.iter().rev().fold(0.0, |x, c| x * base + c)
}

pub(crate) fn sigma<T, F>(table: &[T], func: F) -> f64
where
    T: Copy,
    F: Fn(T) -> f64,
{
    table.iter().copied().map(func).sum()
}

pub(crate) fn inv_angle<F>(f: F, y: f64, mut start: f64, mut end: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    loop {
        if (start - end).abs() < f64::EPSILON {
            return start;
        }
        let x = (start + end) / 2.0;
        let diff = clamp_angle(f(x) - y);
        if diff < 1e-5 || diff > (360.0 - 1e-5) {
            break x;
        } else if diff < 180.0 {
            end = x;
        } else {
            start = x;
        }
    }
}
