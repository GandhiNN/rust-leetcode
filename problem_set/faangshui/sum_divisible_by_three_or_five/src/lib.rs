/* Write a function that calculates the sum of all
numbers divisible by either 3 or 5 up to and including
n

Example Input:
n = 10

Example Output:
33

Explanation:
The numbers divisible by 3 or 5 up to 10 are:
3, 5, 6, and 9
Their sum is 23 + 10 (including n) = 33

*/
pub fn sum_divisible_by_three_or_five(n: &i32) -> i32 {
    let mut sum = 0;
    for i in 1..=*n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_divisible_by_three_or_five_v1() {
        let n = 10;
        let expected = 33;
        assert_eq!(sum_divisible_by_three_or_five(&n), expected);
    }
}
