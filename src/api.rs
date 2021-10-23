use crate::{
    enums,
    models::{
        time,
        timeular::{self, ConvertForResponse},
    },
};
use log::info;
use serde_json::json;

pub struct TimeularService {
    pub client: reqwest::Client,
    pub token: String,
    pub activities: Vec<timeular::Activity>,
}

impl TimeularService {
    pub async fn new(
        client: reqwest::Client,
        api_key: &str,
        api_secret: &str,
    ) -> Result<TimeularService, reqwest::Error> {
        let result = client
            .post(enums::url::Url::Login.value())
            .json(&json!({
                "apiKey": api_key,
                "apiSecret": api_secret
            }))
            .send()
            .await?;

        let token = result.json::<timeular::LoginResponse>().await?.token;

        let timeular_activities = client
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

        Ok(TimeularService {
            client,
            token,
            activities: timeular_activities.activities,
        })
    }

    pub async fn get_timeular_entries(
        &self,
        start: &str,
        end: &str,
    ) -> Result<Vec<time::Entry>, reqwest::Error> {
        let string = enums::url::Url::GetAllEntries(start.to_string(), end.to_string()).value();
        info!("{}", string);
        let timeular_entries = self
            .client
            .get(string)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .json::<timeular::EntriesResponse>()
            .await?;

        info!("{} entries found", timeular_entries.time_entries.len());

        let entries = timeular_entries.convert_timeular_entries_to_time_entries(&self.activities);
        Ok(entries)
    }
}
