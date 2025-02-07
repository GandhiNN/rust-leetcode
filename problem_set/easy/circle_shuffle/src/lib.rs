/* Write a function to calaculate the total number
of ways N apprentices can arrange themselves in a circle */
pub fn circle_shuffle(n: i32) -> i32 {
    if n == 1 { 1 } else { n * circle_shuffle(n - 1) }
}

pub fn circle_shuffle_iterator(n: i32) -> i32 {
    let mut res = 1; // there must be at least 1 apprentice
    for i in 1..n + 1 {
        // not inclusive
        res *= i;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_shuffle_factorial() {
        let n1 = 3;
        let expected1 = 6;
        assert_eq!(circle_shuffle(n1), expected1);

        let n2 = 5;
        let expected2 = 120;
        assert_eq!(circle_shuffle(n2), expected2);
    }

    #[test]
    fn test_circle_shuffle_iterator() {
        let n1 = 3;
        let expected1 = 6;
        assert_eq!(circle_shuffle_iterator(n1), expected1);

        let n2 = 5;
        let expected2 = 120;
        assert_eq!(circle_shuffle(n2), expected2);
    }
}
