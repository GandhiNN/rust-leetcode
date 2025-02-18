/* Write a Sieve of Eratosthenes function to check
the primality of multiple numbers within a limited range.

The sieve works by creating a list of numbers up to a
desired limit N and systematically eliminating the multiples
of each prime number starting from 2.

The numbers that remain unmarked at the end of the process
are prime.
 */
pub fn sieve_of_eratosthenes(n: i32) -> Vec<i32> {
    // Return early when n is less than 2
    if n < 2 {
        return vec![];
    }
    // Create a boolean array with capacity of (n + 1)
    // i.e. [0...10] -> we have to include 0 as well
    // and initialize all entries there as true.
    let mut is_prime = vec![true; (n + 1) as usize];
    // We know that 0 and 1 is not a prime, so mark them as false
    is_prime[0] = false;
    is_prime[1] = false;
    // Start from number 2 i.e. sieve[2] and iterate until i^2 > n
    for i in 2..=((n as f64).sqrt() as i32) {
        // If it's a prime number
        if is_prime[i as usize] {
            // ... then composite numbers using i are not prime numbers
            for j in (i * i..=n).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }
    // Collect and return all indices marked as true
    // which is prime numbers
    let mut res: Vec<i32> = vec![];
    for (idx, val) in is_prime.iter().enumerate() {
        if *val {
            res.push(idx as i32);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_of_eratosthenes_1() {
        let input = 10;
        let output = vec![2, 3, 5, 7];
        assert_eq!(sieve_of_eratosthenes(input), output);
    }

    #[test]
    fn test_sieve_of_eratosthenes_2() {
        let input = 20;
        let output = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(sieve_of_eratosthenes(input), output);
    }

    #[test]
    fn test_sieve_of_eratosthenes_3() {
        let input = 30;
        let output = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(sieve_of_eratosthenes(input), output);
    }
}
