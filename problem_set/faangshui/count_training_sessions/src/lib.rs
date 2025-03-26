/* Write a function to count the number of training
sessions (non-zero values) in a 2D array representing
the weekly schedule (7 days x 3 time slots)

Example Input:
schedule = [
    [1, 0, 2],
    [0, 3, 0],
    [4, 0, 0],
    [1, 1, 1],
    [0, 0, 0],
    [5, 2, 0],
    [0, 0, 3]
]

Example Output:
10

Explanation:
From the 2D array, the non-zero elements
is counted as 1 training day: We just have
to count the occurrences of them, which in this
case = 10 non-zero elements (1, 2, 3, 4, 1, 1, 1, 5, 2, 3)

*/
pub fn count_training_sessions(schedule: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for outer in schedule {
        for inner in outer {
            if *inner > 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_training_sessions() {
        let schedule: Vec<Vec<i32>> = vec![
            vec![1, 0, 2],
            vec![0, 3, 0],
            vec![4, 0, 0],
            vec![1, 1, 1],
            vec![0, 0, 0],
            vec![5, 2, 0],
            vec![0, 0, 3],
        ];
        let result = count_training_sessions(&schedule);
        assert_eq!(result, 10);
    }
}
