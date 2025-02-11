/* Write a function to find the longest common prefix
string amongst an array of strings.

If there is no common prefix, return an empty string ""

Example 1:
    Input: strs = ["flower","flow","flight"]
    Output: "fl"

Example 2:
    Input: strs = ["dog","racecar","car"]
    Output: ""
    Explanation: There is no common prefix among the input strings.
*/
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // Clone input vector and
    // sort it by length of its elements
    let mut strss = strs.clone();
    strss.sort_by_key(|a| a.len());
    // Get the shortest string
    let mut prefix = strss.first().unwrap().to_owned();
    for _i in 0..prefix.len() {
        for w in strs.iter() {
            if w.starts_with(&prefix) {
                continue;
            } else {
                prefix.pop();
                break;
            }
        }
    }
    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs: Vec<String> = vec!["flower".to_owned(), "flow".to_owned(), "flight".to_owned()];
        let expected = "fl";
        assert_eq!(longest_common_prefix(strs), expected);

        let strs2 = vec!["dog".to_owned(), "racecar".to_owned(), "car".to_owned()];
        let expected2 = "";
        assert_eq!(longest_common_prefix(strs2), expected2);
    }
}
