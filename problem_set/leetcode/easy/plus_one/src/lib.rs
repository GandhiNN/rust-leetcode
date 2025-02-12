/* You are given a large integer represneted as an
integer array `digits` where each `digits[i]` is
the ith digit of the integer. The digits are ordered
from most significant to least significant in left-to-right
order. The large integer does not contain any leading 0's

Increment the large integer by one and return the resulting
array of digits.
*/

// This one does not work with a very large number
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    // store concatenated digits as i128 to handle very large numbers
    let mut digits_concat: i128 = 0;
    for digit in digits {
        digits_concat *= 10;
        digits_concat += digit as i128;
    }
    // plus one phase
    digits_concat += 1;
    while digits_concat > 9 {
        result.push((digits_concat % 10) as i32); // cast back to i32
        digits_concat /= 10;
    }
    result.push(digits_concat as i32); // cast back to i32
    result.reverse();
    result
}

pub fn plus_one_v2(digits: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = digits.clone();
    for (idx, val) in result.clone().iter().enumerate().rev() {
        // reverse() does not mutate the original vector
        if *val < 9 {
            result[idx] = *val + 1;
            break;
        } else if *val == 9 {
            result[idx] = 0;
            if idx == 0 {
                // if the first digit is 9, insert 1 at the beginning of the vector
                result.insert(0, 1);
            }
        }
    }
    result
}

pub fn plus_one_v3(digits: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = digits.clone();
    for (idx, val) in result.clone().iter().enumerate().rev() {
        match val.cmp(&9) {
            std::cmp::Ordering::Less => {
                result[idx] = *val + 1;
                break;
            }
            std::cmp::Ordering::Equal => {
                result[idx] = 0;
                if idx == 0 {
                    // handle case if all digits are 9
                    // insert 1 at the beginning of the vector
                    result.insert(0, 1);
                }
            }
            std::cmp::Ordering::Greater => (), // do nothing
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one_1() {
        let digits = vec![1, 2, 3];
        let expected = vec![1, 2, 4];
        assert_eq!(plus_one_v3(digits), expected);
    }

    #[test]
    fn test_plus_one_2() {
        let digits = vec![4, 3, 2, 1];
        let expected = vec![4, 3, 2, 2];
        assert_eq!(plus_one_v3(digits), expected);
    }

    #[test]
    fn test_plus_one_3() {
        let digits = vec![9];
        let expected = vec![1, 0];
        assert_eq!(plus_one_v3(digits), expected);
    }

    #[test]
    fn test_plus_one_4() {
        let digits = vec![1, 9];
        let expected = vec![2, 0];
        assert_eq!(plus_one_v3(digits), expected);
    }

    #[test]
    fn test_plus_one_5() {
        let digits = vec![9, 9];
        let expected = vec![1, 0, 0];
        assert_eq!(plus_one_v3(digits), expected);
    }

    #[test]
    fn test_plus_one_6() {
        let digits = vec![1, 9, 7];
        let expected = vec![1, 9, 8];
        assert_eq!(plus_one_v3(digits), expected);
    }

    #[test]
    fn test_plus_one_7() {
        let digits = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let expected = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1];
        assert_eq!(plus_one_v3(digits), expected);
    }

    #[test]
    fn test_plus_one_8() {
        let digits = vec![
            7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4,
            7, 0, 1, 1, 1, 7, 4, 0, 0, 6,
        ];
        let expected = vec![
            7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4,
            7, 0, 1, 1, 1, 7, 4, 0, 0, 7,
        ];
        assert_eq!(plus_one_v3(digits), expected);
    }
}
