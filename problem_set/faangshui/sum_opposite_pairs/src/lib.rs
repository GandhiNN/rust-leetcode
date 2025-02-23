/* Write a function that, given an array of even size N,
returns a list of N/2 sums. Each sum is the addition of an
element and its corresponding element from the opposite end
of the array.

_Example Input and Output_
input = [1, 2, 3, 4, 5, 6]
output = [7, 7, 7]

Explanation:
[(1+6), (2+5), (3+4)]
*/
pub fn sum_opposite_pairs(array: &[i32]) -> Vec<i32> {
    // define result
    let mut res: Vec<i32> = vec![];
    // input list must be even
    if array.len() % 2 != 0 {
        return vec![];
    } else {
        for i in 0..array.len() - 1 {
            res.push(array[i] + array[array.len() - 1 - i]);
            if i == array.len() / 2 - 1 {
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_opposite_pairs() {
        let weights: &[i32; 6] = &[1, 2, 3, 4, 5, 6];
        let expected: Vec<i32> = vec![7, 7, 7];
        assert_eq!(sum_opposite_pairs(weights), expected);
    }
}
