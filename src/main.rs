use timeular_cli::{console, startup::run};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    log::set_logger(&console::CONSOLE_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    run().await?;

    // println!("{:?}", &args);

    // let query = &args[1];
    // let filename = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    // let matches = setup_cli();

    // if matches.subcommand_matches(ENTRIES_COMMAND).is_some() {
    //     subcommand_entries(&matches, &client, &timeular_token, &activities).await?;
    // }

    // if let Some(matches) = matches.subcommand_matches(SUMMARY_COMMAND) {
    //     subcommand_summary(matches, &client, &timeular_token, &activities).await?;
    // }

    Ok(())
}
