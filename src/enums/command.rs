pub enum Command {
    Summary,
    Entries,
}

pub trait ExtractCommand {
    fn extract_command(&self) -> Command;
}

impl ExtractCommand for [String] {
    fn extract_command(&self) -> Command {
        let command = self.get(1);
        match command.as_deref().map(|s| &s[..]) {
            Some("summary") => Command::Summary,
            Some("entries") => Command::Entries,
            Some(val) => panic!("This command '{}' is not supported.", val),
            None => panic!("No command provided."),
        }
    }
}
