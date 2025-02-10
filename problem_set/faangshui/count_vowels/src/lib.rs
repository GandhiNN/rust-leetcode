/* Write a function that counts the number of vowels
in a string (a, e, i, o, u) - case-insensitive */
pub fn count_vowels(word: String) -> i32 {
    let mut num_vowels = 0;
    for char in word.to_ascii_lowercase().chars() {
        if let 'a' | 'i' | 'u' | 'e' | 'o' = char {
            num_vowels += 1;
        }
    }
    num_vowels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels_in_a_sentence() {
        let sentence = String::from("Who are you?");
        let expected = 5;
        let result = count_vowels(sentence);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_vowels_in_a_word() {
        let word = String::from("ASDsdaf!@3%sdsga#5shdsauyetow");
        let expected = 7;
        let result = count_vowels(word);
        assert_eq!(result, expected);
    }
}
