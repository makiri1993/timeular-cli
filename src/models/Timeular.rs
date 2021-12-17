use crate::models::time;
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ActivitiesResponse {
    pub activities: Vec<Activity>,
    // #[serde(rename(deserialize = "inactiveActivities"))]
    // inactive_activities: Vec<Activity>,
    // #[serde(rename(deserialize = "archivedActivities"))]
    // archived_activities: Vec<Activity>,
}

#[derive(Debug, Deserialize)]
pub struct Activity {
    pub id: String,
    pub name: String,
    // color: String,
    // integration: String,
    // #[serde(rename(deserialize = "spaceId"))]
    // space_id: String,
    // #[serde(rename(deserialize = "deviceSide"))]
    // device_side: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct EntriesResponse {
    #[serde(rename(deserialize = "timeEntries"))]
    pub time_entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    // id: String,
    #[serde(rename(deserialize = "activityId"))]
    pub activity_id: String,
    pub duration: EntryDuration,
    // note: EntryNote,
}

#[derive(Debug, Deserialize)]
pub struct EntryDuration {
    #[serde(rename(deserialize = "startedAt"))]
    pub started_at: NaiveDateTime,
    #[serde(rename(deserialize = "stoppedAt"))]
    pub stopped_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
struct EntryNote {
    // text: Option<String>,
// tags: Vec<String>,
// mentions: Vec<EntryNoteMention>,
}

#[derive(Debug, Deserialize)]
struct EntryNoteMention {
    // id: i32,
// key: String,
// label: String,
// scope: String,
// #[serde(rename(deserialize = "spaceId"))]
// space_id: String,
}

pub trait Convert {
    fn convert_timeular_entry_to_time_entry(&self, activities: &[Activity]) -> time::Entry;
}
pub trait ConvertForResponse {
    fn convert_timeular_entries_to_time_entries(&self, activities: &[Activity])
        -> Vec<time::Entry>;
}

impl Convert for Entry {
    fn convert_timeular_entry_to_time_entry(&self, activities: &[Activity]) -> time::Entry {
        let activity = activities
            .iter()
            .find(|activity| activity.id == self.activity_id);

        let duration = self
            .duration
            .stopped_at
            .signed_duration_since(self.duration.started_at);

        time::Entry {
            activity: match activity {
                None => "❌".to_string(),
                Some(value) => value.name.clone(),
            },
            duration,
            date: self.duration.started_at.date(),
        }
    }
}

impl ConvertForResponse for EntriesResponse {
    fn convert_timeular_entries_to_time_entries(
        &self,
        activities: &[Activity],
    ) -> Vec<time::Entry> {
        self.time_entries
            .iter()
            .map(|timeular_entry| timeular_entry.convert_timeular_entry_to_time_entry(activities))
            .collect()
    }
}
