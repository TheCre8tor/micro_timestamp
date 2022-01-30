/* The #[cfg(test)] annotation on the tests module
tells Rust to compile and run the test code only
when you run cargo test , not when you run cargo
build */

#[cfg(test)]
mod test {
    use claim::{assert_err, assert_ok};
    use micro_timestamp::domain::DateParamName;

    #[test]
    fn should_return_success_if_date_param_name_is_valid() {
        let valid_date = "2015-7-6".to_string();
        let result = DateParamName::parse(&valid_date);
        assert_ok!(result);
    }

    #[test]
    fn should_return_failure_if_date_param_name_is_invalid() {
        let valid_date = "2015-7-".to_string();
        let result = DateParamName::parse(&valid_date);
        assert_err!(result);
    }

    #[test]
    fn should_return_success_if_params_contain_valid_unix_code() {
        let valid_unix_code = "790819200".to_string();
        let result = DateParamName::parse(&valid_unix_code);
        assert_ok!(result);
    }

    #[test]
    fn should_return_out_of_range_error_if_params_contain_an_invalid_unix_code() {
        let valid_unix_code = "79081920055667744".to_string();
        let result = DateParamName::parse(&valid_unix_code);
        assert_err!(result);
    }

    #[test]
    fn should_return_invalid_timestamp_error_if_params_contain_alphabets() {
        let timestamp_with_alphabets = "850003200UIoP".to_string();
        let result = DateParamName::parse(&timestamp_with_alphabets);
        assert_err!(result);
    }
}
