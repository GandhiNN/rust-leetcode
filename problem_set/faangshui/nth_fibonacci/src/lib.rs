pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
/* Write a function that, given a non-negative integer n,
returns the nth Fibonacci number

Example input: n=5
Example output: 5 (the 5th fibonacci number [0, 1, 1, 2, 3, 5]) is 5
*/
pub fn nth_fibonacci(n: i32) -> i64 {
    let mut fib: Vec<i64> = vec![];
    for i in 0..n + 1 {
        if i == 0 {
            fib.push(0);
        } else if i == 1 {
            fib.push(i as i64);
        } else {
            fib.push(fib[i as usize - 1] + fib[i as usize - 2]);
        }
    }
    fib[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_fibonacci() {
        let n = 5;
        let expected = 5;
        assert_eq!(nth_fibonacci(n), expected);
    }
}
