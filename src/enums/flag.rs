use crate::helper::input;

#[derive(Debug, PartialEq)]
pub enum Flag {
    Month(String),
}

pub trait ExtractFlags {
    fn extract_flags(&self) -> Vec<Flag>;
}

impl ExtractFlags for [String] {
    fn extract_flags(&self) -> Vec<Flag> {
        let flags = &self[2..];
        let mut flag_enums: Vec<Flag> = Vec::new();
        let mut n = 0;

        while n < flags.len() {
            log::info!("{:?}", flags.get(n));
            let flag_key = flags.get(n);
            let flag_value = flags.get(n + 1);

            let flag = parse_flag(flag_key, flag_value);

            flag_enums.push(flag);

            n += 2;
        }

        flag_enums
    }
}

fn parse_flag(flag_key: Option<&String>, flag_value: Option<&String>) -> Flag {
    match flag_key.as_deref().map(|s| &s[..]) {
        Some("-m") => validate_month_flag(flag_value.expect("No month provided")),
        Some(_) => todo!(),
        None => todo!(),
    }
}

fn validate_month_flag(value: &str) -> Flag {
    let is_valid_month = input::is_valid_month(value);
    match is_valid_month {
        true => (Flag::Month(value.to_owned())),
        false => panic!(
            "Only these values are valid months. \
                (jan, feb, mar, apr, may, jun, jul, aug, sep, okt, nov, dec)"
        ),
    }
}
