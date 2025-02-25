/* Write a function that counts the number of
digits in a given positive integer using a loop
and without converting the number to a string.

_Example Input and Output_
input = 12345
output = 5
*/
pub fn count_digits(number: i32) -> i32 {
    let mut count = 0;
    let mut n = number;
    while n != 0 {
        count += 1;
        n /= 10; // use floor division
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        let number = 12345;
        let expected = 5;
        assert_eq!(count_digits(number), expected);
    }
}
