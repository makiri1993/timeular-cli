pub enum ApiUrls {
    Login,
    GetAllActivities,
    GetAllEntries(String, String),
}

impl ApiUrls {
    pub fn value(&self) -> String {
        match &*self {
            ApiUrls::GetAllActivities => "https://api.timeular.com/api/v3/activities".to_string(),
            ApiUrls::Login => "https://api.timeular.com/api/v3/developer/sign-in".to_string(),
            ApiUrls::GetAllEntries(start, end) => {
                format!(
                    "https://api.timeular.com/api/v3/time-entries\
                        /2021-{}T00:00:00.000/2021-{}T23:59:59.999",
                    start, end
                )
            }
        }
    }
}

pub struct Flag<'a> {
    pub short: char,
    pub long: &'a str,
    pub about: &'a str,
    pub takes_value: bool,
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

    pub fn flag(&self) -> Flag {
        match *self {
            SubCommand::Summary => Flag {
                short: 'm',
                long: "month",
                about: "set the month for the entries",
                takes_value: true,
            },
            _ => Flag {
                short: '0',
                long: "",
                about: "",
                takes_value: false,
            },
        }
    }
}

pub fn is_valid_month(month: &str) -> Result<(), String> {
    match month {
        "jan" | "feb" | "mar" | "may" | "apr" | "jun" | "jul" | "aug" | "sep" | "okt" | "nov"
        | "dec" => Ok(()),
        &_ => Err("Only these values are valid months. \
        (jan, feb, mar, apr, may, jun, jul, aug, sep, okt, nov, dec)"
            .to_string()),
    }
}
