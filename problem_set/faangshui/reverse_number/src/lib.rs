/* Write a function that reverses the digits of a given
positive integer using a loop and without converting
the number to a string.

_Example Input and Output_
input = 12345
output = 54321
*/
pub fn reverse_number(number: i32) -> i32 {
    if number == 0 {
        return number;
    }
    let mut reversed = 0;
    let mut n = number;
    let mut digits: Vec<i32> = vec![];
    while n != 0 {
        // modulo of 10 to get the last digits
        let digit = n % 10;
        digits.push(digit);
        // floor division with 10 to get the next sequence
        n /= 10;
    }
    let pow = digits.len() - 1;
    for i in 0..digits.len() {
        reversed += digits[i] * 10_i32.pow(pow as u32 - i as u32);
    }
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_number_1() {
        let number = 12345;
        let expected = 54321;
        assert_eq!(reverse_number(number), expected);
    }

    #[test]
    fn test_reverse_number_2() {
        let number = 10000;
        let expected = 1;
        assert_eq!(reverse_number(number), expected);
    }

    #[test]
    fn test_reverse_number_3() {
        let number = 0;
        let expected = 0;
        assert_eq!(reverse_number(number), expected);
    }
}
