/* Given a string of characters, write a function
to find the length of the longest substring where
a character repeats consecutively.

Example input (1) -> chant = "aaabbbaaaccc"
Example output (1) = 3
Explanation: The longest stretch of consecutive characters
is "aaa" or "ccc" with a length of 3

Example input (2) -> chant = "ababab"
Example output (2) = 1
Explanation: Each syllable changes immediately, so the
longest stretch is 1
*/
pub fn longest_chant(chant: &str) -> i32 {
    // initiate counter of characters as 1
    let mut longest = 0;
    let mut counter = 1;
    let chant_length = chant.len();
    // set dummy value for current_char tracker variable
    let mut current_char: char = '-';
    for i in 0..chant_length {
        if current_char != chant.chars().nth(i).unwrap() {
            current_char = chant.chars().nth(i).unwrap();
            if counter > longest {
                longest = counter;
            }
            counter = 1; // reset value of counter
        } else {
            counter += 1;
        }
        // tail check
        if counter > longest {
            longest = counter;
        }
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_chant_1() {
        let input = "aaabbbaaaccc";
        let expected = 3;
        assert_eq!(longest_chant(input), expected);
    }

    #[test]
    fn test_longest_chant_2() {
        let input = "ababab";
        let expected = 1;
        assert_eq!(longest_chant(input), expected);
    }

    #[test]
    fn test_longest_chant_3() {
        let input = "aaaaabbbbcc";
        let expected = 5;
        assert_eq!(longest_chant(input), expected);
    }

    #[test]
    fn test_longest_chant_4() {
        let input = "a";
        let expected = 1;
        assert_eq!(longest_chant(input), expected);
    }
}
