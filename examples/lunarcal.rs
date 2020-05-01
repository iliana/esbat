// Copyright (c) 2020 iliana destroyer of worlds <iliana@buttslol.net>
// SPDX-License-Identifier: CC-BY-NC-4.0
//
// This work is licensed under the Creative Commons Attribution-NonCommercial 4.0 International
// License. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/
// or send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.

// A simple, option-free version of cal(1) that replaces day numbers with lunar phase emoji for
// principal phases.
//
// It assumes that the current local time offset holds true through the entire month, which may
// produce inaccuracies.

use chrono::{Datelike, Duration, Local, TimeZone, Weekday};
use esbat::daily_lunar_phase;

fn main() {
    let today = Local::today();
    println!("{:^20}", today.format("%B %Y"));
    println!("Su Mo Tu We Th Fr Sa");

    let first_day = Local.ymd(today.year(), today.month(), 1);
    let next_month = Local.ymd(today.year(), today.month(), 1) + Duration::days(31);
    let last_day = Local.ymd(next_month.year(), next_month.month(), 1) - Duration::days(1);

    print!("{:>width$}", "", width = 3 * first_day.weekday().num_days_from_sunday() as usize);

    let mut day = first_day;
    let mut weeks = 0;
    while day <= last_day {
        if today == day {
            print!("\x1b[7m");
        }

        let phase = daily_lunar_phase(day);
        if phase.is_principal() {
            print!("{}", phase.as_emoji());
        } else {
            print!("{:>2}", day.day());
        }

        if today == day {
            print!("\x1b[27m");
        }

        if day.weekday() == Weekday::Sat {
            println!();
            weeks += 1;
        } else {
            print!(" ");
        }
        day = day + Duration::days(1);
    }

    for _ in weeks..6 {
        println!();
    }
}
