/* Write a function to identify the "peak" values
in an array. A peak is any element that is strictly
greater than its immediate neighbors to the left
and right. The first and last elements cannot be
peaks

_Example Input and Output_
input = [1, 3, 2, 4, 1, 5, 1]
output = [3, 4, 5]
*/
pub fn find_peaks(array: &[i32]) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    // array must have length > 2
    if array.len() < 2 {
        return vec![];
    } else {
        // Only start from the second element
        // and stop at penultimate element
        for i in 1..array.len() - 1 {
            // if elem i is higher than its previous and
            // its subsequent element, then it's a peak
            if array[i] > array[i - 1] && array[i] > array[i + 1] {
                res.push(array[i]);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_peaks_1() {
        let heights: &[i32; 7] = &[1, 3, 2, 4, 1, 5, 1];
        let expected: Vec<i32> = vec![3, 4, 5];
        assert_eq!(find_peaks(heights), expected);
    }

    #[test]
    fn test_find_peaks_2() {
        let heights: &[i32; 7] = &[10, 20, 15, 25, 30, 20, 10];
        let expected: Vec<i32> = vec![20, 30];
        assert_eq!(find_peaks(heights), expected);
    }

    #[test]
    fn test_find_peaks_3() {
        let heights: &[i32; 5] = &[1, 1, 1, 1, 1];
        let expected: Vec<i32> = vec![];
        assert_eq!(find_peaks(heights), expected);
    }
}
