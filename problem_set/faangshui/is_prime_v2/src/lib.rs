pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
/* Write a function that efficiently determines
whether a given number N is a prime number.

Note: use the fact that to check whether a number
N is prime, it's sufficient to test for divisibility
using numbers from 2 up to the square root of N
*/
pub fn is_prime(number: i32) -> &'static str {
    if number < 2 {
        return "Not prime!";
    }
    for i in 2..number {
        if number % i == 0 {
            return "Not prime!";
        }
        if i * i > number {
            // early return
            return "Prime!";
        }
    }
    "Prime!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_1() {
        let number = 7;
        let expected = "Prime!";
        assert_eq!(is_prime(number), expected);
    }

    #[test]
    fn test_is_prime_2() {
        let number = 6;
        let expected = "Not prime!";
        assert_eq!(is_prime(number), expected);
    }

    #[test]
    fn test_is_prime_3() {
        let number = 1;
        let expected = "Not prime!";
        assert_eq!(is_prime(number), expected);
    }

    #[test]
    fn test_is_prime_4() {
        let number = 11;
        let expected = "Prime!";
        assert_eq!(is_prime(number), expected);
    }

    #[test]
    fn test_is_prime_5() {
        let number = 9;
        let expected = "Not prime!";
        assert_eq!(is_prime(number), expected);
    }

    #[test]
    fn test_is_prime_6() {
        let number = 289;
        let expected = "Not prime!";
        assert_eq!(is_prime(number), expected);
    }
}
