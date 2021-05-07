pub struct Flag<'a> {
    pub short: char,
    pub long: &'a str,
    pub about: &'a str,
    pub takes_value: bool,
}

pub enum Flags {
    Month,
}

impl Flags {
    pub fn value(&self) -> Flag {
        match &self {
            Flags::Month => Flag {
                short: 'm',
                long: "month",
                about: "set the month for the entries",
                takes_value: true,
            },
        }
    }
}
