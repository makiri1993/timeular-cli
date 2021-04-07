use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TimeularLoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct TimeularActivitiesResponse {
    pub activities: Vec<TimeularActivity>,
    #[serde(rename(deserialize = "inactiveActivities"))]
    inactive_activities: Vec<TimeularActivity>,
    #[serde(rename(deserialize = "archivedActivities"))]
    archived_activities: Vec<TimeularActivity>,
}

#[derive(Debug, Deserialize)]
pub struct TimeularActivity {
    pub id: String,
    pub name: String,
    color: String,
    integration: String,
    #[serde(rename(deserialize = "spaceId"))]
    space_id: String,
    #[serde(rename(deserialize = "deviceSide"))]
    device_side: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct TimeularEntriesResponse {
    #[serde(rename(deserialize = "timeEntries"))]
    pub time_entries: Vec<TimeularEntry>,
}

#[derive(Debug, Deserialize)]
pub struct TimeularEntry {
    id: String,
    #[serde(rename(deserialize = "activityId"))]
    pub activity_id: String,
    pub duration: TimeularEntryDuration,
    note: TimeularEntryNote,
}

#[derive(Debug, Deserialize)]
pub struct TimeularEntryDuration {
    #[serde(rename(deserialize = "startedAt"))]
    pub started_at: NaiveDateTime,
    #[serde(rename(deserialize = "stoppedAt"))]
    pub stopped_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
struct TimeularEntryNote {
    text: Option<String>,
    tags: Vec<String>,
    mentions: Vec<TimeularEntryNoteMention>,
}

#[derive(Debug, Deserialize)]
struct TimeularEntryNoteMention {
    id: i32,
    key: String,
    label: String,
    scope: String,
    #[serde(rename(deserialize = "spaceId"))]
    space_id: String,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct TimeEntry {
    pub date: NaiveDate,
    pub activity: String,
    pub duration: Duration,
}
