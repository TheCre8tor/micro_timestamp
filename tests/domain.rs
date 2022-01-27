/* The #[cfg(test)] annotation on the tests module
   tells Rust to compile and run the test code only
   when you run cargo test , not when you run cargo
   build */

#[cfg(test)]
mod test {
    use claim::{assert_ok, assert_err};
    use micro_timestamp::domain::DateParamName;

    #[test]
    fn check_if_value_contain_valid_date() {
        let valid_date = "2015--02".to_string();
        let result = DateParamName::parse(&valid_date);
        assert_ok!(result);
    }
}