use crate::models::flag::Flag;

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
