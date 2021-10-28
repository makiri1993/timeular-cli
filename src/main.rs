use timeular_cli::{configuration, console, startup::run};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    log::set_logger(&console::CONSOLE_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let configuration: configuration::Settings = configuration::get_config();
    log::info!("Config: {:?}", configuration);

    run(&configuration).await?;

    Ok(())
}
