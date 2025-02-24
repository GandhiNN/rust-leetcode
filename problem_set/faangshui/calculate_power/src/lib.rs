/* Write a function that calculates the power of
a number X raised to the power of N using a loop.

_Example Input and Output_
base = 3
exponents = 2
output = 9
explanation -> base^n -> 3^2 = 9
*/
pub fn calculate_power(base: i32, exponent: i32) -> i32 {
    let mut res = base;
    if exponent > 0 {
        for _ in 1..exponent {
            res *= base
        }
    } else if exponent == 0 {
        return 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_power_1() {
        let base = 3;
        let exponents = 2;
        let expected = 9;
        assert_eq!(calculate_power(base, exponents), expected);
    }

    #[test]
    fn test_calculate_power_2() {
        let base = 10;
        let exponents = 3;
        let expected = 1000;
        assert_eq!(calculate_power(base, exponents), expected);
    }

    #[test]
    fn test_calculate_power_3() {
        let base = 9;
        let exponents = 0;
        let expected = 1;
        assert_eq!(calculate_power(base, exponents), expected);
    }
}
