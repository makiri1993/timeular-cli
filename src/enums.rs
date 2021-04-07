pub enum ApiUrls {
    Login,
    GetAllActivities,
    GetAllEntries,
}

impl ApiUrls {
    pub fn value(&self) -> &str {
        match *self {
            ApiUrls::GetAllActivities => "https://api.timeular.com/api/v3/activities",
            ApiUrls::Login => "https://api.timeular.com/api/v3/developer/sign-in",
            ApiUrls::GetAllEntries => "https://api.timeular.com/api/v3/time-entries/2021-03-01T00:00:00.000/2021-03-31T23:59:59.999"
        }
    }
}

pub enum SubCommand {
    Entries,
    Summary,
}

impl SubCommand {
    pub fn value(&self) -> &str {
        match *self {
            SubCommand::Entries => "entries",
            SubCommand::Summary => "summary",
        }
    }
}
