#![allow(clippy::redundant_closure)]
pub fn is_palindrome(x: i32) -> bool {
    // convert input to string and split by chars
    let x_chars: Vec<char> = x.to_string().chars().collect();
    println!("{:?}", x_chars);
    for (i, _v) in x_chars.iter().enumerate() {
        println!("{} vs {}", x_chars[i], x_chars[x_chars.len() - 1 - i]);
        if x_chars[i] != x_chars[(x_chars.len() - 1) - i] {
            return false;
        }
    }
    true
}

pub fn is_palindrome_v2(x: i32) -> bool {
    // If x is less than 0 or x is a negative number, it is not a palindrome
    if x < 0 {
        return false;
    }

    // Create a copy of variable x
    let mut x_copy = x;

    // Find the appropriate divisor
    // to extract the leading digit
    // i.e. keep increasing divisor by factor of 10
    // until x / divisor < 10
    let mut divisor = 1;
    while x / divisor >= 10 {
        divisor *= 10;
    }

    while x_copy != 0 {
        // Extract the leading digit
        let leading = x_copy / divisor;
        // Use modulo operation to get the trailing digit
        // e.g. if the number is 1231, the modulo would be 1
        // which is the trailing number
        let trailing = x_copy % 10;
        if leading != trailing {
            return false;
        }
        // Remove the leading and trailing digit from
        // the original number
        // e.g if the number is 1231, then the divisor would be 1000
        // the modulo would be 231, and then if we divide the modulo
        // by 10, we will end up with 23
        x_copy = (x_copy % divisor) / 10;

        // Reducing divisor by factor of 2
        // because we removed 2 digits from the
        // original number
        divisor /= 100;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let inputs: Vec<i32> = vec![121, 10, -121, -10];
        let expected: Vec<bool> = vec![true, false, false, false];
        let res: Vec<bool> = inputs.into_iter().map(|i| is_palindrome(i)).collect();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_is_palindrome_v2() {
        let inputs: Vec<i32> = vec![121, 10, -121, -10];
        let expected: Vec<bool> = vec![true, false, false, false];
        let res: Vec<bool> = inputs.into_iter().map(|i| is_palindrome_v2(i)).collect();
        assert_eq!(res, expected);
    }
}
