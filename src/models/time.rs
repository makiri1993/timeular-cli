use chrono::{Duration, NaiveDate};
use std::collections::BTreeMap;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Entry {
    pub date: NaiveDate,
    pub activity: String,
    pub duration: Duration,
}

pub trait Summarize {
    fn summarize_entries_in_tree(&self) -> BTreeMap<NaiveDate, i64>;
}

impl Summarize for Vec<Entry> {
    fn summarize_entries_in_tree(&self) -> BTreeMap<NaiveDate, i64> {
        let mut tree_map: BTreeMap<NaiveDate, i64> = BTreeMap::new();
        self.iter().for_each(|activity| {
            let existing_value = tree_map.get(&activity.date).cloned();
            tree_map.insert(
                activity.date,
                existing_value.unwrap_or(0) + activity.duration.num_minutes(),
            );
        });
        tree_map
    }
}
