/* Write a function that determines whether
a given number is a prime number. Return "Prime!"
if the number is prime and "Not prime!" otherwise

A prime number is numbers that are greater than 1
and divisible only by 1 and itself.*/
pub fn is_prime(number: i32) -> &'static str {
    if (number < 2) || (number % 2 == 0) || (number % 3 == 0) {
        return "Not prime!";
    }
    for i in 2..number {
        if number % i == 0 {
            return "Not prime!";
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
