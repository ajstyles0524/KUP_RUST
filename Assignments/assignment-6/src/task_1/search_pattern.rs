/// The _find_pattern function is used to search pattern in given input string
///
/// #Arguments
///
/// input_string - Given input string  to Apply to find pattern
/// search_pattern - string to checked with input_string
///
/// #Return
///
/// Returns the String to given value with index and match or not
pub fn _find_pattern(input_string: String, search_pattern: String) -> String {
    let input_string_vec: Vec<char> = input_string.chars().collect();
    let search_pattern_vec: Vec<char> = search_pattern.chars().collect();
    let mut iteration_count = 0;
    let mut pattern_check;
    let mut temp_index;
    for index in 0..(input_string_vec.len() - search_pattern_vec.len() + 1) {
        temp_index = index;
        pattern_check = index;
        for element_match_index in &search_pattern_vec {
            if element_match_index == &input_string_vec[temp_index] {
                iteration_count += 1;
            }
            if iteration_count == search_pattern.len() {
                return pattern_check.to_string();
            }
            temp_index += 1;
        }
        iteration_count = 0;
    }
    "no match".to_string()
}
