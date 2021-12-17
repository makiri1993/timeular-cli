use crate::helper::input;

#[derive(Debug, PartialEq)]
pub enum Flag {
    Month(String),
    Decimal,
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
            let flag_key = flags.get(n);
            let flag_option = parse_flag(
                flag_key,
                if flag_key.is_some() && flag_key.unwrap() == &"-m".to_string() {
                    {
                        flags.get(n + 1)
                    }
                } else {
                    {
                        None
                    }
                },
            );

            if let Some(flag) = flag_option {
                flag_enums.push(flag)
            }

            n += 1;
        }

        flag_enums
    }
}

fn parse_flag(flag_key: Option<&String>, flag_value: Option<&String>) -> Option<Flag> {
    match flag_key.map(|s| &s[..]) {
        Some("-m") => Option::from(parse_month_flag(flag_value.expect("No month provided"))),
        Some("-d") => Option::from(Flag::Decimal),
        Some(val) if val.contains('-') => {
            panic!("unkown flag");
        }
        Some(_) => None,
        None => todo!("no flag"),
    }
}

fn parse_month_flag(value: &str) -> Flag {
    let is_valid_month = input::is_valid_month(value);
    match is_valid_month {
        true => (Flag::Month(value.to_owned())),
        false => panic!(
            "Only these values are valid months. \
                (jan, feb, mar, apr, may, jun, jul, aug, sep, okt, nov, dec)"
        ),
    }
}
