/* Write a function to find the index of this "balance point"
on the iterator, where the sum of items on the left is equal
to the sum on the right. If no such point exists, return -1.
If the iterator contains only one item, return 0.

(1) Example input:
[1, 7, 3, 6, 5, 6]

(1) Example output:
3

(2) Example input:
[5]

(2) Example output:
0

Explanation: The balance point is at index 3, where the sum
on both sides is 11 => [1, 7, 3].sum() == [5, 6].sum()
*/
pub fn find_balance_point(shelves: &[i32]) -> i32 {
    // need to start at index 1 and end at index n-1
    // because the first and last elements can't be the balance point
    if shelves.len() == 1 {
        return 0;
    }
    for i in 1..shelves.len() - 1 {
        let left_sum = shelves[..i].iter().sum::<i32>();
        let right_sum = shelves[i + 1..].iter().sum::<i32>();
        if left_sum == right_sum {
            return i as i32;
        }
    }
    -1 // cannot find the balance point
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_balance_point() {
        let shelves = vec![1, 7, 3, 6, 5, 6];
        let expected = 3;
        assert_eq!(find_balance_point(&shelves), expected);
    }

    #[test]
    fn test_find_balance_point_failed() {
        let shelves = vec![1, 2, 3, 6, 5, 6];
        let expected = -1;
        assert_eq!(find_balance_point(&shelves), expected);
    }
}
