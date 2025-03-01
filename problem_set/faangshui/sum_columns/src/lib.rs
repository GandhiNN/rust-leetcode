/* Write a function that computes the sum of
each column in a 2D list of numbers and returns
the sums as a list.

Example input:
matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]
output = [12, 15, 18]

explanation:
12 = 1 + 4 + 7
15 = 2 + 5 + 8
18 = 3 + 6 + 9

*/
pub fn sum_columns(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    // Get the max len of inner elements of the matrix
    let mut cols_sum: Vec<i32> = vec![];
    let mut max_len_inner = 0;
    for inner in matrix {
        if inner.len() > max_len_inner {
            max_len_inner = inner.len();
        }
    }
    // loop again the matrix
    for i in 0..max_len_inner {
        let mut sum = 0;
        for elem in matrix {
            sum += elem[i];
        }
        cols_sum.push(sum);
    }
    cols_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_columns_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let sum = vec![12, 15, 18];
        assert_eq!(sum_columns(&matrix), sum);
    }

    #[test]
    fn test_sum_columns_2() {
        let matrix = vec![vec![10, 20], vec![30, 40], vec![50, 60]];
        let sum = vec![90, 120];
        assert_eq!(sum_columns(&matrix), sum);
    }

    #[test]
    fn test_sum_columns_3() {
        let matrix = vec![vec![5], vec![10], vec![15]];
        let sum = vec![30];
        assert_eq!(sum_columns(&matrix), sum);
    }

    #[test]
    fn test_sum_columns_4() {
        let matrix = vec![];
        let sum = vec![];
        assert_eq!(sum_columns(&matrix), sum);
    }

    #[test]
    fn test_sum_columns_5() {
        let matrix = vec![vec![2, 4, 6], vec![1, 3, 5]];
        let sum = vec![3, 7, 11];
        assert_eq!(sum_columns(&matrix), sum);
    }
}
