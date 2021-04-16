pub struct Flag<'a> {
    pub short: char,
    pub long: &'a str,
    pub about: &'a str,
    pub takes_value: bool,
}
