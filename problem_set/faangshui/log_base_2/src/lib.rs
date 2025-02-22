/* Write a function to calculate how many times a
given number can be divided by 2 until it becomes
less than or equal to 1. In other words, the function
calculates the integer part of the logarithm base 2
of a given positive integer by repeatedly dividing the number
by 2.

_Example input_
number = 16
output = 4
*/
pub fn log_base_2(number: i32) -> i32 {
    let mut input = number;
    let mut count = 0;
    loop {
        if input / 2 >= 1 {
            input /= 2;
            count += 1;
            continue;
        } else {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_base_2_16() {
        let number = 16;
        let expected = 4;
        assert_eq!(log_base_2(number), expected);
    }

    #[test]
    fn test_log_base_2_9() {
        let number = 9;
        let expected = 3;
        assert_eq!(log_base_2(number), expected);
    }

    #[test]
    fn test_log_base_2_31() {
        let number = 31;
        let expected = 4;
        assert_eq!(log_base_2(number), expected);
    }
}
