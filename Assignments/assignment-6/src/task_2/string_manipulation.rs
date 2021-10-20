/// The function _desired_output is used to give desired output and accepts three string as input
///
/// #Arguments
///
///string_1 - first input string
///string_2 - second input string
///string_3 - third input string
///
/// #Return
///
/// Returns the String to give the desired output....
pub fn _desired_output(string_1: &str, string_2: &str, string_3: &str) -> String {
    let mut position = 0;
    let mut iteration: usize = 0;
    let mut array: Vec<char> = Vec::new();
    while iteration < string_1.len() {
        if position % 2 == 0 {
            let compare_result_1 =
                if string_1.chars().nth(iteration) < string_2.chars().nth(iteration) {
                    string_1.chars().nth(iteration)
                } else {
                    string_2.chars().nth(iteration)
                };
            let compare_result_2 = if compare_result_1 < string_3.chars().nth(iteration) {
                compare_result_1
            } else {
                string_3.chars().nth(iteration)
            };
            array.push(compare_result_2.unwrap());
        } else {
            let compare_result_1 =
                if string_1.chars().nth(iteration) > string_2.chars().nth(iteration) {
                    string_1.chars().nth(iteration)
                } else {
                    string_2.chars().nth(iteration)
                };
            let compare_result_2 = if compare_result_1 > string_3.chars().nth(iteration) {
                compare_result_1
            } else {
                string_3.chars().nth(iteration)
            };
            array.push(compare_result_2.unwrap());
        }
        iteration += 1;
        position += 1
    }
    let result: String = array.iter().collect();
    result
}
