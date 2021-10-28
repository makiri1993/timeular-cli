use chrono::{Datelike, NaiveDate};
use colored::{ColoredString, Colorize};
use std::collections::BTreeMap;

pub fn print_subcommand_summary(summary: BTreeMap<NaiveDate, i64>, is_decimal: bool) {
    let mut sum_hours = 0.0;

    println!();
    println!();
    if is_decimal {
        println!("{0: >10} | {1: >9} | {2: >8}", "Date", "Weekday", "Time");
    } else {
        println!(
            "{0: >10} | {1: >9} | {2: >8} | {3: >8}",
            "Date", "Weekday", "Hours", "Minutes"
        );
    }
    let mut week_number = 0;
    summary.iter().for_each(|(date, value)| {
        let week_number_entry = date.iso_week().week();
        let weekday = date.weekday();
        let hours = value / 60;
        let minutes = value - hours * 60;

        sum_hours += hours as f64 + minutes as f64 / 60.0;

        let needs_new_line = if week_number != week_number_entry {
            format!("\nWeek {}\n", week_number_entry)
        } else {
            "".to_string()
        }
        .as_str()
        .yellow()
        .bold();

        if is_decimal {
            print_with_decimals(
                &needs_new_line,
                date,
                &weekday.to_string(),
                &(hours as f32),
                &(minutes as f32),
            );
        } else {
            print_with_hours_minutes(
                &needs_new_line,
                date,
                &weekday.to_string(),
                &hours,
                &minutes,
            );
        }

        week_number = week_number_entry;
    });
    println!();
    log::info!("You should have worked at least {}h", summary.len() * 8);
    log::info!("You have worked {:.2}h", sum_hours);
}

fn print_with_hours_minutes(
    week: &ColoredString,
    date: &NaiveDate,
    weekday: &str,
    hours: &i64,
    minutes: &i64,
) {
    println!(
        "{0}{1: <10} | {2: >9} | {3: >7}h | {4: >7}m",
        week, date, weekday, hours, minutes,
    );
}

fn print_with_decimals(week: &str, date: &NaiveDate, weekday: &str, hours: &f32, minutes: &f32) {
    println!(
        "{0}{1: <10} | {2: >9} | {3:>7.2}h",
        week,
        date,
        weekday,
        hours + (minutes / 60.0),
    );
}
