#[cfg(test)]
#[test]
fn _generating_substring_here() {
    use crate::task_1::generating_subset::_generating_substring;
    assert_eq!(
        _generating_substring("Sony".to_string()),
        ["S", "So", "Son", "Sony", "o", "on", "ony", "n", "ny", "y"]
    );
    assert_eq!(
        _generating_substring("One".to_string()),
        ["O", "On", "One", "n", "ne", "e"]
    );
}

#[test]
fn _find_pattern_test() {
    use crate::task_1::search_pattern::_find_pattern;
    assert_eq!(
        _find_pattern("Maruti".to_string(), "ruti".to_string()),
        "2".to_string()
    );
    assert_eq!(
        _find_pattern("Knolway".to_string(), "Dell".to_string()),
        "no match".to_string()
    );
}
#[test]
fn check_string() {
    use crate::task_2::string_manipulation::_desired_output;

    assert_eq!(_desired_output("jjdhid", "ikjhjk", "rtysgi"), "itdsgk");

    assert_eq!(_desired_output("abcd", "efgh", "ijkl"), "ajcl");
}
