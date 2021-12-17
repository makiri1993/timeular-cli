use chrono::{Duration, NaiveDate};
use std::collections::BTreeMap;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Entry {
    pub date: NaiveDate,
    pub activity: String,
    pub duration: Duration,
}

pub fn summarize_entries_in_tree(entries: Vec<Entry>) -> BTreeMap<NaiveDate, i64> {
    let mut tree_map: BTreeMap<NaiveDate, i64> = BTreeMap::new();
    entries.iter().for_each(|entry| {
        *tree_map.entry(entry.date).or_insert(0) += entry.duration.num_seconds();
    });
    tree_map
}
