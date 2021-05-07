pub enum SubCommands {
    Entries,
    Summary,
}

pub struct SubCommand<'a> {
    pub command: &'a str,
    pub description: &'a str,
}

impl SubCommands {
    pub fn value(&self) -> SubCommand {
        match *self {
            SubCommands::Entries => SubCommand {
                command: "entries",
                description: "Shows all entries from the a period of time.",
            },
            SubCommands::Summary => SubCommand {
                command: "summary",
                description: "Summarizes the entries from a period of time.",
            },
        }
    }
}
