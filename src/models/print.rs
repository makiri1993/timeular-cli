use chrono::{Datelike, NaiveDate};
use colored::{ColoredString, Colorize};
use std::collections::BTreeMap;

pub fn print_subcommand_summary(summary: BTreeMap<NaiveDate, i64>, is_decimal: bool) {
    let date_header = "Date".blue();
    let weekday_header = "Weekday".blue();
    let time_header = "Time".blue();
    let hours_header = "Hours".blue();
    let minutes_header = "Minutes".blue();
    let separator = "|".yellow();
    let mut sum_hours = 0.0;

    println!();
    println!();

    if is_decimal {
        println!(
            "{0: >10} {1} {2: >9} {3} {4: >8}",
            date_header, separator, weekday_header, separator, time_header
        );
    } else {
        println!(
            "{0: >10} {1} {2: >9} {3} {4: >8} {5} {6: >8}",
            date_header,
            separator,
            weekday_header,
            separator,
            hours_header,
            separator,
            minutes_header
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
        .bright_blue()
        .bold();

        if is_decimal {
            print_with_decimals(
                &needs_new_line,
                &separator,
                date,
                &weekday.to_string(),
                &(hours as f32),
                &(minutes as f32),
            );
        } else {
            print_with_hours_minutes(
                &needs_new_line,
                &separator,
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
    separator: &ColoredString,
    date: &NaiveDate,
    weekday: &str,
    hours: &i64,
    minutes: &i64,
) {
    let hours_str = format!("{}h", hours);
    println!(
        "{0}{1: <10} {2} {3: >9} {4} {5: >7} {6} {7: >7}m",
        week,
        date,
        separator,
        weekday,
        separator,
        if hours == &9 {
            hours_str.purple()
        } else if hours == &8 {
            hours_str.green()
        } else {
            hours_str.red()
        },
        separator,
        minutes
    );
}

fn print_with_decimals(
    week: &ColoredString,
    separator: &ColoredString,
    date: &NaiveDate,
    weekday: &str,
    hours: &f32,
    minutes: &f32,
) {
    let time = hours + (minutes / 60.0);
    let formatted_time = format!("{:.2}", time);

    println!(
        "{0}{1: <10} {2} {3: >9} {4} {5:>7}h",
        week,
        date,
        separator,
        weekday,
        separator,
        if time > 9.0 {
            formatted_time.purple()
        } else if time >= 8.0 {
            formatted_time.green()
        } else {
            formatted_time.red()
        }
    );
}
