pub fn is_valid_month(month: &str) -> Result<(), String> {
    match month {
        "jan" | "feb" | "mar" | "may" | "apr" | "jun" | "jul" | "aug" | "sep" | "okt" | "nov"
        | "dec" => Ok(()),
        &_ => Err("Only these values are valid months. \
        (jan, feb, mar, apr, may, jun, jul, aug, sep, okt, nov, dec)"
            .to_string()),
    }
}

pub fn convert_input_month_to_date_strings(month: &str) -> (&str, &str) {
    match month {
        "jan" => ("01-01", "01-31"),
        "feb" => ("02-01", "02-28"),
        "mar" => ("03-01", "03-31"),
        "apr" => ("04-01", "04-30"),
        "may" => ("05-01", "05-31"),
        "jun" => ("06-01", "06-30"),
        "jul" => ("07-01", "07-31"),
        "aug" => ("08-01", "08-31"),
        "sep" => ("09-01", "09-30"),
        "okt" => ("10-01", "10-31"),
        "nov" => ("11-01", "11-30"),
        "dec" => ("12-01", "12-31"),
        &_ => ("", ""),
    }
}
