pub enum Url {
    Login,
    GetAllActivities,
    GetAllEntries(String, String),
}

impl Url {
    pub fn value(&self) -> String {
        match &*self {
            Url::GetAllActivities => "https://api.timeular.com/api/v3/activities".to_string(),
            Url::Login => "https://api.timeular.com/api/v3/developer/sign-in".to_string(),
            Url::GetAllEntries(start, end) => {
                format!(
                    "https://api.timeular.com/api/v3/time-entries\
                        /2021-{}T00:00:00.000/2021-{}T23:59:59.999",
                    start, end
                )
            }
        }
    }
}
