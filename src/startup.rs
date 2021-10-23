use crate::{
    api::TimeularService,
    configuration::get_configuration,
    enums::{command::Command, flag::Flag},
    helper::input,
    models::{print::PrettyPrint, time::Summarize},
};

use std::env;

pub async fn run() -> Result<(), reqwest::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    log::info!("Config: {:?}", configuration);

    let args: Vec<String> = env::args().collect();
    let command = extract_command(&args);
    let flags = extract_flags(&args[2..]);
    log::info!("Flags: {:?}", flags);

    let client = reqwest::Client::new();

    let timeular_service = TimeularService::new(
        client,
        &configuration.timeular_api_key,
        &configuration.timeular_api_secret,
    )
    .await?;

    match command {
        Command::Summary => {
            subcommand_summary(&flags, &timeular_service).await?;
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

fn validate_month_flag(value: &str) -> Flag {
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
    flags: &[Flag],
    timeular_service: &TimeularService,
) -> Result<(), reqwest::Error> {
    let month_flag = flags.iter().find(|flag| match flag {
        Flag::Month(_) => true,
    });
    if let Some(Flag::Month(month)) = month_flag {
        log::info!("Value for input: {}", month);
        // log::info!("Value for dec: {:?}", matches.is_present("decimal"));
        let (start, end) = input::convert_input_month_to_date_strings(month);
        log::info!("{} {}", start, end);
        let entries = timeular_service.get_timeular_entries(start, end).await?;

        let summarized_entries = entries.summarize_entries_in_tree();

        summarized_entries.print_subcommand_summary();
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
