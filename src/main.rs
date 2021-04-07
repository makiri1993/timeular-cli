use crate::enums::{ApiUrls, SubCommand};
use crate::models::{
    TimeEntry, TimeularActivitiesResponse, TimeularEntriesResponse, TimeularLoginResponse,
};
use chrono::NaiveDate;
use clap::App;
use log::info;
use serde_json::json;
use std::collections::BTreeMap;
use std::env;

mod console;
mod enums;
mod models;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    log::set_logger(&console::CONSOLE_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

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
        .subcommand(
            App::new(SubCommand::Entries.value())
                .about("Shows all entries from the a period of time."), // .arg("-l, --list 'lists test values'"),
        )
        .subcommand(
            App::new(SubCommand::Summary.value())
                .about("Summarizes the entries from a period of time."), // .arg("-l, --list 'lists test values'"),
        )
        .get_matches();

    let web_client = reqwest::Client::new();
    let timeular_token = get_timeular_token(&web_client).await?;

    let mut entries = get_timeular_data(&web_client, timeular_token).await?;

    entries.sort();

    // You can check the value provided by positional arguments, or option arguments
    // if let Some(i) = matches.value_of("INPUT") {
    //     info!("Value for input: {}", i);
    // }

    if matches
        .subcommand_matches(SubCommand::Entries.value())
        .is_some()
    {
        execute_subcommand_entries(&entries);
    }

    if matches
        .subcommand_matches(SubCommand::Summary.value())
        .is_some()
    {
        execute_subcommand_summary(&entries);
    }

    Ok(())
}

fn execute_subcommand_entries(entries: &[TimeEntry]) {
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

fn execute_subcommand_summary(entries: &[TimeEntry]) {
    let mut hashmap: BTreeMap<NaiveDate, i64> = BTreeMap::new();
    info!("{0: >10} | {1: >8} | {2: >8}", "Date", "Hours", "Minutes");
    entries.iter().for_each(|activity| {
        let existing_value = hashmap.get(&activity.date).cloned();
        hashmap.insert(
            activity.date,
            existing_value.unwrap_or(0) + activity.duration.num_minutes(),
        );
    });
    hashmap.iter().for_each(|(key, value)| {
        let hours = value / 60;
        let minutes = value - hours * 60;
        info!("{0: <10} | {1: >7}h | {2: >7}m", key, hours, minutes);
    });
}

async fn get_timeular_token(web_client: &reqwest::Client) -> Result<String, reqwest::Error> {
    let result = web_client
        .post(ApiUrls::Login.value())
        .json(&json!({
            "apiKey": env::var("TIMEULAR_API_KEY").unwrap_or("No api key provided".to_string()),
            "apiSecret": env::var("TIMEULAR_API_SECRET").unwrap_or("No api secret provided".to_string())
        }))
        .send()
        .await?;

    Ok(result.json::<TimeularLoginResponse>().await?.token)
}

async fn get_timeular_data(
    web_client: &reqwest::Client,
    token: String,
) -> Result<Vec<TimeEntry>, reqwest::Error> {
    let timeular_activities = web_client
        .get(ApiUrls::GetAllActivities.value())
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json::<TimeularActivitiesResponse>()
        .await?;

    info!(
        "{} activities fetched",
        timeular_activities.activities.len()
    );

    let timeular_entries = web_client
        .get(ApiUrls::GetAllEntries.value())
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json::<TimeularEntriesResponse>()
        .await?;

    info!("{} entries found", timeular_entries.time_entries.len());

    let map = timeular_entries
        .time_entries
        .iter()
        .map(|timeular_entry| {
            let activity = timeular_activities
                .activities
                .iter()
                .find(|activity| activity.id == timeular_entry.activity_id);

            let duration = timeular_entry
                .duration
                .stopped_at
                .signed_duration_since(timeular_entry.duration.started_at);

            TimeEntry {
                activity: match activity {
                    None => "❌".to_string(),
                    Some(value) => value.name.clone(),
                },
                duration,
                date: timeular_entry.duration.started_at.date(),
            }
        })
        .collect();
    Ok(map)
}
