pub enum Urls {
    Login,
    GetAllActivities,
    GetAllEntries(String, String),
}

impl Urls {
    pub fn value(&self) -> String {
        match &*self {
            Urls::GetAllActivities => "https://api.timeular.com/api/v3/activities".to_string(),
            Urls::Login => "https://api.timeular.com/api/v3/developer/sign-in".to_string(),
            Urls::GetAllEntries(start, end) => {
                format!(
                    "https://api.timeular.com/api/v3/time-entries\
                        /2021-{}T00:00:00.000/2021-{}T23:59:59.999",
                    start, end
                )
            }
        }
    }
}
