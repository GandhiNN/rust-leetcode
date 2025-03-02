/* Write a function that returns the
index of the minimum value in an array
of weights. If there are multiple loads of
the same minimum weight, return the index
of the first one.

Example input and output:
input = [11, 3, 5, 7, 2, 8, 10]
output = 4
explanation = '2' is the minimum weight in the array
and its index is 4 (zero-indexed)
*/
pub fn min_index(weights: Vec<i32>) -> i32 {
    let mut min_idx = 0;
    let mut smallest = 0;
    for (idx, val) in weights.iter().enumerate() {
        if idx == 0 {
            smallest = *val;
        } else {
            if *val < smallest {
                smallest = *val;
                min_idx = idx;
            }
        }
    }
    min_idx as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_index() {
        let weights = vec![11, 3, 5, 7, 2, 8, 10];
        let expected = 4;
        assert_eq!(min_index(weights), expected);
    }
}
