/* Write a function that takes a string and
returns it reversed, without using built-in
reverse functions

Example input:
text = "temple"

Example output:
"elpmet"
*/
pub fn reverse_string(text: &str) -> String {
    let mut tmp_vec: Vec<char> = Vec::with_capacity(text.len());
    // iterate over the original string and push each character
    // to the first index of the temporary vector
    for c in text.chars() {
        tmp_vec.insert(0, c);
    }
    tmp_vec.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string_1() {
        let input_text = "temple";
        let expected = String::from("elpmet");
        assert_eq!(reverse_string(input_text), expected);
    }

    #[test]
    fn test_reverse_string_2() {
        let input_text = "gandhi";
        let expected = String::from("ihdnag");
        assert_eq!(reverse_string(input_text), expected);
    }
}
