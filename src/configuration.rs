const CONFIG_NAME: &str = "timeular-cli";
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub timeular_api_key: String,
    pub timeular_api_secret: String,
}

impl ::std::default::Default for Settings {
    fn default() -> Self {
        Self {
            timeular_api_key: "".into(),
            timeular_api_secret: "".into(),
        }
    }
}

pub fn get_config() -> Settings {
    let configuration: Settings = confy::load(CONFIG_NAME).expect("Failed to read configuration.");

    if configuration.timeular_api_key.is_empty() || configuration.timeular_api_secret.is_empty() {
        let mut timeular_api_key = String::new();
        let mut timeular_api_secret = String::new();
        println!("No api key and secret provided.");
        println!("Please enter the api key");
        std::io::stdin()
            .read_line(&mut timeular_api_key)
            .expect("Failed to read api key");

        println!("Please enter the api secret");
        std::io::stdin()
            .read_line(&mut timeular_api_secret)
            .expect("Failed to read api key");

        let new_configuration = Settings {
            timeular_api_key: timeular_api_key.trim_end().into(),
            timeular_api_secret: timeular_api_secret.trim_end().into(),
        };
        confy::store(CONFIG_NAME, &new_configuration).expect("error storing api data.");

        return new_configuration;
    }

    configuration
}
