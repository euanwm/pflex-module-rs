#[cfg(test)]
mod error_checking {
    use crate::error_codes::ResponseCodes;

    #[test]
    fn check_success() {
        let code = "0".to_string();
        let result = ResponseCodes::check_code(code);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn check_warning() {
        let code = "1".to_string();
        let result = ResponseCodes::check_code(code);
        assert_eq!(result.is_ok(), true);
        println!("{}", result.unwrap())
    }

    #[test]
    fn check_failure() {
        let code = "-1046".to_string();
        let result = ResponseCodes::check_code(code);
        assert_eq!(result.is_err(), true);
        println!("{}", result.unwrap_err())
    }

    #[test]
    fn check_catch_all() {
        let code = "-6942069".to_string();
        let result = ResponseCodes::check_code(code);
        if result.is_err() {
            assert_eq!(result.unwrap_err(), "-6942069");
        }
    }

    #[test]
    fn print_success_details() {
        println!("Success Description: {}", ResponseCodes::Success);
        println!("Success Code: {}", ResponseCodes::Success.value());
    }
}
