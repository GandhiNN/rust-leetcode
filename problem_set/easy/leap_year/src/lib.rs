pub fn is_leap_year(n: i32) -> String {
    if n % 400 == 0 {
        String::from("Leap Year")
    } else if n % 100 == 0 {
        String::from("Not a Leap Year")
    } else if n % 4 == 0 {
        String::from("Leap Year")
    } else {
        String::from("Not a Leap Year")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        let years = vec![2000, 1900, 2004];
        let res: Vec<String> = years.into_iter().map(|year| is_leap_year(year)).collect();
        let expected = ["Leap Year", "Not a Leap Year", "Leap Year"];
        assert_eq!(res, expected)
    }
}
