/* Write a function to calculate and print the
sum of all the elements in a 2D list (matrix).
Use nested loops to traverse the rows and columns.

Example input and output:
matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]
output = 45

Explanation
1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 = 45
*/
pub fn matrix_sum(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for outer in matrix {
        for inner in outer {
            sum += inner;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_sum_1() {
        let matrix = &vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = 45;
        assert_eq!(matrix_sum(matrix), expected);
    }

    #[test]
    fn test_matrix_sum_2() {
        let matrix = &vec![vec![0, 2], vec![1, 2, 3], vec![9, 5, 2]];
        let expected = 24;
        assert_eq!(matrix_sum(matrix), expected);
    }
}
