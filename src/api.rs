use crate::enums;
use crate::models::{time, timeular};
use log::info;
use serde_json::json;
use std::env;

pub async fn get_timeular_token(web_client: &reqwest::Client) -> Result<String, reqwest::Error> {
    let result = web_client
        .post(enums::api::Urls::Login.value())
        .json(&json!({
            "apiKey": env::var("TIMEULAR_API_KEY").unwrap_or("No api key provided".to_string()),
            "apiSecret": env::var("TIMEULAR_API_SECRET").unwrap_or("No api secret provided".to_string())
        }))
        .send()
        .await?;

    Ok(result.json::<timeular::LoginResponse>().await?.token)
}

pub async fn get_timeular_activities(
    web_client: &reqwest::Client,
    token: &str,
) -> Result<Vec<timeular::Activity>, reqwest::Error> {
    let timeular_activities = web_client
        .get(enums::api::Urls::GetAllActivities.value())
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json::<timeular::ActivitiesResponse>()
        .await?;

    info!(
        "{} activities fetched",
        timeular_activities.activities.len()
    );

    Ok(timeular_activities.activities)
}

pub async fn get_timeular_entries(
    web_client: &reqwest::Client,
    token: &str,
    activities: &[timeular::Activity],
    start: &str,
    end: &str,
) -> Result<Vec<time::Entry>, reqwest::Error> {
    let string = enums::api::Urls::GetAllEntries(start.to_string(), end.to_string()).value();
    info!("{}", string);
    let timeular_entries = web_client
        .get(string)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json::<timeular::EntriesResponse>()
        .await?;

    info!("{} entries found", timeular_entries.time_entries.len());

    let entries = timeular::convert_timeular_entries_to_time_entries(&activities, timeular_entries);
    Ok(entries)
}
