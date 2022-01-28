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
        let valid_unix_code = "790819200000".to_string();
        let result = DateParamName::parse(&valid_unix_code);
        assert_ok!(result);
    }
}
