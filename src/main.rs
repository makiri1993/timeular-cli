use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};
use clap::{App, Arg};
use log::info;

use helper::input;
use models::flag::Flags;
use models::subcommand::SubCommands;
use models::time;

mod api;
mod console;
mod enums;
mod helper;
mod models;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    log::set_logger(&console::CONSOLE_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let summary_flag = Flags::Month.value();

    let subcommand_entries = SubCommands::Entries.value();
    let subcommand_summary = SubCommands::Summary.value();
    let matches = App::new("Timeular CLI")
        .version("0.1")
        .author("Martin Kireew <martin@techstudio.dev>")
        .about("Get data from Timeular and summarize it")
        // .arg(
        //     Arg::new("INPUT")
        //         .about("Sets the input file to use")
        //         .required(true)
        //         .index(1),
        // )
        .subcommand(App::new(subcommand_entries.command).about(subcommand_entries.description))
        .subcommand(
            App::new(subcommand_summary.command)
                .about(subcommand_summary.description)
                .arg(
                    Arg::new(summary_flag.long)
                        .short(summary_flag.short)
                        .about(summary_flag.about)
                        .takes_value(summary_flag.takes_value)
                        .validator(input::is_valid_month),
                ),
        )
        .get_matches();

    let web_client = reqwest::Client::new();
    let timeular_token = api::get_timeular_token(&web_client).await?;

    let activities = api::get_timeular_activities(&web_client, &timeular_token).await?;

    if matches
        .subcommand_matches(subcommand_entries.command)
        .is_some()
    {
        let (start, end) = input::convert_input_month_to_date_strings("feb");
        info!("{} {}", start, end);
        let mut entries =
            api::get_timeular_entries(&web_client, &timeular_token, &activities, "01-01", "12-31")
                .await?;

        entries.sort();
        print_subcommand_entries(&entries);
    }

    if let Some(ref matches) = matches.subcommand_matches(subcommand_summary.command) {
        if let Some(month) = matches.value_of(summary_flag.long) {
            info!("Value for input: {}", month);
            let (start, end) = input::convert_input_month_to_date_strings(month);
            info!("{} {}", start, end);
            let entries =
                api::get_timeular_entries(&web_client, &timeular_token, &activities, start, end)
                    .await?;

            let summarized_entries = time::summarize_entries_in_tree(&entries);

            print_subcommand_summary(summarized_entries);
        }
    }

    Ok(())
}

fn print_subcommand_entries(entries: &[time::Entry]) {
    info!(
        "{0: >10} | {1: >10} | {2: >8} | {3: >8}",
        "Date", "Activity", "Hours", "Minutes"
    );
    entries.iter().for_each(|activity| {
        info!(
            "{0: <10} | {1: >10} | {2: >7}h | {3: >7}m",
            activity.date,
            activity.activity,
            activity.duration.num_hours(),
            activity.duration.num_minutes() - activity.duration.num_hours() * 60
        );
    });
}

fn print_subcommand_summary(entries: BTreeMap<NaiveDate, i64>) {
    let mut sum_hours = 0.0;
    println!(
        "{0: >10} | {1: >9} | {2: >8} | {3: >8}",
        "Date", "Weekday", "Hours", "Minutes"
    );
    let mut week_number = 0;
    entries.iter().for_each(|(key, value)| {
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
        // println!("{:?}", week_number.week());
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
    info!("You should have worked at least {}h", entries.len() * 8);
    info!("You have worked {:.2}h", sum_hours);
}
