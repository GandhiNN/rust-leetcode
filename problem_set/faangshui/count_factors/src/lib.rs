/* Write a function that counts the number of
positive integer factors of a given number.

_Example Input and Output_
input = 12
output = 6
explanation = factors of 12 are 1, 2, 3, 4, 6, 12
*/
pub fn count_factors(number: i32) -> i32 {
    let mut factors: Vec<i32> = vec![];
    // 0 is not a factor of any number because
    // if we divide a number by 0, the value is not defined
    if number == 0 {
        return 0;
    }
    // thus we don't use 0 in the iteration as well
    // we use number + 1 to also include the number itself
    for i in 1..number + 1 {
        if number % i == 0 {
            factors.push(i)
        }
    }
    factors.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_factors() {
        let candles = 12;
        let expected = 6;
        assert_eq!(count_factors(candles), expected);
    }
}
