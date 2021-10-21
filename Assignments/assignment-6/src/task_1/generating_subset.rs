/// The generating_substring function is used to find all substring of input string
///
/// #Arguments
///
/// input_string - Taking String as input and generating substring of input string
///
/// #Return
///
/// Returns a vector<Sting> having all substring of input string
pub fn generating_substring(input_string: String) -> Vec<String> {
    if input_string.is_empty() {
        return vec!["".to_string()];
    }
    let mut all_substring: Vec<String> = Vec::new();
    let mut substring: &str;
    for index_1 in 0..input_string.len() {
        for index_2 in index_1..input_string.len() {
            substring = &input_string[index_1..(index_2 + 1)];
            all_substring.push(substring.to_string());
        }
    }
    all_substring
}
