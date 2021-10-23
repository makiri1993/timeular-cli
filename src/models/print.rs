use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};

pub trait PrettyPrint {
    fn print_subcommand_summary(&self);
}

impl PrettyPrint for BTreeMap<NaiveDate, i64> {
    fn print_subcommand_summary(&self) {
        let mut sum_hours = 0.0;
        println!(
            "{0: >10} | {1: >9} | {2: >8} | {3: >8}",
            "Date", "Weekday", "Hours", "Minutes"
        );
        let mut week_number = 0;
        self.iter().for_each(|(key, value)| {
            let week_number_entry = key.iso_week().week();
            let weekday = key.weekday();
            let hours = value / 60;
            let minutes = value - hours * 60;

            sum_hours += hours as f64 + minutes as f64 / 60.0;

            let needs_new_line = if week_number != week_number_entry {
                format!("\nWeek {}\n", week_number_entry)
            } else {
                "".to_string()
            };

            println!(
                "{0}{1: <10} | {2: >9} | {3: >7}h | {4: >7}m",
                needs_new_line,
                key,
                weekday.to_string(),
                hours,
                minutes,
            );

            week_number = week_number_entry;
        });
        println!();
        log::info!("You should have worked at least {}h", self.len() * 8);
        log::info!("You have worked {:.2}h", sum_hours);
    }
}
