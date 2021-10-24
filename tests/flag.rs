#[cfg(test)]
mod tests {
    use timeular_cli::enums::flag::{ExtractFlags, Flag};

    #[test]
    fn test_month_flag_parsing() {
        let flags = vec![
            "".to_string(),
            "".to_string(),
            "-m".to_string(),
            "feb".to_string(),
        ];

        assert_eq!(flags.extract_flags(), [Flag::Month("feb".to_string())]);
    }

    #[test]
    fn test_month_with_decimal_flag_parsing() {
        let flags = vec![
            "".to_string(),
            "".to_string(),
            "-m".to_string(),
            "feb".to_string(),
            "-d".to_string(),
        ];

        assert_eq!(
            flags.extract_flags(),
            [Flag::Month("feb".to_string()), Flag::Decimal]
        );
    }

    #[test]
    fn test_month_with_decimal_flag_recerse_order_parsing() {
        let flags = vec![
            "".to_string(),
            "".to_string(),
            "-d".to_string(),
            "-m".to_string(),
            "feb".to_string(),
        ];

        assert_eq!(
            flags.extract_flags(),
            [Flag::Decimal, Flag::Month("feb".to_string())]
        );
    }
}
