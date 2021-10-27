use timeular_cli::{console, startup::run};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    log::set_logger(&console::CONSOLE_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    run().await?;

    Ok(())
}
