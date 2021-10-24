use crate::{
    api::TimeularService,
    configuration::get_configuration,
    enums::{
        command::{Command, ExtractCommand},
        flag::{ExtractFlags, Flag},
    },
    helper::input,
    models::{print::PrettyPrint, time::Summarize},
};

use std::env;

pub async fn run() -> Result<(), reqwest::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    log::info!("Config: {:?}", configuration);

    let args: Vec<String> = env::args().collect();
    let command = &args.extract_command();
    let flags = &args.extract_flags();
    // let flags = extract_flags(&args[2..]);
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
            subcommand_summary(flags, &timeular_service).await?;
        }
        Command::Entries => todo!("entries command"),
    }

    Ok(())
}

async fn subcommand_summary(
    flags: &[Flag],
    timeular_service: &TimeularService,
) -> Result<(), reqwest::Error> {
    let month_flag = flags.iter().find(|flag| matches!(flag, Flag::Month(_)));
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
