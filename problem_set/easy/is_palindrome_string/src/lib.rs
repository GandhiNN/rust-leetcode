/* Write a function to check whether a given
string input is a palindrome regardless of case */
pub fn is_palindrome(str_input: &str) -> bool {
    let strlen = str_input.len();
    for (i, v) in str_input.to_lowercase().chars().enumerate() {
        if v == str_input.to_lowercase().chars().nth(strlen-i-1).unwrap() {
            continue
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_1() {
        let strin = "ABBA";
        let expected = true;
        assert_eq!(is_palindrome(strin), expected);
    }

    #[test]
    fn test_is_palindrome_2() {
        let strin = "RaceCar";
        let expected = true;
        assert_eq!(is_palindrome(strin), expected);
    }

    #[test]
    fn test_is_palindrome_3() {
        let strin = "not a palindrome";
        let expected = false;
        assert_eq!(is_palindrome(strin), expected);
    }
}
