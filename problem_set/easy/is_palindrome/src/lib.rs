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
}
