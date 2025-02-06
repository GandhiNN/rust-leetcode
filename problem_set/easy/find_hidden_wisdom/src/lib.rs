/* Write a function that, given a sentence
with single spaces between words an no punctuation
finds the longest word.

Example input:
proverb = "Balance and precision lead to mastery"

Example output:
"precision" -> is the longest word
*/
pub fn find_hidden_wisdom(proverb: &str) -> String {
    let mut longest = "";
    for word in proverb.split_whitespace() {
        if word.len() > longest.len() {
            longest = word
        }
    }
    longest.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_longest_word_from_a_sentence() {
        let proverb = "Balance and precision lead to mastery";
        let expected = "precision";
        assert_eq!(find_hidden_wisdom(proverb), expected);
    }

    #[test]
    fn test_get_longest_word_from_a_word() {
        let word = "a";
        let expected = "a";
        assert_eq!(find_hidden_wisdom(word), expected);
    }
}
