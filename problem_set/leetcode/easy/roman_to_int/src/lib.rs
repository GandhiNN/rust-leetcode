#![allow(clippy::redundant_closure)]
use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let roman_map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut res = 0;
    for (idx, val) in s.chars().enumerate() {
        if idx != 0 {
            if (val == 'V' || val == 'X') && (s.chars().nth(idx - 1).unwrap_or_default() == 'I') {
                res += roman_map.get(&val).unwrap() - 2;
            } else if (val == 'L' || val == 'C')
                && (s.chars().nth(idx - 1).unwrap_or_default() == 'X')
            {
                res += roman_map.get(&val).unwrap() - 20;
            } else if (val == 'D' || val == 'M')
                && (s.chars().nth(idx - 1).unwrap_or_default() == 'C')
            {
                res += roman_map.get(&val).unwrap() - 200;
            } else {
                res += roman_map.get(&val).unwrap();
            }
        } else {
            res += roman_map.get(&val).unwrap();
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let inputs: Vec<String> = vec![
            "III".to_string(),
            "IV".to_string(),
            "IX".to_string(),
            "LVIII".to_string(),
            "MCMXCIV".to_string(),
        ];
        let expected: Vec<i32> = vec![3, 4, 9, 58, 1994];
        let res: Vec<i32> = inputs.into_iter().map(|i| roman_to_int(i)).collect();
        assert_eq!(res, expected);
    }
}
