#[cfg(test)]
mod tests {
    use crate::task_1::generating_subset::generating_substring;
    use crate::task_1::search_pattern::find_pattern;
    use crate::task_2::string_manipulation::desired_output;

    #[test]
    /// Positive test case for generating_substring
    fn generating_substring_test_1() {
        assert_eq!(
            generating_substring("Sony".to_string()),
            ["S", "So", "Son", "Sony", "o", "on", "ony", "n", "ny", "y"]
        );
    }
    /// Negative test case for generating_substring
    #[test]
    fn generating_substring_test_2() {
        assert_ne!(
            generating_substring("One".to_string()),
            ["O", "On", "One", "n"]
        );
    }
    /// Positive test case for find_pattern
    #[test]
    fn find_pattern_test_1() {
        assert_eq!(
            find_pattern("Maruti".to_string(), "ruti".to_string()),
            "2".to_string()
        );
    }
    /// negative  test case for find_pattern
    #[test]
    fn find_pattern_test_2() {
        assert_ne!(
            find_pattern("Knolway".to_string(), "Dell".to_string()),
            "2".to_string()
        );
    }
    /// Positive test case for desired output
    #[test]
    fn check_string_test_1() {
        assert_eq!(desired_output("jjdhid", "ikjhjk", "rtysgi"), "itdsgk");
    }
    /// Negative test case for desired output
    #[test]
    fn check_string_test_2() {
        assert_ne!(desired_output("abcd", "efgh", "ijkl"), "avgl");
    }
}
