use crate::{
    configuration::Settings,
    enums,
    models::{time, timeular},
};
use log::info;
use serde_json::json;

pub async fn get_timeular_token(
    configuration: &Settings,
    web_client: &reqwest::Client,
) -> Result<String, reqwest::Error> {
    let result = web_client
        .post(enums::url::Url::Login.value())
        .json(&json!({
            "apiKey": configuration.timeular_api_key,
            "apiSecret": configuration.timeular_api_secret
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
        .get(enums::url::Url::GetAllActivities.value())
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
    let string = enums::url::Url::GetAllEntries(start.to_string(), end.to_string()).value();
    info!("{}", string);
    let timeular_entries = web_client
        .get(string)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json::<timeular::EntriesResponse>()
        .await?;

    info!("{} entries found", timeular_entries.time_entries.len());

    let entries = timeular::convert_timeular_entries_to_time_entries(activities, timeular_entries);
    Ok(entries)
}
