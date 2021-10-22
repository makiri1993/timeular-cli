use crate::{
    api,
    configuration::get_configuration,
    enums::{command::Command, flag::Flag},
    helper::input,
    models,
};

use std::{collections::BTreeMap, env};

use chrono::{Datelike, NaiveDate};

pub async fn run() -> Result<(), reqwest::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    log::info!("Config: {:?}", configuration);

    let args: Vec<String> = env::args().collect();
    let command = extract_command(&args);
    let flags = extract_flags(&args[2..]);
    log::info!("Flags: {:?}", flags);

    let client = reqwest::Client::new();
    let timeular_token = api::get_timeular_token(&configuration, &client).await?;
    let activities = api::get_timeular_activities(&client, &timeular_token).await?;
    match command {
        Command::Summary => {
            subcommand_summary(&flags, &client, &timeular_token, &activities).await?;
        }
        Command::Entries => todo!(),
    }

    Ok(())
}

fn extract_flags(flags: &[String]) -> Vec<Flag> {
    let mut flag_tupels: Vec<Flag> = Vec::new();
    let mut n = 0;

    while n < flags.len() {
        log::info!("{:?}", flags.get(n));
        let flag_key = flags.get(n);
        let flag_value = flags.get(n + 1);

        let flag = parse_flag(flag_key, flag_value);

        flag_tupels.push(flag);

        n += 2;
    }

    flag_tupels
}

fn parse_flag(flag_key: Option<&String>, flag_value: Option<&String>) -> Flag {
    match flag_key.as_deref().map(|s| &s[..]) {
        Some("-m") => validate_month_flag(flag_value.expect("No month provided")),
        Some(_) => todo!(),
        None => todo!(),
    }
}

fn validate_month_flag(value: &String) -> Flag {
    let is_valid_month = input::is_valid_month(value);
    match is_valid_month {
        true => (Flag::Month(value.to_owned())),
        false => panic!(
            "Only these values are valid months. \
                (jan, feb, mar, apr, may, jun, jul, aug, sep, okt, nov, dec)"
        ),
    }
}

fn extract_command(args: &[String]) -> Command {
    let command = args.get(1);
    match command.as_deref().map(|s| &s[..]) {
        Some("summary") => Command::Summary,
        Some("entries") => Command::Entries,
        Some(val) => panic!("This command '{}' is not supported.", val),
        None => panic!("No command provided."),
    }
}

// fn setup_cli() -> clap::ArgMatches {
//     App::new("Timeular CLI")
//         .version("0.1")
//         .author("Martin Kireew <martin@techstudio.dev>")
//         .about("Get data from Timeular and summarize it")
//         .subcommand(App::new(ENTRIES_COMMAND).about("Shows all entries from the a period of time."))
//         .subcommand(
//             App::new(SUMMARY_COMMAND)
//                 .about("Summarizes the entries from a period of time.")
//                 .arg(
//                     Arg::new(MONTH_FLAG)
//                         .short('m')
//                         .about("set the month for the entries")
//                         .takes_value(true)
//                         .validator(input::is_valid_month),
//                 )
//                 .arg(
//                     Arg::new("decimal")
//                         .about("sets the format of the output")
//                         .short('d')
//                         .long("dec"),
//                 ),
//         )
//         .get_matches()
// }

async fn subcommand_summary(
    flags: &Vec<Flag>,
    client: &reqwest::Client,
    timeular_token: &str,
    activities: &[models::timeular::Activity],
) -> Result<(), reqwest::Error> {
    let month_flag = flags.into_iter().find(|flag| match flag {
        Flag::Month(_) => true,
    });
    if let Some(Flag::Month(month)) = month_flag {
        log::info!("Value for input: {}", month);
        // log::info!("Value for dec: {:?}", matches.is_present("decimal"));
        let (start, end) = input::convert_input_month_to_date_strings(month);
        log::info!("{} {}", start, end);
        let entries =
            api::get_timeular_entries(&client, &timeular_token, &activities, start, end).await?;

        let summarized_entries = models::time::summarize_entries_in_tree(&entries);

        print_subcommand_summary(summarized_entries);
    }
    Ok(())
}

// async fn subcommand_entries(
//     matches: &clap::ArgMatches,
//     client: &reqwest::Client,
//     timeular_token: &str,
//     activities: &[models::timeular::Activity],
// ) -> Result<(), reqwest::Error> {
//     if let Some(month) = matches.value_of(MONTH_FLAG) {
//         let (start, end) = input::convert_input_month_to_date_strings(month);
//         info!("{} {}", start, end);
//         let mut entries =
//             api::get_timeular_entries(client, timeular_token, activities, "01-01", "12-31").await?;

//         entries.sort();
//         print_subcommand_entries(&entries);
//     }
//     Ok(())
// }

// fn print_subcommand_entries(entries: &[time::Entry]) {
//     info!(
//         "{0: >10} | {1: >10} | {2: >8} | {3: >8}",
//         "Date", "Activity", "Hours", "Minutes"
//     );
//     entries.iter().for_each(|activity| {
//         info!(
//             "{0: <10} | {1: >10} | {2: >7}h | {3: >7}m",
//             activity.date,
//             activity.activity,
//             activity.duration.num_hours(),
//             activity.duration.num_minutes() - activity.duration.num_hours() * 60
//         );
//     });
// }

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
    log::info!("You should have worked at least {}h", entries.len() * 8);
    log::info!("You have worked {:.2}h", sum_hours);
}
